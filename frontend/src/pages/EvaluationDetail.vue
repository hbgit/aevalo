<!-- Evaluation Detail Page -->
<template>
  <div class="evaluation-detail">
    <div class="mb-6">
      <RouterLink to="/" class="text-primary hover:underline">← Back to Dashboard</RouterLink>
    </div>

    <div class="card p-8" v-if="evaluation">
      <h1 class="text-3xl font-bold text-gray-900 mb-2">{{ evaluation.title }}</h1>
      <p class="text-gray-600 mb-6">{{ evaluation.description }}</p>

      <div class="grid grid-cols-2 gap-4 mb-8">
        <div>
          <p class="text-sm text-gray-500">Status</p>
          <p class="text-lg font-semibold">{{ evaluation.status }}</p>
        </div>
        <div>
          <p class="text-sm text-gray-500">Scale Type</p>
          <p class="text-lg font-semibold">{{ evaluation.scaleType }}</p>
        </div>
      </div>

      <div class="border-t pt-6">
        <h2 class="text-2xl font-bold mb-4">Questions</h2>
        <div class="space-y-4">
          <!-- Questions will be rendered here -->
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRoute, RouterLink } from 'vue-router'

const route = useRoute()
const evaluation = ref(null)

onMounted(async () => {
  // TODO: Fetch evaluation via GraphQL with optimized fields
  const id = route.params.id
  console.log('Loading evaluation:', id)
})
</script>
