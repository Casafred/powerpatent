<script setup lang="ts">
import { computed } from 'vue'
import { useInputStore } from '../../stores/input'

const store = useInputStore()

const patents = computed(() => store.patents)
const hasResults = computed(() => patents.value.length > 0)

function getStatusTag(status?: string) {
  if (!status) return { text: '未知', type: 'info' as const }
  if (status.includes('授权') || status.toLowerCase().includes('grant')) return { text: status, type: 'success' as const }
  if (status.includes('有效') || status.toLowerCase().includes('active')) return { text: status, type: 'success' as const }
  if (status.includes('失效') || status.toLowerCase().includes('expired')) return { text: status, type: 'danger' as const }
  return { text: status, type: 'info' as const }
}
</script>

<template>
  <div class="patent-list-section" v-if="hasResults">
    <h3>识别结果（{{ patents.length }} 篇专利）</h3>
    <div class="patent-list">
      <div v-for="(patent, index) in patents" :key="index" class="patent-card">
        <div class="patent-header">
          <span class="patent-title">{{ patent.title || '未识别标题' }}</span>
          <el-tag v-if="patent.legalStatus" v-bind="getStatusTag(patent.legalStatus)" size="small" round>
            {{ getStatusTag(patent.legalStatus).text }}
          </el-tag>
        </div>
        <div class="patent-meta">
          <span v-if="patent.publicationNumber">{{ patent.publicationNumber }}</span>
          <span v-if="patent.applicant">{{ patent.applicant }}</span>
          <span v-if="patent.filingDate">{{ patent.filingDate }}</span>
        </div>
        <div class="patent-source">
          <el-tag type="info" size="small">来源：{{ patent.source }}</el-tag>
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

.patent-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.patent-card {
  background: #f5f7fa;
  border-radius: 6px;
  padding: 12px;
}

.patent-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  gap: 8px;
}

.patent-title {
  font-size: 14px;
  font-weight: 500;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.patent-meta {
  display: flex;
  gap: 12px;
  margin-top: 6px;
  font-size: 12px;
  color: #909399;
}

.patent-source {
  margin-top: 6px;
}
</style>
