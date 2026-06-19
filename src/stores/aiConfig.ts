import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { AIConfig } from '../types/ai'
import { DEFAULT_AI_CONFIG } from '../types/ai'

export const useAIConfigStore = defineStore('aiConfig', () => {
  const config = ref<AIConfig>({ ...DEFAULT_AI_CONFIG })

  function updateConfig(newConfig: Partial<AIConfig>) {
    Object.assign(config.value, newConfig)
  }

  function resetConfig() {
    config.value = { ...DEFAULT_AI_CONFIG }
  }

  return {
    config,
    updateConfig,
    resetConfig,
  }
})
