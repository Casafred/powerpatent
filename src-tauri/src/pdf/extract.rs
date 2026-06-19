use anyhow::Result;
use std::path::Path;

/// 从 PDF 文件抽取全部文本内容
pub fn extract_text(pdf_path: &str) -> Result<String> {
    let path = Path::new(pdf_path);
    if !path.exists() {
        anyhow::bail!("PDF 文件不存在: {}", pdf_path);
    }

    let bytes = std::fs::read(pdf_path)
        .map_err(|e| anyhow::anyhow!("读取 PDF 文件失败: {}", e))?;

    let text = pdf_extract::extract_text_from_mem(&bytes)
        .map_err(|e| anyhow::anyhow!("PDF 文本抽取失败: {}", e))?;

    Ok(text)
}

/// 从 PDF 文本中尝试提取专利元信息
pub fn extract_metadata(text: &str) -> PdfExtractResult {
    let mut result = PdfExtractResult::default();

    // 尝试提取公开号
    if let Some(val) = extract_field(text, &[
        r"公开号[：:\s]*(CN\d+[A-Z]?)",
        r"Publication\s+Number[：:\s]*(CN\d+[A-Z]?)",
    ]) {
        result.publication_number = Some(val);
    }

    // 尝试提取申请号
    if let Some(val) = extract_field(text, &[
        r"申请号[：:\s]*(\d+[\.\d]*)",
        r"Application\s+Number[：:\s]*(\d+[\.\d]*)",
    ]) {
        result.application_number = Some(val);
    }

    // 尝试提取申请人
    if let Some(val) = extract_field(text, &[
        r"申请人[：:\s]*([^\n\r,，]+?)[\n\r,，]",
        r"Assignee[：:\s]*([^\n\r]+?)[\n\r]",
    ]) {
        result.applicant = Some(val.trim().to_string());
    }

    // 尝试提取发明名称
    if let Some(val) = extract_field(text, &[
        r"发明名称[：:\s]*([^\n\r]+?)[\n\r]",
        r"Title[：:\s]*([^\n\r]+?)[\n\r]",
    ]) {
        result.title = Some(val.trim().to_string());
    }

    // 尝试提取申请日
    if let Some(val) = extract_field(text, &[
        r"申请日[：:\s]*(\d{4}[年/\-\.]\d{1,2}[月/\-\.]\d{1,2})",
        r"Filing\s+Date[：:\s]*(\d{4}[/\-\.]\d{1,2}[/\-\.]\d{1,2})",
    ]) {
        result.filing_date = Some(val);
    }

    // 尝试提取 IPC 分类号
    if let Some(val) = extract_field(text, &[
        r"IPC[分类号]*[：:\s]*([A-Z]\d{2}[A-Z]\s*\d+/\d+)",
    ]) {
        result.ipc = Some(val);
    }

    result.full_text = text.to_string();
    result
}

fn extract_field(text: &str, patterns: &[&str]) -> Option<String> {
    for pattern in patterns {
        if let Ok(re) = regex::Regex::new(pattern) {
            if let Some(caps) = re.captures(text) {
                if let Some(m) = caps.get(1) {
                    return Some(m.as_str().to_string());
                }
            }
        }
    }
    None
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct PdfExtractResult {
    pub publication_number: Option<String>,
    pub application_number: Option<String>,
    pub applicant: Option<String>,
    pub title: Option<String>,
    pub filing_date: Option<String>,
    pub ipc: Option<String>,
    pub full_text: String,
}
