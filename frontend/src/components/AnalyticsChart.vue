<!-- Analytics Chart Component -->
<template>
  <div class="space-y-4">
    <div class="rounded-2xl border border-slate-200 bg-white p-6">
      <div class="flex items-center justify-between mb-4">
        <h2 class="text-sm font-bold text-slate-900">Avaliações por Categoria</h2>
        <span class="text-xs text-slate-500">Últimos 30 dias</span>
      </div>
      <div class="mt-6 h-48 flex items-end gap-3">
        <div v-for="item in data.categories" :key="item.label" class="flex-1 flex flex-col items-center gap-2">
          <div class="w-full h-full flex items-end">
            <div
              class="w-full rounded-lg transition-transform hover:scale-105"
              :class="item.color"
              :style="{ height: `${(item.value / maxCategory) * 100}%` }"
            ></div>
          </div>
          <span class="text-xs font-medium text-slate-700">{{ item.label }}</span>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border border-slate-200 bg-white p-6">
      <h2 class="text-sm font-bold text-slate-900 mb-4">Distribuição por Status</h2>
      <div class="space-y-4">
        <div v-for="item in data.statuses" :key="item.label" class="space-y-2">
          <div class="flex items-center justify-between text-xs">
            <span class="font-medium text-slate-700">{{ item.label }}</span>
            <span class="text-slate-500">{{ item.value }} ({{ getPercent(item.value, item.total) }}%)</span>
          </div>
          <div class="h-2.5 rounded-full bg-slate-100">
            <div
              class="h-2.5 rounded-full transition-all"
              :class="item.color"
              :style="{ width: `${getPercent(item.value, item.total)}%` }"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <div class="rounded-2xl border border-orange-200 bg-orange-50 p-5">
      <div class="flex items-start gap-3">
        <div class="h-9 w-9 rounded-lg bg-secondary text-white flex items-center justify-center flex-shrink-0">
          <svg class="h-5 w-5" viewBox="0 0 20 20" fill="currentColor">
            <path d="M9 2a1 1 0 00-.894.553L3.382 12a1 1 0 00.894 1.447H7v4a1 1 0 001.447.894l9-4.5A1 1 0 0018 12h-2.764L10.894 2.553A1 1 0 0010 2H9z" />
          </svg>
        </div>
        <div>
          <p class="text-sm font-bold text-orange-700">Dica Inteligente</p>
          <p class="text-xs text-orange-700/90 mt-1">{{ data.insight }}</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps<{
  data: {
    categories: { label: string; value: number; color: string }[]
    statuses: { label: string; value: number; total: number; color: string }[]
    insight: string
  }
}>()

const maxCategory = computed(() => {
  const values = props.data.categories.map((item) => item.value)
  return Math.max(...values, 1)
})

const getPercent = (value: number, total: number) => {
  if (!total) return 0
  return Math.round((value / total) * 100)
}
</script>
