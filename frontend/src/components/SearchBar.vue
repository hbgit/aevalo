<!-- Search Bar Component -->
<template>
  <div class="search-bar">
    <div class="relative">
      <input
        type="text"
        placeholder="Search evaluations by title or question content..."
        class="input-base w-full pl-12"
        @input="onInput"
        @keydown.enter="onSearch"
      />
      <svg class="absolute left-3 top-3 w-6 h-6 text-gray-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const query = ref('')

const emit = defineEmits(['search'])

const onInput = (e: Event) => {
  query.value = (e.target as HTMLInputElement).value
}

const onSearch = () => {
  emit('search', query.value)
}
</script>
