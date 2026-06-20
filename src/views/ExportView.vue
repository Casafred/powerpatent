<script setup lang="ts">
import { ref, computed } from 'vue'
import { useInputStore } from '../stores/input'
import { useModuleConfigStore } from '../stores/moduleConfig'
import { useProjectStore } from '../stores/project'
import { renderHtml, exportHtml } from '../services/tauri'
import { useRouter } from 'vue-router'
import { save } from '@tauri-apps/plugin-dialog'
import { ElNotification } from 'element-plus'

const inputStore = useInputStore()
const configStore = useModuleConfigStore()
const projectStore = useProjectStore()
const router = useRouter()

const embedPdf = ref(true)
const exporting = ref(false)
const previewHtml = ref('')
const previewLoading = ref(false)

const hasPatents = computed(() => inputStore.patents.length > 0)

// 构建渲染配置
function buildModuleConfig() {
  return {
    mode: configStore.mode,
    theme_name: configStore.themeName || null,
    theme_description: configStore.themeDescription || null,
    patents: inputStore.patents,
  }
}

// 预览 HTML
async function handlePreview() {
  if (!hasPatents.value) return
  previewLoading.value = true
  try {
    const html = await renderHtml({
      projectId: projectStore.projectId,
      moduleConfig: buildModuleConfig(),
      embedPdf: embedPdf.value,
    })
    previewHtml.value = html
  } catch (e: any) {
    ElNotification({ title: '预览失败', message: e?.toString() || '生成 HTML 预览时出错', type: 'error' })
  } finally {
    previewLoading.value = false
  }
}

// 导出 HTML 文件
async function handleExport() {
  try {
    const filePath = await save({
      defaultPath: `patent-reader-${projectStore.projectId.slice(0, 8)}.html`,
      filters: [{ name: 'HTML', extensions: ['html'] }],
    })

    if (filePath) {
      exporting.value = true
      await exportHtml({
        projectId: projectStore.projectId,
        outputPath: filePath,
        moduleConfig: buildModuleConfig(),
        embedPdf: embedPdf.value,
      })
      exporting.value = false
      ElNotification({ title: '导出成功', message: `文件已保存到 ${filePath}`, type: 'success' })
    }
  } catch (e: any) {
    ElNotification({ title: '导出失败', message: e?.toString() || '导出 HTML 文件时出错', type: 'error' })
    exporting.value = false
  }
}
</script>

<template>
  <div class="view-container">
    <h2>预览与导出</h2>
    <p class="view-desc">预览生成结果，导出为离线自包含 HTML 文件</p>

    <!-- 无专利数据提示 -->
    <el-alert
      v-if="!hasPatents"
      type="info"
      title="尚未生成分析内容"
      description="请先前往"输入材料"上传专利，再在"生成与重跑"中完成 AI 分析"
      show-icon
      :closable="false"
      style="margin-bottom: 16px"
    />

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
      <div class="preview-header">
        <h3>HTML 预览</h3>
        <el-button type="primary" size="small" :loading="previewLoading" @click="handlePreview">
          生成预览
        </el-button>
      </div>
      <div v-if="previewHtml" class="preview-frame">
        <iframe :srcdoc="previewHtml" class="preview-iframe" sandbox="allow-same-origin" />
      </div>
      <div v-else class="preview-placeholder">
        <el-empty description="点击「生成预览」查看输出效果" />
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
      <el-button @click="router.push({ name: 'generate' })">返回生成</el-button>
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

.config-section {
  background: var(--app-card-bg);
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

.preview-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.preview-header h3 {
  margin-bottom: 0;
}

.summary-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 8px;
}

.summary-card {
  background: var(--app-module-bg);
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
  color: var(--app-text-secondary);
}

.preview-placeholder {
  min-height: 200px;
  display: flex;
  align-items: center;
  justify-content: center;
}

.preview-frame {
  border: 1px solid var(--app-border);
  border-radius: 6px;
  overflow: hidden;
}

.preview-iframe {
  width: 100%;
  height: 600px;
  border: none;
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
