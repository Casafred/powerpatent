<script setup lang="ts">
import { ref, reactive, onMounted, watch } from 'vue'
import { useAIConfigStore } from '../../stores/aiConfig'
import { PROVIDER_PRESETS, type ProviderType, type ConnectionTestResult } from '../../types/ai'
import { testAiConnection, listPrompts, savePrompt, resetPrompt, type PromptTemplate } from '../../services/tauri'

const store = useAIConfigStore()
const visible = defineModel<boolean>('visible', { default: false })

const testingProvider = ref<ProviderType | null>(null)
const testResults = ref<Record<ProviderType, ConnectionTestResult | null>>({
  deepseek: null,
  zhipu: null,
  openai: null,
})

async function testConnection(type: ProviderType) {
  const provider = store.config.providers[type]
  if (!provider.apiKey) {
    testResults.value[type] = { success: false, message: '请先输入 API Key' }
    return
  }
  testingProvider.value = type
  testResults.value[type] = null
  try {
    testResults.value[type] = await testAiConnection({
      providerType: type,
      apiKey: provider.apiKey,
      baseUrl: provider.baseUrl,
      model: provider.model,
    })
  } catch (e: any) {
    testResults.value[type] = { success: false, message: e?.message || '测试失败' }
  } finally {
    testingProvider.value = null
  }
}

function onProviderTypeChange(type: ProviderType) {
  const preset = PROVIDER_PRESETS[type]
  store.updateProvider(type, { baseUrl: preset.baseUrl, model: preset.models[0] })
}

function close() {
  visible.value = false
}

// --- 提示词 Tab ---
const prompts = ref<PromptTemplate[]>([])
const promptsLoading = ref(false)
const selectedPromptId = ref<string | null>(null)
const editingPrompt = reactive({
  name: '',
  description: '',
  temperature: 0.7,
  template: '',
})
const saving = ref(false)
const resetting = ref(false)

const selectedPrompt = () => prompts.value.find(p => p.id === selectedPromptId.value) || null

async function loadPrompts() {
  promptsLoading.value = true
  try {
    prompts.value = await listPrompts()
    if (prompts.value.length > 0 && !selectedPromptId.value) {
      selectPrompt(prompts.value[0].id)
    }
  } catch {
    prompts.value = []
  } finally {
    promptsLoading.value = false
  }
}

function selectPrompt(id: string) {
  selectedPromptId.value = id
  const p = prompts.value.find(item => item.id === id)
  if (p) {
    editingPrompt.name = p.name
    editingPrompt.description = p.description
    editingPrompt.temperature = p.temperature
    editingPrompt.template = p.template
  }
}

async function handleSavePrompt() {
  if (!selectedPromptId.value) return
  saving.value = true
  try {
    await savePrompt({
      promptId: selectedPromptId.value,
      name: editingPrompt.name,
      description: editingPrompt.description,
      temperature: editingPrompt.temperature,
      template: editingPrompt.template,
    })
    await loadPrompts()
  } finally {
    saving.value = false
  }
}

async function handleResetPrompt() {
  if (!selectedPromptId.value) return
  resetting.value = true
  try {
    await resetPrompt({ promptId: selectedPromptId.value })
    await loadPrompts()
    selectPrompt(selectedPromptId.value!)
  } finally {
    resetting.value = false
  }
}

onMounted(() => {
  loadPrompts()
})

watch(visible, (val) => {
  if (val) {
    loadPrompts()
  }
})
</script>

<template>
  <el-drawer
    :model-value="visible"
    title="设置"
    direction="rtl"
    size="520px"
    @close="close"
  >
    <el-tabs>
      <el-tab-pane label="AI 服务商">
        <!-- AI 服务商配置 -->
        <div class="settings-section">
          <p class="section-hint">选择默认分析服务商并配置 API Key，至少配置一个</p>

          <div
            v-for="(preset, type) in PROVIDER_PRESETS"
            :key="type"
            class="provider-card"
            :class="{ active: store.config.activeProvider === type }"
            @click="store.setActiveProvider(type as ProviderType)"
          >
            <div class="provider-header">
              <el-radio
                :model-value="store.config.activeProvider"
                :value="type"
                @click.stop
                @change="store.setActiveProvider(type as ProviderType)"
              >
                {{ preset.name }}
              </el-radio>
              <el-tag
                v-if="store.config.providers[type as ProviderType].apiKey"
                type="success"
                size="small"
              >已配置</el-tag>
            </div>

            <div v-if="store.config.activeProvider === type" class="provider-form">
              <el-form label-position="top" size="small">
                <el-form-item label="API Key">
                  <el-input
                    :model-value="store.config.providers[type as ProviderType].apiKey"
                    @update:model-value="(v: string) => store.updateProvider(type as ProviderType, { apiKey: v })"
                    type="password"
                    show-password
                    placeholder="输入 API Key"
                  />
                </el-form-item>
                <el-form-item label="Base URL">
                  <el-input
                    :model-value="store.config.providers[type as ProviderType].baseUrl"
                    @update:model-value="(v: string) => store.updateProvider(type as ProviderType, { baseUrl: v })"
                    @change="onProviderTypeChange(type as ProviderType)"
                  />
                </el-form-item>
                <el-form-item label="模型">
                  <el-select
                    :model-value="store.config.providers[type as ProviderType].model"
                    @update:model-value="(v: string) => store.updateProvider(type as ProviderType, { model: v })"
                  >
                    <el-option
                      v-for="model in preset.models"
                      :key="model"
                      :label="model"
                      :value="model"
                    />
                  </el-select>
                </el-form-item>
              </el-form>

              <div class="test-row">
                <el-button
                  size="small"
                  :loading="testingProvider === type"
                  @click="testConnection(type as ProviderType)"
                >
                  测试连接
                </el-button>
                <span
                  v-if="testResults[type as ProviderType]"
                  class="test-result"
                  :class="testResults[type as ProviderType]?.success ? 'success' : 'error'"
                >
                  {{ testResults[type as ProviderType]?.message }}
                </span>
              </div>
            </div>
          </div>
        </div>

        <!-- OCR 引擎 -->
        <div class="settings-section">
          <h3>OCR 引擎</h3>
          <el-radio-group
            :model-value="store.config.ocr"
            @update:model-value="(v: any) => store.updateConfig({ ocr: v })"
          >
            <el-radio value="paddle_ocr_vl">PaddleOCR-VL（免费，推荐）</el-radio>
            <el-radio value="glm_ocr">GLM OCR（需智谱 API Key）</el-radio>
          </el-radio-group>
        </div>

        <!-- 重置 -->
        <div class="settings-section">
          <el-button type="danger" text size="small" @click="store.resetConfig()">
            重置所有设置
          </el-button>
        </div>
      </el-tab-pane>

      <el-tab-pane label="提示词">
        <div v-loading="promptsLoading" class="prompts-container">
          <div v-if="prompts.length === 0 && !promptsLoading" class="prompts-empty">
            暂无提示词模板
          </div>
          <div v-else class="prompts-layout">
            <!-- 左侧列表 -->
            <div class="prompts-list">
              <div
                v-for="p in prompts"
                :key="p.id"
                class="prompt-item"
                :class="{ active: selectedPromptId === p.id }"
                @click="selectPrompt(p.id)"
              >
                <span class="prompt-item-name">{{ p.name }}</span>
                <el-tag
                  v-if="p.isUserModified"
                  type="warning"
                  size="small"
                >已自定义</el-tag>
              </div>
            </div>

            <!-- 右侧详情 -->
            <div v-if="selectedPrompt()" class="prompts-detail">
              <el-form label-position="top" size="small">
                <el-form-item label="名称">
                  <el-input v-model="editingPrompt.name" />
                </el-form-item>
                <el-form-item label="描述">
                  <el-input v-model="editingPrompt.description" type="textarea" :rows="2" />
                </el-form-item>
                <el-form-item label="Temperature">
                  <div class="temperature-row">
                    <el-slider
                      v-model="editingPrompt.temperature"
                      :min="0"
                      :max="2"
                      :step="0.1"
                      :show-tooltip="false"
                      style="flex: 1"
                    />
                    <span class="temperature-value">{{ editingPrompt.temperature.toFixed(1) }}</span>
                  </div>
                </el-form-item>
                <el-form-item label="模板内容">
                  <el-input
                    v-model="editingPrompt.template"
                    type="textarea"
                    :rows="8"
                    class="prompt-template-textarea"
                  />
                </el-form-item>
              </el-form>
              <div class="prompts-actions">
                <el-button
                  type="primary"
                  size="small"
                  :loading="saving"
                  @click="handleSavePrompt"
                >保存</el-button>
                <el-button
                  size="small"
                  :loading="resetting"
                  @click="handleResetPrompt"
                >重置</el-button>
              </div>
            </div>
            <div v-else class="prompts-detail prompts-detail-empty">
              请在左侧选择一个提示词模板
            </div>
          </div>
        </div>
      </el-tab-pane>
    </el-tabs>
  </el-drawer>
</template>

<style scoped>
.settings-section {
  margin-bottom: 24px;
}

.settings-section h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 8px;
}

.section-hint {
  font-size: 12px;
  color: var(--app-text-secondary);
  margin-bottom: 12px;
}

.provider-card {
  background: var(--app-module-bg);
  border: 2px solid transparent;
  border-radius: 8px;
  padding: 12px;
  margin-bottom: 8px;
  cursor: pointer;
  transition: border-color 0.2s, background 0.2s;
}

.provider-card:hover {
  background: var(--app-hover-bg);
}

.provider-card.active {
  border-color: #409eff;
  background: var(--app-active-bg);
}

.provider-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
}

.provider-form {
  margin-top: 12px;
  padding-top: 12px;
  border-top: 1px solid var(--app-border);
}

.test-row {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 8px;
}

.test-result {
  font-size: 12px;
}

.test-result.success {
  color: #67c23a;
}

.test-result.error {
  color: #f56c6c;
}

/* 提示词 Tab */
.prompts-container {
  min-height: 200px;
}

.prompts-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  height: 200px;
  color: var(--app-text-secondary);
  font-size: 13px;
}

.prompts-layout {
  display: flex;
  gap: 16px;
  height: calc(100vh - 180px);
  min-height: 400px;
}

.prompts-list {
  width: 160px;
  flex-shrink: 0;
  border-right: 1px solid var(--app-border);
  padding-right: 12px;
  overflow-y: auto;
}

.prompt-item {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 6px;
  padding: 8px 10px;
  border-radius: 6px;
  cursor: pointer;
  font-size: 13px;
  color: var(--app-text);
  transition: background 0.2s;
  margin-bottom: 4px;
}

.prompt-item:hover {
  background: var(--app-hover-bg);
}

.prompt-item.active {
  background: var(--app-active-bg);
  color: #409eff;
  font-weight: 500;
}

.prompt-item-name {
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.prompts-detail {
  flex: 1;
  overflow-y: auto;
  padding-left: 4px;
}

.prompts-detail-empty {
  display: flex;
  align-items: center;
  justify-content: center;
  color: var(--app-text-secondary);
  font-size: 13px;
}

.temperature-row {
  display: flex;
  align-items: center;
  gap: 12px;
  width: 100%;
}

.temperature-value {
  font-size: 13px;
  color: var(--app-text);
  min-width: 28px;
  text-align: right;
  font-variant-numeric: tabular-nums;
}

.prompt-template-textarea :deep(.el-textarea__inner) {
  font-family: 'Menlo', 'Monaco', 'Consolas', monospace;
  font-size: 12px;
  line-height: 1.6;
}

.prompts-actions {
  display: flex;
  gap: 8px;
  margin-top: 12px;
}
</style>
