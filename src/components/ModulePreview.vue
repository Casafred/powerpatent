<script setup lang="ts">
const props = defineProps<{
  moduleId: string
  output: any
}>()
</script>

<template>
  <div class="module-preview">
    <pre class="output-json">{{ JSON.stringify(output, null, 2) }}</pre>
  </div>
</template>

<style scoped>
.module-preview {
  font-size: 12px;
}
.output-json {
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
  font-family: 'Menlo', 'Monaco', 'Consolas', monospace;
  font-size: 11px;
  line-height: 1.5;
  color: var(--app-text);
}
</style>
