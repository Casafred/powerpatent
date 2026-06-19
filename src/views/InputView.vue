<script setup lang="ts">
import FileDrop from '../components/input/FileDrop.vue'
import PatentList from '../components/input/PatentList.vue'
import { useInputStore } from '../stores/input'
import { useRouter } from 'vue-router'

const store = useInputStore()
const router = useRouter()

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
