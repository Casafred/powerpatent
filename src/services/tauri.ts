/**
 * Tauri IPC 调用封装
 * 自动检测 Tauri 环境，非 Tauri 环境下返回模拟数据
 */
import { invoke } from '@tauri-apps/api/core'

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window

function safeInvoke<T>(cmd: string, args?: Record<string, unknown>): Promise<T> {
  if (!isTauri) {
    console.warn(`[Tauri] Not in Tauri context, skipping invoke: ${cmd}`)
    return Promise.reject(new Error('当前不在 Tauri 桌面环境中，此功能需要安装桌面应用后使用'))
  }
  return invoke<T>(cmd, args)
}

/** 输入处理 */
export async function processInput(files: string[]): Promise<any[]> {
  return safeInvoke('process_input', { files })
}

/** PDF 文本抽取 */
export async function extractPdfText(pdfPath: string): Promise<any> {
  return safeInvoke('extract_pdf_text', { pdfPath })
}

/** PDF 图像抽取 */
export async function extractPdfImages(pdfPath: string): Promise<any> {
  return safeInvoke('extract_pdf_images', { pdfPath })
}

/** 字段映射 */
export async function mapFields(tablePath: string, mapping: Record<number, string>): Promise<any> {
  return safeInvoke('map_fields', { tablePath, mapping })
}

/** OCR 识别 */
export async function ocrPdf(pdfPath: string, engine: string): Promise<any> {
  return safeInvoke('ocr_pdf', { pdfPath, engine })
}

/** AI 生成（单板块） */
export async function generateModule(params: {
  projectId: string
  patentId: string
  moduleId: string
  level: string
  provider: any
  patentData: any
}): Promise<any> {
  return safeInvoke('generate_module', {
    project_id: params.projectId,
    patent_id: params.patentId,
    module_id: params.moduleId,
    level: params.level,
    provider: params.provider,
    patent_data: params.patentData,
  })
}

/** 缓存查询 */
export async function getCachedModule(params: {
  projectId: string
  patentId: string
  moduleId: string
}): Promise<any> {
  return safeInvoke('get_cached_module', {
    project_id: params.projectId,
    patent_id: params.patentId,
    module_id: params.moduleId,
  })
}

/** 板块重跑 */
export async function rerunModule(params: {
  projectId: string
  patentId: string
  moduleId: string
  options: any
}): Promise<any> {
  return safeInvoke('rerun_module', {
    project_id: params.projectId,
    patent_id: params.patentId,
    module_id: params.moduleId,
    options: params.options,
  })
}

/** HTML 渲染导出 */
export async function renderHtml(params: {
  projectId: string
  moduleConfig: any
  embedPdf: boolean
}): Promise<string> {
  return safeInvoke('render_html', {
    project_id: params.projectId,
    module_config: params.moduleConfig,
    embed_pdf: params.embedPdf,
  })
}

/** 导出 HTML 文件 */
export async function exportHtml(params: {
  projectId: string
  outputPath: string
  moduleConfig: any
  embedPdf: boolean
}): Promise<void> {
  return safeInvoke('export_html', {
    project_id: params.projectId,
    output_path: params.outputPath,
    module_config: params.moduleConfig,
    embed_pdf: params.embedPdf,
  })
}

/** 测试 AI 连接 */
export async function testAiConnection(params: {
  providerType: string
  apiKey: string
  baseUrl: string
  model: string
}): Promise<{ success: boolean; message: string; latency?: number }> {
  return safeInvoke('test_ai_connection', {
    provider_type: params.providerType,
    api_key: params.apiKey,
    base_url: params.baseUrl,
    model: params.model,
  })
}
