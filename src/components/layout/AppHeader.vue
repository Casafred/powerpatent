<script setup lang="ts">
import { ref } from 'vue'
import { useSettingsStore, type ThemeMode } from '../../stores/settings'
import SettingsDrawer from '../settings/SettingsDrawer.vue'

const settings = useSettingsStore()
const settingsVisible = ref(false)

function toggleTheme() {
  const modes: ThemeMode[] = ['light', 'dark', 'system']
  const idx = modes.indexOf(settings.theme)
  settings.setTheme(modes[(idx + 1) % modes.length])
}

function getThemeIcon() {
  switch (settings.theme) {
    case 'light': return 'Sunny'
    case 'dark': return 'Moon'
    case 'system': return 'Monitor'
  }
}

function getThemeLabel() {
  switch (settings.theme) {
    case 'light': return '浅色'
    case 'dark': return '深色'
    case 'system': return '跟随系统'
  }
}
</script>

<template>
  <header class="app-header">
    <div class="header-left">
      <span class="app-name">PatentReader</span>
      <span class="app-divider">|</span>
      <span class="app-subtitle">专利解读生成器</span>
    </div>
    <div class="header-right">
      <el-tooltip content="设置" placement="bottom">
        <button class="header-btn" @click="settingsVisible = true">
          <el-icon :size="14"><Setting /></el-icon>
        </button>
      </el-tooltip>
      <el-tooltip :content="getThemeLabel()" placement="bottom">
        <button class="header-btn" @click="toggleTheme">
          <el-icon :size="14"><component :is="getThemeIcon()" /></el-icon>
        </button>
      </el-tooltip>
      <span class="version-tag">v0.1.0</span>
    </div>
  </header>

  <SettingsDrawer v-model:visible="settingsVisible" />
</template>

<style scoped>
.app-header {
  height: var(--app-header-height);
  background: var(--app-header-bg);
  color: var(--app-header-text);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  flex-shrink: 0;
  font-size: 13px;
  user-select: none;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-name {
  font-weight: 600;
  font-size: 14px;
}

.app-divider {
  opacity: 0.3;
}

.app-subtitle {
  opacity: 0.7;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 4px;
}

.header-btn {
  background: rgba(255, 255, 255, 0.1);
  border: none;
  color: inherit;
  cursor: pointer;
  padding: 4px 8px;
  border-radius: 4px;
  display: flex;
  align-items: center;
  transition: background 0.2s;
}

.header-btn:hover {
  background: rgba(255, 255, 255, 0.2);
}

.version-tag {
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  opacity: 0.6;
  margin-left: 4px;
}
</style>
