use serde::{Deserialize, Serialize};

/// AI Provider 类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AIProvider {
    OpenAI,
    Zhipu,
    DeepSeek,
}

/// AI 客户端配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AIClientConfig {
    pub provider: AIProvider,
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}

/// 聊天消息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

impl ChatMessage {
    pub fn system(content: impl Into<String>) -> Self {
        Self { role: "system".into(), content: content.into() }
    }
    pub fn user(content: impl Into<String>) -> Self {
        Self { role: "user".into(), content: content.into() }
    }
    pub fn assistant(content: impl Into<String>) -> Self {
        Self { role: "assistant".into(), content: content.into() }
    }
}

/// AI 调用请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream: Option<bool>,
}

/// AI 调用响应（非流式）
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatResponse {
    pub id: Option<String>,
    pub choices: Vec<ChatChoice>,
    pub usage: Option<ChatUsage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatChoice {
    pub index: i32,
    pub message: ChatMessage,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatUsage {
    pub prompt_tokens: Option<i32>,
    pub completion_tokens: Option<i32>,
    pub total_tokens: Option<i32>,
}

/// SSE 流式响应的 delta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamDelta {
    pub content: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamChoice {
    pub index: i32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

/// 板块生成结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleOutput {
    pub module_id: String,
    pub patent_id: String,
    pub level: String,
    pub output_json: serde_json::Value,
    pub model: String,
    pub provider: String,
    pub confidence: Option<f32>,
    pub source_refs: Vec<String>,
    pub created_at: String,
}
