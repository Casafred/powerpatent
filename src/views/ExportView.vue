<script setup lang="ts">
// Step 5: 预览与导出
</script>

<template>
  <div class="view-container">
    <h2>预览与导出</h2>
    <p class="view-desc">实时预览生成结果，导出为离线 HTML 文件</p>

    <div class="placeholder-card">
      <el-empty description="预览与导出面板（待实现）">
        <template #image>
          <el-icon :size="48" color="#c0c4cc"><Download /></el-icon>
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
