<script setup lang="ts">
// 顶部标题栏
</script>

<template>
  <header class="app-header">
    <div class="header-left">
      <span class="app-name">PatentReader</span>
      <span class="app-divider">|</span>
      <span class="app-subtitle">专利解读生成器</span>
    </div>
    <div class="header-right">
      <span class="version-tag">v0.1.0</span>
    </div>
  </header>
</template>

<style scoped>
.app-header {
  height: var(--app-header-height);
  background: #1d1e1f;
  color: #fff;
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 16px;
  flex-shrink: 0;
  font-size: 13px;
  user-select: none;
}

.header-left {
  display: flex;
  align-items: center;
  gap: 8px;
}

.app-name {
  font-weight: 600;
  font-size: 14px;
}

.app-divider {
  opacity: 0.3;
}

.app-subtitle {
  opacity: 0.7;
}

.header-right {
  display: flex;
  align-items: center;
  gap: 8px;
}

.version-tag {
  background: rgba(255, 255, 255, 0.1);
  padding: 2px 8px;
  border-radius: 4px;
  font-size: 11px;
  opacity: 0.6;
}
</style>
