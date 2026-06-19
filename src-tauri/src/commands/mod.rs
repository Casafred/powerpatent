pub mod input;
pub mod generate;
pub mod cache;
pub mod export;

use crate::types::patent::PatentData;

/// 处理输入文件
#[tauri::command]
pub async fn process_input(files: Vec<String>) -> Result<Vec<PatentData>, String> {
    log::info!("process_input called with {} files", files.len());
    Ok(vec![])
}

/// PDF 文本抽取
#[tauri::command]
pub async fn extract_pdf_text(pdf_path: String) -> Result<serde_json::Value, String> {
    log::info!("extract_pdf_text called: {}", pdf_path);
    Ok(serde_json::json!({}))
}

/// PDF 图像抽取
#[tauri::command]
pub async fn extract_pdf_images(pdf_path: String) -> Result<serde_json::Value, String> {
    log::info!("extract_pdf_images called: {}", pdf_path);
    Ok(serde_json::json!([]))
}

/// 字段映射
#[tauri::command]
pub async fn map_fields(table_path: String, _mapping: serde_json::Value) -> Result<PatentData, String> {
    log::info!("map_fields called: {}", table_path);
    Err("map_fields not implemented yet".to_string())
}

/// OCR 识别
#[tauri::command]
pub async fn ocr_pdf(pdf_path: String, engine: String) -> Result<serde_json::Value, String> {
    log::info!("ocr_pdf called: {} engine={}", pdf_path, engine);
    Ok(serde_json::json!({}))
}

/// AI 生成（单板块）
#[tauri::command]
pub async fn generate_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    level: String,
    _provider: serde_json::Value,
) -> Result<serde_json::Value, String> {
    log::info!(
        "generate_module called: project={} patent={} module={} level={}",
        project_id, patent_id, module_id, level
    );
    Ok(serde_json::json!({}))
}

/// 缓存查询
#[tauri::command]
pub async fn get_cached_module(
    project_id: String,
    patent_id: String,
    module_id: String,
) -> Result<Option<serde_json::Value>, String> {
    log::info!(
        "get_cached_module called: project={} patent={} module={}",
        project_id, patent_id, module_id
    );
    Ok(None)
}

/// 板块重跑
#[tauri::command]
pub async fn rerun_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    _options: serde_json::Value,
) -> Result<serde_json::Value, String> {
    log::info!(
        "rerun_module called: project={} patent={} module={}",
        project_id, patent_id, module_id
    );
    Ok(serde_json::json!({}))
}

/// HTML 渲染
#[tauri::command]
pub async fn render_html(
    project_id: String,
    _module_config: serde_json::Value,
    embed_pdf: bool,
) -> Result<String, String> {
    log::info!("render_html called: project={} embed_pdf={}", project_id, embed_pdf);
    Ok(String::new())
}

/// 导出 HTML 文件
#[tauri::command]
pub async fn export_html(
    project_id: String,
    output_path: String,
    _module_config: serde_json::Value,
    embed_pdf: bool,
) -> Result<(), String> {
    log::info!(
        "export_html called: project={} path={} embed_pdf={}",
        project_id, output_path, embed_pdf
    );
    Ok(())
}
