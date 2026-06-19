use anyhow::Result;
use std::path::Path;

// PDF 图像抽取 - TODO: 需要集成 mupdf 或 pdf-render crate
// 当前阶段先返回空结果，后续 Phase 2.5 实现

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct FigureImage {
    pub figure_num: String,
    pub image_base64: String,
    pub page_number: u32,
}

/// 从 PDF 抽取图像（占位实现）
pub fn extract_images(_pdf_path: &str) -> Result<Vec<FigureImage>> {
    // TODO: 使用 mupdf 渲染 PDF 页面为图片
    Ok(vec![])
}
