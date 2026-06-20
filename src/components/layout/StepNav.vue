<script setup lang="ts">
const props = defineProps<{
  currentStep: number
}>()

const emit = defineEmits<{
  navigate: [step: number]
}>()

const steps = [
  { num: 1, title: '输入材料', icon: 'Upload' },
  { num: 2, title: '模式与板块', icon: 'Setting' },
  { num: 3, title: '生成与重跑', icon: 'VideoPlay' },
  { num: 4, title: '预览与导出', icon: 'Download' },
]
</script>

<template>
  <aside class="step-nav">
    <div class="step-nav-inner">
      <div
        v-for="step in steps"
        :key="step.num"
        class="step-item"
        :class="{ active: currentStep === step.num }"
        @click="emit('navigate', step.num)"
      >
        <div class="step-indicator">
          <span class="step-num">{{ step.num }}</span>
        </div>
        <span class="step-title">{{ step.title }}</span>
      </div>
    </div>
  </aside>
</template>

<style scoped>
.step-nav {
  width: var(--app-sidebar-width);
  background: var(--app-sidebar-bg);
  border-right: 1px solid var(--app-border);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  user-select: none;
  transition: background-color 0.3s;
}

.step-nav-inner {
  padding: 20px 0;
  display: flex;
  flex-direction: column;
}

.step-item {
  display: flex;
  align-items: center;
  gap: 10px;
  padding: 10px 20px;
  cursor: pointer;
  color: var(--app-text-secondary);
  font-size: 13px;
  transition: background-color 0.2s, color 0.2s;
}

.step-item:hover {
  background: var(--app-hover-bg);
  color: var(--app-text);
}

.step-item.active {
  background: var(--app-sidebar-active);
  color: var(--app-sidebar-active-text);
  font-weight: 500;
}

.step-indicator {
  width: 22px;
  height: 22px;
  border-radius: 6px;
  border: 1.5px solid var(--app-border);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-size: 11px;
  font-weight: 600;
  transition: all 0.2s;
}

.step-item.active .step-indicator {
  border-color: var(--app-sidebar-active-text);
  background: var(--app-sidebar-active-text);
  color: #fff;
}

.step-title {
  white-space: nowrap;
}
</style>
