<script setup lang="ts">
import { RouterView } from 'vue-router'
import { useAuthStore } from '@/stores/auth'
import { onMounted } from 'vue'

const authStore = useAuthStore()

onMounted(async () => {
  // Try to restore session from stored token
  await authStore.restoreSession()
})
</script>

<template>
  <RouterView />
</template>
