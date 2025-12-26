<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute } from 'vue-router'
import { useOrganizationStore } from '@/stores/organization'
import { orgsApi, type Member } from '@/api/orgs'
import { useAuthStore } from '@/stores/auth'

const route = useRoute()
const orgStore = useOrganizationStore()
const authStore = useAuthStore()

const members = ref<Member[]>([])
const loading = ref(false)

async function loadMembers() {
  const slug = route.params.slug as string
  if (!authStore.token) return

  loading.value = true
  try {
    members.value = await orgsApi.listMembers(authStore.token, slug)
  } finally {
    loading.value = false
  }
}

onMounted(async () => {
  await orgStore.fetchOrganization(route.params.slug as string)
  await loadMembers()
})
</script>

<template>
  <div class="space-y-6">
    <h1 class="text-2xl font-bold text-gray-900 dark:text-gray-100">Organization Settings</h1>

    <div v-if="orgStore.currentOrg" class="space-y-6">
      <!-- General Settings -->
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700">
          <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">General</h2>
        </div>
        <div class="p-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Organization Name
            </label>
            <input
              type="text"
              :value="orgStore.currentOrg.name"
              class="input"
              readonly
            />
          </div>
          <div>
            <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-1">
              Slug
            </label>
            <input
              type="text"
              :value="orgStore.currentOrg.slug"
              class="input"
              readonly
            />
          </div>
        </div>
      </div>

      <!-- Members -->
      <div class="card">
        <div class="px-6 py-4 border-b border-gray-200 dark:border-gray-700 flex items-center justify-between">
          <h2 class="text-lg font-medium text-gray-900 dark:text-gray-100">Members</h2>
          <button class="btn btn-secondary text-sm">Invite Member</button>
        </div>
        <div class="divide-y divide-gray-200 dark:divide-gray-700">
          <div
            v-for="member in members"
            :key="member.id"
            class="px-6 py-4 flex items-center justify-between"
          >
            <div class="flex items-center space-x-3">
              <img
                v-if="member.avatar_url"
                :src="member.avatar_url"
                :alt="member.username"
                class="w-10 h-10 rounded-full"
              />
              <div
                v-else
                class="w-10 h-10 rounded-full bg-gray-200 dark:bg-gray-700 flex items-center justify-center"
              >
                <span class="text-gray-600 dark:text-gray-400 font-medium">
                  {{ member.username.charAt(0).toUpperCase() }}
                </span>
              </div>
              <div>
                <p class="font-medium text-gray-900 dark:text-gray-100">
                  {{ member.display_name || member.username }}
                </p>
                <p class="text-sm text-gray-500 dark:text-gray-400">@{{ member.username }}</p>
              </div>
            </div>
            <span
              class="px-2 py-1 text-xs rounded-full"
              :class="{
                'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200':
                  member.role === 'owner',
                'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200':
                  member.role === 'admin',
                'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200':
                  member.role === 'member'
              }"
            >
              {{ member.role }}
            </span>
          </div>
        </div>
      </div>

      <!-- Danger Zone -->
      <div class="card border-red-200 dark:border-red-900">
        <div class="px-6 py-4 border-b border-red-200 dark:border-red-900">
          <h2 class="text-lg font-medium text-red-600 dark:text-red-400">Danger Zone</h2>
        </div>
        <div class="p-6">
          <p class="text-gray-600 dark:text-gray-400 mb-4">
            Once you delete an organization, there is no going back. Please be certain.
          </p>
          <button class="btn bg-red-600 text-white hover:bg-red-700">
            Delete Organization
          </button>
        </div>
      </div>
    </div>
  </div>
</template>
