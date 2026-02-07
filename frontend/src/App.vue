<template>
  <div id="app" class="min-h-screen bg-slate-50">
    <template v-if="!isAuthLayout">
      <header class="bg-white border-b border-slate-200 sticky top-0 z-10">
      <div class="max-w-7xl mx-auto px-4 py-3 flex items-center gap-6">
        <div class="flex items-center gap-2">
          <div class="h-9 w-9 rounded-xl bg-gradient-to-br from-primary to-accent text-white font-bold flex items-center justify-center">
            E
          </div>
          <div class="leading-tight">
            <p class="text-xs uppercase tracking-widest text-slate-400">Aevale</p>
            <p class="text-sm font-semibold text-slate-900">Dashboard</p>
          </div>
        </div>

        <div class="flex-1">
          <SearchBar placeholder="Buscar avaliações, categorias ou conteúdo" />
        </div>

        <div class="flex items-center gap-3">
          <span class="text-xs px-2.5 py-1 rounded-full bg-emerald-50 text-emerald-600 border border-emerald-100">
            Sincronizado
          </span>
          <button class="h-9 w-9 rounded-full border border-slate-200 text-slate-500 hover:text-primary hover:border-primary transition">
            <svg class="h-4 w-4 mx-auto" viewBox="0 0 20 20" fill="currentColor">
              <path d="M10 2a6 6 0 00-6 6v1.586l-.707.707A1 1 0 004 12h12a1 1 0 00.707-1.707L16 9.586V8a6 6 0 00-6-6z" />
              <path d="M10 18a2 2 0 001.995-1.85L12 16H8a2 2 0 001.85 1.995L10 18z" />
            </svg>
          </button>
          <button class="h-9 w-9 rounded-full border border-slate-200 text-slate-500 hover:text-primary hover:border-primary transition">
            <svg class="h-4 w-4 mx-auto" viewBox="0 0 20 20" fill="currentColor">
              <path d="M11.983 1.5a1.5 1.5 0 00-2.966 0l-.168 1.008a5.507 5.507 0 00-1.392.803l-1.02-.417a1.5 1.5 0 00-1.97.868l-.5 1.207a1.5 1.5 0 00.616 1.882l.88.52a5.61 5.61 0 000 1.606l-.88.52a1.5 1.5 0 00-.616 1.882l.5 1.207a1.5 1.5 0 001.97.868l1.02-.417c.424.34.896.62 1.392.803l.168 1.008a1.5 1.5 0 002.966 0l.168-1.008a5.507 5.507 0 001.392-.803l1.02.417a1.5 1.5 0 001.97-.868l.5-1.207a1.5 1.5 0 00-.616-1.882l-.88-.52c.067-.53.067-1.076 0-1.606l.88-.52a1.5 1.5 0 00.616-1.882l-.5-1.207a1.5 1.5 0 00-1.97-.868l-1.02.417a5.507 5.507 0 00-1.392-.803l-.168-1.008zM10 13a3 3 0 110-6 3 3 0 010 6z" />
            </svg>
          </button>
          <UserDropdown />
        </div>
      </div>
      </header>

      <main class="max-w-7xl mx-auto px-4 py-6">
        <RouterView />
      </main>

      <button class="fixed bottom-6 right-6 px-3 py-2 rounded-full bg-primary text-white text-xs font-semibold shadow-lg hover:bg-accent transition">
        Ajuda
      </button>
    </template>

    <template v-else>
      <RouterView />
    </template>
  </div>
</template>

<script setup lang="ts">
import { RouterView, useRoute } from 'vue-router'
import { computed } from 'vue'
import SearchBar from './components/SearchBar.vue'
import UserDropdown from './components/UserDropdown.vue'

const route = useRoute()
const isAuthLayout = computed(() => route.meta?.layout === 'auth')

// Debug: log when component mounts
console.log('App.vue mounted', { route: route.path })
</script>

<style scoped>
/* App-level styles */
</style>
