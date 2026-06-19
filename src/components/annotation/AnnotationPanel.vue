<script setup lang="ts">
import { ref, computed } from 'vue'
import { useAnnotationStore } from '../../stores/annotation'

const props = defineProps<{
  patentId: string
  moduleId: string
}>()

const store = useAnnotationStore()
const newText = ref('')
const newQuote = ref('')

const annotations = computed(() => store.getAnnotations(props.patentId, props.moduleId))

function add() {
  if (!newText.value.trim()) return
  store.addAnnotation({
    patentId: props.patentId,
    moduleId: props.moduleId,
    text: newText.value.trim(),
    quote: newQuote.value.trim() || undefined,
  })
  newText.value = ''
  newQuote.value = ''
}

function remove(id: string) {
  store.deleteAnnotation(id)
}
</script>

<template>
  <div class="annotation-panel">
    <h4>批注</h4>

    <div v-if="annotations.length > 0" class="annotation-list">
      <div v-for="a in annotations" :key="a.id" class="annotation-item">
        <div v-if="a.quote" class="annotation-quote">{{ a.quote }}</div>
        <p class="annotation-text">{{ a.text }}</p>
        <div class="annotation-meta">
          <span class="annotation-time">{{ a.updatedAt.slice(0, 16).replace('T', ' ') }}</span>
          <button class="annotation-delete" @click="remove(a.id)">删除</button>
        </div>
      </div>
    </div>

    <div v-else class="annotation-empty">暂无批注</div>

    <div class="annotation-input">
      <el-input
        v-model="newText"
        type="textarea"
        :rows="2"
        placeholder="添加批注..."
        size="small"
      />
      <el-button type="primary" size="small" :disabled="!newText.trim()" @click="add">
        添加
      </el-button>
    </div>
  </div>
</template>

<style scoped>
.annotation-panel h4 {
  font-size: 13px;
  font-weight: 600;
  margin-bottom: 8px;
  color: var(--app-text);
}

.annotation-list {
  display: flex;
  flex-direction: column;
  gap: 6px;
  margin-bottom: 8px;
}

.annotation-item {
  background: var(--app-module-bg);
  border-radius: 4px;
  padding: 8px;
  border-left: 3px solid #e6a23c;
}

.annotation-quote {
  font-size: 11px;
  color: var(--app-text-secondary);
  font-style: italic;
  margin-bottom: 4px;
  padding: 2px 4px;
  background: var(--app-hover-bg);
  border-radius: 2px;
}

.annotation-text {
  font-size: 12px;
  color: var(--app-text);
  margin-bottom: 4px;
}

.annotation-meta {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.annotation-time {
  font-size: 10px;
  color: var(--app-text-placeholder);
}

.annotation-delete {
  background: none;
  border: none;
  color: #f56c6c;
  font-size: 11px;
  cursor: pointer;
  padding: 0;
}

.annotation-delete:hover {
  text-decoration: underline;
}

.annotation-empty {
  font-size: 12px;
  color: var(--app-text-placeholder);
  text-align: center;
  padding: 8px;
}

.annotation-input {
  display: flex;
  flex-direction: column;
  gap: 4px;
  align-items: flex-end;
}
</style>
