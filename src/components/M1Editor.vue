<script setup lang="ts">
import { reactive, watch } from 'vue'
import type { PatentData } from '../types/patent'

const props = defineProps<{
  modelValue: PatentData
}>()

const emit = defineEmits<{
  'update:modelValue': [value: PatentData]
}>()

const form = reactive<Partial<PatentData>>({})

// 同步外部数据到表单
watch(() => props.modelValue, (val) => {
  if (!val) return
  const fields: (keyof PatentData)[] = [
    'publicationNumber', 'grantNumber', 'applicationNumber',
    'applicant', 'inventor', 'title',
    'filingDate', 'priorityDate', 'publicationDate', 'grantDate',
    'legalStatus', 'ipc', 'cpc', 'abstractText',
  ]
  for (const f of fields) {
    if ((form as any)[f] !== val[f]) {
      (form as any)[f] = val[f] ?? ''
    }
  }
}, { immediate: true, deep: true })

function onInput() {
  const updated = { ...props.modelValue }
  const fields: (keyof PatentData)[] = [
    'publicationNumber', 'grantNumber', 'applicationNumber',
    'applicant', 'inventor', 'title',
    'filingDate', 'priorityDate', 'publicationDate', 'grantDate',
    'legalStatus', 'ipc', 'cpc', 'abstractText',
  ]
  for (const f of fields) {
    const v = (form as any)[f]
    if (v && typeof v === 'string' && v.trim()) {
      (updated as any)[f] = v.trim()
    } else {
      (updated as any)[f] = undefined
    }
  }
  emit('update:modelValue', updated)
}

interface FieldDef {
  key: string
  label: string
  span?: number
  type?: string
}

const fieldGroups: { title: string; fields: FieldDef[] }[] = [
  {
    title: '基本著录信息',
    fields: [
      { key: 'publicationNumber', label: '公开号/专利号' },
      { key: 'grantNumber', label: '授权号' },
      { key: 'applicationNumber', label: '申请号' },
      { key: 'title', label: '发明名称', span: 2 },
      { key: 'applicant', label: '申请人' },
      { key: 'inventor', label: '发明人' },
    ],
  },
  {
    title: '日期信息',
    fields: [
      { key: 'filingDate', label: '申请日' },
      { key: 'priorityDate', label: '优先权日' },
      { key: 'publicationDate', label: '公开日' },
      { key: 'grantDate', label: '授权日' },
    ],
  },
  {
    title: '分类与摘要',
    fields: [
      { key: 'legalStatus', label: '法律状态' },
      { key: 'ipc', label: 'IPC分类号' },
      { key: 'cpc', label: 'CPC分类号' },
      { key: 'abstractText', label: '摘要', span: 2, type: 'textarea' },
    ],
  },
]
</script>

<template>
  <div class="m1-editor">
    <div v-for="group in fieldGroups" :key="group.title" class="field-group">
      <h4 class="group-title">{{ group.title }}</h4>
      <div class="field-grid">
        <div
          v-for="f in group.fields"
          :key="f.key"
          class="field-item"
          :class="{ 'span-2': f.span === 2 }"
        >
          <label class="field-label">{{ f.label }}</label>
          <el-input
            v-if="f.type === 'textarea'"
            v-model="(form as any)[f.key]"
            type="textarea"
            :rows="3"
            size="small"
            @input="onInput"
          />
          <el-input
            v-else
            v-model="(form as any)[f.key]"
            size="small"
            :placeholder="f.label"
            @input="onInput"
          />
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.m1-editor {
  padding: 4px 0;
}

.field-group {
  margin-bottom: 12px;
}

.group-title {
  font-size: 12px;
  color: var(--app-text-secondary);
  font-weight: 600;
  margin-bottom: 6px;
  padding-bottom: 4px;
  border-bottom: 1px solid var(--app-border);
}

.field-grid {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 8px;
}

.field-item.span-2 {
  grid-column: span 2;
}

.field-label {
  display: block;
  font-size: 11px;
  color: var(--app-text-secondary);
  margin-bottom: 2px;
}
</style>
