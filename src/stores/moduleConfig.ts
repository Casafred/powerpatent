import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { ModuleConfig, ViewMode } from '../types/module'

export const useModuleConfigStore = defineStore('moduleConfig', () => {
  const mode = ref<ViewMode>('single')
  const themeName = ref('')
  const themeDescription = ref('')
  const globalExtended = ref<Partial<Record<string, 'full' | 'lite' | 'off'>>>({
    M5: 'full',
    M6: 'full',
    M7: 'full',
    E1: 'off',
    E2: 'full',
    E3: 'full',
    E4: 'off',
    E5: 'off',
    E6: 'off',
    E7: 'off',
    E8: 'off',
  })

  function setMode(newMode: ViewMode) {
    mode.value = newMode
  }

  function setModuleLevel(moduleId: string, level: 'full' | 'lite' | 'off') {
    globalExtended.value[moduleId] = level
  }

  function getConfig(): ModuleConfig {
    return {
      mode: mode.value,
      themeName: themeName.value || undefined,
      themeDescription: themeDescription.value || undefined,
      patents: [],
      globalExtended: globalExtended.value as any,
    }
  }

  return {
    mode,
    themeName,
    themeDescription,
    globalExtended,
    setMode,
    setModuleLevel,
    getConfig,
  }
})
