import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { orgsApi, type Organization } from '@/api/orgs'
import { useAuthStore } from './auth'

export const useOrganizationStore = defineStore('organization', () => {
  const organizations = ref<Organization[]>([])
  const currentOrg = ref<Organization | null>(null)
  const loading = ref(false)

  const authStore = useAuthStore()

  async function fetchOrganizations() {
    if (!authStore.token) return

    loading.value = true
    try {
      organizations.value = await orgsApi.list(authStore.token)
    } finally {
      loading.value = false
    }
  }

  async function fetchOrganization(slug: string) {
    if (!authStore.token) return

    loading.value = true
    try {
      currentOrg.value = await orgsApi.get(authStore.token, slug)
    } finally {
      loading.value = false
    }
  }

  async function createOrganization(name: string, slug: string, description?: string) {
    if (!authStore.token) return

    const org = await orgsApi.create(authStore.token, { name, slug, description })
    organizations.value.push(org)
    return org
  }

  function setCurrentOrg(org: Organization | null) {
    currentOrg.value = org
  }

  return {
    organizations,
    currentOrg,
    loading,
    fetchOrganizations,
    fetchOrganization,
    createOrganization,
    setCurrentOrg
  }
})
