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
  { num: 3, title: 'AI 配置', icon: 'Cpu' },
  { num: 4, title: '生成与重跑', icon: 'VideoPlay' },
  { num: 5, title: '预览与导出', icon: 'Download' },
]

const maxReachedStep = computed(() => props.currentStep)
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
  background: #fff;
  border-right: 1px solid var(--app-border);
  flex-shrink: 0;
  display: flex;
  flex-direction: column;
  user-select: none;
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
  color: #a8abb2;
  font-size: 13px;
  position: relative;
}

.step-item.clickable {
  cursor: pointer;
}

.step-item.clickable:hover {
  background: #f5f7fa;
  color: #606266;
}

.step-item.active {
  background: #ecf5ff;
  color: #409eff;
  font-weight: 500;
}

.step-item.done {
  color: #67c23a;
}

.step-indicator {
  width: 24px;
  height: 24px;
  border-radius: 50%;
  border: 2px solid #dcdfe6;
  display: flex;
  align-items: center;
  justify-content: center;
  flex-shrink: 0;
  font-size: 12px;
  font-weight: 600;
  transition: all 0.2s;
}

.step-item.active .step-indicator {
  border-color: #409eff;
  background: #409eff;
  color: #fff;
}

.step-item.done .step-indicator {
  border-color: #67c23a;
  background: #67c23a;
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
  background: #dcdfe6;
}

.step-line.done {
  background: #67c23a;
}
</style>
