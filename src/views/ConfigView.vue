<script setup lang="ts">
import { computed } from 'vue'
import { useModuleConfigStore } from '../stores/moduleConfig'
import { useInputStore } from '../stores/input'
import { useRouter } from 'vue-router'
import { MODULE_REGISTRY, type ModuleId, type ModuleLevel } from '../types/module'

const configStore = useModuleConfigStore()
const inputStore = useInputStore()
const router = useRouter()

const mode = computed({
  get: () => configStore.mode,
  set: (v) => configStore.setMode(v),
})

const themeName = computed({
  get: () => configStore.themeName,
  set: (v) => { configStore.themeName = v },
})

const themeDescription = computed({
  get: () => configStore.themeDescription,
  set: (v) => { configStore.themeDescription = v },
})

const requiredModules = computed(() => MODULE_REGISTRY.filter(m => m.required))
const extendedModules = computed(() => MODULE_REGISTRY.filter(m => !m.required))

function getModuleLevel(id: ModuleId): ModuleLevel {
  return (configStore.globalExtended[id] as ModuleLevel) ?? 'off'
}

function setModuleLevel(id: ModuleId, level: ModuleLevel) {
  configStore.setModuleLevel(id, level)
}

function applyPreset(preset: 'quick' | 'standard' | 'deep') {
  const presets: Record<string, Record<string, ModuleLevel>> = {
    quick: { M5: 'lite', M6: 'lite', M7: 'lite', E1: 'off', E2: 'off', E3: 'off', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
    standard: { M5: 'full', M6: 'full', M7: 'full', E1: 'off', E2: 'full', E3: 'full', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
    deep: { M5: 'full', M6: 'full', M7: 'full', E1: 'full', E2: 'full', E3: 'full', E4: 'off', E5: 'off', E6: 'off', E7: 'off', E8: 'off' },
  }
  const p = presets[preset]
  for (const [id, level] of Object.entries(p)) {
    configStore.setModuleLevel(id, level)
  }
}

function goNext() {
  router.push({ name: 'ai' })
}

function goBack() {
  router.push({ name: 'input' })
}
</script>

<template>
  <div class="view-container">
    <h2>模式与板块</h2>
    <p class="view-desc">选择解读模式，配置输出板块组合</p>

    <!-- 模式选择 -->
    <div class="config-section">
      <h3>解读模式</h3>
      <el-radio-group v-model="mode">
        <el-radio-button value="single">单篇深读</el-radio-button>
        <el-radio-button value="multi">多篇策展</el-radio-button>
      </el-radio-group>
    </div>

    <!-- 多篇模式：主题信息 -->
    <div v-if="mode === 'multi'" class="config-section">
      <h3>主题信息</h3>
      <el-form label-position="top" size="small">
        <el-form-item label="主题名称">
          <el-input v-model="themeName" placeholder="如：美工刀系列" />
        </el-form-item>
        <el-form-item label="主题描述">
          <el-input v-model="themeDescription" type="textarea" :rows="2" placeholder="简要描述主题背景" />
        </el-form-item>
      </el-form>
    </div>

    <!-- 预设选择 -->
    <div class="config-section">
      <h3>快速预设</h3>
      <div class="preset-buttons">
        <el-button @click="applyPreset('quick')">快速概览</el-button>
        <el-button type="primary" @click="applyPreset('standard')">标准解读</el-button>
        <el-button @click="applyPreset('deep')">深度研读</el-button>
      </div>
    </div>

    <!-- 必要板块 -->
    <div class="config-section">
      <h3>必要板块（恒定输出）</h3>
      <div class="module-grid">
        <div v-for="mod in requiredModules" :key="mod.id" class="module-card required">
          <div class="module-header">
            <span class="module-id">{{ mod.id }}</span>
            <span class="module-name">{{ mod.name }}</span>
            <el-tag v-if="mod.aiGenerated" type="warning" size="small">AI</el-tag>
          </div>
          <p class="module-desc">{{ mod.description }}</p>
        </div>
      </div>
    </div>

    <!-- 拓展板块 -->
    <div class="config-section">
      <h3>拓展板块（可配置）</h3>
      <div class="module-grid">
        <div v-for="mod in extendedModules" :key="mod.id" class="module-card">
          <div class="module-header">
            <span class="module-id">{{ mod.id }}</span>
            <span class="module-name">{{ mod.name }}</span>
            <el-tag v-if="mod.aiGenerated" type="warning" size="small">AI</el-tag>
          </div>
          <p class="module-desc">{{ mod.description }}</p>
          <el-radio-group
            :model-value="getModuleLevel(mod.id)"
            @update:model-value="(v: any) => setModuleLevel(mod.id, v)"
            size="small"
          >
            <el-radio-button value="full">Full</el-radio-button>
            <el-radio-button value="lite">Lite</el-radio-button>
            <el-radio-button value="off">Off</el-radio-button>
          </el-radio-group>
        </div>
      </div>
    </div>

    <!-- 专利列表概览 -->
    <div v-if="inputStore.patents.length > 0" class="config-section">
      <h3>已输入专利（{{ inputStore.patents.length }} 篇）</h3>
      <div class="patent-chips">
        <el-tag
          v-for="(p, i) in inputStore.patents"
          :key="i"
          closable
          @close="inputStore.removePatent(i)"
        >
          {{ p.title || p.publicationNumber || `专利 ${i + 1}` }}
        </el-tag>
      </div>
    </div>

    <!-- 导航 -->
    <div class="view-footer">
      <el-button @click="goBack">上一步</el-button>
      <el-button type="primary" @click="goNext">
        下一步：AI 配置
        <el-icon class="el-icon--right"><ArrowRight /></el-icon>
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.view-container h2 {
  font-size: 18px;
  font-weight: 600;
  margin-bottom: 4px;
}

.view-desc {
  color: #909399;
  font-size: 13px;
  margin-bottom: 20px;
}

.config-section {
  background: #fff;
  border: 1px solid var(--app-border);
  border-radius: 8px;
  padding: 16px;
  margin-bottom: 16px;
}

.config-section h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 12px;
}

.preset-buttons {
  display: flex;
  gap: 8px;
}

.module-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
}

.module-card {
  background: #f5f7fa;
  border-radius: 6px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.module-card.required {
  background: #f0f9eb;
}

.module-header {
  display: flex;
  align-items: center;
  gap: 6px;
}

.module-id {
  background: #409eff;
  color: #fff;
  padding: 1px 6px;
  border-radius: 4px;
  font-size: 11px;
  font-weight: 600;
}

.module-name {
  font-size: 13px;
  font-weight: 500;
}

.module-desc {
  font-size: 12px;
  color: #909399;
}

.patent-chips {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
}

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
