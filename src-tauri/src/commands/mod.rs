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
use std::path::PathBuf;
use tauri::Manager;
/// 板块 ID → Prompt 模板 ID 映射
const MODULE_PROMPT_MAP: &[(&str, &str)] = &[
    ("M3", "m3_family"),
    ("M4", "m4_summary"),
    ("M5", "m5_claims"),
    ("M6", "m6_embodiments"),
    ("M7", "m7_alternatives"),
    ("M8", "m8_family_claims_diff"),
    ("E2", "e2_figure_comparison"),
    ("E4", "e4_comparison_matrix"),
    ("E5", "e5_tech_timeline"),
    ("E6", "e6_applicant_profile"),
    ("E7", "e7_design_around"),
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
                let patent = process_pdf(file_path).await?;
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
        // 收集所有 PDF 的 (文件名提取的编号 → 路径) 映射
        let pdf_map: Vec<(String, String)> = patents.iter()
            .filter(|p| p.pdf_file_path.is_some())
            .filter_map(|p| {
                let path = p.pdf_file_path.as_ref().unwrap();
                let filename = path.rsplit(|c| c == '/' || c == '\\').next().unwrap_or("");
                // 从文件名中提取编号（去掉 .pdf 扩展名）
                let stem = filename.strip_suffix(".pdf")
                    .or_else(|| filename.strip_suffix(".PDF"))
                    .unwrap_or(filename);
                // 同时也用专利自身识别出的公开号/申请号
                let mut ids = vec![stem.to_uppercase()];
                if let Some(ref pn) = p.publication_number {
                    ids.push(pn.to_uppercase());
                }
                if let Some(ref an) = p.application_number {
                    ids.push(an.to_uppercase());
                }
                // 返回所有可能的标识
                Some(ids.into_iter().map(|id| (id, path.clone())).collect::<Vec<_>>())
            })
            .flatten()
            .collect();

        // 对非 PDF 来源的专利，尝试匹配 PDF
        for patent in &mut patents {
            if patent.source != InputSource::Pdf && patent.pdf_file_path.is_none() {
                // 尝试用公开号匹配
                let mut matched_path: Option<String> = None;
                if let Some(ref pn) = patent.publication_number {
                    let pn_upper = pn.to_uppercase();
                    for (id, path) in &pdf_map {
                        if pn_upper.contains(id) || id.contains(&pn_upper) {
                            matched_path = Some(path.clone());
                            break;
                        }
                    }
                    // 也尝试去掉 CN/US 等国家代码前缀匹配
                    if matched_path.is_none() {
                        let pn_clean = pn_upper.trim_start_matches(|c: char| c.is_ascii_alphabetic());
                        for (id, path) in &pdf_map {
                            let id_clean = id.trim_start_matches(|c: char| c.is_ascii_alphabetic());
                            if !pn_clean.is_empty() && (pn_clean == id_clean || pn_clean.contains(id_clean) || id_clean.contains(pn_clean)) {
                                matched_path = Some(path.clone());
                                break;
                            }
                        }
                    }
                }
                // 尝试用申请号匹配
                if matched_path.is_none() {
                    if let Some(ref an) = patent.application_number {
                        let an_upper = an.to_uppercase();
                        for (id, path) in &pdf_map {
                            if an_upper.contains(id) || id.contains(&an_upper) {
                                matched_path = Some(path.clone());
                                break;
                            }
                        }
                    }
                }
                if let Some(pdf_path) = matched_path {
                    patent.pdf_file_path = Some(pdf_path);
                    patent.source = InputSource::Mixed;
                }
            } else if patent.source == InputSource::Pdf {
                patent.source = InputSource::Mixed;
            }
        }
    }

    Ok(patents)
}

async fn process_pdf(path: &str) -> Result<PatentData, String> {
    log::info!("处理 PDF: {}", path);

    // 1. 先尝试直接文本抽取
    let text = crate::pdf::extract::extract_text(path)
        .map_err(|e| format!("PDF 文本抽取失败: {}", e))?;
    let meta = crate::pdf::extract::extract_metadata(&text);

    // 2. 如果文本内容太少（扫描件），自动调用 PaddleOCR
    let needs_ocr = text.trim().len() < 100;

    let (final_text, ocr_used) = if needs_ocr {
        log::info!("PDF 文本不足 ({} chars)，自动调用 PaddleOCR", text.trim().len());
        match crate::ocr::ocr_pdf(path, &crate::ocr::OcrEngine::PaddleOcrVl, None).await {
            Ok(ocr_result) => {
                let ocr_text = if ocr_result.markdown.is_some() && !ocr_result.markdown.as_ref().unwrap().is_empty() {
                    ocr_result.markdown.unwrap()
                } else {
                    ocr_result.text
                };
                log::info!("PaddleOCR 完成，获得 {} chars", ocr_text.len());
                (ocr_text, true)
            }
            Err(e) => {
                log::warn!("PaddleOCR 失败: {}，回退到原始文本", e);
                (text, false)
            }
        }
    } else {
        (text, false)
    };

    // 如果用了 OCR，重新提取元信息
    let final_meta = if ocr_used {
        crate::pdf::extract::extract_metadata(&final_text)
    } else {
        meta.clone()
    };

    Ok(PatentData {
        publication_number: final_meta.publication_number.or(meta.publication_number),
        application_number: final_meta.application_number.or(meta.application_number),
        applicant: final_meta.applicant.or(meta.applicant),
        title: final_meta.title.or(meta.title),
        filing_date: final_meta.filing_date.or(meta.filing_date),
        ipc: final_meta.ipc.or(meta.ipc),
        claims_text: extract_section(&final_text, &["权利要求书", "CLAIMS"]),
        description_text: extract_section(&final_text, &["说明书", "DESCRIPTION"]),
        abstract_text: extract_section(&final_text, &["摘要", "ABSTRACT"]),
        source: InputSource::Pdf,
        needs_ocr: ocr_used,
        pdf_file_path: Some(path.to_string()),
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

    let ocr_engine = match engine.as_str() {
        "paddle_ocr_vl" => crate::ocr::OcrEngine::PaddleOcrVl,
        "glm_ocr" => crate::ocr::OcrEngine::GlmOcr,
        _ => crate::ocr::OcrEngine::PaddleOcrVl,
    };

    let glm_config = if matches!(ocr_engine, crate::ocr::OcrEngine::GlmOcr) {
        Some(crate::ocr::GlmOcrConfig {
            api_key: String::new(), // 从前端传入
            base_url: "https://open.bigmodel.cn/api/paas/v4".to_string(),
            model: "glm-4v-plus".to_string(),
        })
    } else {
        None
    };

    let result = crate::ocr::ocr_pdf(&pdf_path, &ocr_engine, glm_config)
        .await
        .map_err(|e| format!("OCR 失败: {}", e))?;

    Ok(serde_json::to_value(result).unwrap_or_default())
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

/// 获取 Prompt 目录路径（优先使用 Tauri 资源目录）
fn get_prompts_dir(app_handle: &tauri::AppHandle) -> String {
    // 1. 尝试 Tauri 资源目录（打包后的应用）
    if let Ok(resource_dir) = app_handle.path().resource_dir() {
        let prompts_path = resource_dir.join("prompts");
        if prompts_path.exists() {
            return prompts_path.display().to_string();
        }
    }

    // 2. 开发阶段使用项目源码目录
    let dev_path = format!("{}/prompts", env!("CARGO_MANIFEST_DIR"));
    if std::path::Path::new(&dev_path).exists() {
        return dev_path;
    }

    // 3. 相对于可执行文件
    if let Ok(exe_dir) = std::env::current_exe()
        .map(|p| p.parent().map(|pp| pp.display().to_string()).unwrap_or_default())
    {
        let exe_path = format!("{}/prompts", exe_dir);
        if std::path::Path::new(&exe_path).exists() {
            return exe_path;
        }
    }

    // 4. 回退到 CARGO_MANIFEST_DIR（即使不存在也返回，让后续逻辑报错）
    format!("{}/prompts", env!("CARGO_MANIFEST_DIR"))
}

/// AI 生成（单板块）
#[tauri::command]
pub async fn generate_module(
    app_handle: tauri::AppHandle,
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
    let prompts_dir = get_prompts_dir(&app_handle);
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
    app_handle: tauri::AppHandle,
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

    generate_module(app_handle, project_id, patent_id, module_id, level, provider, patent_data).await
}

/// HTML 渲染
#[tauri::command]
pub async fn render_html(
    project_id: String,
    module_config: serde_json::Value,
    embed_pdf: bool,
) -> Result<String, String> {
    log::info!("render_html called: project={} embed_pdf={}", project_id, embed_pdf);

    // 1. 从缓存加载所有板块输出
    let cache = CacheManager::open(&get_cache_db_path())
        .map_err(|e| format!("打开缓存失败: {}", e))?;
    let entries = cache.list_project(&project_id)
        .map_err(|e| format!("查询缓存失败: {}", e))?;

    // 2. 构建板块输出映射
    let mut modules: HashMap<String, serde_json::Value> = HashMap::new();
    for entry in &entries {
        let output: serde_json::Value = serde_json::from_str(&entry.output_json)
            .unwrap_or(serde_json::json!({}));
        let key = format!("{}_{}", entry.patent_id, entry.module_id);

        // 为板块渲染 HTML 子模板
        let renderer = crate::render::template::HtmlRenderer::new()
            .map_err(|e| format!("创建渲染器失败: {}", e))?;

        let rendered = renderer.render_module(&entry.module_id, &output)
            .unwrap_or_default();

        modules.insert(key, serde_json::Value::String(rendered));
    }

    // 3. 获取专利数据（从 module_config 中提取）
    let patents: Vec<serde_json::Value> = module_config.get("patents")
        .and_then(|v| v.as_array())
        .cloned()
        .unwrap_or_default();

    if patents.is_empty() {
        return Err("没有专利数据可供渲染".to_string());
    }

    // 4. 为非 AI 板块（M1/M2）也渲染子模板
    let renderer = crate::render::template::HtmlRenderer::new()
        .map_err(|e| format!("创建渲染器失败: {}", e))?;

    let mut all_modules = modules.clone();
    for patent in &patents {
        let patent_id = patent.get("publication_number")
            .or_else(|| patent.get("application_number"))
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");

        // M1 基本信息
        if let Ok(rendered) = renderer.render_module("M1", patent) {
            all_modules.insert(format!("{}_M1", patent_id), serde_json::Value::String(rendered));
        }
        // M2 法律状态
        if let Ok(rendered) = renderer.render_module("M2", patent) {
            all_modules.insert(format!("{}_M2", patent_id), serde_json::Value::String(rendered));
        }
    }

    // 5. 构建渲染数据
    let mode = module_config.get("mode")
        .and_then(|v| v.as_str())
        .unwrap_or("single");
    let theme_name = module_config.get("theme_name")
        .and_then(|v| v.as_str());
    let theme_description = module_config.get("theme_description")
        .and_then(|v| v.as_str());

    let render_data = crate::render::template::build_render_data(
        &patents, &all_modules, mode, theme_name, theme_description,
    );

    // 6. 注入内联 CSS
    let css = crate::render::assets::get_inline_css();
    let mut data_obj = render_data.as_object()
        .cloned()
        .unwrap_or_default();
    data_obj.insert("inline_css".to_string(), serde_json::Value::String(css));

    let final_data = serde_json::Value::Object(data_obj);

    // 7. 渲染 HTML
    let html = if mode == "multi" {
        renderer.render_multi(&final_data)
    } else {
        renderer.render_single(&final_data)
    }.map_err(|e| format!("渲染 HTML 失败: {}", e))?;

    Ok(html)
}

/// 导出 HTML 文件
#[tauri::command]
pub async fn export_html(
    project_id: String,
    output_path: String,
    module_config: serde_json::Value,
    embed_pdf: bool,
) -> Result<(), String> {
    log::info!("export_html: project={} path={} embed_pdf={}", project_id, output_path, embed_pdf);

    // 渲染 HTML
    let html = render_html(project_id, module_config, embed_pdf).await?;

    // 写入文件
    std::fs::write(&output_path, html)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    log::info!("HTML 已导出到: {}", output_path);
    Ok(())
}

/// 测试 AI 连接
#[tauri::command]
pub async fn test_ai_connection(
    provider_type: String,
    api_key: String,
    base_url: String,
    model: String,
) -> Result<serde_json::Value, String> {
    use std::time::Instant;

    log::info!("test_ai_connection: type={} base_url={} model={}", provider_type, base_url, model);

    if api_key.is_empty() {
        return Ok(serde_json::json!({
            "success": false,
            "message": "请先输入 API Key"
        }));
    }

    // 构建 API URL
    let url = {
        let mut base = base_url.trim_end_matches('/').to_string();
        if provider_type == "zhipu" {
            if !base.ends_with("/v4") { base.push_str("/v4"); }
        } else {
            if !base.ends_with("/v1") { base.push_str("/v1"); }
        }
        format!("{}/chat/completions", base)
    };

    let start = Instant::now();

    let client = crate::ai::proxy::create_http_client();

    let body = serde_json::json!({
        "model": model,
        "messages": [{"role": "user", "content": "Hi"}],
        "max_tokens": 5,
        "stream": false,
    });

    let response = client
        .post(&url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key))
        .json(&body)
        .send()
        .await;

    let latency = start.elapsed().as_millis() as u64;

    match response {
        Ok(resp) => {
            if resp.status().is_success() {
                Ok(serde_json::json!({
                    "success": true,
                    "message": format!("连接成功 ({}ms)", latency),
                    "latency": latency,
                }))
            } else {
                let status = resp.status().as_u16();
                let error_text = resp.text().await.unwrap_or_default();
                let short_error = if error_text.len() > 200 {
                    format!("{}...", &error_text[..200])
                } else {
                    error_text
                };
                Ok(serde_json::json!({
                    "success": false,
                    "message": format!("HTTP {}: {}", status, short_error),
                    "latency": latency,
                }))
            }
        }
        Err(e) => {
            Ok(serde_json::json!({
                "success": false,
                "message": format!("网络错误: {}", e),
                "latency": latency,
            }))
        }
    }
}

/// 获取用户自定义 Prompt 目录
fn get_user_prompts_dir() -> String {
    let data_dir = dirs_data_dir();
    let user_prompts_dir = format!("{}/prompts", data_dir);
    std::fs::create_dir_all(&user_prompts_dir).ok();
    user_prompts_dir
}

/// 列出所有 Prompt 模板（含用户自定义覆盖）
#[tauri::command]
pub async fn list_prompts(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    log::info!("list_prompts called");

    // 1. 加载内置模板
    let prompts_dir = get_prompts_dir(&app_handle);
    let builtin_mgr = PromptManager::load_from_dir(&prompts_dir)
        .map_err(|e| format!("加载内置 Prompt 模板失败: {}", e))?;

    // 2. 加载用户自定义覆盖模板
    let user_prompts_dir = get_user_prompts_dir();
    let user_mgr = PromptManager::load_from_dir(&user_prompts_dir)
        .map_err(|e| format!("加载用户 Prompt 模板失败: {}", e))?;

    // 3. 合并：用户覆盖优先
    let mut result = Vec::new();

    // 收集所有模板 ID（内置 + 用户自定义）
    let mut all_ids: Vec<String> = builtin_mgr.template_ids();
    for id in user_mgr.template_ids() {
        if !all_ids.contains(&id) {
            all_ids.push(id);
        }
    }

    for id in &all_ids {
        let is_user_modified = user_mgr.get(id).is_some();
        let template = if is_user_modified {
            user_mgr.get(id).unwrap()
        } else {
            match builtin_mgr.get(id) {
                Some(t) => t,
                None => continue,
            }
        };

        result.push(serde_json::json!({
            "id": template.id,
            "name": template.name,
            "description": template.description,
            "temperature": template.temperature,
            "template": template.prompt_template,
            "isUserModified": is_user_modified,
        }));
    }

    Ok(serde_json::Value::Array(result))
}

/// 保存用户自定义 Prompt 模板
#[tauri::command]
pub async fn save_prompt(
    prompt_id: String,
    name: String,
    description: String,
    temperature: f64,
    template: String,
) -> Result<(), String> {
    log::info!("save_prompt called: {}", prompt_id);

    let user_prompts_dir = get_user_prompts_dir();
    let file_path = format!("{}/{}.yaml", user_prompts_dir, prompt_id);

    // 构建模板数据（与原始 YAML 结构一致）
    let prompt_data = serde_json::json!({
        "id": prompt_id,
        "name": name,
        "description": description,
        "temperature": temperature as f32,
        "prompt_template": template,
    });

    let yaml_content = serde_yaml::to_string(&prompt_data)
        .map_err(|e| format!("序列化 YAML 失败: {}", e))?;

    std::fs::write(&file_path, yaml_content)
        .map_err(|e| format!("写入文件失败: {}", e))?;

    log::info!("用户 Prompt 模板已保存: {}", file_path);
    Ok(())
}

/// 重置 Prompt 模板（删除用户自定义覆盖）
#[tauri::command]
pub async fn reset_prompt(prompt_id: String) -> Result<(), String> {
    log::info!("reset_prompt called: {}", prompt_id);

    let user_prompts_dir = get_user_prompts_dir();
    let file_path = PathBuf::from(&user_prompts_dir).join(format!("{}.yaml", prompt_id));

    if file_path.exists() {
        std::fs::remove_file(&file_path)
            .map_err(|e| format!("删除文件失败: {}", e))?;
        log::info!("用户 Prompt 模板已删除: {:?}", file_path);
    }

    Ok(())
}
