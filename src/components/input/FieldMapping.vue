<script setup lang="ts">
import { ref, computed, watch } from 'vue'

const props = defineProps<{
  headers: string[]
  rows: string[][]
  autoMapping: Record<number, string>
}>()

const emit = defineEmits<{
  confirm: [mapping: Record<number, string>]
}>()

// 可映射的字段列表
const availableFields = [
  { value: 'publication_number', label: '公开号' },
  { value: 'grant_number', label: '授权号' },
  { value: 'application_number', label: '申请号' },
  { value: 'applicant', label: '申请人' },
  { value: 'inventor', label: '发明人' },
  { value: 'filing_date', label: '申请日' },
  { value: 'priority_date', label: '优先权日' },
  { value: 'publication_date', label: '公开日' },
  { value: 'grant_date', label: '授权日' },
  { value: 'legal_status', label: '法律状态' },
  { value: 'ipc', label: 'IPC分类号' },
  { value: 'cpc', label: 'CPC分类号' },
  { value: 'title', label: '标题/发明名称' },
  { value: 'abstract_text', label: '摘要' },
  { value: 'claims_text', label: '权利要求' },
  { value: 'description_text', label: '说明书' },
]

// 当前列映射（colIndex → fieldName）
const mapping = ref<Record<number, string>>({})

// 初始化自动映射
watch(() => props.autoMapping, (val) => {
  mapping.value = { ...val }
}, { immediate: true })

// 每列的映射状态
function getMappingStatus(colIndex: number): 'matched' | 'unmatched' | 'missing' {
  const field = mapping.value[colIndex]
  if (!field) return 'missing'
  return 'matched'
}

// 预览数据（前3行）
const previewRows = computed(() => props.rows.slice(0, 3))

function handleConfirm() {
  emit('confirm', { ...mapping.value })
}
</script>

<template>
  <div class="field-mapping" v-if="headers.length > 0">
    <h3>字段映射</h3>
    <p class="mapping-desc">自动识别结果如下，可手动调整列与字段的对应关系</p>

    <el-table :data="headers.map((h, i) => ({ index: i, header: h }))" size="small" border>
      <el-table-column label="列名" prop="header" width="160" />
      <el-table-column label="映射字段" width="200">
        <template #default="{ row }">
          <el-select
            v-model="mapping[row.index]"
            placeholder="未映射"
            clearable
            size="small"
            style="width: 100%"
          >
            <el-option
              v-for="field in availableFields"
              :key="field.value"
              :label="field.label"
              :value="field.value"
            />
          </el-select>
        </template>
      </el-table-column>
      <el-table-column label="状态" width="80" align="center">
        <template #default="{ row }">
          <el-tag
            :type="getMappingStatus(row.index) === 'matched' ? 'success' : 'warning'"
            size="small"
            round
          >
            {{ getMappingStatus(row.index) === 'matched' ? '已映射' : '未映射' }}
          </el-tag>
        </template>
      </el-table-column>
      <el-table-column label="数据预览">
        <template #default="{ row }">
          <div class="preview-cells">
            <span v-for="(r, ri) in previewRows" :key="ri" class="preview-cell">
              {{ r[row.index] || '-' }}
            </span>
          </div>
        </template>
      </el-table-column>
    </el-table>

    <div class="mapping-actions">
      <el-button type="primary" size="small" @click="handleConfirm">确认映射</el-button>
    </div>
  </div>
</template>

<style scoped>
.field-mapping h3 {
  font-size: 14px;
  font-weight: 600;
  margin-bottom: 4px;
}

.mapping-desc {
  color: #909399;
  font-size: 12px;
  margin-bottom: 12px;
}

.preview-cells {
  display: flex;
  gap: 8px;
}

.preview-cell {
  font-size: 12px;
  color: #606266;
  max-width: 120px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.mapping-actions {
  margin-top: 12px;
  display: flex;
  justify-content: flex-end;
}
</style>
