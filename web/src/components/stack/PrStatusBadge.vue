<script setup lang="ts">
import type { MergeRequest } from '@/api/repos'

const props = defineProps<{
  mr: MergeRequest
}>()

const stateColors: Record<string, string> = {
  open: 'bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200',
  draft: 'bg-gray-100 text-gray-800 dark:bg-gray-700 dark:text-gray-200',
  merged: 'bg-purple-100 text-purple-800 dark:bg-purple-900 dark:text-purple-200',
  closed: 'bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200'
}

const stateIcons: Record<string, string> = {
  open: 'M10.75 8a.75.75 0 01.75.75v4.5a.75.75 0 01-1.5 0v-4.5a.75.75 0 01.75-.75zm0 10a1 1 0 100-2 1 1 0 000 2z',
  draft: 'M3 5a2 2 0 012-2h1.862c.767 0 1.477.396 1.878 1.05l.726 1.181A1 1 0 0010.326 6H17a2 2 0 012 2v8a2 2 0 01-2 2H5a2 2 0 01-2-2V5z',
  merged: 'M7 9a2 2 0 012-2h6a2 2 0 012 2v6a2 2 0 01-2 2H9a2 2 0 01-2-2V9z',
  closed: 'M10 18a8 8 0 100-16 8 8 0 000 16zM8.707 7.293a1 1 0 00-1.414 1.414L8.586 10l-1.293 1.293a1 1 0 101.414 1.414L10 11.414l1.293 1.293a1 1 0 001.414-1.414L11.414 10l1.293-1.293a1 1 0 00-1.414-1.414L10 8.586 8.707 7.293z'
}
</script>

<template>
  <a
    :href="mr.url"
    target="_blank"
    rel="noopener noreferrer"
    class="inline-flex items-center px-2 py-1 rounded text-xs font-medium hover:opacity-80 transition-opacity"
    :class="stateColors[mr.state] || stateColors.open"
    :title="mr.title"
  >
    <svg class="w-3 h-3 mr-1" fill="currentColor" viewBox="0 0 20 20">
      <path :d="stateIcons[mr.state] || stateIcons.open" />
    </svg>
    #{{ mr.number }}
  </a>
</template>
