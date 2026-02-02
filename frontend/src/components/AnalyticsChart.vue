<!-- Analytics Chart Component -->
<template>
  <div class="analytics-chart card p-6">
    <h2 class="text-2xl font-bold mb-4">Analytics</h2>

    <div class="space-y-6">
      <div>
        <h3 class="text-sm font-semibold text-gray-600 mb-2">Evaluations by Category</h3>
        <div class="space-y-2">
          <div v-for="(count, category) in data" :key="category" class="flex items-center">
            <div class="w-32 text-sm text-gray-700">{{ category }}</div>
            <div class="flex-1 bg-gray-200 rounded h-6">
              <div class="bg-primary h-6 rounded" :style="{ width: (count / 100) * 100 + '%' }"></div>
            </div>
            <div class="w-12 text-right text-sm font-semibold">{{ count }}</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
defineProps<{
  data: Record<string, number>
}>()
</script>
