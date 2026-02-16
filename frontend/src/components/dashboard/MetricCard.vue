<template>
  <div class="rounded-2xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 shadow-sm hover:shadow-md transition-shadow p-6">
    <div class="flex items-start justify-between mb-4">
      <div class="flex-1">
        <p class="text-sm text-slate-600 dark:text-slate-400 font-medium mb-1">
          {{ label }}
        </p>
        <p class="text-3xl font-bold text-slate-900 dark:text-white">
          {{ value }}
        </p>
      </div>
      <span class="text-3xl">{{ icon }}</span>
    </div>

    <!-- Trend indicator -->
    <div v-if="trend !== undefined" class="flex items-center gap-2 text-sm">
      <span :class="trend > 0 ? 'text-green-600 dark:text-green-400' : 'text-red-600 dark:text-red-400'">
        {{ trend > 0 ? '↑' : '↓' }} {{ Math.abs(trend) }}%
      </span>
      <span class="text-slate-500 dark:text-slate-400">{{ trendLabel }}</span>
    </div>

    <!-- Progress bar (optional) -->
    <div v-if="progress !== undefined" class="mt-4">
      <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2">
        <div
          class="bg-gradient-to-r from-primary to-accent h-2 rounded-full transition-all"
          :style="{ width: progress + '%' }"
        />
      </div>
      <p class="text-xs text-slate-500 dark:text-slate-400 mt-2">
        {{ progress }}% concluído
      </p>
    </div>

    <!-- Action button -->
    <slot name="action">
      <button
        v-if="actionLabel"
        @click="$emit('action')"
        class="mt-4 w-full px-3 py-2 text-sm font-medium text-primary hover:bg-primary/5 rounded-lg transition"
      >
        {{ actionLabel }}
      </button>
    </slot>
  </div>
</template>

<script setup lang="ts">
interface Props {
  label?: string
  value?: string | number
  icon?: string
  trend?: number
  trendLabel?: string
  progress?: number
  actionLabel?: string
}

withDefaults(defineProps<Props>(), {
  label: 'Métrica',
  value: '0',
  icon: '📊',
  trendLabel: 'desde ontem'
})

defineEmits<{
  action: []
}>()
</script>
