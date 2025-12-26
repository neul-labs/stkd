import { defineStore } from 'pinia'
import { ref } from 'vue'
import { reposApi, type Repository, type Stack } from '@/api/repos'
import { useAuthStore } from './auth'

export const useRepositoriesStore = defineStore('repositories', () => {
  const repositories = ref<Repository[]>([])
  const currentRepo = ref<Repository | null>(null)
  const currentStack = ref<Stack | null>(null)
  const loading = ref(false)

  const authStore = useAuthStore()

  async function fetchRepositories(orgSlug: string) {
    if (!authStore.token) return

    loading.value = true
    try {
      repositories.value = await reposApi.list(authStore.token, orgSlug)
    } finally {
      loading.value = false
    }
  }

  async function fetchStack(repoId: string) {
    if (!authStore.token) return

    loading.value = true
    try {
      currentStack.value = await reposApi.getStack(authStore.token, repoId)
    } finally {
      loading.value = false
    }
  }

  async function connectRepository(
    orgSlug: string,
    provider: string,
    owner: string,
    name: string
  ) {
    if (!authStore.token) return

    const repo = await reposApi.connect(authStore.token, orgSlug, {
      provider,
      owner,
      name
    })
    repositories.value.push(repo)
    return repo
  }

  function setCurrentRepo(repo: Repository | null) {
    currentRepo.value = repo
  }

  return {
    repositories,
    currentRepo,
    currentStack,
    loading,
    fetchRepositories,
    fetchStack,
    connectRepository,
    setCurrentRepo
  }
})
