<script setup lang="ts">
import { ref, computed } from 'vue'
import { useInputStore } from '../stores/input'
import { useModuleConfigStore } from '../stores/moduleConfig'
import { useAIConfigStore } from '../stores/aiConfig'
import { useProjectStore } from '../stores/project'
import { generateModule, getCachedModule, rerunModule } from '../services/tauri'
import { MODULE_REGISTRY, type ModuleId, type ModuleLevel } from '../types/module'
import { useRouter } from 'vue-router'

const inputStore = useInputStore()
const configStore = useModuleConfigStore()
const aiConfigStore = useAIConfigStore()
const projectStore = useProjectStore()
const router = useRouter()

// 每个板块的生成状态
interface ModuleState {
  id: string
  name: string
  status: 'pending' | 'generating' | 'done' | 'error' | 'cached'
  output: any
  error: string | null
  model: string
  cached: boolean
}

const moduleStates = ref<ModuleState[]>([])
const generating = ref(false)
const currentModule = ref<string | null>(null)

// 需要生成的板块列表
const modulesToGenerate = computed(() => {
  const levels = configStore.globalExtended
  return MODULE_REGISTRY
    .filter(m => {
      const level = levels[m.id as ModuleId] as ModuleLevel | undefined
      return level !== 'off' && m.aiGenerated
    })
    .map(m => ({
      id: m.id,
      name: m.name,
      level: (levels[m.id as ModuleId] as ModuleLevel) || 'full',
    }))
})

// 初始化板块状态
function initModuleStates() {
  moduleStates.value = modulesToGenerate.value.map(m => ({
    id: m.id,
    name: m.name,
    status: 'pending',
    output: null,
    error: null,
    model: '',
    cached: false,
  }))
}

// 生成所有板块
async function generateAll() {
  if (inputStore.patents.length === 0) {
    return
  }

  initModuleStates()
  generating.value = true

  const provider = aiConfigStore.config.analysis

  for (const patent of inputStore.patents) {
    const patentId = patent.publicationNumber || patent.applicationNumber || 'unknown'

    for (let i = 0; i < moduleStates.value.length; i++) {
      const mod = moduleStates.value[i]
      const moduleConfig = modulesToGenerate.value.find(m => m.id === mod.id)
      if (!moduleConfig) continue

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
          level: moduleConfig.level,
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
      }
    }
  }

  generating.value = false
  currentModule.value = null
}

// 重跑单个板块
async function rerun(index: number) {
  const mod = moduleStates.value[index]
  if (!mod || inputStore.patents.length === 0) return

  const patent = inputStore.patents[0]
  const patentId = patent.publicationNumber || patent.applicationNumber || 'unknown'
  const moduleConfig = modulesToGenerate.value.find(m => m.id === mod.id)

  mod.status = 'generating'
  mod.error = null

  try {
    const result = await rerunModule({
      projectId: projectStore.projectId,
      patentId,
      moduleId: mod.id,
      options: {
        provider: aiConfigStore.config.analysis,
        patent_data: patent,
        level: moduleConfig?.level || 'full',
      },
    })

    mod.status = 'done'
    mod.output = result.output
    mod.model = result.model
    mod.cached = false
  } catch (e: any) {
    mod.status = 'error'
    mod.error = e?.toString() || '重跑失败'
  }
}

function getStatusType(status: ModuleState['status']) {
  switch (status) {
    case 'pending': return 'info'
    case 'generating': return 'warning'
    case 'done': return 'success'
    case 'cached': return 'success'
    case 'error': return 'danger'
  }
}

function getStatusText(status: ModuleState['status']) {
  switch (status) {
    case 'pending': return '待生成'
    case 'generating': return '生成中...'
    case 'done': return '已完成'
    case 'cached': return '已缓存'
    case 'error': return '失败'
  }
}

function goNext() {
  router.push({ name: 'export' })
}

function goBack() {
  router.push({ name: 'ai' })
}
</script>

<template>
  <div class="view-container">
    <h2>生成与重跑</h2>
    <p class="view-desc">AI 逐板块生成解读内容，支持板块级重跑</p>

    <!-- 专利概览 -->
    <div v-if="inputStore.patents.length > 0" class="info-bar">
      <el-tag type="info">{{ inputStore.patents.length }} 篇专利</el-tag>
      <el-tag type="info">{{ modulesToGenerate.length }} 个 AI 板块</el-tag>
    </div>

    <!-- 生成按钮 -->
    <div class="action-bar" v-if="moduleStates.length === 0">
      <el-button
        type="primary"
        size="large"
        :disabled="inputStore.patents.length === 0"
        @click="generateAll"
      >
        <el-icon><VideoPlay /></el-icon>
        开始生成
      </el-button>
      <p v-if="inputStore.patents.length === 0" class="hint">请先在"输入材料"步骤添加专利</p>
    </div>

    <!-- 生成进度面板 -->
    <div v-if="moduleStates.length > 0" class="progress-panel">
      <div class="progress-header">
        <span>生成进度</span>
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
              v-if="mod.status === 'done' || mod.status === 'cached' || mod.status === 'error'"
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

          <!-- 输出预览 -->
          <div v-if="mod.output && (mod.status === 'done' || mod.status === 'cached')" class="module-output">
            <pre>{{ JSON.stringify(mod.output, null, 2).slice(0, 300) }}{{ JSON.stringify(mod.output, null, 2).length > 300 ? '...' : '' }}</pre>
          </div>
        </div>
      </div>
    </div>

    <!-- 导航 -->
    <div class="view-footer">
      <el-button @click="goBack">上一步</el-button>
      <el-button
        type="primary"
        :disabled="moduleStates.length === 0 || generating"
        @click="goNext"
      >
        下一步：预览与导出
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

.info-bar {
  display: flex;
  gap: 8px;
  margin-bottom: 16px;
}

.action-bar {
  text-align: center;
  padding: 48px 0;
}

.hint {
  color: #909399;
  font-size: 12px;
  margin-top: 8px;
}

.progress-panel {
  background: #fff;
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
  background: #f5f7fa;
  border-radius: 6px;
  border-left: 3px solid transparent;
}

.module-item.active {
  border-left-color: #409eff;
  background: #ecf5ff;
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
  color: #909399;
  background: #f0f0f0;
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
  background: #fafafa;
  border: 1px solid #eee;
  border-radius: 4px;
  padding: 8px;
  max-height: 120px;
  overflow-y: auto;
}

.module-output pre {
  font-size: 11px;
  color: #606266;
  white-space: pre-wrap;
  word-break: break-all;
}

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
