<script setup lang="ts">
import { computed } from 'vue'

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

const maxReachedStep = computed(() => 4) // 允许点击所有步骤
</script>

<template>
  <aside class="step-nav">
    <div class="step-nav-inner">
      <div
        v-for="step in steps"
        :key="step.num"
        class="step-item"
        :class="{
          active: currentStep === step.num,
          done: currentStep > step.num,
          clickable: step.num <= maxReachedStep,
        }"
        @click="step.num <= maxReachedStep && emit('navigate', step.num)"
      >
        <div class="step-indicator">
          <el-icon v-if="currentStep > step.num" class="done-icon"><Check /></el-icon>
          <span v-else class="step-num">{{ step.num }}</span>
        </div>
        <span class="step-title">{{ step.title }}</span>
        <div v-if="step.num < steps.length" class="step-line" :class="{ done: currentStep > step.num }" />
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
  cursor: default;
  color: var(--app-text-placeholder);
  font-size: 13px;
  position: relative;
  transition: background-color 0.2s, color 0.2s;
}

.step-item.clickable {
  cursor: pointer;
}

.step-item.clickable:hover {
  background: var(--app-hover-bg);
  color: var(--app-text-secondary);
}

.step-item.active {
  background: var(--app-sidebar-active);
  color: var(--app-sidebar-active-text);
  font-weight: 500;
}

.step-item.done {
  color: var(--app-sidebar-done);
}

.step-indicator {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid var(--app-border);
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.2s;
}

.step-item.active .step-indicator {
  border-color: var(--app-sidebar-active-text);
  background: var(--app-sidebar-active-text);
  color: #fff;
}

.step-item.done .step-indicator {
  border-color: var(--app-sidebar-done);
  background: var(--app-sidebar-done);
  color: #fff;
}

.step-num {
  font-size: 11px;
}

.done-icon {
  font-size: 14px;
}

.step-title {
  white-space: nowrap;
}

.step-line {
  position: absolute;
  left: 31px;
  top: 34px;
  width: 2px;
  height: 24px;
  background: var(--app-border);
  transition: background-color 0.2s;
}

.step-line.done {
  background: var(--app-sidebar-done);
}
</style>
