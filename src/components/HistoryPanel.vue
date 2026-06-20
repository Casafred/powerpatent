<script setup lang="ts">
import { useHistoryStore } from '../stores/history'
import { useInputStore } from '../stores/input'
import { useModuleConfigStore } from '../stores/moduleConfig'
import { useProjectStore } from '../stores/project'
import { Delete, RefreshRight } from '@element-plus/icons-vue'

const emit = defineEmits<{
  (e: 'restored', sessionId: string): void
}>()

const historyStore = useHistoryStore()
const inputStore = useInputStore()
const moduleConfigStore = useModuleConfigStore()
const projectStore = useProjectStore()

function formatDate(iso: string): string {
  const d = new Date(iso)
  return d.toLocaleString('zh-CN', {
    year: 'numeric',
    month: '2-digit',
    day: '2-digit',
    hour: '2-digit',
    minute: '2-digit',
  })
}

function restoreSession(id: string) {
  const session = historyStore.getSession(id)
  if (!session) return

  // Restore patents
  inputStore.setPatents(session.patents)

  // Restore module config
  const cfg = session.moduleConfig
  if (cfg) {
    if (cfg.mode !== undefined) moduleConfigStore.setMode(cfg.mode)
    if (cfg.activePreset !== undefined) moduleConfigStore.applyPreset(cfg.activePreset)
    if (cfg.themeName !== undefined) moduleConfigStore.themeName = cfg.themeName
    if (cfg.themeDescription !== undefined) moduleConfigStore.themeDescription = cfg.themeDescription
    if (cfg.globalExtended) {
      for (const [moduleId, level] of Object.entries(cfg.globalExtended)) {
        moduleConfigStore.setModuleLevel(moduleId as any, level as any)
      }
    }
    if (cfg.patentOverrides) {
      for (const [patentId, override] of Object.entries(cfg.patentOverrides)) {
        const o = override as any
        if (o.isKey !== undefined) moduleConfigStore.setKeyPatent(patentId, o.isKey)
        if (o.levels) {
          for (const [moduleId, level] of Object.entries(o.levels)) {
            moduleConfigStore.setPatentModuleLevel(patentId, moduleId as any, level as any)
          }
        }
      }
    }
  }

  // Restore project ID
  projectStore.projectId = session.projectId

  emit('restored', id)
}

function handleDelete(id: string) {
  historyStore.deleteSession(id)
}

function handleClearAll() {
  historyStore.clearAll()
}
</script>

<template>
  <div class="history-panel">
    <div v-if="historyStore.sessions.length === 0" class="history-empty">
      <el-empty description="暂无历史记录" />
    </div>

    <template v-else>
      <div class="history-header">
        <span class="history-count">共 {{ historyStore.sessions.length }} 条记录</span>
        <el-button type="danger" text size="small" @click="handleClearAll">
          清空全部
        </el-button>
      </div>

      <div class="history-list">
        <el-card
          v-for="session in historyStore.sessions"
          :key="session.id"
          shadow="hover"
          class="session-card"
        >
          <div class="session-top">
            <div class="session-label">{{ session.label }}</div>
            <div class="session-actions">
              <el-button
                type="primary"
                size="small"
                :icon="RefreshRight"
                @click="restoreSession(session.id)"
              >
                恢复
              </el-button>
              <el-button
                type="danger"
                size="small"
                :icon="Delete"
                text
                @click="handleDelete(session.id)"
              >
                删除
              </el-button>
            </div>
          </div>

          <div class="session-meta">
            <span class="session-date">{{ formatDate(session.createdAt) }}</span>
            <el-tag size="small" type="info">{{ session.patentCount }} 篇专利</el-tag>
          </div>

          <div v-if="session.patentTitles.length > 0" class="session-titles">
            <span
              v-for="(title, idx) in session.patentTitles"
              :key="idx"
              class="title-item"
            >{{ title }}</span>
            <span v-if="session.patentCount > 3" class="title-more">
              ...等 {{ session.patentCount }} 篇
            </span>
          </div>
        </el-card>
      </div>
    </template>
  </div>
</template>

<style scoped>
.history-panel {
  height: 100%;
  display: flex;
  flex-direction: column;
}

.history-empty {
  flex: 1;
  display: flex;
  align-items: center;
  justify-content: center;
}

.history-header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 8px 0;
  border-bottom: 1px solid var(--app-border);
  margin-bottom: 12px;
}

.history-count {
  font-size: 13px;
  color: var(--app-text-secondary);
}

.history-list {
  flex: 1;
  overflow-y: auto;
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.session-card {
  background: var(--app-card-bg);
  border-color: var(--app-border);
}

.session-card :deep(.el-card__body) {
  padding: 12px 16px;
}

.session-top {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 8px;
}

.session-label {
  font-size: 14px;
  font-weight: 500;
  color: var(--app-text);
  line-height: 1.4;
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.session-actions {
  display: flex;
  align-items: center;
  gap: 4px;
  flex-shrink: 0;
}

.session-meta {
  display: flex;
  align-items: center;
  gap: 8px;
  margin-top: 6px;
}

.session-date {
  font-size: 12px;
  color: var(--app-text-secondary);
}

.session-titles {
  margin-top: 8px;
  display: flex;
  flex-wrap: wrap;
  gap: 4px;
}

.title-item {
  font-size: 12px;
  color: var(--app-text-secondary);
  background: var(--app-module-bg);
  padding: 2px 8px;
  border-radius: 4px;
  max-width: 180px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.title-more {
  font-size: 12px;
  color: var(--app-text-placeholder);
  padding: 2px 4px;
}
</style>
