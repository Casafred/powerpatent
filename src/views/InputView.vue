<script setup lang="ts">
import { ref, computed } from 'vue'
import FileDrop from '../components/input/FileDrop.vue'
import PatentList from '../components/input/PatentList.vue'
import HistoryPanel from '../components/HistoryPanel.vue'
import { useInputStore } from '../stores/input'
import { useHistoryStore } from '../stores/history'

const store = useInputStore()
const historyStore = useHistoryStore()

const hasOcrNeeded = computed(() => store.patents.some(p => p.needsOcr))
const showHistory = ref(false)

function onSessionRestored() {
  showHistory.value = false
}
</script>

<template>
  <div class="view-container">
    <div class="view-header">
      <h2>输入材料</h2>
      <el-button
        :type="historyStore.sessions.length > 0 ? 'default' : 'info'"
        size="small"
        @click="showHistory = !showHistory"
      >
        <el-icon><Clock /></el-icon>
        历史记录
        <el-tag v-if="historyStore.sessions.length > 0" size="small" type="info" round style="margin-left: 4px;">
          {{ historyStore.sessions.length }}
        </el-tag>
      </el-button>
    </div>
    <p class="view-desc">选择输入模式，上传专利文件。左侧导航可随时切换到其他页面</p>

    <!-- 历史记录面板 -->
    <div v-if="showHistory" class="history-section">
      <HistoryPanel @restored="onSessionRestored" />
    </div>

    <FileDrop />
    <PatentList />

    <el-alert
      v-if="hasOcrNeeded"
      type="warning"
      title="检测到扫描件 PDF"
      description="部分 PDF 为扫描件，文本内容较少。系统将自动调用 PaddleOCR 进行识别，也可在设置中切换 OCR 引擎。"
      show-icon
      :closable="false"
      style="margin-top: 12px"
    />
  </div>
</template>

<style scoped>
.view-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 4px;
}

.view-header h2 {
  font-size: 18px;
  font-weight: 600;
  margin: 0;
}

.view-desc {
  color: var(--app-text-secondary);
  font-size: 13px;
  margin-bottom: 16px;
}

.history-section {
  margin-bottom: 16px;
  max-height: 300px;
  overflow-y: auto;
  background: var(--app-card-bg);
  border: 1px solid var(--app-border);
  border-radius: 8px;
  padding: 12px;
}
</style>
