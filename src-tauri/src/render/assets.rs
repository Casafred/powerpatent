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
.container { max-width: 960px; margin: 0 auto; padding: 24px; }

/* Header */
.header { background: linear-gradient(135deg, #1a1a2e 0%, #16213e 100%); color: #fff; border-radius: 12px; padding: 32px; margin-bottom: 24px; }
.header-inner { text-align: center; }
.logo { font-size: 12px; text-transform: uppercase; letter-spacing: 3px; color: #409eff; margin-bottom: 8px; }
.header h1 { font-size: 24px; font-weight: 600; }
.header .meta { color: rgba(255,255,255,0.6); font-size: 13px; margin-top: 8px; }
.header .theme-desc { color: rgba(255,255,255,0.8); font-size: 14px; margin-top: 4px; }

/* TOC */
.toc { background: #fff; border: 1px solid #e4e7ed; border-radius: 8px; padding: 16px; margin-bottom: 24px; }
.toc h3 { font-size: 14px; margin-bottom: 8px; }
.toc ol { padding-left: 20px; }
.toc li { margin: 4px 0; }
.toc a { color: #409eff; text-decoration: none; }
.toc a:hover { text-decoration: underline; }

/* Patent Section */
.patent-section { background: #fff; border: 1px solid #e4e7ed; border-radius: 12px; margin-bottom: 24px; overflow: hidden; }
.patent-header-bar { display: flex; justify-content: space-between; align-items: center; padding: 16px 24px; background: #fafbfc; border-bottom: 1px solid #e4e7ed; }
.patent-title-area { display: flex; align-items: center; gap: 12px; }
.patent-index { background: #409eff; color: #fff; font-size: 12px; font-weight: 700; padding: 2px 8px; border-radius: 4px; }
.patent-title-area h2 { font-size: 18px; color: #1a1a2e; }
.patent-id-area { display: flex; gap: 8px; }
.patent-id-tag { background: #ecf5ff; color: #409eff; font-size: 12px; padding: 2px 8px; border-radius: 4px; font-weight: 500; }

/* Tabs */
.tab-container { padding: 0; }
.tab-nav { display: flex; gap: 0; border-bottom: 2px solid #e4e7ed; padding: 0 16px; background: #fafbfc; overflow-x: auto; }
.tab-btn { padding: 10px 16px; border: none; background: transparent; cursor: pointer; font-size: 13px; color: #909399; font-weight: 500; border-bottom: 2px solid transparent; margin-bottom: -2px; white-space: nowrap; transition: all 0.2s; }
.tab-btn:hover { color: #409eff; }
.tab-btn.active { color: #409eff; border-bottom-color: #409eff; }
.tab-content { padding: 20px 24px; }
.tab-pane { display: none; }
.tab-pane.active { display: block; }

/* Info Table */
.info-table { width: 100%; border-collapse: collapse; margin: 8px 0; }
.info-table th { text-align: left; width: 140px; padding: 10px 14px; background: #f5f7fa; color: #606266; font-size: 13px; font-weight: 500; border: 1px solid #ebeef5; white-space: nowrap; }
.info-table td { padding: 10px 14px; font-size: 13px; border: 1px solid #ebeef5; line-height: 1.6; }
.info-table td.abstract-text { white-space: pre-wrap; }

/* Legal Status */
.legal-status { padding: 4px 0; }
.status-badge { display: inline-block; background: #67c23a; color: #fff; padding: 4px 12px; border-radius: 20px; font-size: 13px; font-weight: 600; margin-bottom: 12px; }
.timeline { display: flex; flex-direction: column; gap: 8px; margin: 12px 0; padding-left: 16px; border-left: 2px solid #409eff; }
.timeline-item { display: flex; gap: 12px; }
.timeline-label { color: #909399; font-size: 13px; min-width: 80px; }
.timeline-date { font-size: 13px; font-weight: 500; }
.key-dates { margin-top: 12px; }
.key-dates h4 { font-size: 13px; color: #606266; margin-bottom: 8px; }
.key-date-item { display: flex; justify-content: space-between; padding: 6px 0; border-bottom: 1px solid #f0f0f0; font-size: 13px; }

/* Summary */
.summary-section { display: flex; flex-direction: column; gap: 16px; }
.summary-item { padding: 12px 16px; background: #fafbfc; border-radius: 8px; border-left: 3px solid #409eff; }
.summary-label { font-size: 12px; color: #409eff; font-weight: 600; text-transform: uppercase; letter-spacing: 1px; margin-bottom: 4px; }
.summary-value { font-size: 14px; line-height: 1.6; }
.one-line-summary { background: linear-gradient(135deg, #ecf5ff 0%, #f0f9eb 100%); padding: 16px 20px; border-radius: 8px; font-weight: 600; font-size: 16px; color: #303133; text-align: center; }

/* Claims */
.claims-section h4 { font-size: 14px; color: #606266; margin: 16px 0 8px; }
.claim-card { padding: 12px 16px; margin-bottom: 8px; background: #fafbfc; border: 1px solid #ebeef5; border-radius: 8px; border-left: 3px solid #409eff; }
.claim-card.dependent { border-left-color: #e6a23c; background: #fffbf0; }
.claim-num { font-weight: 700; color: #409eff; margin-bottom: 4px; font-size: 13px; }
.claim-text { font-size: 13px; color: #606266; margin-bottom: 8px; line-height: 1.6; }
.claim-bilingual { display: grid; grid-template-columns: 1fr 1fr; gap: 16px; margin-bottom: 8px; }
.claim-translation { padding: 10px 14px; background: #f0f9eb; border-radius: 6px; border: 1px solid #e1f3d8; }
.claim-original { padding: 10px 14px; background: #f5f7fa; border-radius: 6px; border: 1px solid #ebeef5; }
.claim-bilingual .claim-label { font-size: 11px; color: #909399; font-weight: 600; margin-bottom: 4px; text-transform: uppercase; letter-spacing: 1px; }
.claim-bilingual .claim-text { margin-bottom: 0; }
.features { margin: 8px 0; }
.features strong { font-size: 12px; color: #909399; }
.features ul { padding-left: 20px; font-size: 13px; margin-top: 4px; }
.scope-text { font-size: 12px; color: #909399; font-style: italic; }
.scope-narrowing { font-size: 12px; color: #e6a23c; margin-top: 4px; padding: 4px 8px; background: #fffbf0; border-radius: 4px; border-left: 2px solid #e6a23c; }

/* Original text display for M1 */
.original-text { font-size: 12px; color: #909399; }
.original-block { font-size: 12px; color: #909399; margin-top: 4px; padding: 6px 10px; background: #f5f7fa; border-radius: 4px; border: 1px solid #ebeef5; white-space: pre-wrap; line-height: 1.5; }

/* Embodiments */
.embodiments-section { display: flex; flex-direction: column; gap: 12px; }
.embodiment-card { padding: 16px; background: #fafbfc; border: 1px solid #ebeef5; border-radius: 8px; border-left: 3px solid #67c23a; }
.embodiment-card h4 { color: #67c23a; margin-bottom: 6px; font-size: 14px; }
.embodiment-card p { font-size: 13px; line-height: 1.6; }
.param-table { width: 100%; border-collapse: collapse; margin: 8px 0; }
.param-table th, .param-table td { padding: 6px 10px; border: 1px solid #ebeef5; font-size: 12px; }
.param-table th { background: #f5f7fa; font-weight: 500; }
.advantage { font-size: 13px; color: #67c23a; margin-top: 4px; }

/* Alternatives */
.alternatives-section { display: flex; flex-direction: column; gap: 12px; }
.alternative-card { padding: 16px; background: #fffbf0; border: 1px solid #fbe8c8; border-radius: 8px; border-left: 3px solid #e6a23c; }
.alternative-card p { font-size: 13px; line-height: 1.6; }
.alternative-card .related { font-size: 12px; color: #909399; margin-top: 4px; }
.alternative-card .scope { font-size: 12px; color: #e6a23c; margin-top: 2px; }

/* Family */
.family-section { padding: 4px 0; }
.family-diff-section { padding: 4px 0; }
.overview-text { font-size: 14px; line-height: 1.6; margin-bottom: 12px; }

/* Generic JSON */
.field-group { display: flex; flex-direction: column; gap: 8px; }
.field-row { display: flex; gap: 12px; padding: 6px 0; border-bottom: 1px solid #f0f0f0; }
.field-label { min-width: 120px; color: #909399; font-size: 13px; font-weight: 500; }
.field-value { font-size: 13px; line-height: 1.6; flex: 1; }
.list-group { display: flex; flex-direction: column; gap: 8px; }
.list-item { padding: 8px 12px; background: #fafbfc; border-radius: 6px; border: 1px solid #ebeef5; }

/* Footer */
.footer { text-align: center; padding: 24px 0; color: #c0c4cc; font-size: 12px; border-top: 1px solid #e4e7ed; margin-top: 24px; }

/* PDF Embed */
.pdf-embed-container { width: 100%; height: 600px; }
.pdf-embed-iframe { width: 100%; height: 100%; border: none; }
.pdf-notice { text-align: center; padding: 40px 20px; color: #909399; font-size: 14px; background: #fafbfc; border: 1px dashed #dcdfe6; border-radius: 8px; margin: 20px 0; }

/* Figures (E2) */
.figures-section { display: flex; flex-direction: column; gap: 16px; }
.figure-card { padding: 16px; background: #fafbfc; border: 1px solid #ebeef5; border-radius: 8px; border-left: 3px solid #409eff; }
.figure-header { display: flex; align-items: center; gap: 8px; margin-bottom: 8px; }
.figure-num { background: #409eff; color: #fff; font-size: 12px; font-weight: 700; padding: 2px 8px; border-radius: 4px; }
.figure-title { font-size: 15px; font-weight: 600; color: #303133; }
.figure-desc { font-size: 13px; color: #606266; line-height: 1.6; margin-bottom: 8px; }
.figure-summary { font-size: 12px; color: #909399; font-style: italic; margin-top: 8px; padding: 6px 10px; background: #f5f7fa; border-radius: 4px; }
.figure-image { max-width: 100%; border: 1px solid #ebeef5; border-radius: 6px; margin: 8px 0; background: #fff; }
.figure-image-container { text-align: center; margin: 8px 0; }
.figure-image-container img { max-width: 100%; max-height: 400px; border: 1px solid #ebeef5; border-radius: 6px; }
.element-table { width: 100%; border-collapse: collapse; margin: 8px 0; }
.element-table th, .element-table td { padding: 6px 10px; border: 1px solid #ebeef5; font-size: 12px; }
.element-table th { background: #f5f7fa; font-weight: 500; text-align: left; }

/* Print */
@media print { body { background: #fff; } .container { max-width: 100%; padding: 0; } .tab-pane { display: block !important; } .tab-nav { display: none; } }
"#;
