use crate::ai::models::{AIClientConfig, AIProvider, ChatMessage, ChatRequest, ChatResponse};
use anyhow::Result;
use futures::StreamExt;

/// 根据服务商类型返回 API 路径前缀
fn api_path_prefix(provider: &AIProvider) -> &'static str {
    match provider {
        AIProvider::Zhipu => "/v4",
        _ => "/v1",
    }
}

/// AI 客户端
pub struct AIClient {
    config: AIClientConfig,
    http: reqwest::Client,
}

impl AIClient {
    pub fn new(config: AIClientConfig) -> Self {
        let http = crate::ai::proxy::create_http_client();
        Self { config, http }
    }

    /// 非流式调用，等待完整 JSON 返回
    pub async fn chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<ChatResponse> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(temperature),
            stream: Some(false),
        };

        let prefix = api_path_prefix(&self.config.provider);
        let url = format!("{}{}/chat/completions", self.config.base_url.trim_end_matches('/'), prefix);

        let response = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("AI 请求失败: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("AI 请求失败 ({}): {}", status, body);
        }

        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| anyhow::anyhow!("AI 响应解析失败: {}", e))?;

        Ok(chat_response)
    }

    /// SSE 流式调用，逐 token 返回
    pub async fn stream_chat(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<tokio::sync::mpsc::Receiver<Result<String>>> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: Some(temperature),
            stream: Some(true),
        };

        let prefix = api_path_prefix(&self.config.provider);
        let url = format!("{}{}/chat/completions", self.config.base_url.trim_end_matches('/'), prefix);

        let response = self.http
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| anyhow::anyhow!("AI 流式请求失败: {}", e))?;

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            anyhow::bail!("AI 流式请求失败 ({}): {}", status, body);
        }

        let (tx, rx) = tokio::sync::mpsc::channel(100);

        tokio::spawn(async move {
            let mut stream = response.bytes_stream();
            let mut buffer = String::new();

            while let Some(chunk_result) = stream.next().await {
                match chunk_result {
                    Ok(chunk) => {
                        let text = String::from_utf8_lossy(&chunk);
                        buffer.push_str(&text);

                        // 处理 SSE 行
                        while let Some(pos) = buffer.find('\n') {
                            let line = buffer[..pos].trim().to_string();
                            buffer = buffer[pos + 1..].to_string();

                            if line.starts_with("data: ") {
                                let data = &line[6..];
                                if data == "[DONE]" {
                                    let _ = tx.send(Ok("[DONE]".to_string())).await;
                                    return;
                                }
                                if let Ok(parsed) = serde_json::from_value::<serde_json::Value>(
                                    serde_json::from_str::<serde_json::Value>(data).unwrap_or_default()
                                ) {
                                    if let Some(choices) = parsed.get("choices").and_then(|c| c.as_array()) {
                                        for choice in choices {
                                            if let Some(delta) = choice.get("delta") {
                                                if let Some(content) = delta.get("content").and_then(|c| c.as_str()) {
                                                    let _ = tx.send(Ok(content.to_string())).await;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        let _ = tx.send(Err(anyhow::anyhow!("流式读取错误: {}", e))).await;
                        return;
                    }
                }
            }
        });

        Ok(rx)
    }

    /// 简化的非流式调用，直接返回文本内容
    pub async fn chat_text(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<String> {
        let response = self.chat(messages, temperature).await?;
        if let Some(choice) = response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            anyhow::bail!("AI 响应无内容")
        }
    }

    /// 调用 AI 并解析 JSON 输出
    pub async fn chat_json(
        &self,
        messages: Vec<ChatMessage>,
        temperature: f32,
    ) -> Result<serde_json::Value> {
        let text = self.chat_text(messages, temperature).await?;
        // 尝试从文本中提取 JSON（可能被 markdown 代码块包裹）
        let json_str = extract_json_from_text(&text);
        let value: serde_json::Value = serde_json::from_str(&json_str)
            .map_err(|e| anyhow::anyhow!("AI 输出 JSON 解析失败: {}\n原始文本: {}", e, &text[..text.len().min(500)]))?;
        Ok(value)
    }

    pub fn config(&self) -> &AIClientConfig {
        &self.config
    }
}

/// 从 AI 返回文本中提取 JSON（处理 markdown 代码块包裹的情况）
fn extract_json_from_text(text: &str) -> String {
    let trimmed = text.trim();

    // 如果直接是 JSON
    if trimmed.starts_with('{') || trimmed.starts_with('[') {
        return trimmed.to_string();
    }

    // 尝试从 markdown 代码块中提取
    if let Some(start) = trimmed.find("```json") {
        let json_start = start + 7;
        if let Some(end) = trimmed[json_start..].find("```") {
            return trimmed[json_start..json_start + end].trim().to_string();
        }
    }

    // 尝试从普通代码块中提取
    if let Some(start) = trimmed.find("```") {
        let json_start = start + 3;
        // 跳过可能的语言标识行
        let json_start = trimmed[json_start..].find('\n').map(|pos| json_start + pos + 1).unwrap_or(json_start);
        if let Some(end) = trimmed[json_start..].find("```") {
            return trimmed[json_start..json_start + end].trim().to_string();
        }
    }

    // 尝试找到第一个 { 或 [
    if let Some(start) = trimmed.find('{').or_else(|| trimmed.find('[')) {
        let open_char = trimmed.chars().nth(start).unwrap();
        let close_char = if open_char == '{' { '}' } else { ']' };
        // 从末尾找匹配的关闭字符
        if let Some(end) = trimmed.rfind(close_char) {
            if end > start {
                return trimmed[start..=end].to_string();
            }
        }
    }

    trimmed.to_string()
}
