import { defineStore } from 'pinia'
import { ref } from 'vue'

export interface Annotation {
  id: string
  patentId: string
  moduleId: string
  text: string
  quote?: string
  createdAt: string
  updatedAt: string
}

export const useAnnotationStore = defineStore('annotation', () => {
  const annotations = ref<Annotation[]>([])

  function addAnnotation(annotation: Omit<Annotation, 'id' | 'createdAt' | 'updatedAt'>) {
    const now = new Date().toISOString()
    annotations.value.push({
      ...annotation,
      id: crypto.randomUUID(),
      createdAt: now,
      updatedAt: now,
    })
  }

  function updateAnnotation(id: string, text: string) {
    const item = annotations.value.find(a => a.id === id)
    if (item) {
      item.text = text
      item.updatedAt = new Date().toISOString()
    }
  }

  function deleteAnnotation(id: string) {
    const idx = annotations.value.findIndex(a => a.id === id)
    if (idx >= 0) annotations.value.splice(idx, 1)
  }

  function getAnnotations(patentId: string, moduleId?: string): Annotation[] {
    return annotations.value.filter(a => {
      if (a.patentId !== patentId) return false
      if (moduleId && a.moduleId !== moduleId) return false
      return true
    })
  }

  function exportAnnotations(): string {
    return JSON.stringify(annotations.value, null, 2)
  }

  function importAnnotations(json: string) {
    try {
      const imported: Annotation[] = JSON.parse(json)
      for (const a of imported) {
        if (!annotations.value.some(e => e.id === a.id)) {
          annotations.value.push(a)
        }
      }
    } catch {
      // ignore invalid JSON
    }
  }

  function clearAll() {
    annotations.value = []
  }

  return {
    annotations,
    addAnnotation,
    updateAnnotation,
    deleteAnnotation,
    getAnnotations,
    exportAnnotations,
    importAnnotations,
    clearAll,
  }
})
