<script setup lang="ts">
import { onMounted, watch, computed } from 'vue'
import { useRoute } from 'vue-router'
import { useRepositoriesStore } from '@/stores/repositories'
import StackTree from '@/components/stack/StackTree.vue'

const route = useRoute()
const repoStore = useRepositoriesStore()

const currentRepo = computed(() => {
  return repoStore.repositories.find((r) => r.id === route.params.repoId)
})

async function loadData() {
  const repoId = route.params.repoId as string
  await repoStore.fetchStack(repoId)
}

onMounted(loadData)
watch(() => route.params.repoId, loadData)
</script>

<template>
  <div class="space-y-6">
    <div v-if="currentRepo" class="flex items-center justify-between">
      <div>
        <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">
          {{ currentRepo.full_name }}
        </h1>
        <p class="text-gray-600 dark:text-gray-400">
          {{ currentRepo.provider }} · Default branch: {{ currentRepo.default_branch }}
        </p>
      </div>
      <button class="btn btn-secondary">Sync</button>
    </div>

    <div class="card">
      <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
        <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">Stack</h2>
      </div>
      <div class="p-4">
        <div v-if="repoStore.loading" class="flex justify-center py-8">
          <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-stack-600"></div>
        </div>
        <StackTree
          v-else-if="repoStore.currentStack"
          :branches="repoStore.currentStack.branches"
          :default-branch="currentRepo?.default_branch || 'main'"
        />
      </div>
    </div>
  </div>
</template>
