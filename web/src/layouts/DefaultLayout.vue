<script setup lang="ts">
import { RouterView } from 'vue-router'
import HeaderComponent from '@/components/common/HeaderComponent.vue'
import SidebarComponent from '@/components/common/SidebarComponent.vue'
</script>

<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900">
    <HeaderComponent />
    <div class="flex">
      <SidebarComponent />
      <main class="flex-1 p-6">
        <RouterView />
      </main>
    </div>
  </div>
</template>
