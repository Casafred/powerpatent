pub mod input;
pub mod generate;
pub mod cache;
pub mod export;

use crate::types::patent::{InputSource, PatentData};

/// 处理输入文件
/// 根据文件类型（PDF/XLSX/CSV）分别处理，返回专利数据列表
#[tauri::command]
pub async fn process_input(files: Vec<String>) -> Result<Vec<PatentData>, String> {
    log::info!("process_input called with {} files", files.len());

    let mut patents = Vec::new();

    for file_path in &files {
        let ext = file_path
            .rsplit('.')
            .next()
            .unwrap_or("")
            .to_lowercase();

        match ext.as_str() {
            "pdf" => {
                let patent = process_pdf(file_path)?;
                patents.push(patent);
            }
            "xlsx" | "xls" => {
                let table_patents = process_xlsx(file_path)?;
                patents.extend(table_patents);
            }
            "csv" => {
                let table_patents = process_csv_file(file_path)?;
                patents.extend(table_patents);
            }
            _ => {
                log::warn!("不支持的文件类型: {}", file_path);
            }
        }
    }

    // 如果同时有 PDF 和表格输入，标记为混合输入
    let has_pdf = files.iter().any(|f| f.to_lowercase().ends_with(".pdf"));
    let has_table = files.iter().any(|f| {
        let lower = f.to_lowercase();
        lower.ends_with(".xlsx") || lower.ends_with(".xls") || lower.ends_with(".csv")
    });

    if has_pdf && has_table {
        for patent in &mut patents {
            if patent.source == InputSource::Pdf {
                patent.source = InputSource::Mixed;
            }
        }
    }

    Ok(patents)
}

/// 处理 PDF 文件
fn process_pdf(path: &str) -> Result<PatentData, String> {
    log::info!("处理 PDF: {}", path);

    // 抽取文本
    let text = crate::pdf::extract::extract_text(path)
        .map_err(|e| format!("PDF 文本抽取失败: {}", e))?;

    // 从文本提取元信息
    let meta = crate::pdf::extract::extract_metadata(&text);

    let patent = PatentData {
        publication_number: meta.publication_number,
        application_number: meta.application_number,
        applicant: meta.applicant,
        title: meta.title,
        filing_date: meta.filing_date,
        ipc: meta.ipc,
        claims_text: extract_section(&text, &["权利要求书", "CLAIMS"]),
        description_text: extract_section(&text, &["说明书", "DESCRIPTION"]),
        abstract_text: extract_section(&text, &["摘要", "ABSTRACT"]),
        source: InputSource::Pdf,
        ..Default::default()
    };

    Ok(patent)
}

/// 从 PDF 文本中提取特定章节
fn extract_section(text: &str, section_names: &[&str]) -> Option<String> {
    for name in section_names {
        // 查找章节标题位置
        if let Some(start) = text.find(name) {
            // 从标题后开始，到下一个章节标题或文本末尾
            let content_start = start + name.len();
            let remaining = &text[content_start..];

            // 查找下一个可能的章节标题
            let end = find_next_section(remaining);
            let section_text = if let Some(end_pos) = end {
                remaining[..end_pos].trim()
            } else {
                remaining.trim()
            };

            if !section_text.is_empty() {
                return Some(section_text.to_string());
            }
        }
    }
    None
}

/// 查找下一个章节标题位置
fn find_next_section(text: &str) -> Option<usize> {
    let section_markers = [
        "\n权利要求书", "\n说明书", "\n摘要", "\n附图说明",
        "\n具体实施方式", "\n发明内容", "\n技术领域", "\n背景技术",
        "\nCLAIMS", "\nDESCRIPTION", "\nABSTRACT",
    ];

    let mut earliest: Option<usize> = None;
    for marker in section_markers {
        if let Some(pos) = text.find(marker) {
            match earliest {
                None => earliest = Some(pos),
                Some(current) if pos < current => earliest = Some(pos),
                _ => {}
            }
        }
    }
    earliest
}

/// 处理 XLSX 文件
fn process_xlsx(path: &str) -> Result<Vec<PatentData>, String> {
    log::info!("处理 XLSX: {}", path);

    let result = crate::table::xlsx::parse_xlsx(path)
        .map_err(|e| format!("XLSX 解析失败: {}", e))?;

    let mapping = crate::table::mapping::auto_map_columns(&result.headers);
    log::info!("自动映射了 {} 个字段", mapping.len());

    let patents: Vec<PatentData> = result.rows.iter()
        .map(|row| crate::table::mapping::row_to_patent_data(row, &result.headers, &mapping))
        .collect();

    Ok(patents)
}

/// 处理 CSV 文件
fn process_csv_file(path: &str) -> Result<Vec<PatentData>, String> {
    log::info!("处理 CSV: {}", path);

    let result = crate::table::csv::parse_csv(path)
        .map_err(|e| format!("CSV 解析失败: {}", e))?;

    let mapping = crate::table::mapping::auto_map_columns(&result.headers);
    log::info!("自动映射了 {} 个字段", mapping.len());

    let patents: Vec<PatentData> = result.rows.iter()
        .map(|row| crate::table::mapping::row_to_patent_data(row, &result.headers, &mapping))
        .collect();

    Ok(patents)
}

/// PDF 文本抽取
#[tauri::command]
pub async fn extract_pdf_text(pdf_path: String) -> Result<serde_json::Value, String> {
    log::info!("extract_pdf_text called: {}", pdf_path);

    let text = crate::pdf::extract::extract_text(&pdf_path)
        .map_err(|e| format!("{}", e))?;

    let meta = crate::pdf::extract::extract_metadata(&text);

    Ok(serde_json::to_value(meta).unwrap_or_default())
}

/// PDF 图像抽取
#[tauri::command]
pub async fn extract_pdf_images(pdf_path: String) -> Result<serde_json::Value, String> {
    log::info!("extract_pdf_images called: {}", pdf_path);

    let images = crate::pdf::images::extract_images(&pdf_path)
        .map_err(|e| format!("{}", e))?;

    Ok(serde_json::to_value(images).unwrap_or_default())
}

/// 字段映射（手动映射）
#[tauri::command]
pub async fn map_fields(
    table_path: String,
    mapping: serde_json::Value,
) -> Result<serde_json::Value, String> {
    log::info!("map_fields called: {}", table_path);

    let ext = table_path.rsplit('.').next().unwrap_or("").to_lowercase();

    let (headers, rows) = match ext.as_str() {
        "xlsx" | "xls" => {
            let result = crate::table::xlsx::parse_xlsx(&table_path)
                .map_err(|e| format!("{}", e))?;
            (result.headers, result.rows)
        }
        "csv" => {
            let result = crate::table::csv::parse_csv(&table_path)
                .map_err(|e| format!("{}", e))?;
            (result.headers, result.rows)
        }
        _ => return Err("不支持的文件类型".to_string()),
    };

    // 解析前端传来的映射关系 { colIndex: fieldName }
    let column_mapping: std::collections::HashMap<usize, String> = serde_json::from_value(mapping)
        .map_err(|e| format!("映射格式错误: {}", e))?;

    let patents: Vec<PatentData> = rows.iter()
        .map(|row| crate::table::mapping::row_to_patent_data(row, &headers, &column_mapping))
        .collect();

    Ok(serde_json::to_value(patents).unwrap_or_default())
}

/// OCR 识别
#[tauri::command]
pub async fn ocr_pdf(pdf_path: String, engine: String) -> Result<serde_json::Value, String> {
    log::info!("ocr_pdf called: {} engine={}", pdf_path, engine);
    // TODO: Phase 2.6 - OCR 双引擎集成
    Ok(serde_json::json!({ "status": "not_implemented", "message": "OCR 功能将在后续版本实现" }))
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
    // TODO: Phase 3
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
    // TODO: Phase 3
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
    // TODO: Phase 3
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
    // TODO: Phase 4
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
    // TODO: Phase 4
    Ok(())
}
