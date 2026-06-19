use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::time::Duration;

/// PaddleOCR-V2 API 常量
const PADDLE_OCR_V2_URL: &str = "https://paddleocr.aistudio-app.com/api/v2/ocr/jobs";
const PADDLE_OCR_V2_TOKEN: &str = "70b270c8275606a7a97f8c4e8617cdeb935ed74c";
const PADDLE_OCR_V2_MODEL: &str = "PaddleOCR-VL-1.6";
const PADDLE_OCR_V2_POLL_INTERVAL_SECS: u64 = 5;
const PADDLE_OCR_V2_POLL_TIMEOUT_SECS: u64 = 300;

/// OCR 结果
#[derive(Debug, Serialize, Clone)]
pub struct OcrResult {
    pub text: String,
    pub markdown: Option<String>,
    pub layout: Option<serde_json::Value>,
}

/// PaddleOCR-VL 在线 API 客户端
/// 使用异步任务模式：提交 → 轮询 → 获取 JSONL 结果
/// 参考: https://github.com/Casafred/history-helper
pub struct PaddleOcrClient {
    client: Client,
}

impl PaddleOcrClient {
    pub fn new() -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(180))
                .build()
                .unwrap_or_else(|_| Client::new()),
        }
    }

    /// 对 PDF 文件执行 OCR
    pub async fn ocr_pdf(&self, pdf_path: &str) -> Result<OcrResult> {
        let path = Path::new(pdf_path);
        if !path.exists() {
            anyhow::bail!("PDF 文件不存在: {}", pdf_path);
        }

        let pdf_bytes = std::fs::read(pdf_path)?;
        self.extract_with_paddle_vl(&pdf_bytes).await
    }

    /// 对图片文件执行 OCR
    pub async fn ocr_image(&self, image_path: &str) -> Result<OcrResult> {
        let path = Path::new(image_path);
        if !path.exists() {
            anyhow::bail!("图片文件不存在: {}", image_path);
        }

        let image_bytes = std::fs::read(image_path)?;
        self.extract_with_paddle_vl(&image_bytes).await
    }

    /// PaddleOCR-VL 核心流程：提交任务 → 轮询状态 → 获取结果
    async fn extract_with_paddle_vl(&self, file_bytes: &[u8]) -> Result<OcrResult> {
        // Step 1: 提交 OCR 任务（multipart 上传）
        let optional_payload = serde_json::json!({
            "useDocOrientationClassify": true,
            "useDocUnwarping": false,
            "useChartRecognition": false,
        });

        let form = reqwest::multipart::Form::new()
            .text("model", PADDLE_OCR_V2_MODEL.to_string())
            .text("optionalPayload", optional_payload.to_string())
            .part(
                "file",
                reqwest::multipart::Part::bytes(file_bytes.to_vec())
                    .file_name("document.pdf")
                    .mime_str("application/pdf")
                    .unwrap_or_else(|_| reqwest::multipart::Part::bytes(vec![]).file_name("document.pdf")),
            );

        let submit_resp = self
            .client
            .post(PADDLE_OCR_V2_URL)
            .header("Authorization", format!("bearer {}", PADDLE_OCR_V2_TOKEN))
            .multipart(form)
            .timeout(Duration::from_secs(30))
            .send()
            .await?;

        let submit_status = submit_resp.status();
        if !submit_status.is_success() {
            let body = submit_resp.text().await.unwrap_or_default();
            anyhow::bail!("PaddleOCR 提交失败: HTTP {} - {}", submit_status, &body[..body.len().min(300)]);
        }

        let submit_data: serde_json::Value = submit_resp.json().await?;
        let job_id = submit_data
            .get("data")
            .and_then(|d| d.get("jobId"))
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string();

        if job_id.is_empty() {
            anyhow::bail!("PaddleOCR 提交成功但未返回 jobId");
        }

        log::info!("PaddleOCR 任务已提交: {}", job_id);

        // Step 2: 轮询任务状态
        let poll_client = Client::builder()
            .timeout(Duration::from_secs(15))
            .build()
            .unwrap_or_default();

        let start = std::time::Instant::now();
        let mut jsonl_url = String::new();

        loop {
            if start.elapsed().as_secs() > PADDLE_OCR_V2_POLL_TIMEOUT_SECS {
                anyhow::bail!("PaddleOCR 轮询超时 ({}s)", PADDLE_OCR_V2_POLL_TIMEOUT_SECS);
            }

            let poll_resp = match poll_client
                .get(format!("{}/{}", PADDLE_OCR_V2_URL, job_id))
                .header("Authorization", format!("bearer {}", PADDLE_OCR_V2_TOKEN))
                .send()
                .await
            {
                Ok(r) => r,
                Err(e) => {
                    log::warn!("PaddleOCR 轮询请求失败: {}, 重试中...", e);
                    tokio::time::sleep(Duration::from_secs(PADDLE_OCR_V2_POLL_INTERVAL_SECS)).await;
                    continue;
                }
            };

            let poll_data: serde_json::Value = match poll_resp.json().await {
                Ok(d) => d,
                Err(e) => {
                    log::warn!("PaddleOCR 轮询 JSON 解析失败: {}, 重试中...", e);
                    tokio::time::sleep(Duration::from_secs(PADDLE_OCR_V2_POLL_INTERVAL_SECS)).await;
                    continue;
                }
            };

            let d = poll_data.get("data").cloned().unwrap_or_default();
            let state = d.get("state").and_then(|v| v.as_str()).unwrap_or("");

            match state {
                "done" => {
                    jsonl_url = d
                        .get("resultUrl")
                        .and_then(|r| r.get("jsonUrl"))
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    if jsonl_url.is_empty() {
                        anyhow::bail!("PaddleOCR 任务完成但未返回 jsonUrl");
                    }
                    log::info!("PaddleOCR 任务完成，获取结果");
                    break;
                }
                "failed" => {
                    let error_msg = d.get("errorMsg").and_then(|v| v.as_str()).unwrap_or("未知错误");
                    anyhow::bail!("PaddleOCR 任务失败: {}", error_msg);
                }
                "running" => {
                    if let Some(prog) = d.get("extractProgress") {
                        let extracted = prog.get("extractedPages").and_then(|v| v.as_u64()).unwrap_or(0);
                        let total = prog.get("totalPages").and_then(|v| v.as_u64()).unwrap_or(0);
                        log::info!("PaddleOCR 处理中: {}/{}", extracted, total);
                    }
                }
                _ => {
                    log::info!("PaddleOCR 状态: {}", state);
                }
            }

            tokio::time::sleep(Duration::from_secs(PADDLE_OCR_V2_POLL_INTERVAL_SECS)).await;
        }

        // Step 3: 获取 JSONL 结果
        let jsonl_resp = self
            .client
            .get(&jsonl_url)
            .timeout(Duration::from_secs(60))
            .send()
            .await?;

        let jsonl_text = jsonl_resp.text().await?;

        // 解析 JSONL
        let mut all_markdown = Vec::new();
        let mut all_text = Vec::new();

        for line in jsonl_text.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }

            let parsed: serde_json::Value = match serde_json::from_str(line) {
                Ok(v) => v,
                Err(_) => continue,
            };

            let results = parsed
                .get("result")
                .and_then(|r| r.get("layoutParsingResults"))
                .and_then(|v| v.as_array())
                .cloned()
                .unwrap_or_default();

            for page_result in &results {
                // 提取 markdown
                if let Some(md_text) = page_result
                    .get("markdown")
                    .and_then(|m| m.get("text"))
                    .and_then(|t| t.as_str())
                {
                    if !md_text.is_empty() {
                        all_markdown.push(md_text.to_string());
                    }
                }

                // 提取文本块
                let pruned = page_result.get("prunedResult").cloned().unwrap_or_default();
                let parsing_list = pruned
                    .get("parsing_res_list")
                    .and_then(|v| v.as_array())
                    .cloned()
                    .unwrap_or_default();

                for block in &parsing_list {
                    let content = block
                        .get("block_content")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();
                    let label = block
                        .get("block_label")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    let is_text_label = ["text", "title", "table", "formula"].contains(&label.as_str());
                    if !content.is_empty() && is_text_label {
                        all_text.push(content);
                    }
                }
            }
        }

        let markdown = all_markdown.join("\n\n---\n\n");
        let text = all_text.join("\n");

        log::info!("PaddleOCR 结果: markdown={} chars, text={} chars", markdown.len(), text.len());

        Ok(OcrResult {
            text,
            markdown: if markdown.is_empty() { None } else { Some(markdown) },
            layout: None,
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
