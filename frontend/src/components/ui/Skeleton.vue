<template>
  <div class="space-y-2">
    <div
      v-for="n in lines"
      :key="n"
      :class="[
        'h-4 bg-gradient-to-r from-slate-200 to-slate-100 rounded-lg animate-pulse',
        'dark:from-slate-700 dark:to-slate-600',
        {
          'w-3/4': n === lines,
          'w-full': n !== lines
        }
      ]"
    />
  </div>
</template>

<script setup lang="ts">
interface Props {
  lines?: number
}

withDefaults(defineProps<Props>(), {
  lines: 3
})
</script>
