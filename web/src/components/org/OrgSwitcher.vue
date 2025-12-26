<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useOrganizationStore } from '@/stores/organization'

const router = useRouter()
const orgStore = useOrganizationStore()
const isOpen = ref(false)

onMounted(async () => {
  await orgStore.fetchOrganizations()
})

function selectOrg(slug: string) {
  isOpen.value = false
  router.push({ name: 'organization', params: { slug } })
}
</script>

<template>
  <div class="relative">
    <button
      @click="isOpen = !isOpen"
      class="flex items-center space-x-2 px-3 py-2 text-sm font-medium text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700 rounded-lg"
    >
      <span v-if="orgStore.currentOrg">
        {{ orgStore.currentOrg.name }}
      </span>
      <span v-else class="text-gray-500">Select organization</span>
      <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7" />
      </svg>
    </button>

    <div
      v-if="isOpen"
      class="absolute left-0 mt-2 w-56 bg-white dark:bg-gray-800 rounded-lg shadow-lg border border-gray-200 dark:border-gray-700 z-50"
    >
      <ul class="py-1">
        <li v-for="org in orgStore.organizations" :key="org.id">
          <button
            @click="selectOrg(org.slug)"
            class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            {{ org.name }}
          </button>
        </li>
        <li v-if="orgStore.organizations.length === 0">
          <span class="block px-4 py-2 text-sm text-gray-500">No organizations</span>
        </li>
      </ul>
    </div>
  </div>
</template>
