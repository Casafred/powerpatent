<script setup lang="ts">
import { ref } from 'vue'
import { useAIConfigStore } from '../../stores/aiConfig'
import { PROVIDER_PRESETS, type ProviderType, type ConnectionTestResult } from '../../types/ai'
import { testAiConnection } from '../../services/tauri'

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
</script>

<template>
  <el-drawer
    :model-value="visible"
    title="设置"
    direction="rtl"
    size="420px"
    @close="close"
  >
    <!-- AI 服务商配置 -->
    <div class="settings-section">
      <h3>AI 服务商</h3>
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
</style>
