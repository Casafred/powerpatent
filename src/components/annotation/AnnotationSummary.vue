<script setup lang="ts">
import { computed } from 'vue'
import { useAnnotationStore } from '../../stores/annotation'

const props = defineProps<{
  patentId: string
}>()

const annotationStore = useAnnotationStore()

const groupedAnnotations = computed(() => {
  const annotations = annotationStore.getAnnotationsByPatent(props.patentId)
  const groups: Record<string, typeof annotations> = {}
  for (const a of annotations) {
    const key = a.moduleId || 'general'
    if (!groups[key]) groups[key] = []
    groups[key].push(a)
  }
  return groups
})

const moduleNames: Record<string, string> = {
  M3: '同族保护情况',
  M4: '一句话概要',
  M5: '权利要求范围解读',
  M6: '实施例归纳',
  M7: '其他揭示方案',
  M8: '同族权利要求差异',
  E2: '附图对照',
  E4: '对比矩阵',
  E5: '技术演进时间线',
  E6: '申请人画像',
  E7: '规避设计',
}
</script>

<template>
  <div class="annotation-summary">
    <div v-if="Object.keys(groupedAnnotations).length === 0" class="empty-hint">
      暂无批注
    </div>
    <div v-for="(annotations, moduleId) in groupedAnnotations" :key="moduleId" class="annotation-group">
      <div class="group-header">{{ moduleNames[moduleId] || moduleId }}</div>
      <div v-for="a in annotations" :key="a.id" class="annotation-item">
        <div v-if="a.quote" class="annotation-quote">"{{ a.quote }}"</div>
        <div class="annotation-text">{{ a.text }}</div>
        <div class="annotation-time">{{ a.createdAt }}</div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.annotation-summary {
  padding: 8px;
}
.empty-hint {
  color: var(--app-text-placeholder);
  font-size: 13px;
  text-align: center;
  padding: 16px;
}
.annotation-group {
  margin-bottom: 12px;
}
.group-header {
  font-size: 13px;
  font-weight: 600;
  color: var(--app-text);
  margin-bottom: 6px;
  padding-bottom: 4px;
  border-bottom: 1px solid var(--app-border);
}
.annotation-item {
  padding: 6px 8px;
  margin-bottom: 4px;
  background: var(--app-module-bg);
  border-radius: 4px;
  font-size: 12px;
}
.annotation-quote {
  color: var(--app-text-secondary);
  font-style: italic;
  margin-bottom: 4px;
  padding-left: 8px;
  border-left: 2px solid var(--app-border);
}
.annotation-text {
  color: var(--app-text);
}
.annotation-time {
  color: var(--app-text-placeholder);
  font-size: 11px;
  margin-top: 2px;
}
</style>
