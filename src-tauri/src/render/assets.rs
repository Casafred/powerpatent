use anyhow::Result;
use base64::{Engine, engine::general_purpose::STANDARD as BASE64};

/// 将文件内容转为 base64 data URI
pub fn file_to_data_uri(path: &str, mime_type: &str) -> Result<String> {
    let bytes = std::fs::read(path)
        .map_err(|e| anyhow::anyhow!("读取文件失败 {}: {}", path, e))?;
    Ok(bytes_to_data_uri(&bytes, mime_type))
}

/// 将字节转为 base64 data URI
pub fn bytes_to_data_uri(bytes: &[u8], mime_type: &str) -> String {
    let b64 = BASE64.encode(bytes);
    format!("data:{};base64,{}", mime_type, b64)
}

/// 将图片文件内联为 base64 data URI
pub fn image_to_data_uri(path: &str) -> Result<String> {
    let ext = path.rsplit('.').next().unwrap_or("").to_lowercase();
    let mime = match ext.as_str() {
        "png" => "image/png",
        "jpg" | "jpeg" => "image/jpeg",
        "gif" => "image/gif",
        "svg" => "image/svg+xml",
        "webp" => "image/webp",
        _ => "application/octet-stream",
    };
    file_to_data_uri(path, mime)
}

/// 将 PDF 文件内联为 base64 data URI
pub fn pdf_to_data_uri(path: &str) -> Result<String> {
    file_to_data_uri(path, "application/pdf")
}

/// 获取内联 CSS 样式
pub fn get_inline_css() -> String {
    INLINE_CSS.to_string()
}

const INLINE_CSS: &str = r#"
* { box-sizing: border-box; margin: 0; padding: 0; }
body { font-family: -apple-system, 'PingFang SC', 'Microsoft YaHei', 'Helvetica Neue', sans-serif; line-height: 1.6; color: #333; background: #f5f7fa; }
.container { max-width: 900px; margin: 0 auto; padding: 24px; }
.header { text-align: center; padding: 32px 0 24px; border-bottom: 2px solid #e4e7ed; margin-bottom: 24px; }
.header h1 { font-size: 24px; color: #1a1a2e; }
.header .meta { color: #909399; font-size: 13px; margin-top: 8px; }
.header .theme-desc { color: #606266; font-size: 14px; margin-top: 4px; }
.toc { background: #fff; border: 1px solid #e4e7ed; border-radius: 8px; padding: 16px; margin-bottom: 24px; }
.toc h3 { font-size: 14px; margin-bottom: 8px; }
.toc ol { padding-left: 20px; }
.toc li { margin: 4px 0; }
.toc a { color: #409eff; text-decoration: none; }
.toc a:hover { text-decoration: underline; }
.patent-section { background: #fff; border: 1px solid #e4e7ed; border-radius: 8px; padding: 24px; margin-bottom: 24px; }
.patent-section h2 { font-size: 20px; color: #1a1a2e; margin-bottom: 16px; padding-bottom: 8px; border-bottom: 1px solid #f0f0f0; }
.patent-section h3 { font-size: 16px; color: #303133; margin-bottom: 12px; }
.module { margin-bottom: 20px; padding: 16px; background: #fafbfc; border-radius: 6px; border-left: 3px solid #409eff; }
.module h2, .module h3 { margin-bottom: 12px; }
.info-table { width: 100%; border-collapse: collapse; }
.info-table th { text-align: left; width: 120px; padding: 8px 12px; background: #f5f7fa; color: #606266; font-size: 13px; font-weight: 500; border: 1px solid #ebeef5; }
.info-table td { padding: 8px 12px; font-size: 13px; border: 1px solid #ebeef5; }
.summary-card { display: flex; flex-direction: column; gap: 12px; }
.summary-item { padding: 8px 0; }
.summary-item strong { color: #409eff; font-size: 13px; }
.summary-item p { margin-top: 4px; font-size: 14px; }
.summary-one-line { background: #ecf5ff; padding: 12px 16px; border-radius: 6px; font-weight: 500; font-size: 15px; color: #303133; }
.claims-analysis h4 { font-size: 14px; color: #606266; margin: 12px 0 8px; }
.claim-item { padding: 12px; margin-bottom: 8px; background: #fff; border: 1px solid #ebeef5; border-radius: 4px; }
.claim-item.dependent { background: #f9f9f9; }
.claim-header { font-weight: 600; color: #409eff; margin-bottom: 4px; }
.claim-text { font-size: 13px; color: #606266; margin-bottom: 8px; }
.claim-features ul { padding-left: 20px; font-size: 13px; }
.claim-scope { font-size: 13px; color: #909399; font-style: italic; }
.scope-narrowing { font-size: 12px; color: #e6a23c; }
.embodiment-card { padding: 12px; margin-bottom: 8px; background: #fff; border: 1px solid #ebeef5; border-radius: 4px; }
.embodiment-card h4 { color: #409eff; margin-bottom: 4px; }
.param-table { width: 100%; border-collapse: collapse; margin: 8px 0; }
.param-table th, .param-table td { padding: 4px 8px; border: 1px solid #ebeef5; font-size: 12px; }
.param-table th { background: #f5f7fa; }
.advantage { font-size: 13px; color: #67c23a; }
.alternative-card { padding: 12px; margin-bottom: 8px; background: #fff; border: 1px solid #ebeef5; border-radius: 4px; border-left: 3px solid #e6a23c; }
.alternative-card .related { font-size: 12px; color: #909399; }
.alternative-card .scope { font-size: 12px; color: #e6a23c; }
.family-overview p { margin-bottom: 8px; }
.legal-status p { margin-bottom: 4px; font-size: 14px; }
.footer { text-align: center; padding: 24px 0; color: #c0c4cc; font-size: 12px; border-top: 1px solid #e4e7ed; margin-top: 24px; }
@media print { body { background: #fff; } .container { max-width: 100%; padding: 0; } .module { break-inside: avoid; } }
"#;
