<script setup lang="ts">
// Step 4: 生成与重跑
</script>

<template>
  <div class="view-container">
    <h2>生成与重跑</h2>
    <p class="view-desc">AI 逐板块生成，支持板块级重跑</p>

    <div class="placeholder-card">
      <el-empty description="生成进度面板（待实现）">
        <template #image>
          <el-icon :size="48" color="#c0c4cc"><VideoPlay /></el-icon>
        </template>
      </el-empty>
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
  color: #909399;
  font-size: 13px;
  margin-bottom: 20px;
}

.placeholder-card {
  background: #fff;
  border: 1px solid var(--app-border);
  border-radius: 8px;
  padding: 48px 24px;
}
</style>
