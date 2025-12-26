<script setup lang="ts">
import type { Branch } from '@/api/repos'
import PrStatusBadge from './PrStatusBadge.vue'

const props = defineProps<{
  branch: Branch
  depth: number
  hasChildren: boolean
}>()
</script>

<template>
  <div
    class="flex items-center py-2 px-3 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800"
    :style="{ paddingLeft: `${depth * 24 + 12}px` }"
  >
    <!-- Tree connector -->
    <div class="flex items-center mr-2 text-gray-400">
      <span v-if="depth > 0" class="mr-1">{{ hasChildren ? '├' : '└' }}─</span>
      <span class="text-lg">{{ branch.status === 'merged' ? '●' : '○' }}</span>
    </div>

    <!-- Branch name -->
    <span
      class="font-medium mr-3"
      :class="{
        'text-green-600 dark:text-green-400': branch.status === 'merged',
        'text-gray-900 dark:text-gray-100': branch.status !== 'merged'
      }"
    >
      {{ branch.name }}
    </span>

    <!-- Status badge -->
    <span
      v-if="branch.status !== 'local'"
      class="text-xs px-2 py-0.5 rounded-full mr-2"
      :class="{
        'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200':
          branch.status === 'merged',
        'bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200':
          branch.status === 'active',
        'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200':
          branch.status === 'closed'
      }"
    >
      {{ branch.status }}
    </span>

    <!-- PR badge -->
    <PrStatusBadge v-if="branch.mr" :mr="branch.mr" />

    <!-- Commit SHA -->
    <span v-if="branch.head_sha" class="text-xs text-gray-500 dark:text-gray-400 ml-auto font-mono">
      {{ branch.head_sha.slice(0, 7) }}
    </span>
  </div>
</template>
