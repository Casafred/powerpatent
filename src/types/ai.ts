/** AI 服务配置类型 */
export type ProviderType = 'openai' | 'zhipu' | 'deepseek'

export interface AIProviderConfig {
  type: ProviderType
  apiKey: string
  baseUrl: string
  model: string
}

export interface AIConfig {
  analysis: AIProviderConfig
  translate: AIProviderConfig
  ocr: 'paddleocr' | 'glm'
}

/** 默认 AI 配置 */
export const DEFAULT_AI_CONFIG: AIConfig = {
  analysis: {
    type: 'deepseek',
    apiKey: '',
    baseUrl: 'https://api.deepseek.com',
    model: 'deepseek-chat',
  },
  translate: {
    type: 'deepseek',
    apiKey: '',
    baseUrl: 'https://api.deepseek.com',
    model: 'deepseek-v4-flash',
  },
  ocr: 'paddleocr',
}
