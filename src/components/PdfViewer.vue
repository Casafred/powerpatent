<script setup lang="ts">
import { ref, watch, onMounted, nextTick } from 'vue'
import * as pdfjsLib from 'pdfjs-dist'
import { readFile } from '@tauri-apps/plugin-fs'

// 初始化 Worker：尝试多种方式，确保 PDF 渲染可用
async function initWorker() {
  try {
    // 方式1：使用 Vite 的 ?worker import（推荐方式）
    const PdfWorker = (await import('pdfjs-dist/build/pdf.worker.min.mjs?worker')).default
    pdfjsLib.GlobalWorkerOptions.workerPort = new PdfWorker()
    console.log('[PdfViewer] Worker initialized via ?worker import')
  } catch (e) {
    console.warn('[PdfViewer] ?worker import failed, trying ?url fallback:', e)
    try {
      // 方式2：使用 Vite 的 ?url import
      const workerUrl = (await import('pdfjs-dist/build/pdf.worker.min.mjs?url')).default
      pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl
      console.log('[PdfViewer] Worker initialized via ?url import')
    } catch (e2) {
      console.warn('[PdfViewer] ?url import failed, using main-thread fallback:', e2)
      // 方式3：不使用 Worker，主线程渲染（性能较差但最可靠）
      pdfjsLib.GlobalWorkerOptions.workerSrc = ''
    }
  }
}

const props = defineProps<{
  src: string
}>()

const loading = ref(false)
const error = ref<string | null>(null)
const scale = ref(1.0)
const canvasContainer = ref<HTMLDivElement | null>(null)
const pageInfo = ref('')

async function renderPdf() {
  if (!props.src) return
  loading.value = true
  error.value = null

  try {
    const data = await readFile(props.src)
    const typedArray = new Uint8Array(data as ArrayBuffer)

    const pdf = await pdfjsLib.getDocument({ data: typedArray }).promise
    pageInfo.value = `共 ${pdf.numPages} 页`

    const container = canvasContainer.value
    if (!container) return

    container.innerHTML = ''

    for (let i = 1; i <= pdf.numPages; i++) {
      const page = await pdf.getPage(i)
      const viewport = page.getViewport({ scale: scale.value * window.devicePixelRatio })

      const canvas = document.createElement('canvas')
      canvas.width = viewport.width
      canvas.height = viewport.height
      // CSS 尺寸按逻辑像素显示
      canvas.style.width = `${viewport.width / window.devicePixelRatio}px`
      canvas.style.height = `${viewport.height / window.devicePixelRatio}px`
      canvas.style.display = 'block'
      canvas.style.margin = '0 auto 12px'
      canvas.style.background = '#fff'

      const ctx = canvas.getContext('2d')!
      await page.render({ canvas, canvasContext: ctx, viewport }).promise

      container.appendChild(canvas)
    }
  } catch (e: any) {
    console.error('[PdfViewer] renderPdf error:', e)
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

watch(() => props.src, () => {
  if (props.src) renderPdf()
})

onMounted(async () => {
  await initWorker()
  if (props.src) renderPdf()
})
</script>

<template>
  <div class="pdf-viewer">
    <div class="pdf-toolbar">
      <el-button-group size="small">
        <el-button @click="zoomOut" :disabled="scale <= 0.5">缩小</el-button>
        <el-button @click="resetZoom">{{ Math.round(scale * 100) }}%</el-button>
        <el-button @click="zoomIn" :disabled="scale >= 3.0">放大</el-button>
      </el-button-group>
      <span v-if="pageInfo" class="page-info">{{ pageInfo }}</span>
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
  align-items: center;
  gap: 12px;
  padding: 8px;
  background: var(--app-module-bg);
  border-bottom: 1px solid var(--app-border);
  flex-shrink: 0;
}

.page-info {
  font-size: 12px;
  color: var(--app-text-secondary);
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
