<script setup lang="ts">
import { computed, ref } from 'vue'
import { useModuleConfigStore, PRESETS, type PresetKey } from '../stores/moduleConfig'
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

// 当前展开的逐篇配置专利 ID
const expandedPatentId = ref<string | null>(null)

function getModuleLevel(id: ModuleId): ModuleLevel {
  return (configStore.globalExtended[id] as ModuleLevel) ?? 'off'
}

function setModuleLevel(id: ModuleId, level: ModuleLevel) {
  configStore.setModuleLevel(id, level)
}

function applyPreset(key: PresetKey) {
  configStore.applyPreset(key)
}

function getPatentLabel(patent: any, index: number): string {
  return patent.title || patent.publicationNumber || patent.applicationNumber || `专利 ${index + 1}`
}

function getPatentId(patent: any): string {
  return patent.publicationNumber || patent.applicationNumber || `patent-${Date.now()}`
}

function togglePatentExpand(patentId: string) {
  expandedPatentId.value = expandedPatentId.value === patentId ? null : patentId
}

function goNext() {
  router.push({ name: 'generate' })
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
      <div class="preset-cards">
        <div
          v-for="(preset, key) in PRESETS"
          :key="key"
          class="preset-card"
          :class="{ active: configStore.activePreset === key }"
          @click="applyPreset(key as PresetKey)"
        >
          <div class="preset-label">{{ preset.label }}</div>
          <div class="preset-desc">{{ preset.desc }}</div>
          <div class="preset-modules">
            <span
              v-for="mod in extendedModules"
              :key="mod.id"
              class="preset-dot"
              :class="'level-' + preset.levels[mod.id]"
              :title="mod.name + ': ' + preset.levels[mod.id]"
            >{{ mod.id }}</span>
          </div>
        </div>
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

    <!-- 逐篇板块配置 -->
    <div v-if="inputStore.patents.length > 0" class="config-section">
      <h3>逐篇板块配置</h3>
      <p class="section-hint">点击专利名称展开，可为每篇专利单独设置板块级别（覆盖全局设置）</p>
      <div class="patent-config-list">
        <div
          v-for="(patent, index) in inputStore.patents"
          :key="index"
          class="patent-config-item"
        >
          <div
            class="patent-config-header"
            @click="togglePatentExpand(getPatentId(patent))"
          >
            <span class="patent-config-name">{{ getPatentLabel(patent, index) }}</span>
            <span v-if="patent.publicationNumber" class="patent-config-id">{{ patent.publicationNumber }}</span>
            <div class="patent-config-right">
              <el-tag
                v-if="configStore.patentOverrides[getPatentId(patent)]"
                type="warning"
                size="small"
              >已自定义</el-tag>
              <el-icon :class="{ rotated: expandedPatentId === getPatentId(patent) }">
                <ArrowDown />
              </el-icon>
            </div>
          </div>

          <!-- 展开的逐篇配置 -->
          <div v-if="expandedPatentId === getPatentId(patent)" class="patent-config-body">
            <div v-if="mode === 'multi'" class="patent-key-toggle">
              <el-checkbox
                :model-value="configStore.patentOverrides[getPatentId(patent)]?.isKey ?? false"
                @update:model-value="(v: boolean) => configStore.setKeyPatent(getPatentId(patent), v)"
              >
                标记为关键专利
              </el-checkbox>
            </div>
            <div class="patent-module-grid">
              <div
                v-for="mod in extendedModules.filter(m => m.aiGenerated)"
                :key="mod.id"
                class="patent-module-row"
              >
                <span class="patent-module-name">{{ mod.id }} {{ mod.name }}</span>
                <el-radio-group
                  :model-value="configStore.getEffectiveLevel(getPatentId(patent), mod.id)"
                  @update:model-value="(v: any) => configStore.setPatentModuleLevel(getPatentId(patent), mod.id, v)"
                  size="small"
                >
                  <el-radio-button value="full">Full</el-radio-button>
                  <el-radio-button value="lite">Lite</el-radio-button>
                  <el-radio-button value="off">Off</el-radio-button>
                </el-radio-group>
              </div>
            </div>
            <div class="patent-config-actions">
              <el-button
                size="small"
                text
                @click="configStore.clearPatentOverride(getPatentId(patent))"
              >
                重置为全局设置
              </el-button>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- 导航 -->
    <div class="view-footer">
      <el-button @click="goBack">上一步</el-button>
      <el-button type="primary" @click="goNext">
        下一步：生成与重跑
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
  color: var(--app-text-secondary);
  font-size: 13px;
  margin-bottom: 20px;
}

.config-section {
  background: var(--app-card-bg);
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

.section-hint {
  font-size: 12px;
  color: var(--app-text-secondary);
  margin-bottom: 12px;
}

/* 预设卡片 */
.preset-cards {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(220px, 1fr));
  gap: 12px;
}

.preset-card {
  background: var(--app-module-bg);
  border: 2px solid transparent;
  border-radius: 8px;
  padding: 14px;
  cursor: pointer;
  transition: border-color 0.2s, background 0.2s;
}

.preset-card:hover {
  background: var(--app-hover-bg);
}

.preset-card.active {
  border-color: #409eff;
  background: var(--app-active-bg);
}

.preset-label {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.preset-desc {
  font-size: 12px;
  color: var(--app-text-secondary);
  margin-bottom: 10px;
}

.preset-modules {
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.preset-dot {
  font-size: 10px;
  padding: 1px 5px;
  border-radius: 3px;
  font-weight: 600;
}

.preset-dot.level-full {
  background: #409eff;
  color: #fff;
}

.preset-dot.level-lite {
  background: #e6a23c;
  color: #fff;
}

.preset-dot.level-off {
  background: var(--app-hover-bg);
  color: var(--app-text-secondary);
}

/* 板块网格 */
.module-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 12px;
}

.module-card {
  background: var(--app-module-bg);
  border-radius: 6px;
  padding: 12px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.module-card.required {
  background: var(--app-module-required-bg);
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
  color: var(--app-text-secondary);
}

/* 逐篇配置 */
.patent-config-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
}

.patent-config-item {
  background: var(--app-module-bg);
  border-radius: 6px;
  overflow: hidden;
}

.patent-config-header {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 10px 12px;
  cursor: pointer;
  transition: background 0.15s;
}

.patent-config-header:hover {
  background: var(--app-hover-bg);
}

.patent-config-name {
  font-size: 13px;
  font-weight: 500;
  flex: 1;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.patent-config-id {
  font-size: 11px;
  color: var(--app-text-secondary);
}

.patent-config-right {
  display: flex;
  align-items: center;
  gap: 6px;
}

.patent-config-right .el-icon {
  transition: transform 0.2s;
  font-size: 12px;
  color: var(--app-text-secondary);
}

.patent-config-right .el-icon.rotated {
  transform: rotate(180deg);
}

.patent-config-body {
  padding: 0 12px 12px;
  border-top: 1px solid var(--app-border);
}

.patent-key-toggle {
  padding: 8px 0 4px;
}

.patent-module-grid {
  display: flex;
  flex-direction: column;
  gap: 8px;
  padding-top: 8px;
}

.patent-module-row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
}

.patent-module-name {
  font-size: 12px;
  color: var(--app-text-secondary);
  white-space: nowrap;
}

.patent-config-actions {
  padding-top: 8px;
  text-align: right;
}

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
