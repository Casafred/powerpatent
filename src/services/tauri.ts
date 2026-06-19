/**
 * Tauri IPC 调用封装
 * 统一管理所有前后端通信
 */
import { invoke } from '@tauri-apps/api/core'

/** 输入处理 */
export async function processInput(files: string[]): Promise<any> {
  return invoke('process_input', { files })
}

/** PDF 文本抽取 */
export async function extractPdfText(pdfPath: string): Promise<any> {
  return invoke('extract_pdf_text', { pdfPath })
}

/** PDF 图像抽取 */
export async function extractPdfImages(pdfPath: string): Promise<any> {
  return invoke('extract_pdf_images', { pdfPath })
}

/** 字段映射 */
export async function mapFields(tablePath: string, mapping: any): Promise<any> {
  return invoke('map_fields', { tablePath, mapping })
}

/** OCR 识别 */
export async function ocrPdf(pdfPath: string, engine: string): Promise<any> {
  return invoke('ocr_pdf', { pdfPath, engine })
}

/** AI 生成（单板块） */
export async function generateModule(params: {
  projectId: string
  patentId: string
  moduleId: string
  level: string
  provider: any
}): Promise<any> {
  return invoke('generate_module', params)
}

/** 缓存查询 */
export async function getCachedModule(params: {
  projectId: string
  patentId: string
  moduleId: string
}): Promise<any> {
  return invoke('get_cached_module', params)
}

/** 板块重跑 */
export async function rerunModule(params: {
  projectId: string
  patentId: string
  moduleId: string
  options: any
}): Promise<any> {
  return invoke('rerun_module', params)
}

/** HTML 渲染导出 */
export async function renderHtml(params: {
  projectId: string
  moduleConfig: any
  embedPdf: boolean
}): Promise<string> {
  return invoke('render_html', params)
}

/** 导出 HTML 文件 */
export async function exportHtml(params: {
  projectId: string
  outputPath: string
  moduleConfig: any
  embedPdf: boolean
}): Promise<void> {
  return invoke('export_html', params)
}
