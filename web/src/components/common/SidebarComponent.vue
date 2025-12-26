<script setup lang="ts">
import { useRoute } from 'vue-router'
import { useOrganizationStore } from '@/stores/organization'
import { useRepositoriesStore } from '@/stores/repositories'
import { computed, watch } from 'vue'

const route = useRoute()
const orgStore = useOrganizationStore()
const repoStore = useRepositoriesStore()

const currentSlug = computed(() => route.params.slug as string | undefined)

watch(
  currentSlug,
  async (slug) => {
    if (slug) {
      await repoStore.fetchRepositories(slug)
    }
  },
  { immediate: true }
)
</script>

<template>
  <aside class="w-64 bg-white dark:bg-gray-800 border-r border-gray-200 dark:border-gray-700 min-h-[calc(100vh-57px)]">
    <nav class="p-4">
      <div v-if="currentSlug" class="space-y-4">
        <!-- Repositories Section -->
        <div>
          <h3 class="text-xs font-semibold text-gray-500 dark:text-gray-400 uppercase tracking-wider mb-2">
            Repositories
          </h3>
          <ul class="space-y-1">
            <li v-for="repo in repoStore.repositories" :key="repo.id">
              <router-link
                :to="{ name: 'repository', params: { slug: currentSlug, repoId: repo.id } }"
                class="block px-3 py-2 text-sm rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                :class="{
                  'bg-gray-100 dark:bg-gray-700': route.params.repoId === repo.id
                }"
              >
                <div class="flex items-center">
                  <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M3 7v10a2 2 0 002 2h14a2 2 0 002-2V9a2 2 0 00-2-2h-6l-2-2H5a2 2 0 00-2 2z"
                    />
                  </svg>
                  {{ repo.name }}
                </div>
              </router-link>
            </li>
          </ul>
        </div>

        <!-- Settings Link -->
        <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
          <router-link
            :to="{ name: 'org-settings', params: { slug: currentSlug } }"
            class="flex items-center px-3 py-2 text-sm rounded-lg text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
          >
            <svg class="w-4 h-4 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"
              />
              <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"
              />
            </svg>
            Settings
          </router-link>
        </div>
      </div>

      <div v-else class="text-sm text-gray-500 dark:text-gray-400">
        Select an organization to view repositories
      </div>
    </nav>
  </aside>
</template>
