<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import AppHeader from './AppHeader.vue'
import StepNav from './StepNav.vue'

const route = useRoute()
const router = useRouter()

const currentStep = computed(() => {
  return (route.meta.step as number) ?? 1
})

function goToStep(step: number) {
  const routes = ['input', 'config', 'generate', 'export']
  const target = routes[step - 1]
  if (target) {
    router.push({ name: target })
  }
}
</script>

<template>
  <div class="app-container">
    <AppHeader />
    <div class="app-body">
      <StepNav :current-step="currentStep" @navigate="goToStep" />
      <main class="app-main">
        <router-view />
      </main>
    </div>
  </div>
</template>

<style scoped>
.app-container {
  display: flex;
  flex-direction: column;
  height: 100vh;
  overflow: hidden;
  background: var(--app-bg);
  color: var(--app-text);
  transition: background-color 0.3s, color 0.3s;
}

.app-body {
  display: flex;
  flex: 1;
  overflow: hidden;
}

.app-main {
  flex: 1;
  overflow-y: auto;
  padding: 24px 32px;
  background: var(--app-bg);
  transition: background-color 0.3s;
}
</style>
