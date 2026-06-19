import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { loadPersisted, persist } from '../utils/persist'

export type ThemeMode = 'light' | 'dark' | 'system'

export const useSettingsStore = defineStore('settings', () => {
  // 主题
  const theme = ref<ThemeMode>(loadPersisted('theme', 'system'))

  // 应用主题到 DOM
  function applyTheme(mode?: ThemeMode) {
    const m = mode || theme.value
    const isDark = m === 'dark' || (m === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches)
    document.documentElement.classList.toggle('dark', isDark)
  }

  function setTheme(mode: ThemeMode) {
    theme.value = mode
    applyTheme(mode)
  }

  // 监听系统主题变化
  if (typeof window !== 'undefined') {
    window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', () => {
      if (theme.value === 'system') applyTheme()
    })
  }

  // 初始化应用主题
  applyTheme()

  // 持久化
  watch(theme, (val) => persist('theme', val))

  return {
    theme,
    setTheme,
    applyTheme,
  }
})
