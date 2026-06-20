<script setup lang="ts">
import { ref, computed } from 'vue'
import { useInputStore } from '../stores/input'
import { useModuleConfigStore } from '../stores/moduleConfig'
import { useAIConfigStore } from '../stores/aiConfig'
import { useProjectStore } from '../stores/project'
import { useAnnotationStore } from '../stores/annotation'
import { useHistoryStore } from '../stores/history'
import { generateModule, getCachedModule, rerunModule } from '../services/tauri'
import { MODULE_REGISTRY, type ModuleId, type ModuleLevel } from '../types/module'
import { getActiveProvider } from '../types/ai'
import AnnotationPanel from '../components/annotation/AnnotationPanel.vue'
import AnnotationSummary from '../components/annotation/AnnotationSummary.vue'
import ModulePreview from '../components/ModulePreview.vue'
import PdfViewer from '../components/PdfViewer.vue'
import { convertFileSrc } from '@tauri-apps/api/core'
import { useRouter } from 'vue-router'
import { ElNotification } from 'element-plus'

const inputStore = useInputStore()
const configStore = useModuleConfigStore()
const aiConfigStore = useAIConfigStore()
const projectStore = useProjectStore()
const annotationStore = useAnnotationStore()
const historyStore = useHistoryStore()
const router = useRouter()

// 每个板块的生成状态
interface ModuleState {
  id: string
  name: string
  status: 'pending' | 'generating' | 'done' | 'error' | 'cached' | 'skipped'
  output: any
  error: string | null
  model: string
  cached: boolean
  aiGenerated: boolean
}

const moduleStates = ref<ModuleState[]>([])
const generating = ref(false)
const currentModule = ref<string | null>(null)

// 需要展示的所有板块（含非 AI 模块）
const allActiveModules = computed(() => {
  const levels = configStore.globalExtended
  return MODULE_REGISTRY
    .filter(m => {
      if (m.required) return true // 必要板块始终显示
      const level = levels[m.id as ModuleId] as ModuleLevel | undefined
      return level !== 'off'
    })
})

// 需要调用 AI 生成的板块
const aiModules = computed(() => allActiveModules.value.filter(m => m.aiGenerated))

// 初始化板块状态
function initModuleStates() {
  moduleStates.value = allActiveModules.value.map(m => ({
    id: m.id,
    name: m.name,
    status: m.aiGenerated ? 'pending' : 'skipped' as const,
    output: null,
    error: null,
    model: '',
    cached: false,
    aiGenerated: m.aiGenerated,
  }))
}

// 获取 PDF 可访问 URL
function getPdfSrc(patent: any): string {
  if (!patent?.pdfFilePath) return ''
  try {
    return convertFileSrc(patent.pdfFilePath)
  } catch {
    return ''
  }
}

function getPatentId(patent: any): string {
  return patent?.publicationNumber || patent?.applicationNumber || 'unknown'
}

// 生成所有板块
async function generateAll() {
  if (inputStore.patents.length === 0) {
    ElNotification({ title: '无法生成', message: '请先在"输入材料"步骤添加专利', type: 'warning' })
    return
  }

  if (!getActiveProvider(aiConfigStore.config).apiKey) {
    ElNotification({ title: '缺少 API Key', message: '请先在设置中配置 AI 服务商 API Key', type: 'warning' })
    return
  }

  initModuleStates()
  generating.value = true

  const provider = getActiveProvider(aiConfigStore.config)
  let errorCount = 0

  for (const patent of inputStore.patents) {
    const patentId = getPatentId(patent)

    for (let i = 0; i < moduleStates.value.length; i++) {
      const mod = moduleStates.value[i]
      if (!mod.aiGenerated) continue // 非 AI 模块跳过生成

      const level = (configStore.globalExtended[mod.id as ModuleId] as ModuleLevel) || 'full'
      currentModule.value = mod.id
      mod.status = 'generating'

      try {
        // 先查缓存
        const cached = await getCachedModule({
          projectId: projectStore.projectId,
          patentId,
          moduleId: mod.id,
        })

        if (cached) {
          mod.status = 'cached'
          mod.output = cached.output
          mod.model = cached.model
          mod.cached = true
          continue
        }

        // 调用 AI 生成
        const result = await generateModule({
          projectId: projectStore.projectId,
          patentId,
          moduleId: mod.id,
          level,
          provider,
          patentData: patent,
        })

        mod.status = 'done'
        mod.output = result.output
        mod.model = result.model
        mod.cached = result.cached
      } catch (e: any) {
        mod.status = 'error'
        mod.error = e?.toString() || '生成失败'
        errorCount++
      }
    }
  }

  generating.value = false
  currentModule.value = null

  // 自动保存历史记录
  const successCount = moduleStates.value.filter(m => m.status === 'done' || m.status === 'cached').length
  if (successCount > 0) {
    historyStore.saveSession({
      projectId: projectStore.projectId,
      patents: inputStore.patents,
      moduleConfig: configStore.getConfig(),
    })
  }

  if (errorCount > 0) {
    ElNotification({
      title: '生成完成（部分失败）',
      message: `${errorCount} 个板块生成失败，可点击"重跑"重试`,
      type: 'warning',
    })
  } else if (aiModules.value.length > 0) {
    ElNotification({ title: '生成完成', message: '所有板块已成功生成', type: 'success' })
  }
}

// 重跑单个板块
async function rerun(index: number) {
  const mod = moduleStates.value[index]
  if (!mod || inputStore.patents.length === 0) return

  const patent = inputStore.patents[0]
  const patentId = getPatentId(patent)
  const level = (configStore.globalExtended[mod.id as ModuleId] as ModuleLevel) || 'full'

  mod.status = 'generating'
  mod.error = null

  try {
    const result = await rerunModule({
      projectId: projectStore.projectId,
      patentId,
      moduleId: mod.id,
      options: {
        provider: getActiveProvider(aiConfigStore.config),
        patent_data: patent,
        level,
      },
    })

    mod.status = 'done'
    mod.output = result.output
    mod.model = result.model
    mod.cached = false
  } catch (e: any) {
    mod.status = 'error'
    mod.error = e?.toString() || '重跑失败'
    ElNotification({ title: '重跑失败', message: mod.error || '未知错误', type: 'error' })
  }
}

function getStatusType(status: ModuleState['status']) {
  switch (status) {
    case 'pending': return 'info'
    case 'generating': return 'warning'
    case 'done': return 'success'
    case 'cached': return 'success'
    case 'error': return 'danger'
    case 'skipped': return 'info'
  }
}

function getStatusText(status: ModuleState['status']) {
  switch (status) {
    case 'pending': return '待生成'
    case 'generating': return '生成中...'
    case 'done': return '已完成'
    case 'cached': return '已缓存'
    case 'error': return '失败'
    case 'skipped': return '无需AI'
  }
}

function goNext() {
  router.push({ name: 'export' })
}
</script>

<template>
  <div class="view-container">
    <h2>生成与重跑</h2>
    <p class="view-desc">AI 逐板块生成解读内容，支持板块级重跑</p>

    <!-- 空状态：未开始生成 -->
    <div v-if="moduleStates.length === 0" class="empty-state">
      <el-icon :size="48" color="var(--app-text-placeholder)"><VideoPlay /></el-icon>
      <p class="empty-title">准备生成</p>
      <p class="empty-desc">
        {{ inputStore.patents.length === 0
          ? '请先在"输入材料"步骤添加专利'
          : `${inputStore.patents.length} 篇专利 × ${allActiveModules.length} 个板块（其中 ${aiModules.length} 个需 AI 生成）`
        }}
      </p>
      <el-button
        type="primary"
        size="large"
        :disabled="inputStore.patents.length === 0"
        @click="generateAll"
      >
        <el-icon><VideoPlay /></el-icon>
        开始生成
      </el-button>
    </div>

    <!-- 生成进度面板 -->
    <div v-if="moduleStates.length > 0" class="progress-panel">
      <div class="progress-header">
        <span>生成进度（{{ moduleStates.filter(m => m.status === 'done' || m.status === 'cached').length }}/{{ aiModules.length }} AI 板块）</span>
        <el-button v-if="!generating" type="primary" size="small" @click="generateAll">
          重新生成全部
        </el-button>
      </div>

      <div class="module-list">
        <div
          v-for="(mod, index) in moduleStates"
          :key="mod.id"
          class="module-item"
          :class="{ active: currentModule === mod.id }"
        >
          <div class="module-info">
            <span class="module-id">{{ mod.id }}</span>
            <span class="module-name">{{ mod.name }}</span>
            <el-tag :type="getStatusType(mod.status)" size="small" round>
              {{ getStatusText(mod.status) }}
            </el-tag>
          </div>

          <div class="module-actions">
            <span v-if="mod.model" class="model-tag">{{ mod.model }}</span>
            <el-button
              v-if="mod.aiGenerated && (mod.status === 'done' || mod.status === 'cached' || mod.status === 'error')"
              type="primary"
              text
              size="small"
              @click="rerun(index)"
            >
              重跑
            </el-button>
          </div>

          <div v-if="mod.status === 'generating'" class="module-progress">
            <el-progress :percentage="100" :indeterminate="true" :show-text="false" />
          </div>

          <div v-if="mod.error" class="module-error">
            {{ mod.error }}
          </div>

          <!-- AI 输出预览 -->
          <div v-if="mod.output && (mod.status === 'done' || mod.status === 'cached')" class="module-output">
            <ModulePreview :module-id="mod.id" :output="mod.output" />
          </div>

          <!-- E1 PDF 原文浏览 -->
          <div v-if="mod.id === 'E1'" class="module-pdf-section">
            <div v-for="patent in inputStore.patents" :key="getPatentId(patent)" class="pdf-patent-block">
              <div v-if="getPdfSrc(patent)" class="pdf-wrapper">
                <div class="pdf-patent-label">{{ patent.title || getPatentId(patent) }}</div>
                <PdfViewer :src="getPdfSrc(patent)" />
              </div>
              <div v-else class="pdf-unavailable">
                <el-icon :size="20" color="#909399"><Document /></el-icon>
                <span>未检测到 PDF 文件路径，请确认已通过 PDF 上传输入</span>
              </div>
            </div>
          </div>

          <!-- E3 批注汇总 -->
          <div v-if="mod.id === 'E3'" class="module-annotation-summary">
            <div v-for="patent in inputStore.patents" :key="getPatentId(patent)" class="annotation-patent-block">
              <AnnotationSummary :patent-id="getPatentId(patent)" />
            </div>
          </div>

          <!-- 批注面板（每个 AI 模块下方） -->
          <div v-if="mod.aiGenerated && (mod.status === 'done' || mod.status === 'cached')" class="module-annotation">
            <AnnotationPanel
              :patent-id="getPatentId(inputStore.patents[0])"
              :module-id="mod.id"
            />
          </div>
        </div>
      </div>
    </div>

    <!-- 导航 -->
    <div class="view-footer">
      <el-button @click="router.push({ name: 'config' })">返回配置</el-button>
      <el-button
        type="primary"
        :disabled="moduleStates.length === 0 || generating"
        @click="goNext"
      >
        前往导出
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

/* 空状态 */
.empty-state {
  text-align: center;
  padding: 48px 0;
  background: var(--app-card-bg);
  border: 1px solid var(--app-border);
  border-radius: 8px;
}

.empty-title {
  font-size: 16px;
  font-weight: 500;
  margin-top: 12px;
  color: var(--app-text);
}

.empty-desc {
  color: var(--app-text-secondary);
  font-size: 13px;
  margin-top: 4px;
  margin-bottom: 16px;
}

.progress-panel {
  background: var(--app-card-bg);
  border: 1px solid var(--app-border);
  border-radius: 8px;
  padding: 16px;
}

.progress-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  font-size: 14px;
  font-weight: 600;
}

.module-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.module-item {
  padding: 12px;
  background: var(--app-module-bg);
  border-radius: 6px;
  border-left: 3px solid transparent;
}

.module-item.active {
  border-left-color: #409eff;
  background: var(--app-active-bg);
}

.module-info {
  display: flex;
  align-items: center;
  gap: 8px;
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
  flex: 1;
}

.module-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}

.model-tag {
  font-size: 11px;
  color: var(--app-text-secondary);
  background: var(--app-hover-bg);
  padding: 1px 6px;
  border-radius: 3px;
}

.module-progress {
  margin-top: 8px;
}

.module-error {
  margin-top: 6px;
  color: #f56c6c;
  font-size: 12px;
}

.module-output {
  margin-top: 8px;
  background: var(--app-code-bg);
  border: 1px solid var(--app-border);
  border-radius: 4px;
  padding: 8px;
  max-height: 400px;
  overflow-y: auto;
}

.module-annotation {
  margin-top: 8px;
}

/* E1 PDF */
.module-pdf-section {
  margin-top: 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.pdf-patent-block {
  border: 1px solid var(--app-border);
  border-radius: 6px;
  overflow: hidden;
}

.pdf-patent-label {
  padding: 6px 10px;
  font-size: 12px;
  font-weight: 500;
  color: var(--app-text-secondary);
  background: var(--app-module-bg);
  border-bottom: 1px solid var(--app-border);
}

.pdf-wrapper {
  height: 500px;
}

.pdf-unavailable {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 16px;
  color: var(--app-text-placeholder);
  font-size: 13px;
}

/* E3 批注汇总 */
.module-annotation-summary {
  margin-top: 8px;
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.annotation-patent-block {
  border: 1px solid var(--app-border);
  border-radius: 6px;
  padding: 8px;
}

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
