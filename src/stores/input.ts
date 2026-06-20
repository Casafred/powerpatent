import { defineStore } from 'pinia'
import { ref, watch } from 'vue'
import type { PatentData } from '../types/patent'
import { loadPersisted, persist } from '../utils/persist'

export const useInputStore = defineStore('input', () => {
  const patents = ref<PatentData[]>(loadPersisted('patents', []))
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

  // 持久化
  watch(patents, (val) => persist('patents', val), { deep: true })

  return {
    patents,
    loading,
    error,
    setPatents,
    clearPatents,
    removePatent,
  }
})
