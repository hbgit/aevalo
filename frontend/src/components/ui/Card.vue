<template>
  <div
    :class="[
      'rounded-2xl border border-slate-200 bg-white shadow-sm hover:shadow-md transition-shadow',
      'dark:bg-slate-800 dark:border-slate-700',
      {
        'p-6': !noPadding,
        [elevationClass]: true
      }
    ]"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type Elevation = 'sm' | 'md' | 'lg'

interface Props {
  elevation?: Elevation
  noPadding?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  elevation: 'sm',
  noPadding: false
})

const elevationClass = computed(() => {
  const elevations = {
    sm: 'shadow-sm hover:shadow-md',
    md: 'shadow-md hover:shadow-lg',
    lg: 'shadow-lg hover:shadow-xl'
  }
  return elevations[props.elevation]
})
</script>
