use anyhow::Result;
use serde::Serialize;

use crate::ai::client::AIClient;
use crate::ai::models::{AIClientConfig, AIProvider, ChatMessage};

/// GLM OCR 结果
#[derive(Debug, Serialize, Clone)]
pub struct GlmOcrResult {
    pub text: String,
    pub markdown: Option<String>,
}

/// 使用 GLM-4V 进行 OCR
/// 需要智谱 API Key，通过视觉模型识别 PDF/图片内容
pub async fn ocr_with_glm(
    api_key: &str,
    base_url: &str,
    model: &str,
    image_base64: &str,
) -> Result<GlmOcrResult> {
    let config = AIClientConfig {
        provider: AIProvider::Zhipu,
        api_key: api_key.to_string(),
        base_url: base_url.to_string(),
        model: model.to_string(),
    };

    let client = AIClient::new(config);

    // 构建多模态消息
    let content = serde_json::json!([
        {
            "type": "text",
            "text": "请识别并提取这张图片中的所有文字内容，保持原始格式和结构。如果有表格，请用 Markdown 表格格式输出。"
        },
        {
            "type": "image_url",
            "image_url": {
                "url": format!("data:image/png;base64,{}", image_base64)
            }
        }
    ]);

    let messages = vec![ChatMessage {
        role: "user".to_string(),
        content: content.to_string(),
    }];

    let result = client.chat_text(messages, 0.1).await?;

    Ok(GlmOcrResult {
        text: result.clone(),
        markdown: Some(result),
    })
}
