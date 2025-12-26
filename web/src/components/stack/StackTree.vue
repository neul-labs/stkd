<script setup lang="ts">
import { computed } from 'vue'
import type { Branch } from '@/api/repos'
import StackNode from './StackNode.vue'

const props = defineProps<{
  branches: Branch[]
  defaultBranch: string
}>()

// Build tree structure from flat branch list
interface TreeNode {
  branch: Branch
  children: TreeNode[]
  depth: number
}

const tree = computed(() => {
  const branchMap = new Map<string, Branch>()
  props.branches.forEach((b) => branchMap.set(b.name, b))

  // Find root branches (those whose parent is the default branch or null)
  const roots: TreeNode[] = []
  const nodeMap = new Map<string, TreeNode>()

  // Create nodes for all branches
  props.branches.forEach((branch) => {
    nodeMap.set(branch.name, {
      branch,
      children: [],
      depth: 0
    })
  })

  // Build tree relationships
  props.branches.forEach((branch) => {
    const node = nodeMap.get(branch.name)!
    if (!branch.parent_name || branch.parent_name === props.defaultBranch) {
      roots.push(node)
    } else {
      const parent = nodeMap.get(branch.parent_name)
      if (parent) {
        parent.children.push(node)
      } else {
        roots.push(node)
      }
    }
  })

  // Calculate depths
  function setDepth(nodes: TreeNode[], depth: number) {
    nodes.forEach((node) => {
      node.depth = depth
      setDepth(node.children, depth + 1)
    })
  }
  setDepth(roots, 0)

  return roots
})

// Flatten tree for rendering
function flattenTree(nodes: TreeNode[], result: TreeNode[] = []): TreeNode[] {
  nodes.forEach((node) => {
    result.push(node)
    flattenTree(node.children, result)
  })
  return result
}

const flatNodes = computed(() => flattenTree(tree.value))
</script>

<template>
  <div class="space-y-1">
    <div v-if="branches.length === 0" class="text-gray-500 dark:text-gray-400 text-sm py-4">
      No branches in this stack
    </div>
    <StackNode
      v-for="node in flatNodes"
      :key="node.branch.id"
      :branch="node.branch"
      :depth="node.depth"
      :has-children="node.children.length > 0"
    />
  </div>
</template>
