<script setup lang="ts">
import { computed, ref } from 'vue'
import { useInputStore } from '../../stores/input'
import { open } from '@tauri-apps/plugin-dialog'

const store = useInputStore()

const patents = computed(() => store.patents)
const hasResults = computed(() => patents.value.length > 0)

// 是否有混合来源（Excel+PDF 模式）
const hasMixedSource = computed(() =>
  patents.value.some(p => p.source === 'mixed') ||
  (patents.value.some(p => p.source === 'table') && patents.value.some(p => p.source === 'pdf'))
)

// 未关联 PDF 的专利
const unlinkedPatents = computed(() =>
  patents.value.filter(p => p.source === 'table' && !p.pdfFilePath)
)

// 手动关联：选择 PDF 文件绑定到指定专利
async function linkPdf(index: number) {
  try {
    const selected = await open({
      multiple: false,
      filters: [{ name: 'PDF 文件', extensions: ['pdf'] }],
    })
    if (selected && typeof selected === 'string') {
      store.patents[index].pdfFilePath = selected
      store.patents[index].source = 'mixed'
    }
  } catch (e) {
    console.error('选择 PDF 失败', e)
  }
}

function unlinkPdf(index: number) {
  store.patents[index].pdfFilePath = undefined
  if (store.patents[index].source === 'mixed') {
    store.patents[index].source = 'table'
  }
}

function getStatusTag(status?: string) {
  if (!status) return { text: '未知', type: 'info' as const }
  if (status.includes('授权') || status.toLowerCase().includes('grant')) return { text: status, type: 'success' as const }
  if (status.includes('有效') || status.toLowerCase().includes('active')) return { text: status, type: 'success' as const }
  if (status.includes('失效') || status.toLowerCase().includes('expired')) return { text: status, type: 'danger' as const }
  return { text: status, type: 'info' as const }
}

function removePatent(index: number) {
  store.removePatent(index)
}

function getPdfFileName(path: string): string {
  return path.split(/[/\\]/).pop() || path
}

function getSourceLabel(source: string): string {
  switch (source) {
    case 'pdf': return 'PDF'
    case 'table': return 'Excel'
    case 'mixed': return 'Excel+PDF'
    default: return source
  }
}

function getSourceTagType(source: string) {
  switch (source) {
    case 'pdf': return 'danger' as const
    case 'table': return 'success' as const
    case 'mixed': return 'warning' as const
    default: return 'info' as const
  }
}
</script>

<template>
  <div class="patent-list-section" v-if="hasResults">
    <div class="list-header">
      <h3>识别结果（{{ patents.length }} 篇专利）</h3>
      <el-button type="danger" text size="small" @click="store.clearPatents()">清空全部</el-button>
    </div>

    <!-- 关联状态汇总 -->
    <div v-if="hasMixedSource" class="link-summary">
      <div class="link-stat">
        <el-tag type="success" size="small" round>已关联 PDF：{{ patents.filter(p => p.pdfFilePath).length }}</el-tag>
      </div>
      <div v-if="unlinkedPatents.length > 0" class="link-stat">
        <el-tag type="warning" size="small" round>未关联 PDF：{{ unlinkedPatents.length }}</el-tag>
      </div>
      <span class="link-hint">公开号与 PDF 文件名自动匹配，也可手动绑定</span>
    </div>

    <div class="patent-list">
      <div v-for="(patent, index) in patents" :key="index" class="patent-card" :class="{ 'patent-unlinked': patent.source === 'table' && !patent.pdfFilePath }">
        <div class="patent-header">
          <span class="patent-title">{{ patent.title || '未识别标题' }}</span>
          <div class="patent-header-actions">
            <el-tag v-if="patent.legalStatus" v-bind="getStatusTag(patent.legalStatus)" size="small" round>
              {{ getStatusTag(patent.legalStatus).text }}
            </el-tag>
            <el-button type="danger" text size="small" @click="removePatent(index)">
              <el-icon><Delete /></el-icon>
            </el-button>
          </div>
        </div>
        <div class="patent-meta">
          <span v-if="patent.publicationNumber" class="meta-item">
            <span class="meta-label">公开号</span>{{ patent.publicationNumber }}
          </span>
          <span v-if="patent.applicant" class="meta-item">
            <span class="meta-label">申请人</span>{{ patent.applicant }}
          </span>
          <span v-if="patent.filingDate" class="meta-item">
            <span class="meta-label">申请日</span>{{ patent.filingDate }}
          </span>
        </div>
        <div class="patent-footer">
          <el-tag :type="getSourceTagType(patent.source)" size="small">
            {{ getSourceLabel(patent.source) }}
          </el-tag>

          <!-- PDF 关联状态 -->
          <div class="pdf-link-row">
            <template v-if="patent.pdfFilePath">
              <el-icon color="#67c23a"><Document /></el-icon>
              <span class="pdf-filename" :title="patent.pdfFilePath">{{ getPdfFileName(patent.pdfFilePath) }}</span>
              <el-button type="danger" text size="small" @click="unlinkPdf(index)">取消关联</el-button>
            </template>
            <template v-else-if="patent.source === 'table'">
              <el-icon color="#e6a23c"><Warning /></el-icon>
              <span class="pdf-unlinked-text">未关联 PDF</span>
              <el-button type="primary" text size="small" @click="linkPdf(index)">手动绑定</el-button>
            </template>
            <template v-else-if="patent.source === 'pdf'">
              <el-icon color="#409eff"><Document /></el-icon>
              <span class="pdf-filename">PDF 原文</span>
            </template>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.patent-list-section h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 12px;
}

.list-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.list-header h3 {
  margin-bottom: 0;
}

/* 关联汇总 */
.link-summary {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-bottom: 12px;
  padding: 8px 12px;
  background: var(--app-card-bg);
  border: 1px solid var(--app-border);
  border-radius: 6px;
  flex-wrap: wrap;
}

.link-hint {
  font-size: 11px;
  color: var(--app-text-placeholder);
}

.patent-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.patent-card {
  background: var(--app-module-bg);
  border-radius: 6px;
  padding: 12px;
  border-left: 3px solid transparent;
}

.patent-card.patent-unlinked {
  border-left-color: #e6a23c;
}

.patent-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.patent-header-actions {
  display: flex;
  align-items: center;
  gap: 4px;
}

.patent-title {
  font-size: 14px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  flex: 1;
}

.patent-meta {
  display: flex;
  gap: 16px;
  margin-top: 6px;
  font-size: 12px;
  color: var(--app-text-secondary);
  flex-wrap: wrap;
}

.meta-item {
  display: flex;
  gap: 4px;
}

.meta-label {
  color: var(--app-text-placeholder);
  font-size: 11px;
}

.patent-footer {
  display: flex;
  align-items: center;
  gap: 12px;
  margin-top: 8px;
  padding-top: 8px;
  border-top: 1px solid var(--app-border);
  flex-wrap: wrap;
}

.pdf-link-row {
  display: flex;
  align-items: center;
  gap: 4px;
  flex: 1;
  min-width: 0;
}

.pdf-filename {
  font-size: 12px;
  color: var(--app-text-secondary);
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
  max-width: 200px;
}

.pdf-unlinked-text {
  font-size: 12px;
  color: #e6a23c;
}
</style>
