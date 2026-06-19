import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AIConfig, AIProviderConfig, ProviderType } from '../types/ai'
import { DEFAULT_AI_CONFIG } from '../types/ai'
import { loadPersisted, persist } from '../utils/persist'

export const useAIConfigStore = defineStore('aiConfig', () => {
  const config = ref<AIConfig>(loadPersisted('aiConfig', DEFAULT_AI_CONFIG))

  function updateConfig(newConfig: Partial<AIConfig>) {
    Object.assign(config.value, newConfig)
  }

  function resetConfig() {
    config.value = JSON.parse(JSON.stringify(DEFAULT_AI_CONFIG))
  }

  /** 获取当前活跃的 provider 配置 */
  function getActiveProvider(): AIProviderConfig {
    return config.value.providers[config.value.activeProvider]
  }

  /** 设置活跃服务商 */
  function setActiveProvider(type: ProviderType) {
    config.value.activeProvider = type
  }

  /** 更新某个服务商的配置 */
  function updateProvider(type: ProviderType, updates: Partial<AIProviderConfig>) {
    Object.assign(config.value.providers[type], updates)
  }

  /** 重置某个服务商为默认 */
  function resetProvider(type: ProviderType) {
    const preset = DEFAULT_AI_CONFIG.providers[type]
    config.value.providers[type] = { ...preset }
  }

  // 持久化（deep watch）
  watch(config, (val) => persist('aiConfig', val), { deep: true })

  return {
    config,
    updateConfig,
    resetConfig,
    getActiveProvider,
    setActiveProvider,
    updateProvider,
    resetProvider,
  }
})
