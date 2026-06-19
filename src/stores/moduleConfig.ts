import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { ModuleConfig, ViewMode, ModuleId, ModuleLevel, PatentModuleConfig } from '../types/module'
import { loadPersisted, persist } from '../utils/persist'

export type PresetKey = 'quick' | 'standard' | 'deep'

export const PRESETS: Record<PresetKey, { label: string; desc: string; levels: Record<string, ModuleLevel> }> = {
  quick: {
    label: '快速概览',
    desc: '仅生成精简版核心板块，适合快速浏览',
    levels: { M5: 'lite', M6: 'lite', M7: 'lite', E1: 'off', E2: 'off', E3: 'off', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
  },
  standard: {
    label: '标准解读',
    desc: '完整核心板块 + 附图与批注，适合常规解读',
    levels: { M5: 'full', M6: 'full', M7: 'full', E1: 'off', E2: 'full', E3: 'full', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
  },
  deep: {
    label: '深度研读',
    desc: '全部板块完整输出，含 PDF 原文与附图',
    levels: { M5: 'full', M6: 'full', M7: 'full', E1: 'full', E2: 'full', E3: 'full', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
  },
}

export const useModuleConfigStore = defineStore('moduleConfig', () => {
  const DEFAULT_LEVELS: Partial<Record<ModuleId, ModuleLevel>> = {
    M5: 'full', M6: 'full', M7: 'full',
    E1: 'off', E2: 'full', E3: 'full',
    E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off',
  }

  const saved = loadPersisted('moduleConfig', {
    mode: 'single' as ViewMode,
    themeName: '',
    themeDescription: '',
    activePreset: 'standard' as PresetKey,
    globalExtended: DEFAULT_LEVELS,
    patentOverrides: {} as Record<string, PatentModuleConfig>,
  })

  const mode = ref<ViewMode>(saved.mode)
  const themeName = ref(saved.themeName)
  const themeDescription = ref(saved.themeDescription)
  const activePreset = ref<PresetKey>(saved.activePreset)
  const globalExtended = ref<Partial<Record<ModuleId, ModuleLevel>>>(saved.globalExtended)
  const patentOverrides = ref<Record<string, PatentModuleConfig>>(saved.patentOverrides)

  function setMode(newMode: ViewMode) {
    mode.value = newMode
  }

  function setModuleLevel(moduleId: ModuleId, level: ModuleLevel) {
    globalExtended.value[moduleId] = level
  }

  function applyPreset(key: PresetKey) {
    activePreset.value = key
    const levels = PRESETS[key].levels
    for (const [id, level] of Object.entries(levels)) {
      globalExtended.value[id as ModuleId] = level
    }
  }

  // 设置某篇专利的板块级别覆盖
  function setPatentModuleLevel(patentId: string, moduleId: ModuleId, level: ModuleLevel) {
    if (!patentOverrides.value[patentId]) {
      patentOverrides.value[patentId] = { patentId, isKey: false, levels: {} }
    }
    patentOverrides.value[patentId].levels[moduleId] = level
  }

  // 设置关键专利
  function setKeyPatent(patentId: string, isKey: boolean) {
    if (!patentOverrides.value[patentId]) {
      patentOverrides.value[patentId] = { patentId, isKey, levels: {} }
    } else {
      patentOverrides.value[patentId].isKey = isKey
    }
  }

  // 获取某篇专利某板块的最终级别（覆盖 > 全局）
  function getEffectiveLevel(patentId: string, moduleId: ModuleId): ModuleLevel {
    const override = patentOverrides.value[patentId]?.levels?.[moduleId]
    if (override) return override
    return globalExtended.value[moduleId] ?? 'off'
  }

  // 清除某篇专利的覆盖配置
  function clearPatentOverride(patentId: string) {
    delete patentOverrides.value[patentId]
  }

  function getConfig(): ModuleConfig {
    return {
      mode: mode.value,
      themeName: themeName.value || undefined,
      themeDescription: themeDescription.value || undefined,
      patents: Object.values(patentOverrides.value),
      globalExtended: globalExtended.value as any,
    }
  }

  // 持久化
  watch(
    [mode, themeName, themeDescription, activePreset, globalExtended, patentOverrides],
    () => {
      persist('moduleConfig', {
        mode: mode.value,
        themeName: themeName.value,
        themeDescription: themeDescription.value,
        activePreset: activePreset.value,
        globalExtended: globalExtended.value,
        patentOverrides: patentOverrides.value,
      })
    },
    { deep: true },
  )

  return {
    mode,
    themeName,
    themeDescription,
    activePreset,
    globalExtended,
    patentOverrides,
    setMode,
    setModuleLevel,
    applyPreset,
    setPatentModuleLevel,
    setKeyPatent,
    getEffectiveLevel,
    clearPatentOverride,
    getConfig,
  }
})
