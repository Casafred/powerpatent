<script setup lang="ts">
import { computed } from 'vue'
import { useAIConfigStore } from '../stores/aiConfig'
import { useRouter } from 'vue-router'
import type { ProviderType } from '../types/ai'
import { PROVIDER_PRESETS as providerPresets } from '../services/ai'

const store = useAIConfigStore()
const router = useRouter()

const analysisProvider = computed(() => store.config.analysis)
const translateProvider = computed(() => store.config.translate)
const ocrEngine = computed({
  get: () => store.config.ocr,
  set: (v) => store.updateConfig({ ocr: v }),
})

function onProviderTypeChange(target: 'analysis' | 'translate', type: ProviderType) {
  const preset = providerPresets[type]
  if (target === 'analysis') {
    store.config.analysis.type = type
    store.config.analysis.baseUrl = preset.baseUrl
    store.config.analysis.model = preset.models[0]
  } else {
    store.config.translate.type = type
    store.config.translate.baseUrl = preset.baseUrl
    store.config.translate.model = preset.models[1] || preset.models[0]
  }
}

function goNext() {
  router.push({ name: 'generate' })
}

function goBack() {
  router.push({ name: 'config' })
}
</script>

<template>
  <div class="view-container">
    <h2>AI 配置</h2>
    <p class="view-desc">选择 AI 服务商，配置 API Key 和模型</p>

    <!-- 分析模型配置 -->
    <div class="config-section">
      <h3>分析模型（强模型，用于概要/权要解读/对比等）</h3>
      <el-form label-position="top" size="small">
        <el-form-item label="服务商">
          <el-select
            :model-value="analysisProvider.type"
            @update:model-value="(v: any) => onProviderTypeChange('analysis', v)"
          >
            <el-option label="DeepSeek" value="deepseek" />
            <el-option label="智谱 GLM" value="zhipu" />
            <el-option label="OpenAI 兼容" value="openai" />
          </el-select>
        </el-form-item>
        <el-form-item label="API Key">
          <el-input v-model="analysisProvider.apiKey" type="password" show-password placeholder="输入 API Key" />
        </el-form-item>
        <el-form-item label="Base URL">
          <el-input v-model="analysisProvider.baseUrl" />
        </el-form-item>
        <el-form-item label="模型">
          <el-input v-model="analysisProvider.model" />
        </el-form-item>
      </el-form>
    </div>

    <!-- 翻译/快速模型配置 -->
    <div class="config-section">
      <h3>快速模型（用于结构化归纳/列表等）</h3>
      <el-form label-position="top" size="small">
        <el-form-item label="服务商">
          <el-select
            :model-value="translateProvider.type"
            @update:model-value="(v: any) => onProviderTypeChange('translate', v)"
          >
            <el-option label="DeepSeek" value="deepseek" />
            <el-option label="智谱 GLM" value="zhipu" />
            <el-option label="OpenAI 兼容" value="openai" />
          </el-select>
        </el-form-item>
        <el-form-item label="API Key">
          <el-input v-model="translateProvider.apiKey" type="password" show-password placeholder="输入 API Key" />
        </el-form-item>
        <el-form-item label="Base URL">
          <el-input v-model="translateProvider.baseUrl" />
        </el-form-item>
        <el-form-item label="模型">
          <el-input v-model="translateProvider.model" />
        </el-form-item>
      </el-form>
    </div>

    <!-- OCR 引擎 -->
    <div class="config-section">
      <h3>OCR 引擎</h3>
      <el-radio-group v-model="ocrEngine">
        <el-radio value="paddleocr">PaddleOCR-VL（免费）</el-radio>
        <el-radio value="glm">GLM OCR（付费，精度更高）</el-radio>
      </el-radio-group>
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

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
