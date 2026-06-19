use anyhow::Result;
use base64::Engine;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;

/// PaddleOCR-VL API 响应
#[derive(Debug, Deserialize)]
struct PaddleOcrResponse {
    code: Option<i64>,
    message: Option<String>,
    data: Option<PaddleOcrData>,
}

#[derive(Debug, Deserialize)]
struct PaddleOcrData {
    #[serde(default)]
    content: Option<String>,
    #[serde(default)]
    layout: Option<serde_json::Value>,
    #[serde(default)]
    markdown: Option<String>,
}

/// OCR 结果
#[derive(Debug, Serialize, Clone)]
pub struct OcrResult {
    pub text: String,
    pub markdown: Option<String>,
    pub layout: Option<serde_json::Value>,
}

/// PaddleOCR-VL 在线 API 客户端
/// 参考: https://github.com/Casafred/history-helper
pub struct PaddleOcrClient {
    client: Client,
    api_url: String,
}

impl PaddleOcrClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(std::time::Duration::from_secs(120))
                .build()
                .unwrap_or_else(|_| Client::new()),
            api_url: "https://paddle-ocr-vl.aip.baidubce.com/api/v1/ocr".to_string(),
        }
    }

    /// 使用自定义 API URL
    pub fn with_url(mut self, url: &str) -> Self {
        self.api_url = url.to_string();
        self
    }

    /// 对 PDF 文件执行 OCR
    /// 将 PDF 逐页转为图片后调用 PaddleOCR-VL API
    pub async fn ocr_pdf(&self, pdf_path: &str) -> Result<OcrResult> {
        let path = Path::new(pdf_path);
        if !path.exists() {
            anyhow::bail!("PDF 文件不存在: {}", pdf_path);
        }

        // 读取 PDF 文件并 base64 编码
        let pdf_bytes = std::fs::read(pdf_path)?;
        let pdf_b64 = base64::engine::general_purpose::STANDARD.encode(&pdf_bytes);

        // 调用 PaddleOCR-VL API
        self.call_api(&pdf_b64, "pdf").await
    }

    /// 对图片文件执行 OCR
    pub async fn ocr_image(&self, image_path: &str) -> Result<OcrResult> {
        let path = Path::new(image_path);
        if !path.exists() {
            anyhow::bail!("图片文件不存在: {}", image_path);
        }

        let image_bytes = std::fs::read(image_path)?;
        let image_b64 = base64::engine::general_purpose::STANDARD.encode(&image_bytes);

        self.call_api(&image_b64, "image").await
    }

    /// 对 base64 编码的数据调用 API
    async fn call_api(&self, base64_data: &str, file_type: &str) -> Result<OcrResult> {
        let body = serde_json::json!({
            "file": base64_data,
            "fileType": file_type,
            "options": {
                "lang": "ch",
                "useLayoutAnalysis": true,
                "useTableRecognition": true,
            }
        });

        let response = self.client
            .post(&self.api_url)
            .header("Content-Type", "application/json")
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("PaddleOCR API 请求失败: HTTP {} - {}", status, text);
        }

        let result: PaddleOcrResponse = response.json().await?;

        if result.code.map(|c| c != 0).unwrap_or(false) {
            anyhow::bail!(
                "PaddleOCR API 返回错误: {}",
                result.message.unwrap_or_else(|| "未知错误".to_string())
            );
        }

        let data = result.data.unwrap_or(PaddleOcrData {
            content: None,
            layout: None,
            markdown: None,
        });

        Ok(OcrResult {
            text: data.content.unwrap_or_default(),
            markdown: data.markdown,
            layout: data.layout,
        })
    }
}

/// 使用 PaddleOCR 对 PDF 执行 OCR 并提取文本
pub async fn ocr_pdf_with_paddle(pdf_path: &str) -> Result<OcrResult> {
    let client = PaddleOcrClient::new();
    client.ocr_pdf(pdf_path).await
}

/// 使用 PaddleOCR 对图片执行 OCR
pub async fn ocr_image_with_paddle(image_path: &str) -> Result<OcrResult> {
    let client = PaddleOcrClient::new();
    client.ocr_image(image_path).await
}
