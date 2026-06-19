<script setup lang="ts">
// Step 1: 输入材料
</script>

<template>
  <div class="view-container">
    <h2>输入材料</h2>
    <p class="view-desc">上传专利 PDF 或结构化表格文件</p>

    <div class="placeholder-card">
      <el-empty description="文件上传区域（待实现）">
        <template #image>
          <el-icon :size="48" color="#c0c4cc"><Upload /></el-icon>
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
