<template>
  <div class="w-full h-64 rounded-2xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 shadow-sm p-6">
    <h3 class="text-sm font-semibold text-slate-900 dark:text-white mb-4">
      {{ title }}
    </h3>

    <!-- Simple bar chart representation -->
    <div class="flex items-end gap-2 h-40">
      <div
        v-for="(bar, index) in bars"
        :key="index"
        class="flex-1 flex flex-col items-center"
      >
        <!-- Bar -->
        <div
          class="w-full bg-gradient-to-t from-primary to-accent rounded-t transition-all hover:opacity-75"
          :style="{ height: (bar.value / maxValue) * 100 + '%', minHeight: '4px' }"
          :title="`${bar.label}: ${bar.value}`"
        />

        <!-- Label -->
        <p class="text-xs text-slate-600 dark:text-slate-400 mt-2 text-center truncate w-full">
          {{ bar.label }}
        </p>
      </div>
    </div>

    <!-- Legend -->
    <div class="mt-4 flex items-center justify-between text-xs text-slate-500 dark:text-slate-400">
      <span>{{ minValue }}</span>
      <span class="font-medium text-primary">{{ maxValue }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface BarData {
  label: string
  value: number
}

interface Props {
  title?: string
  bars?: BarData[]
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Gráfico de Barras',
  bars: () => [
    { label: 'Seg', value: 45 },
    { label: 'Ter', value: 52 },
    { label: 'Qua', value: 38 },
    { label: 'Qui', value: 60 },
    { label: 'Sex', value: 55 },
    { label: 'Sab', value: 30 },
    { label: 'Dom', value: 25 }
  ]
})

const maxValue = computed(() => {
  return Math.max(...props.bars.map(b => b.value), 100)
})

const minValue = computed(() => 0)
</script>
