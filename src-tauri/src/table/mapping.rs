use crate::types::patent::{InputSource, PatentData};
use std::collections::HashMap;

// 字段映射：自动识别列名 + 手动映射

/// 已知的列名映射表（中英文常见列名 → 字段名）
const KNOWN_COLUMNS: &[(&str, &str)] = &[
    // 著录信息
    ("公开号", "publication_number"),
    ("publication number", "publication_number"),
    ("pn", "publication_number"),
    ("授权号", "grant_number"),
    ("grant number", "grant_number"),
    ("申请号", "application_number"),
    ("application number", "application_number"),
    ("an", "application_number"),
    ("申请人", "applicant"),
    ("assignee", "applicant"),
    ("applicant", "applicant"),
    ("发明人", "inventor"),
    ("inventor", "inventor"),
    ("申请日", "filing_date"),
    ("filing date", "filing_date"),
    ("优先权日", "priority_date"),
    ("priority date", "priority_date"),
    ("公开日", "publication_date"),
    ("publication date", "publication_date"),
    ("授权日", "grant_date"),
    ("grant date", "grant_date"),
    ("法律状态", "legal_status"),
    ("legal status", "legal_status"),
    ("ipc", "ipc"),
    ("ipc分类号", "ipc"),
    ("cpc", "cpc"),
    ("cpc分类号", "cpc"),
    ("同族", "family_members"),
    ("family", "family_members"),
    ("family members", "family_members"),
    // 文本内容
    ("权利要求", "claims_text"),
    ("claims", "claims_text"),
    ("说明书", "description_text"),
    ("description", "description_text"),
    ("specification", "description_text"),
    ("摘要", "abstract_text"),
    ("abstract", "abstract_text"),
    ("标题", "title"),
    ("title", "title"),
    ("发明名称", "title"),
];

/// 自动识别列名映射
pub fn auto_map_columns(headers: &[String]) -> HashMap<usize, String> {
    let mut mapping = HashMap::new();

    for (idx, header) in headers.iter().enumerate() {
        let lower = header.trim().to_lowercase();
        for &(known_name, field_name) in KNOWN_COLUMNS {
            if lower == known_name.to_lowercase() || lower.contains(known_name) {
                mapping.insert(idx, field_name.to_string());
                break;
            }
        }
    }

    mapping
}

/// 根据映射将表格行转为 PatentData
pub fn row_to_patent_data(
    row: &[String],
    headers: &[String],
    column_mapping: &HashMap<usize, String>,
) -> PatentData {
    let mut data = PatentData {
        source: InputSource::Table,
        ..Default::default()
    };

    for (&col_idx, field_name) in column_mapping {
        if col_idx < row.len() {
            let value = row[col_idx].trim().to_string();
            if value.is_empty() {
                continue;
            }
            match field_name.as_str() {
                "publication_number" => data.publication_number = Some(value),
                "grant_number" => data.grant_number = Some(value),
                "application_number" => data.application_number = Some(value),
                "applicant" => data.applicant = Some(value),
                "inventor" => data.inventor = Some(value),
                "filing_date" => data.filing_date = Some(value),
                "priority_date" => data.priority_date = Some(value),
                "publication_date" => data.publication_date = Some(value),
                "grant_date" => data.grant_date = Some(value),
                "legal_status" => data.legal_status = Some(value),
                "ipc" => data.ipc = Some(value),
                "cpc" => data.cpc = Some(value),
                "title" => data.title = Some(value),
                "abstract_text" => data.abstract_text = Some(value),
                "claims_text" => data.claims_text = Some(value),
                "description_text" => data.description_text = Some(value),
                _ => {}
            }
        }
    }

    data
}

/// 获取所有可映射的字段名列表
pub fn get_available_fields() -> Vec<(&'static str, &'static str)> {
    vec![
        ("publication_number", "公开号"),
        ("grant_number", "授权号"),
        ("application_number", "申请号"),
        ("applicant", "申请人"),
        ("inventor", "发明人"),
        ("filing_date", "申请日"),
        ("priority_date", "优先权日"),
        ("publication_date", "公开日"),
        ("grant_date", "授权日"),
        ("legal_status", "法律状态"),
        ("ipc", "IPC分类号"),
        ("cpc", "CPC分类号"),
        ("title", "标题"),
        ("abstract_text", "摘要"),
        ("claims_text", "权利要求"),
        ("description_text", "说明书"),
    ]
}
