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
    projectId: params.projectId,
    patentId: params.patentId,
    moduleId: params.moduleId,
    level: params.level,
    provider: params.provider,
    patentData: params.patentData,
  })
}

/** 缓存查询 */
export async function getCachedModule(params: {
  projectId: string
  patentId: string
  moduleId: string
}): Promise<any> {
  return safeInvoke('get_cached_module', {
    projectId: params.projectId,
    patentId: params.patentId,
    moduleId: params.moduleId,
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
    projectId: params.projectId,
    patentId: params.patentId,
    moduleId: params.moduleId,
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
    projectId: params.projectId,
    moduleConfig: params.moduleConfig,
    embedPdf: params.embedPdf,
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
    projectId: params.projectId,
    outputPath: params.outputPath,
    moduleConfig: params.moduleConfig,
    embedPdf: params.embedPdf,
  })
}

/** 提示词模板 */
export interface PromptTemplate {
  id: string
  name: string
  description: string
  temperature: number
  template: string
  isUserModified: boolean
}

/** 获取所有提示词模板 */
export async function listPrompts(): Promise<PromptTemplate[]> {
  return safeInvoke('list_prompts')
}

/** 保存提示词模板 */
export async function savePrompt(params: {
  promptId: string
  name: string
  description: string
  temperature: number
  template: string
}): Promise<void> {
  return safeInvoke('save_prompt', params)
}

/** 重置提示词模板 */
export async function resetPrompt(params: { promptId: string }): Promise<void> {
  return safeInvoke('reset_prompt', params)
}

/** 测试 AI 连接（统一走浏览器 fetch，自动继承系统代理） */
export async function testAiConnection(params: {
  providerType: string
  apiKey: string
  baseUrl: string
  model: string
}): Promise<{ success: boolean; message: string; latency?: number }> {
  return testAiConnectionFetch(params)
}

/** 浏览器端直接 fetch 测试连接 */
async function testAiConnectionFetch(params: {
  providerType: string
  apiKey: string
  baseUrl: string
  model: string
}): Promise<{ success: boolean; message: string; latency?: number }> {
  if (!params.apiKey) {
    return { success: false, message: '请先输入 API Key' }
  }

  let base = params.baseUrl.replace(/\/+$/, '')
  if (params.providerType === 'zhipu') {
    if (!base.endsWith('/v4')) base += '/v4'
  } else {
    if (!base.endsWith('/v1')) base += '/v1'
  }
  const url = `${base}/chat/completions`

  const start = performance.now()
  try {
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${params.apiKey}`,
      },
      body: JSON.stringify({
        model: params.model,
        messages: [{ role: 'user', content: 'Hi' }],
        max_tokens: 5,
        stream: false,
      }),
    })
    const latency = Math.round(performance.now() - start)
    if (response.ok) {
      return { success: true, message: `连接成功 (${latency}ms)`, latency }
    }
    const errorText = await response.text()
    return { success: false, message: `HTTP ${response.status}: ${errorText.slice(0, 200)}`, latency }
  } catch (err) {
    const latency = Math.round(performance.now() - start)
    return { success: false, message: `网络错误: ${(err as Error).message}`, latency }
  }
}
