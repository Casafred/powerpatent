import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface CacheEntry {
  projectId: string
  patentId: string
  moduleId: string
  level: string
  outputJson: string
  model: string
  provider: string
  promptVersion: string
  temperature: number
  rerunCount: number
  createdAt: string
  updatedAt: string
}

export const useCacheStore = defineStore('cache', () => {
  const entries = ref<CacheEntry[]>([])
  const loading = ref(false)

  function getEntry(
    projectId: string,
    patentId: string,
    moduleId: string,
  ): CacheEntry | undefined {
    return entries.value.find(
      (e) => e.projectId === projectId && e.patentId === patentId && e.moduleId === moduleId,
    )
  }

  function upsertEntry(entry: CacheEntry) {
    const idx = entries.value.findIndex(
      (e) =>
        e.projectId === entry.projectId &&
        e.patentId === entry.patentId &&
        e.moduleId === entry.moduleId &&
        e.level === entry.level,
    )
    if (idx >= 0) {
      entries.value[idx] = entry
    } else {
      entries.value.push(entry)
    }
  }

  function clearCache() {
    entries.value = []
  }

  return {
    entries,
    loading,
    getEntry,
    upsertEntry,
    clearCache,
  }
})
