<!-- Search Bar Component -->
<template>
  <div class="search-bar">
    <div class="relative">
      <input
        type="text"
        :placeholder="placeholder"
        class="w-full rounded-full border border-slate-200 bg-slate-50 px-4 py-2.5 pl-11 text-sm text-slate-700 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary"
        @input="onInput"
        @keydown.enter="onSearch"
      />
      <svg class="absolute left-3.5 top-3.5 w-4 h-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
      </svg>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

withDefaults(defineProps<{
  placeholder?: string
}>(), {
  placeholder: 'Buscar avaliações, categorias ou conteúdo',
})

const query = ref('')

const emit = defineEmits(['search'])

const onInput = (e: Event) => {
  query.value = (e.target as HTMLInputElement).value
}

const onSearch = () => {
  emit('search', query.value)
}
</script>
