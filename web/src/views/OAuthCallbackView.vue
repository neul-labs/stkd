<script setup lang="ts">
import { onMounted, ref } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { post } from '@/api/client'
import type { LoginResponse } from '@/api/auth'

const route = useRoute()
const router = useRouter()
const authStore = useAuthStore()

const error = ref<string | null>(null)
const loading = ref(true)

onMounted(async () => {
  const code = route.query.code as string
  const state = route.query.state as string
  const provider = route.path.includes('github') ? 'github' : 'gitlab'

  if (!code) {
    error.value = 'No authorization code received'
    loading.value = false
    return
  }

  try {
    // Exchange code for token
    const response = await post<LoginResponse>(
      `/auth/oauth/${provider}/callback?code=${code}&state=${state}`
    )

    authStore.setToken(response.token)
    authStore.setUser(response.user)

    // Redirect to dashboard or original destination
    const redirect = route.query.redirect as string || '/'
    router.push(redirect)
  } catch (err) {
    error.value = 'Failed to complete authentication'
    loading.value = false
  }
})
</script>

<template>
  <div class="min-h-screen flex items-center justify-center bg-gray-50 dark:bg-gray-900">
    <div class="text-center">
      <div v-if="loading" class="space-y-4">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-stack-600 mx-auto"></div>
        <p class="text-gray-600 dark:text-gray-400">Completing authentication...</p>
      </div>

      <div v-else-if="error" class="space-y-4">
        <div class="text-red-500 text-lg">{{ error }}</div>
        <router-link to="/login" class="btn btn-primary">Back to Login</router-link>
      </div>
    </div>
  </div>
</template>
