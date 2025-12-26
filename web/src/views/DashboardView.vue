<script setup lang="ts">
import { onMounted } from 'vue'
import { useOrganizationStore } from '@/stores/organization'

const orgStore = useOrganizationStore()

onMounted(async () => {
  await orgStore.fetchOrganizations()
})
</script>

<template>
  <div class="space-y-6">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Dashboard</h1>

    <div v-if="orgStore.loading" class="flex justify-center py-8">
      <div class="animate-spin rounded-full h-8 w-8 border-b-2 border-stack-600"></div>
    </div>

    <div v-else-if="orgStore.organizations.length === 0" class="card p-8 text-center">
      <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100 mb-2">
        Welcome to Stack!
      </h2>
      <p class="text-gray-600 dark:text-gray-400 mb-4">
        Get started by creating your first organization.
      </p>
      <button class="btn btn-primary">Create Organization</button>
    </div>

    <div v-else class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
      <router-link
        v-for="org in orgStore.organizations"
        :key="org.id"
        :to="{ name: 'organization', params: { slug: org.slug } }"
        class="card p-6 hover:shadow-md transition-shadow"
      >
        <div class="flex items-center space-x-4">
          <div
            v-if="org.avatar_url"
            class="w-12 h-12 rounded-lg bg-gray-200 dark:bg-gray-700 overflow-hidden"
          >
            <img :src="org.avatar_url" :alt="org.name" class="w-full h-full object-cover" />
          </div>
          <div
            v-else
            class="w-12 h-12 rounded-lg bg-stack-100 dark:bg-stack-900 flex items-center justify-center"
          >
            <span class="text-stack-600 dark:text-stack-400 font-bold text-lg">
              {{ org.name.charAt(0).toUpperCase() }}
            </span>
          </div>
          <div>
            <h3 class="font-medium text-gray-900 dark:text-gray-100">{{ org.name }}</h3>
            <p class="text-sm text-gray-500 dark:text-gray-400">{{ org.role }}</p>
          </div>
        </div>
        <p v-if="org.description" class="mt-3 text-sm text-gray-600 dark:text-gray-400">
          {{ org.description }}
        </p>
      </router-link>
    </div>
  </div>
</template>
