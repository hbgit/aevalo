<!-- Evaluation List Component -->
<template>
  <div class="evaluation-list card p-6">
    <h2 class="text-2xl font-bold mb-4">Your Evaluations</h2>

    <div class="overflow-x-auto">
      <table class="w-full">
        <thead class="border-b">
          <tr>
            <th class="text-left py-2 font-semibold">Title</th>
            <th class="text-left py-2 font-semibold">Category</th>
            <th class="text-left py-2 font-semibold">Status</th>
            <th class="text-left py-2 font-semibold">Created</th>
            <th class="text-left py-2 font-semibold">Actions</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="evaluation in evaluations" :key="evaluation.id" class="border-b hover:bg-gray-50">
            <td class="py-3">{{ evaluation.title }}</td>
            <td class="py-3">{{ evaluation.category }}</td>
            <td class="py-3">
              <span :class="`px-2 py-1 rounded text-sm ${getStatusColor(evaluation.status)}`">
                {{ evaluation.status }}
              </span>
            </td>
            <td class="py-3">{{ formatDate(evaluation.createdAt) }}</td>
            <td class="py-3">
              <RouterLink :to="`/evaluation/${evaluation.id}`" class="text-primary hover:underline">View</RouterLink>
            </td>
          </tr>
        </tbody>
      </table>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'

interface Evaluation {
  id: string
  title: string
  category: string
  status: string
  createdAt: string
}

defineProps<{
  evaluations: Evaluation[]
}>()

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    'open': 'bg-green-100 text-green-800',
    'draft': 'bg-yellow-100 text-yellow-800',
    'closed': 'bg-gray-100 text-gray-800',
  }
  return colors[status] || 'bg-gray-100 text-gray-800'
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString()
}
</script>
