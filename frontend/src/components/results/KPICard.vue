<template>
  <div
    class="rounded-xl p-6 border"
    :class="highlight ? 'bg-purple-700/20 border-purple-500/50' : 'bg-slate-700/30 border-slate-600'"
  >
    <p class="text-sm text-slate-400 mb-2">{{ title }}</p>
    <div class="flex items-end gap-3">
      <span class="text-4xl font-bold text-white">{{ value }}</span>
      <span class="text-2xl mb-1">{{ icon }}</span>
    </div>
    <p v-if="change" class="text-xs text-green-400 mt-3">
      ↑ {{ change }} vs período anterior
    </p>
  </div>
</template>

<script setup lang="ts">
interface Props {
  title: string
  value: string | number
  icon: string
  change?: string
  highlight?: boolean
}

defineProps<Props>()
</script>
