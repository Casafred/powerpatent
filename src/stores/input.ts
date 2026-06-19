import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { PatentData } from '../types/patent'

export const useInputStore = defineStore('input', () => {
  const files = ref<File[]>([])
  const patents = ref<PatentData[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  function addFiles(newFiles: File[]) {
    files.value.push(...newFiles)
  }

  function removeFile(index: number) {
    files.value.splice(index, 1)
  }

  function clearFiles() {
    files.value = []
    patents.value = []
    error.value = null
  }

  return {
    files,
    patents,
    loading,
    error,
    addFiles,
    removeFile,
    clearFiles,
  }
})
