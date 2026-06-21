import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useProjectStore = defineStore('project', () => {
  const projectId = ref<string>(crypto.randomUUID())
  const projectName = ref('')

  function newProject() {
    projectId.value = crypto.randomUUID()
    projectName.value = ''
  }

  return {
    projectId,
    projectName,
    newProject,
  }
})
