<script setup lang="ts">
import { computed } from 'vue'
import FileDrop from '../components/input/FileDrop.vue'
import PatentList from '../components/input/PatentList.vue'
import { useInputStore } from '../stores/input'
import { useRouter } from 'vue-router'

const store = useInputStore()
const router = useRouter()

const hasOcrNeeded = computed(() => store.patents.some(p => p.needsOcr))

function goNext() {
  router.push({ name: 'config' })
}
</script>

<template>
  <div class="view-container">
    <h2>输入材料</h2>
    <p class="view-desc">上传专利 PDF 或结构化表格文件，系统将自动识别并提取专利信息</p>

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

    <div class="view-footer" v-if="store.patents.length > 0">
      <el-button type="primary" @click="goNext">
        下一步：模式与板块
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

.view-footer {
  margin-top: 24px;
  display: flex;
  justify-content: flex-end;
}
</style>
