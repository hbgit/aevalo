<!-- Dashboard Page Component -->
<template>
  <div class="dashboard">
    <!-- Header with Search -->
    <div class="mb-8">
      <h1 class="text-3xl font-bold text-gray-900 mb-4">Dashboard</h1>
      <SearchBar @search="handleSearch" />
    </div>

    <!-- Status Tracker Card -->
    <div class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-8">
      <div class="card p-6">
        <h3 class="text-lg font-semibold text-gray-700">Active Evaluations</h3>
        <p class="text-4xl font-bold text-primary mt-2">{{ activeCount }}</p>
      </div>
      <div class="card p-6">
        <h3 class="text-lg font-semibold text-gray-700">Total Responses</h3>
        <p class="text-4xl font-bold text-secondary mt-2">{{ totalResponses }}</p>
      </div>
      <div class="card p-6">
        <h3 class="text-lg font-semibold text-gray-700">Categories</h3>
        <p class="text-4xl font-bold text-accent mt-2">{{ categoryCount }}</p>
      </div>
    </div>

    <!-- Main Content Area -->
    <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
      <!-- Evaluation List -->
      <div class="lg:col-span-2">
        <EvaluationList :evaluations="evaluations" />
      </div>

      <!-- Analytics Chart -->
      <div class="lg:col-span-1">
        <AnalyticsChart :data="analyticsData" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import SearchBar from '../components/SearchBar.vue'
import EvaluationList from '../components/EvaluationList.vue'
import AnalyticsChart from '../components/AnalyticsChart.vue'

const evaluations = ref([])
const activeCount = ref(0)
const totalResponses = ref(0)
const categoryCount = ref(0)
const analyticsData = ref({})

onMounted(async () => {
  // TODO: Fetch evaluations via GraphQL
  activeCount.value = 5
  totalResponses.value = 127
  categoryCount.value = 3
})

const handleSearch = (query: string) => {
  // TODO: Implement GraphQL search with optimized fields
  console.log('Search:', query)
}
</script>
