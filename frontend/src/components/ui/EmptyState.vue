<template>
  <div class="flex flex-col items-center justify-center py-12 px-4 text-center">
    <div class="text-6xl mb-4">{{ icon }}</div>
    
    <h3 class="text-lg font-bold text-slate-900 dark:text-white mb-2">
      {{ title }}
    </h3>
    
    <p class="text-sm text-slate-600 dark:text-slate-400 max-w-sm mb-6">
      {{ description }}
    </p>
    
    <slot name="actions">
      <button
        v-if="actionLabel"
        @click="$emit('action')"
        class="px-4 py-2 bg-primary text-white rounded-lg font-medium hover:bg-primary/90 transition"
      >
        {{ actionLabel }}
      </button>
    </slot>
  </div>
</template>

<script setup lang="ts">
interface Props {
  icon?: string
  title?: string
  description?: string
  actionLabel?: string
}

withDefaults(defineProps<Props>(), {
  icon: '🎯',
  title: 'Nenhum item',
  description: 'Comece criando algo novo!'
})

defineEmits<{
  action: []
}>()
</script>
