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

const files = ref<InputFileInfo[]>([])
const processing = ref(false)
const error = ref<string | null>(null)

const hasFiles = computed(() => files.value.length > 0)
const hasPdf = computed(() => files.value.some(f => f.type === 'pdf'))
const hasTable = computed(() => files.value.some(f => f.type === 'xlsx' || f.type === 'csv'))

function detectFileType(ext: string): InputFileInfo['type'] {
  const lower = ext.toLowerCase()
  if (lower === 'pdf') return 'pdf'
  if (lower === 'xlsx' || lower === 'xls') return 'xlsx'
  if (lower === 'csv') return 'csv'
  return 'unknown'
}

function getInputSource(): InputSource {
  if (hasPdf.value && hasTable.value) return 'mixed'
  if (hasPdf.value) return 'pdf'
  return 'table'
}

function addFilePaths(paths: string[]) {
  for (const path of paths) {
    const name = path.split(/[/\\]/).pop() || path
    const ext = name.includes('.') ? name.split('.').pop()! : ''
    const type = detectFileType(ext)
    if (type === 'unknown') continue
    // 避免重复添加
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
  try {
    const selected = await open({
      multiple: true,
      filters: [
        { name: '专利文件', extensions: ['pdf', 'xlsx', 'xls', 'csv'] },
        { name: 'PDF', extensions: ['pdf'] },
        { name: '表格', extensions: ['xlsx', 'xls', 'csv'] },
      ],
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
  // Tauri 环境下拖拽文件通过 dataTransfer 获取路径
  const paths: string[] = []
  if (e.dataTransfer) {
    for (let i = 0; i < e.dataTransfer.files.length; i++) {
      const file = e.dataTransfer.files[i]
      // 在 Tauri 中，file.path 包含完整文件路径
      const filePath = (file as any).path as string | undefined
      if (filePath) {
        paths.push(filePath)
      }
    }
  }
  if (paths.length > 0) {
    addFilePaths(paths)
  }
}

function handleDragOver(e: DragEvent) {
  e.preventDefault()
  isDragging.value = true
}

function handleDragLeave() {
  isDragging.value = false
}

async function processFiles() {
  if (files.value.length === 0) return
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

defineExpose({ files, hasFiles, getInputSource })
</script>

<template>
  <div class="file-drop-section">
    <!-- 拖拽上传区域 -->
    <div
      class="drop-zone"
      :class="{ dragging: isDragging }"
      @drop="handleDrop"
      @dragover="handleDragOver"
      @dragleave="handleDragLeave"
      @click="openFileDialog"
    >
      <el-icon :size="40" color="var(--app-text-placeholder)"><UploadFilled /></el-icon>
      <p class="drop-text">拖拽文件到此处，或点击选择文件</p>
      <p class="drop-hint">支持 PDF、XLSX、XLS、CSV 格式</p>
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
            <el-icon :size="20" class="file-icon">
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
          输入模式：{{ getInputSource() === 'mixed' ? '混合输入（表格 + PDF）' : getInputSource() === 'pdf' ? 'PDF 输入' : '表格输入' }}
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

.drop-zone {
  border: 2px dashed var(--app-dropzone-border);
  border-radius: 8px;
  padding: 40px 24px;
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
