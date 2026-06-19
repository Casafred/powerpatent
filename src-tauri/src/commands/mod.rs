pub mod input;
pub mod generate;
pub mod cache;
pub mod export;

use crate::types::patent::{InputSource, PatentData};
use crate::ai::client::AIClient;
use crate::ai::models::ChatMessage;
use crate::ai::provider::config_from_json;
use crate::ai::prompt::PromptManager;
use crate::cache::sqlite::{CacheManager, CacheEntry};
use std::collections::HashMap;

/// 板块 ID → Prompt 模板 ID 映射
const MODULE_PROMPT_MAP: &[(&str, &str)] = &[
    ("M3", "m3_family"),
    ("M4", "m4_summary"),
    ("M5", "m5_claims"),
    ("M6", "m6_embodiments"),
    ("M7", "m7_alternatives"),
];

/// 处理输入文件
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

fn process_pdf(path: &str) -> Result<PatentData, String> {
    log::info!("处理 PDF: {}", path);
    let text = crate::pdf::extract::extract_text(path)
        .map_err(|e| format!("PDF 文本抽取失败: {}", e))?;
    let meta = crate::pdf::extract::extract_metadata(&text);

    Ok(PatentData {
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
    })
}

fn extract_section(text: &str, section_names: &[&str]) -> Option<String> {
    for name in section_names {
        if let Some(start) = text.find(name) {
            let content_start = start + name.len();
            let remaining = &text[content_start..];
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

fn process_xlsx(path: &str) -> Result<Vec<PatentData>, String> {
    log::info!("处理 XLSX: {}", path);
    let result = crate::table::xlsx::parse_xlsx(path)
        .map_err(|e| format!("XLSX 解析失败: {}", e))?;
    let mapping = crate::table::mapping::auto_map_columns(&result.headers);
    log::info!("自动映射了 {} 个字段", mapping.len());
    Ok(result.rows.iter()
        .map(|row| crate::table::mapping::row_to_patent_data(row, &result.headers, &mapping))
        .collect())
}

fn process_csv_file(path: &str) -> Result<Vec<PatentData>, String> {
    log::info!("处理 CSV: {}", path);
    let result = crate::table::csv::parse_csv(path)
        .map_err(|e| format!("CSV 解析失败: {}", e))?;
    let mapping = crate::table::mapping::auto_map_columns(&result.headers);
    log::info!("自动映射了 {} 个字段", mapping.len());
    Ok(result.rows.iter()
        .map(|row| crate::table::mapping::row_to_patent_data(row, &result.headers, &mapping))
        .collect())
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
    let column_mapping: HashMap<usize, String> = serde_json::from_value(mapping)
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
    Ok(serde_json::json!({ "status": "not_implemented", "message": "OCR 功能将在后续版本实现" }))
}

/// 获取缓存数据库路径
fn get_cache_db_path() -> String {
    let data_dir = dirs_data_dir();
    std::fs::create_dir_all(&data_dir).ok();
    format!("{}/cache.db", data_dir)
}

fn dirs_data_dir() -> String {
    // 尝试使用平台数据目录
    if let Ok(home) = std::env::var("HOME") {
        format!("{}/.patent-reader", home)
    } else if let Ok(appdata) = std::env::var("APPDATA") {
        format!("{}/PatentReader", appdata)
    } else {
        "./data".to_string()
    }
}

/// 获取 Prompt 目录路径
fn get_prompts_dir() -> String {
    // 开发阶段使用项目目录
    let exe_dir = std::env::current_exe()
        .map(|p| p.parent().map(|pp| pp.display().to_string()).unwrap_or_default())
        .unwrap_or_default();

    // 尝试多个路径
    for dir in &[
        format!("{}/prompts", env!("CARGO_MANIFEST_DIR")),
        format!("{}/prompts", exe_dir),
        "./prompts".to_string(),
    ] {
        if std::path::Path::new(dir).exists() {
            return dir.to_string();
        }
    }

    format!("{}/prompts", env!("CARGO_MANIFEST_DIR"))
}

/// AI 生成（单板块）
#[tauri::command]
pub async fn generate_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    level: String,
    provider: serde_json::Value,
    patent_data: serde_json::Value,
) -> Result<serde_json::Value, String> {
    log::info!(
        "generate_module: project={} patent={} module={} level={}",
        project_id, patent_id, module_id, level
    );

    // 1. 查找 Prompt 模板
    let prompt_id = MODULE_PROMPT_MAP.iter()
        .find(|(mid, _)| *mid == module_id)
        .map(|(_, pid)| pid.to_string());

    let prompt_id = match prompt_id {
        Some(id) => id,
        None => {
            // M1/M2 不需要 AI 生成，直接返回元信息
            return Ok(serde_json::json!({
                "module_id": module_id,
                "patent_id": patent_id,
                "level": level,
                "output": patent_data,
                "source": "metadata",
            }));
        }
    };

    // 2. 加载 Prompt 模板
    let prompts_dir = get_prompts_dir();
    let prompt_mgr = PromptManager::load_from_dir(&prompts_dir)
        .map_err(|e| format!("加载 Prompt 模板失败: {}", e))?;

    let template = prompt_mgr.get(&prompt_id)
        .ok_or_else(|| format!("Prompt 模板 {} 不存在", prompt_id))?;

    // 3. 构建 Prompt 变量
    let mut template_data: HashMap<String, String> = HashMap::new();
    let empty_map = serde_json::Map::new();
    let data_obj = patent_data.as_object().unwrap_or(&empty_map);

    for field in &template.input_fields {
        let value = data_obj.get(field)
            .and_then(|v| v.as_str())
            .unwrap_or("(未提供)");
        template_data.insert(field.clone(), value.to_string());
    }

    // 4. 渲染 Prompt
    let rendered_prompt = prompt_mgr.render(&prompt_id, &template_data)
        .map_err(|e| format!("渲染 Prompt 失败: {}", e))?;

    // 5. 创建 AI 客户端
    let client_config = config_from_json(&provider)?;
    let client = AIClient::new(client_config);

    // 6. 调用 AI
    let messages = vec![
        ChatMessage::system("你是一位专业的专利分析师，请严格按照要求的 JSON 格式输出，不要输出任何其他内容。"),
        ChatMessage::user(rendered_prompt),
    ];

    let temperature = template.temperature;
    let output_json = client.chat_json(messages, temperature).await
        .map_err(|e| format!("AI 生成失败: {}", e))?;

    // 7. 写入缓存
    let cache = CacheManager::open(&get_cache_db_path())
        .map_err(|e| format!("打开缓存失败: {}", e))?;

    let entry = CacheEntry {
        project_id,
        patent_id,
        module_id: module_id.clone(),
        level: level.clone(),
        output_json: output_json.to_string(),
        model: client.config().model.clone(),
        provider: format!("{:?}", client.config().provider).to_lowercase(),
        prompt_version: "1".to_string(),
        temperature,
        rerun_count: 0,
        created_at: String::new(),
        updated_at: String::new(),
    };

    cache.upsert(&entry)
        .map_err(|e| format!("写入缓存失败: {}", e))?;

    Ok(serde_json::json!({
        "module_id": module_id,
        "patent_id": entry.patent_id,
        "level": level,
        "output": output_json,
        "model": entry.model,
        "provider": entry.provider,
        "cached": false,
    }))
}

/// 缓存查询
#[tauri::command]
pub async fn get_cached_module(
    project_id: String,
    patent_id: String,
    module_id: String,
) -> Result<Option<serde_json::Value>, String> {
    log::info!(
        "get_cached_module: project={} patent={} module={}",
        project_id, patent_id, module_id
    );

    let cache = CacheManager::open(&get_cache_db_path())
        .map_err(|e| format!("打开缓存失败: {}", e))?;

    match cache.get(&project_id, &patent_id, &module_id, "full") {
        Ok(Some(entry)) => {
            let output: serde_json::Value = serde_json::from_str(&entry.output_json)
                .unwrap_or(serde_json::json!({}));
            Ok(Some(serde_json::json!({
                "module_id": entry.module_id,
                "patent_id": entry.patent_id,
                "level": entry.level,
                "output": output,
                "model": entry.model,
                "provider": entry.provider,
                "cached": true,
                "rerun_count": entry.rerun_count,
                "updated_at": entry.updated_at,
            })))
        }
        Ok(None) => Ok(None),
        Err(e) => Err(format!("查询缓存失败: {}", e)),
    }
}

/// 板块重跑
#[tauri::command]
pub async fn rerun_module(
    project_id: String,
    patent_id: String,
    module_id: String,
    options: serde_json::Value,
) -> Result<serde_json::Value, String> {
    log::info!(
        "rerun_module: project={} patent={} module={}",
        project_id, patent_id, module_id
    );

    // 重跑就是重新调用 generate_module
    let provider = options.get("provider").cloned().unwrap_or(serde_json::json!({}));
    let patent_data = options.get("patent_data").cloned().unwrap_or(serde_json::json!({}));
    let level = options.get("level").and_then(|v| v.as_str()).unwrap_or("full").to_string();

    generate_module(project_id, patent_id, module_id, level, provider, patent_data).await
}

/// HTML 渲染
#[tauri::command]
pub async fn render_html(
    project_id: String,
    _module_config: serde_json::Value,
    _embed_pdf: bool,
) -> Result<String, String> {
    log::info!("render_html called: project={}", project_id);
    // TODO: Phase 4
    Ok(String::new())
}

/// 导出 HTML 文件
#[tauri::command]
pub async fn export_html(
    project_id: String,
    output_path: String,
    _module_config: serde_json::Value,
    _embed_pdf: bool,
) -> Result<(), String> {
    log::info!("export_html: project={} path={}", project_id, output_path);
    // TODO: Phase 4
    Ok(())
}
