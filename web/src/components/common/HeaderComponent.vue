<script setup lang="ts">
import { useAuthStore } from '@/stores/auth'
import OrgSwitcher from '@/components/org/OrgSwitcher.vue'

const authStore = useAuthStore()

function toggleDarkMode() {
  document.documentElement.classList.toggle('dark')
}
</script>

<template>
  <header class="bg-white dark:bg-gray-800 border-b border-gray-200 dark:border-gray-700">
    <div class="flex items-center justify-between px-6 py-3">
      <div class="flex items-center space-x-6">
        <router-link to="/" class="text-xl font-bold text-stack-600">
          Stack
        </router-link>
        <OrgSwitcher />
      </div>

      <div class="flex items-center space-x-4">
        <button
          @click="toggleDarkMode"
          class="p-2 text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
        >
          <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z"
            />
          </svg>
        </button>

        <div v-if="authStore.user" class="flex items-center space-x-3">
          <img
            v-if="authStore.user.avatar_url"
            :src="authStore.user.avatar_url"
            :alt="authStore.user.username"
            class="w-8 h-8 rounded-full"
          />
          <span class="text-sm font-medium text-gray-700 dark:text-gray-300">
            {{ authStore.user.display_name || authStore.user.username }}
          </span>
          <button
            @click="authStore.logout()"
            class="text-sm text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200"
          >
            Logout
          </button>
        </div>
      </div>
    </div>
  </header>
</template>
