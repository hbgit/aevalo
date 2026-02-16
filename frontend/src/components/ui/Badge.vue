<template>
  <span
    :class="[
      'inline-block px-2.5 py-1 rounded-full text-xs font-semibold transition',
      variantClasses,
      {
        'animate-pulse': animated
      }
    ]"
  >
    <slot />
  </span>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type BadgeVariant = 'success' | 'warning' | 'danger' | 'info' | 'secondary' | 'primary'

interface Props {
  variant?: BadgeVariant
  animated?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'secondary',
  animated: false
})

const variantClasses = computed(() => {
  const variants = {
    success: 'bg-emerald-100 dark:bg-emerald-900/30 text-emerald-700 dark:text-emerald-300',
    warning: 'bg-amber-100 dark:bg-amber-900/30 text-amber-700 dark:text-amber-300',
    danger: 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300',
    info: 'bg-blue-100 dark:bg-blue-900/30 text-blue-700 dark:text-blue-300',
    secondary: 'bg-slate-100 dark:bg-slate-700 text-slate-700 dark:text-slate-300',
    primary: 'bg-primary/20 dark:bg-primary/30 text-primary dark:text-primary'
  }
  return variants[props.variant]
})
</script>
