import { defineStore } from 'pinia'
import { ref } from 'vue'
import type { PatentData } from '../types/patent'

export const useInputStore = defineStore('input', () => {
  const patents = ref<PatentData[]>([])
  const loading = ref(false)
  const error = ref<string | null>(null)

  function setPatents(data: PatentData[]) {
    patents.value = data
    error.value = null
  }

  function clearPatents() {
    patents.value = []
    error.value = null
  }

  function removePatent(index: number) {
    patents.value.splice(index, 1)
  }

  return {
    patents,
    loading,
    error,
    setPatents,
    clearPatents,
    removePatent,
  }
})
