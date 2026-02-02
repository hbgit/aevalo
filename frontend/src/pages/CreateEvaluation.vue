<!-- Create Evaluation Page -->
<template>
  <div class="create-evaluation">
    <h1 class="text-3xl font-bold text-gray-900 mb-8">Create New Evaluation</h1>

    <div class="grid grid-cols-1 md:grid-cols-2 gap-8">
      <!-- Template Selection -->
      <div class="card p-6">
        <h2 class="text-xl font-semibold mb-4">From Template</h2>
        <div class="space-y-2">
          <button v-for="template in templates" :key="template.id" class="btn-primary w-full text-left">
            {{ template.name }}
          </button>
        </div>
      </div>

      <!-- AI Generation -->
      <div class="card p-6">
        <h2 class="text-xl font-semibold mb-4">AI-Generated</h2>
        <textarea class="input-base w-full h-32 mb-4" placeholder="Describe your evaluation needs..."></textarea>
        <button class="btn-secondary w-full">Generate with AI</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const templates = ref([
  { id: 1, name: 'NPS Survey' },
  { id: 2, name: 'Customer Satisfaction' },
  { id: 3, name: 'Employee Feedback' },
])
</script>
