use anyhow::Result;
use calamine::{open_workbook, Reader, Xlsx};
use std::path::Path;

/// 解析 XLSX 文件，返回所有工作表的数据
pub fn parse_xlsx(path: &str) -> Result<XlsxResult> {
    let p = Path::new(path);
    if !p.exists() {
        anyhow::bail!("XLSX 文件不存在: {}", path);
    }

    let mut workbook: Xlsx<_> = open_workbook(path)
        .map_err(|e| anyhow::anyhow!("打开 XLSX 失败: {}", e))?;

    let mut result = XlsxResult::default();

    // 读取第一个工作表
    if let Some(sheet_name) = workbook.sheet_names().first().cloned() {
        result.sheet_name = sheet_name.clone();
        if let Ok(range) = workbook.worksheet_range(&sheet_name) {
            let mut rows_iter = range.rows();

            // 读取表头
            if let Some(header_row) = rows_iter.next() {
                result.headers = header_row.iter().map(|c| c.to_string()).collect();
            }

            // 读取数据行
            for row in rows_iter {
                let cells: Vec<String> = row.iter().map(|c| c.to_string()).collect();
                result.rows.push(cells);
            }
        }
    }

    Ok(result)
}

#[derive(Debug, Default, serde::Serialize, serde::Deserialize)]
pub struct XlsxResult {
    pub sheet_name: String,
    pub headers: Vec<String>,
    pub rows: Vec<Vec<String>>,
}
