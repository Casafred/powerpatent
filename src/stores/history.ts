import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import { loadPersisted, persist } from '../utils/persist'

export interface HistorySession {
  id: string
  label: string
  createdAt: string
  projectId: string
  patents: any[] // snapshot of input patent data
  moduleConfig: any // snapshot of module config
  patentCount: number
  patentTitles: string[] // first few titles for preview
}

export const useHistoryStore = defineStore('history', () => {
  const sessions = ref<HistorySession[]>(loadPersisted('history', []))

  function saveSession(params: {
    projectId: string
    patents: any[]
    moduleConfig: any
    label?: string
  }): HistorySession {
    const session: HistorySession = {
      id: crypto.randomUUID(),
      label: params.label || `分析 ${new Date().toLocaleString('zh-CN')}`,
      createdAt: new Date().toISOString(),
      projectId: params.projectId,
      patents: JSON.parse(JSON.stringify(params.patents)),
      moduleConfig: JSON.parse(JSON.stringify(params.moduleConfig)),
      patentCount: params.patents.length,
      patentTitles: params.patents.slice(0, 3).map((p: any) => p.title || p.publicationNumber || '未命名'),
    }
    sessions.value.unshift(session) // newest first
    // Keep max 50 sessions
    if (sessions.value.length > 50) {
      sessions.value = sessions.value.slice(0, 50)
    }
    return session
  }

  function deleteSession(id: string) {
    const idx = sessions.value.findIndex(s => s.id === id)
    if (idx >= 0) sessions.value.splice(idx, 1)
  }

  function getSession(id: string): HistorySession | undefined {
    return sessions.value.find(s => s.id === id)
  }

  function clearAll() {
    sessions.value = []
  }

  // persist
  watch(sessions, (val) => persist('history', val), { deep: true })

  return { sessions, saveSession, deleteSession, getSession, clearAll }
})
