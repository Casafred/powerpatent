<script setup lang="ts">
import { ref, computed } from 'vue'
import { useInputStore } from '../../stores/input'
import { processInput } from '../../services/tauri'
import { open } from '@tauri-apps/plugin-dialog'
import type { InputSource } from '../../types/patent'

const store = useInputStore()
const isDragging = ref(false)

export interface InputFileInfo {
  path: string
  name: string
  ext: string
  type: 'pdf' | 'xlsx' | 'csv' | 'unknown'
  size?: number
}

export type InputMode = 'pdf' | 'excel_pdf' | 'excel'

const inputMode = ref<InputMode>('pdf')
const files = ref<InputFileInfo[]>([])
const processing = ref(false)
const error = ref<string | null>(null)

const hasFiles = computed(() => files.value.length > 0)
const hasPdf = computed(() => files.value.some(f => f.type === 'pdf'))
const hasTable = computed(() => files.value.some(f => f.type === 'xlsx' || f.type === 'csv'))

const modeConfig: Record<InputMode, { label: string; desc: string; icon: string; acceptFilters: { name: string; extensions: string[] }[] }> = {
  pdf: {
    label: 'PDF 原文',
    desc: '上传专利 PDF 原文，支持内嵌阅读、OCR 识别',
    icon: 'Document',
    acceptFilters: [
      { name: 'PDF 文件', extensions: ['pdf'] },
    ],
  },
  excel_pdf: {
    label: 'Excel + PDF',
    desc: '结构化数据与原文 PDF 对应，数据更精准',
    icon: 'Files',
    acceptFilters: [
      { name: '专利文件', extensions: ['pdf', 'xlsx', 'xls', 'csv'] },
      { name: 'PDF', extensions: ['pdf'] },
      { name: '表格', extensions: ['xlsx', 'xls', 'csv'] },
    ],
  },
  excel: {
    label: '纯 Excel',
    desc: '仅上传结构化表格数据，无 PDF 原文浏览',
    icon: 'Grid',
    acceptFilters: [
      { name: '表格文件', extensions: ['xlsx', 'xls', 'csv'] },
    ],
  },
}

function detectFileType(ext: string): InputFileInfo['type'] {
  const lower = ext.toLowerCase()
  if (lower === 'pdf') return 'pdf'
  if (lower === 'xlsx' || lower === 'xls') return 'xlsx'
  if (lower === 'csv') return 'csv'
  return 'unknown'
}

function addFilePaths(paths: string[]) {
  for (const path of paths) {
    const name = path.split(/[/\\]/).pop() || path
    const ext = name.includes('.') ? name.split('.').pop()! : ''
    const type = detectFileType(ext)
    if (type === 'unknown') continue
    if (files.value.some(f => f.path === path)) continue
    files.value.push({ path, name, ext, type })
  }
}

function removeFile(index: number) {
  files.value.splice(index, 1)
}

function clearAll() {
  files.value = []
  store.clearPatents()
  error.value = null
}

async function openFileDialog() {
  const cfg = modeConfig[inputMode.value]
  try {
    const selected = await open({
      multiple: true,
      filters: cfg.acceptFilters,
    })
    if (selected) {
      const paths = Array.isArray(selected) ? selected : [selected]
      addFilePaths(paths)
    }
  } catch (e) {
    console.error('文件选择失败', e)
  }
}

function handleDrop(e: DragEvent) {
  isDragging.value = false
  const paths: string[] = []
  if (e.dataTransfer) {
    for (let i = 0; i < e.dataTransfer.files.length; i++) {
      const file = e.dataTransfer.files[i]
      const filePath = (file as any).path as string | undefined
      if (filePath) paths.push(filePath)
    }
  }
  if (paths.length > 0) addFilePaths(paths)
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragging.value = true
}

function handleDragLeave() {
  isDragging.value = false
}

// 校验文件是否符合当前模式
function validateFiles(): string | null {
  if (inputMode.value === 'pdf' && !files.value.some(f => f.type === 'pdf')) {
    return '当前模式为"PDF 原文"，请至少添加一个 PDF 文件'
  }
  if (inputMode.value === 'excel' && !files.value.some(f => f.type === 'xlsx' || f.type === 'csv')) {
    return '当前模式为"纯 Excel"，请至少添加一个表格文件'
  }
  if (inputMode.value === 'excel_pdf') {
    if (!files.value.some(f => f.type === 'pdf')) return 'Excel+PDF 模式需要至少一个 PDF 文件'
    if (!files.value.some(f => f.type === 'xlsx' || f.type === 'csv')) return 'Excel+PDF 模式需要至少一个表格文件'
  }
  return null
}

async function processFiles() {
  if (files.value.length === 0) return
  const validationError = validateFiles()
  if (validationError) {
    error.value = validationError
    return
  }
  processing.value = true
  error.value = null
  try {
    const paths = files.value.map(f => f.path)
    const result = await processInput(paths)
    store.patents = result || []
    store.loading = false
  } catch (e: any) {
    error.value = e?.toString() || '处理失败'
  } finally {
    processing.value = false
  }
}

function getInputSource(): InputSource {
  if (hasPdf.value && hasTable.value) return 'mixed'
  if (hasPdf.value) return 'pdf'
  return 'table'
}

defineExpose({ files, hasFiles, getInputSource, inputMode })
</script>

<template>
  <div class="file-drop-section">
    <!-- 输入模式选择 -->
    <div class="mode-selector">
      <div
        v-for="(cfg, key) in modeConfig"
        :key="key"
        class="mode-card"
        :class="{ active: inputMode === key }"
        @click="inputMode = key as InputMode"
      >
        <el-icon :size="22">
          <Document v-if="key === 'pdf'" />
          <Files v-else-if="key === 'excel_pdf'" />
          <Grid v-else />
        </el-icon>
        <div class="mode-info">
          <span class="mode-label">{{ cfg.label }}</span>
          <span class="mode-desc">{{ cfg.desc }}</span>
        </div>
      </div>
    </div>

    <!-- 拖拽上传区域 -->
    <div
      class="drop-zone"
      :class="{ dragging: isDragging }"
      @drop="handleDrop"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @click="openFileDialog"
    >
      <el-icon :size="36" color="var(--app-text-placeholder)"><UploadFilled /></el-icon>
      <p class="drop-text">拖拽文件到此处，或点击选择文件</p>
      <p class="drop-hint">
        <template v-if="inputMode === 'pdf'">仅接受 PDF 格式</template>
        <template v-else-if="inputMode === 'excel'">仅接受 XLSX / XLS / CSV 格式</template>
        <template v-else>支持 PDF + XLSX / XLS / CSV 混合上传</template>
      </p>
    </div>

    <!-- 文件列表 -->
    <div v-if="hasFiles" class="file-list-section">
      <div class="file-list-header">
        <span>已添加 {{ files.length }} 个文件</span>
        <el-button type="danger" text size="small" @click="clearAll">清空全部</el-button>
      </div>

      <div class="file-list">
        <div v-for="(file, index) in files" :key="file.path" class="file-item">
          <div class="file-info">
            <el-icon :size="18" class="file-icon">
              <Document v-if="file.type === 'pdf'" />
              <Grid v-else />
            </el-icon>
            <span class="file-name" :title="file.name">{{ file.name }}</span>
            <el-tag :type="file.type === 'pdf' ? 'danger' : 'success'" size="small" round>
              {{ file.type.toUpperCase() }}
            </el-tag>
          </div>
          <el-button type="danger" text size="small" @click="removeFile(index)">
            <el-icon><Delete /></el-icon>
          </el-button>
        </div>
      </div>

      <!-- 输入类型提示 -->
      <div class="input-type-hint">
        <el-tag type="info" size="small">
          输入模式：{{ inputMode === 'pdf' ? 'PDF 原文' : inputMode === 'excel_pdf' ? 'Excel + PDF' : '纯 Excel' }}
        </el-tag>
      </div>

      <!-- 处理按钮 -->
      <div class="action-bar">
        <el-button type="primary" :loading="processing" @click="processFiles">
          {{ processing ? '处理中...' : '开始处理' }}
        </el-button>
      </div>
    </div>

    <!-- 错误提示 -->
    <el-alert v-if="error" :title="error" type="error" show-icon closable @close="error = null" />
  </div>
</template>

<style scoped>
.file-drop-section {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* 模式选择器 */
.mode-selector {
  display: grid;
  grid-template-columns: repeat(3, 1fr);
  gap: 10px;
}

.mode-card {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 12px;
  border: 2px solid var(--app-border);
  border-radius: 8px;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--app-card-bg);
}

.mode-card:hover {
  border-color: #c0c4cc;
  background: var(--app-hover-bg);
}

.mode-card.active {
  border-color: #409eff;
  background: var(--app-active-bg);
}

.mode-info {
  display: flex;
  flex-direction: column;
  gap: 2px;
  min-width: 0;
}

.mode-label {
  font-size: 13px;
  font-weight: 600;
  color: var(--app-text);
}

.mode-desc {
  font-size: 11px;
  color: var(--app-text-secondary);
  line-height: 1.3;
}

/* 拖拽区域 */
.drop-zone {
  border: 2px dashed var(--app-dropzone-border);
  border-radius: 8px;
  padding: 32px 24px;
  text-align: center;
  cursor: pointer;
  transition: all 0.2s;
  background: var(--app-dropzone-bg);
}

.drop-zone:hover {
  border-color: #409eff;
  background: var(--app-dropzone-hover);
}

.drop-zone.dragging {
  border-color: #409eff;
  background: var(--app-dropzone-hover);
  transform: scale(1.01);
}

.drop-text {
  margin-top: 8px;
  color: var(--app-text-secondary);
  font-size: 14px;
}

.drop-hint {
  margin-top: 4px;
  color: var(--app-text-placeholder);
  font-size: 12px;
}

/* 文件列表 */
.file-list-section {
  background: var(--app-card-bg);
  border: 1px solid var(--app-border);
  border-radius: 8px;
  padding: 16px;
}

.file-list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
  font-size: 13px;
  color: var(--app-text-secondary);
}

.file-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.file-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 8px 12px;
  background: var(--app-module-bg);
  border-radius: 6px;
}

.file-info {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 0;
}

.file-icon {
  flex-shrink: 0;
}

.file-name {
  font-size: 13px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.input-type-hint {
  margin-top: 12px;
}

.action-bar {
  margin-top: 16px;
  display: flex;
  justify-content: flex-end;
}
</style>
