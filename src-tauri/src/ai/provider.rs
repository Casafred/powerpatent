use crate::ai::models::{AIProvider, AIClientConfig};

/// Provider 预设配置
pub struct ProviderPreset {
    pub base_url: &'static str,
    pub strong_model: &'static str,
    pub fast_model: &'static str,
}

/// 获取 Provider 预设
pub fn get_preset(provider: &AIProvider) -> ProviderPreset {
    match provider {
        AIProvider::DeepSeek => ProviderPreset {
            base_url: "https://api.deepseek.com",
            strong_model: "deepseek-chat",
            fast_model: "deepseek-v4-flash",
        },
        AIProvider::Zhipu => ProviderPreset {
            base_url: "https://open.bigmodel.cn/api/paas",
            strong_model: "glm-4-plus",
            fast_model: "glm-4-flash",
        },
        AIProvider::OpenAI => ProviderPreset {
            base_url: "https://api.openai.com",
            strong_model: "gpt-4o",
            fast_model: "gpt-4o-mini",
        },
    }
}

/// 从前端配置创建 AIClientConfig
pub fn config_from_json(provider_json: &serde_json::Value) -> Result<AIClientConfig, String> {
    let provider_type = provider_json.get("type")
        .and_then(|v| v.as_str())
        .unwrap_or("deepseek");

    let provider = match provider_type {
        "deepseek" => AIProvider::DeepSeek,
        "zhipu" => AIProvider::Zhipu,
        "openai" => AIProvider::OpenAI,
        _ => return Err(format!("未知的 AI Provider: {}", provider_type)),
    };

    let preset = get_preset(&provider);

    let api_key = provider_json.get("apiKey")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let base_url = provider_json.get("baseUrl")
        .and_then(|v| v.as_str())
        .unwrap_or(preset.base_url)
        .to_string();

    let model = provider_json.get("model")
        .and_then(|v| v.as_str())
        .unwrap_or(preset.strong_model)
        .to_string();

    Ok(AIClientConfig {
        provider,
        api_key,
        base_url,
        model,
    })
}
