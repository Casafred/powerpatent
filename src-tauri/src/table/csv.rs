use anyhow::Result;
use std::fs;
use std::path::Path;

/// 解析 CSV 文件，返回表头和数据行
pub fn parse_csv(path: &str) -> Result<CsvResult> {
    let p = Path::new(path);
    if !p.exists() {
        anyhow::bail!("CSV 文件不存在: {}", path);
    }

    let content = fs::read_to_string(path)
        .map_err(|e| anyhow::anyhow!("读取 CSV 失败: {}", e))?;

    let mut reader = csv::ReaderBuilder::new()
        .flexible(true)
        .from_reader(content.as_bytes());

    let headers = reader.headers()
        .map(|h| h.iter().map(|s| s.to_string()).collect())
        .unwrap_or_default();

    let rows: Vec<Vec<String>> = reader.records()
        .filter_map(|r| r.ok())
        .map(|r| r.iter().map(|s| s.to_string()).collect())
        .collect();

    Ok(CsvResult { headers, rows })
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct CsvResult {
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
