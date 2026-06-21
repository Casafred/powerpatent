<script setup lang="ts">
import { ref, watch, onMounted } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import pdfjsWorkerSrc from 'pdfjs-dist/build/pdf.worker.min.mjs?raw'
import { readFile } from '@tauri-apps/plugin-fs'

// Tauri 环境下 Worker 无法从自定义协议 URL 加载，
// 使用 Blob URL 方式内联 worker 代码
const workerBlob = new Blob([pdfjsWorkerSrc], { type: 'text/javascript' })
pdfjsLib.GlobalWorkerOptions.workerSrc = URL.createObjectURL(workerBlob)

const props = defineProps<{
  src: string
}>()

const loading = ref(false)
const error = ref<string | null>(null)
const scale = ref(1.0)
const canvasContainer = ref<HTMLDivElement | null>(null)

async function renderPdf() {
  if (!props.src) return
  loading.value = true
  error.value = null

  try {
    // 通过 Tauri fs 读取文件为 Uint8Array
    const data = await readFile(props.src)
    const typedArray = new Uint8Array(data)
    const pdf = await pdfjsLib.getDocument({ data: typedArray }).promise

    const container = canvasContainer.value
    if (!container) return

    // 清空旧内容
    container.innerHTML = ''

    for (let i = 1; i <= pdf.numPages; i++) {
      const page = await pdf.getPage(i)
      const viewport = page.getViewport({ scale: scale.value })

      const canvas = document.createElement('canvas')
      canvas.width = viewport.width
      canvas.height = viewport.height
      canvas.style.display = 'block'
      canvas.style.margin = '0 auto 12px'

      const ctx = canvas.getContext('2d')!
      await page.render({ canvas, canvasContext: ctx, viewport }).promise

      container.appendChild(canvas)
    }
  } catch (e: any) {
    error.value = e?.toString() || 'PDF 加载失败'
  } finally {
    loading.value = false
  }
}

function zoomIn() {
  scale.value = Math.min(scale.value + 0.25, 3.0)
  renderPdf()
}

function zoomOut() {
  scale.value = Math.max(scale.value - 0.25, 0.5)
  renderPdf()
}

function resetZoom() {
  scale.value = 1.0
  renderPdf()
}

watch(() => props.src, () => renderPdf())
onMounted(() => renderPdf())
</script>

<template>
  <div class="pdf-viewer">
    <div class="pdf-toolbar">
      <el-button-group size="small">
        <el-button @click="zoomOut" :disabled="scale <= 0.5">缩小</el-button>
        <el-button @click="resetZoom">{{ Math.round(scale * 100) }}%</el-button>
        <el-button @click="zoomIn" :disabled="scale >= 3.0">放大</el-button>
      </el-button-group>
    </div>

    <div v-if="loading" class="pdf-loading">
      <el-icon class="is-loading" :size="24"><Loading /></el-icon>
      <span>加载 PDF 中...</span>
    </div>

    <div v-if="error" class="pdf-error">
      <el-icon :size="20" color="#f56c6c"><Warning /></el-icon>
      <span>{{ error }}</span>
    </div>

    <div ref="canvasContainer" class="pdf-canvas-container" />
  </div>
</template>

<style scoped>
.pdf-viewer {
  display: flex;
  flex-direction: column;
  height: 100%;
  overflow: hidden;
}

.pdf-toolbar {
  display: flex;
  justify-content: center;
  padding: 8px;
  background: var(--app-module-bg);
  border-bottom: 1px solid var(--app-border);
  flex-shrink: 0;
}

.pdf-loading,
.pdf-error {
  display: flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  padding: 24px;
  color: var(--app-text-secondary);
  font-size: 13px;
}

.pdf-canvas-container {
  flex: 1;
  overflow-y: auto;
  padding: 12px;
  background: #525659;
}
</style>
