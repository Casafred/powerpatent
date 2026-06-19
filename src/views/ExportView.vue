<script setup lang="ts">
import { ref, computed } from 'vue'
import { useInputStore } from '../stores/input'
import { useProjectStore } from '../stores/project'
import { useRouter } from 'vue-router'
import { save } from '@tauri-apps/plugin-dialog'

const inputStore = useInputStore()
const projectStore = useProjectStore()
const router = useRouter()

const embedPdf = ref(true)
const exporting = ref(false)

const hasPatents = computed(() => inputStore.patents.length > 0)

async function handleExport() {
  try {
    const filePath = await save({
      defaultPath: `patent-reader-${projectStore.projectId.slice(0, 8)}.html`,
      filters: [{ name: 'HTML', extensions: ['html'] }],
    })

    if (filePath) {
      exporting.value = true
      // TODO: Phase 4 - 调用 render_html + export_html
      exporting.value = false
    }
  } catch (e) {
    console.error('导出失败', e)
  }
}

function goBack() {
  router.push({ name: 'generate' })
}
</script>

<template>
  <div class="view-container">
    <h2>预览与导出</h2>
    <p class="view-desc">预览生成结果，导出为离线自包含 HTML 文件</p>

    <!-- 导出配置 -->
    <div class="config-section">
      <h3>导出配置</h3>
      <el-form label-position="left" label-width="120px" size="small">
        <el-form-item label="内嵌 PDF 原文">
          <el-switch v-model="embedPdf" />
        </el-form-item>
      </el-form>
    </div>

    <!-- 专利概览 -->
    <div class="config-section" v-if="hasPatents">
      <h3>输出内容概览</h3>
      <div class="summary-grid">
        <div v-for="patent in inputStore.patents" :key="patent.publicationNumber" class="summary-card">
          <div class="summary-title">{{ patent.title || '未识别标题' }}</div>
          <div class="summary-meta">
            <span v-if="patent.publicationNumber">{{ patent.publicationNumber }}</span>
            <span v-if="patent.applicant">{{ patent.applicant }}</span>
          </div>
        </div>
      </div>
    </div>

    <!-- 预览区域 -->
    <div class="config-section">
      <h3>HTML 预览</h3>
      <div class="preview-placeholder">
        <el-empty description="预览功能将在 Phase 4 实现">
          <template #image>
            <el-icon :size="48" color="#c0c4cc"><Monitor /></el-icon>
          </template>
        </el-empty>
      </div>
    </div>

    <!-- 导出按钮 -->
    <div class="export-actions">
      <el-button
        type="primary"
        size="large"
        :loading="exporting"
        :disabled="!hasPatents"
        @click="handleExport"
      >
        <el-icon><Download /></el-icon>
        导出 HTML 文件
      </el-button>
    </div>

    <!-- 导航 -->
    <div class="view-footer">
      <el-button @click="goBack">上一步</el-button>
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

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 8px;
}

.summary-card {
  background: #f5f7fa;
  border-radius: 6px;
  padding: 10px;
}

.summary-title {
  font-size: 13px;
  font-weight: 500;
}

.summary-meta {
  display: flex;
  gap: 12px;
  margin-top: 4px;
  font-size: 12px;
  color: #909399;
}

.preview-placeholder {
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.export-actions {
  text-align: center;
  padding: 16px 0;
}

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: space-between;
}
</style>
