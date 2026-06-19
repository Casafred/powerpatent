pub mod paddle;
pub mod glm;
pub mod label_filter;

use anyhow::Result;

/// OCR 引擎类型
#[derive(Debug, Clone)]
pub enum OcrEngine {
    PaddleOcrVl,
    GlmOcr,
}

/// 统一 OCR 结果
#[derive(Debug, Clone, serde::Serialize)]
pub struct UnifiedOcrResult {
    pub text: String,
    pub markdown: Option<String>,
    pub layout: Option<serde_json::Value>,
    pub engine: String,
}

/// 对 PDF 执行 OCR（根据引擎类型选择实现）
pub async fn ocr_pdf(pdf_path: &str, engine: &OcrEngine, glm_config: Option<GlmOcrConfig>) -> Result<UnifiedOcrResult> {
    match engine {
        OcrEngine::PaddleOcrVl => {
            let result = paddle::ocr_pdf_with_paddle(pdf_path).await?;
            Ok(UnifiedOcrResult {
                text: result.text,
                markdown: result.markdown,
                layout: result.layout,
                engine: "paddle_ocr_vl".to_string(),
            })
        }
        OcrEngine::GlmOcr => {
            let config = glm_config.ok_or_else(|| anyhow::anyhow!("GLM OCR 需要提供 API 配置"))?;
            // GLM OCR 需要先转图片，这里简化处理：读取 PDF base64
            let pdf_bytes = std::fs::read(pdf_path)?;
            let pdf_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &pdf_bytes);
            let result = glm::ocr_with_glm(&config.api_key, &config.base_url, &config.model, &pdf_b64).await?;
            Ok(UnifiedOcrResult {
                text: result.text,
                markdown: result.markdown,
                layout: None,
                engine: "glm_ocr".to_string(),
            })
        }
    }
}

/// GLM OCR 配置
#[derive(Debug, Clone)]
pub struct GlmOcrConfig {
    pub api_key: String,
    pub base_url: String,
    pub model: String,
}
