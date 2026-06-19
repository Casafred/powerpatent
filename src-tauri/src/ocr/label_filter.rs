use regex::Regex;

/// 从 OCR 文本中提取附图标号
/// 匹配模式: "1", "2", "10" 等数字标号，通常出现在附图说明中
pub fn extract_figure_labels(text: &str) -> Vec<FigureLabel> {
    let mut labels = Vec::new();

    // 匹配 "图1", "图2", "图10" 等
    let fig_re = Regex::new(r"图\s*(\d+)").unwrap();
    for cap in fig_re.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            labels.push(FigureLabel {
                label: format!("图{}", m.as_str()),
                number: m.as_str().parse().unwrap_or(0),
            });
        }
    }

    // 匹配附图标号: "1-", "2-" 等（出现在零件说明中）
    let part_re = Regex::new(r"(?:^|\s)(\d{1,3})[-\s]").unwrap();
    for cap in part_re.captures_iter(text) {
        if let Some(m) = cap.get(1) {
            let num: u32 = m.as_str().parse().unwrap_or(0);
            if num > 0 && num < 1000 {
                labels.push(FigureLabel {
                    label: m.as_str().to_string(),
                    number: num,
                });
            }
        }
    }

    labels.sort_by_key(|l| l.number);
    labels.dedup_by_key(|l| l.number);
    labels
}

/// 附图标号
#[derive(Debug, Clone, serde::Serialize)]
pub struct FigureLabel {
    pub label: String,
    pub number: u32,
}
