<script setup lang="ts">
const props = defineProps<{
  moduleId: string
  output: any
}>()

// M1 字段标签映射
const m1FieldLabels: Record<string, string> = {
  publication_number: '公开号',
  application_number: '申请号',
  applicant: '申请人',
  inventor: '发明人',
  title: '发明名称',
  title_original: '原文标题',
  filing_date: '申请日',
  publication_date: '公开日',
  ipc: 'IPC分类号',
  cpc: 'CPC分类号',
  abstract_text: '摘要',
  abstract_text_original: '原文摘要',
}

// 判断是否有原文对照
function hasOriginal(key: string): boolean {
  const origKey = key + '_original'
  return props.output[origKey] && props.output[origKey] !== props.output[key]
}

function getOriginal(key: string): string {
  return props.output[key + '_original'] || ''
}

// 判断是否为图片数据
function isFigureImage(fig: any): boolean {
  return fig && fig.imageBase64
}
</script>

<template>
  <div class="module-preview">
    <!-- M1 专利基本信息：结构化展示 -->
    <template v-if="moduleId === 'M1'">
      <div class="m1-info-table">
        <template v-for="(label, key) in m1FieldLabels" :key="key">
          <template v-if="output[key]">
            <div class="info-row" v-if="!key.endsWith('_original')">
              <span class="info-label">{{ label }}</span>
              <span class="info-value">
                {{ output[key] }}
                <span v-if="hasOriginal(key)" class="original-hint">
                  <br>原文：{{ getOriginal(key) }}
                </span>
              </span>
            </div>
          </template>
        </template>
      </div>
    </template>

    <!-- M5 权利要求：双语对照展示 -->
    <template v-else-if="moduleId === 'M5'">
      <div class="m5-claims">
        <div v-if="output.independent_claims?.length" class="claims-group">
          <h4>独立权利要求</h4>
          <div v-for="claim in output.independent_claims" :key="claim.claim_number" class="claim-card">
            <div class="claim-num">权利要求 {{ claim.claim_number }}</div>
            <div v-if="claim.claim_text_original && claim.claim_text_original !== claim.claim_text" class="claim-bilingual">
              <div class="claim-translation">
                <div class="claim-label">中文翻译</div>
                <div class="claim-text">{{ claim.claim_text }}</div>
              </div>
              <div class="claim-original">
                <div class="claim-label">原文</div>
                <div class="claim-text">{{ claim.claim_text_original }}</div>
              </div>
            </div>
            <div v-else class="claim-text">{{ claim.claim_text }}</div>
            <div v-if="claim.core_features?.length" class="features">
              <strong>必要技术特征：</strong>
              <ul>
                <li v-for="f in claim.core_features" :key="f">{{ f }}</li>
              </ul>
            </div>
            <div v-if="claim.scope_summary" class="scope-text">{{ claim.scope_summary }}</div>
          </div>
        </div>
        <div v-if="output.dependent_claims?.length" class="claims-group">
          <h4>从属权利要求</h4>
          <div v-for="claim in output.dependent_claims" :key="claim.claim_number" class="claim-card dependent">
            <div class="claim-num">权利要求 {{ claim.claim_number }}（引用权利要求 {{ claim.depends_on }}）</div>
            <div v-if="claim.additional_limitation_original && claim.additional_limitation_original !== claim.additional_limitation" class="claim-bilingual">
              <div class="claim-translation">
                <div class="claim-label">中文翻译</div>
                <div class="claim-text">{{ claim.additional_limitation }}</div>
              </div>
              <div class="claim-original">
                <div class="claim-label">原文</div>
                <div class="claim-text">{{ claim.additional_limitation_original }}</div>
              </div>
            </div>
            <div v-else class="claim-text">{{ claim.additional_limitation }}</div>
            <div v-if="claim.scope_narrowing" class="scope-narrowing">范围缩小：{{ claim.scope_narrowing }}</div>
          </div>
        </div>
      </div>
    </template>

    <!-- E2 附图：展示图片 -->
    <template v-else-if="moduleId === 'E2'">
      <div class="e2-figures">
        <div v-for="(fig, idx) in (output.figures || [])" :key="idx" class="figure-card">
          <div class="figure-header">
            <span class="figure-num">{{ fig.figure_number || fig.figureNumber || `图${idx + 1}` }}</span>
            <span class="figure-title">{{ fig.title || '' }}</span>
          </div>
          <div v-if="isFigureImage(fig)" class="figure-image-container">
            <img :src="`data:image/png;base64,${fig.imageBase64}`" :alt="fig.figure_number || `图${idx + 1}`" class="figure-image" />
          </div>
          <div v-if="fig.description" class="figure-desc">{{ fig.description }}</div>
        </div>
        <!-- OCR 来源的图片 -->
        <div v-for="(fig, idx) in (output.ocr_figures || [])" :key="`ocr-${idx}`" class="figure-card ocr-figure">
          <div class="figure-header">
            <span class="figure-num">{{ fig.figureNum || fig.figure_num || `图${idx + 1}` }}</span>
            <span class="figure-title">OCR 识别附图（第{{ fig.pageNumber || fig.page_number }}页）</span>
          </div>
          <div v-if="fig.imageBase64 || fig.image_base64" class="figure-image-container">
            <img :src="`data:image/png;base64,${fig.imageBase64 || fig.image_base64}`" :alt="fig.figureNum || `图${idx + 1}`" class="figure-image" />
          </div>
        </div>
      </div>
    </template>

    <!-- 其他模块：JSON 预览 -->
    <template v-else>
      <pre class="output-json">{{ JSON.stringify(output, null, 2) }}</pre>
    </template>
  </div>
</template>

<style scoped>
.module-preview {
  font-size: 12px;
}

/* M1 信息表 */
.m1-info-table {
  display: flex;
  flex-direction: column;
  gap: 4px;
}
.info-row {
  display: flex;
  gap: 12px;
  padding: 4px 0;
  border-bottom: 1px solid var(--app-border, #ebeef5);
}
.info-label {
  min-width: 80px;
  color: var(--app-text-secondary, #909399);
  font-size: 12px;
  font-weight: 500;
  flex-shrink: 0;
}
.info-value {
  font-size: 12px;
  line-height: 1.5;
  color: var(--app-text, #303133);
}
.original-hint {
  font-size: 11px;
  color: var(--app-text-secondary, #909399);
}

/* M5 权利要求 */
.m5-claims {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.claims-group h4 {
  font-size: 13px;
  color: var(--app-text-secondary, #606266);
  margin: 8px 0 4px;
}
.claim-card {
  padding: 8px 12px;
  background: var(--app-module-bg, #fafbfc);
  border: 1px solid var(--app-border, #ebeef5);
  border-radius: 6px;
  border-left: 3px solid #409eff;
}
.claim-card.dependent {
  border-left-color: #e6a23c;
  background: #fffbf0;
}
.claim-num {
  font-weight: 700;
  color: #409eff;
  font-size: 12px;
  margin-bottom: 4px;
}
.claim-text {
  font-size: 12px;
  color: var(--app-text, #606266);
  line-height: 1.5;
}
.claim-bilingual {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 12px;
  margin-bottom: 4px;
}
.claim-translation {
  padding: 8px 10px;
  background: #f0f9eb;
  border-radius: 4px;
  border: 1px solid #e1f3d8;
}
.claim-original {
  padding: 8px 10px;
  background: var(--app-module-bg, #f5f7fa);
  border-radius: 4px;
  border: 1px solid var(--app-border, #ebeef5);
}
.claim-label {
  font-size: 10px;
  color: #909399;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 2px;
}
.features {
  margin: 4px 0;
}
.features strong {
  font-size: 11px;
  color: #909399;
}
.features ul {
  padding-left: 16px;
  font-size: 12px;
  margin-top: 2px;
}
.scope-text {
  font-size: 11px;
  color: #909399;
  font-style: italic;
}
.scope-narrowing {
  font-size: 11px;
  color: #e6a23c;
  margin-top: 4px;
  padding: 2px 6px;
  background: #fffbf0;
  border-radius: 3px;
  border-left: 2px solid #e6a23c;
}

/* E2 附图 */
.e2-figures {
  display: flex;
  flex-direction: column;
  gap: 8px;
}
.figure-card {
  padding: 8px 12px;
  background: var(--app-module-bg, #fafbfc);
  border: 1px solid var(--app-border, #ebeef5);
  border-radius: 6px;
  border-left: 3px solid #409eff;
}
.figure-card.ocr-figure {
  border-left-color: #67c23a;
}
.figure-header {
  display: flex;
  align-items: center;
  gap: 6px;
  margin-bottom: 4px;
}
.figure-num {
  background: #409eff;
  color: #fff;
  font-size: 11px;
  font-weight: 700;
  padding: 1px 6px;
  border-radius: 3px;
}
.figure-title {
  font-size: 13px;
  font-weight: 500;
}
.figure-image-container {
  text-align: center;
  margin: 4px 0;
}
.figure-image-container img {
  max-width: 100%;
  max-height: 300px;
  border: 1px solid var(--app-border, #ebeef5);
  border-radius: 4px;
}
.figure-desc {
  font-size: 12px;
  color: var(--app-text-secondary, #606266);
  line-height: 1.5;
}

/* 通用 JSON */
.output-json {
  white-space: pre-wrap;
  word-break: break-all;
  margin: 0;
  font-family: 'Menlo', 'Monaco', 'Consolas', monospace;
  font-size: 11px;
  line-height: 1.5;
  color: var(--app-text);
}
</style>
