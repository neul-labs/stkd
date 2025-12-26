<script setup lang="ts">
import { onMounted, watch } from 'vue'
import { useRoute } from 'vue-router'
import { useOrganizationStore } from '@/stores/organization'
import { useRepositoriesStore } from '@/stores/repositories'

const route = useRoute()
const orgStore = useOrganizationStore()
const repoStore = useRepositoriesStore()

async function loadData() {
  const slug = route.params.slug as string
  await Promise.all([
    orgStore.fetchOrganization(slug),
    repoStore.fetchRepositories(slug)
  ])
}

onMounted(loadData)
watch(() => route.params.slug, loadData)
</script>

<template>
  <div class="space-y-6">
    <div v-if="orgStore.currentOrg" class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {{ orgStore.currentOrg.name }}
        </h1>
        <p v-if="orgStore.currentOrg.description" class="text-gray-600 dark:text-gray-400">
          {{ orgStore.currentOrg.description }}
        </p>
      </div>
      <button class="btn btn-primary">Connect Repository</button>
    </div>

    <div v-if="repoStore.loading" class="flex justify-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-stack-600"></div>
    </div>

    <div v-else-if="repoStore.repositories.length === 0" class="card p-8 text-center">
      <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-2">No repositories</h2>
      <p class="text-gray-600 dark:text-gray-400 mb-4">
        Connect a GitHub or GitLab repository to start tracking stacks.
      </p>
      <button class="btn btn-primary">Connect Repository</button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 gap-4">
      <router-link
        v-for="repo in repoStore.repositories"
        :key="repo.id"
        :to="{
          name: 'repository',
          params: { slug: route.params.slug, repoId: repo.id }
        }"
        class="card p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-center justify-between">
          <div>
            <h3 class="font-medium text-gray-900 dark:text-gray-100">{{ repo.full_name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">
              {{ repo.provider }} · {{ repo.default_branch }}
            </p>
          </div>
          <span
            class="px-2 py-1 text-xs rounded-full"
            :class="
              repo.is_active
                ? 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200'
                : 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200'
            "
          >
            {{ repo.is_active ? 'Active' : 'Inactive' }}
          </span>
        </div>
        <p v-if="repo.synced_at" class="mt-2 text-xs text-gray-500 dark:text-gray-400">
          Last synced: {{ new Date(repo.synced_at).toLocaleString() }}
        </p>
      </router-link>
    </div>
  </div>
</template>
