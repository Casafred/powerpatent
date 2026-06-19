import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { AIConfig } from '../types/ai'
import { DEFAULT_AI_CONFIG } from '../types/ai'
import { loadPersisted, persist } from '../utils/persist'

export const useAIConfigStore = defineStore('aiConfig', () => {
  const config = ref<AIConfig>(loadPersisted('aiConfig', DEFAULT_AI_CONFIG))

  function updateConfig(newConfig: Partial<AIConfig>) {
    Object.assign(config.value, newConfig)
  }

  function resetConfig() {
    config.value = { ...DEFAULT_AI_CONFIG }
  }

  // 持久化（deep watch）
  watch(config, (val) => persist('aiConfig', val), { deep: true })

  return {
    config,
    updateConfig,
    resetConfig,
  }
})
