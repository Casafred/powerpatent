/** AI 服务配置类型 */
export type ProviderType = 'openai' | 'zhipu' | 'deepseek'

export interface AIProviderConfig {
  type: ProviderType
  apiKey: string
  baseUrl: string
  model: string
}

export interface AIConfig {
  /** 各服务商配置，key 为 ProviderType */
  providers: Record<ProviderType, AIProviderConfig>
  /** 当前分析使用的服务商 */
  activeProvider: ProviderType
  /** OCR 引擎 */
  ocr: 'paddle_ocr_vl' | 'glm_ocr'
}

export interface ConnectionTestResult {
  success: boolean
  message: string
  latency?: number
}

/** 服务商预设 */
export const PROVIDER_PRESETS: Record<ProviderType, { baseUrl: string; models: string[]; name: string }> = {
  deepseek: {
    name: 'DeepSeek',
    baseUrl: 'https://api.deepseek.com',
    models: ['deepseek-v4-pro', 'deepseek-v4-flash', 'deepseek-chat', 'deepseek-reasoner'],
  },
  zhipu: {
    name: '智谱 GLM',
    baseUrl: 'https://open.bigmodel.cn/api/paas',
    models: ['glm-5.1', 'glm-5', 'glm-4-plus', 'glm-4-flash', 'glm-4-air'],
  },
  openai: {
    name: 'OpenAI',
    baseUrl: 'https://api.openai.com',
    models: ['gpt-4o', 'gpt-4o-mini', 'gpt-4-turbo'],
  },
}

function createDefaultProviderConfig(type: ProviderType): AIProviderConfig {
  const preset = PROVIDER_PRESETS[type]
  return {
    type,
    apiKey: '',
    baseUrl: preset.baseUrl,
    model: preset.models[0],
  }
}

/** 默认 AI 配置 */
export const DEFAULT_AI_CONFIG: AIConfig = {
  providers: {
    deepseek: createDefaultProviderConfig('deepseek'),
    zhipu: createDefaultProviderConfig('zhipu'),
    openai: createDefaultProviderConfig('openai'),
  },
  activeProvider: 'deepseek',
  ocr: 'paddle_ocr_vl',
}

/** 获取当前活跃的 provider 配置 */
export function getActiveProvider(config: AIConfig): AIProviderConfig {
  return config.providers[config.activeProvider]
}

/** 构建 API URL */
export function buildApiUrl(type: ProviderType, baseUrl: string): string {
  let base = baseUrl.replace(/\/+$/, '')
  if (type === 'zhipu') {
    if (!base.endsWith('/v4')) base += '/v4'
  } else {
    if (!base.endsWith('/v1')) base += '/v1'
  }
  return base
}

/** 测试 AI 连接 */
export async function testAIConnection(
  type: ProviderType,
  apiKey: string,
  baseUrl: string,
  model: string,
): Promise<ConnectionTestResult> {
  const start = performance.now()
  try {
    const url = buildApiUrl(type, baseUrl) + '/chat/completions'
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${apiKey}`,
      },
      body: JSON.stringify({
        model,
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
