/**
 * AI 配置管理服务
 */
import { useAIConfigStore } from '../stores/aiConfig'
import type { AIProviderConfig, ProviderType } from '../types/ai'

const PROVIDER_PRESETS: Record<ProviderType, { baseUrl: string; models: string[] }> = {
  deepseek: {
    baseUrl: 'https://api.deepseek.com',
    models: ['deepseek-chat', 'deepseek-v4-flash'],
  },
  zhipu: {
    baseUrl: 'https://open.bigmodel.cn/api/paas',
    models: ['glm-4-plus', 'glm-4-flash'],
  },
  openai: {
    baseUrl: 'https://api.openai.com',
    models: ['gpt-4o', 'gpt-4o-mini'],
  },
}

export function getProviderPresets(type: ProviderType) {
  return PROVIDER_PRESETS[type]
}

export function buildProviderConfig(type: ProviderType, apiKey: string, model: string): AIProviderConfig {
  const preset = PROVIDER_PRESETS[type]
  return {
    type,
    apiKey,
    baseUrl: preset.baseUrl,
    model,
  }
}

export function validateAIConfig(): { valid: boolean; message: string } {
  const store = useAIConfigStore()
  const { analysis, translate } = store.config

  if (!analysis.apiKey) {
    return { valid: false, message: '请配置分析模型的 API Key' }
  }
  if (!translate.apiKey) {
    return { valid: false, message: '请配置翻译/快速模型的 API Key' }
  }
  return { valid: true, message: '' }
}
