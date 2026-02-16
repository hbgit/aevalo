<template>
  <button
    :type="type"
    :disabled="disabled"
    :class="[
      'inline-flex items-center justify-center gap-2 font-semibold transition-all rounded-lg',
      sizeClasses,
      variantClasses,
      {
        'opacity-50 cursor-not-allowed': disabled,
        'opacity-75': loading
      }
    ]"
    @click="$emit('click')"
  >
    <span v-if="loading" class="inline-block animate-spin">⏳</span>
    <slot v-if="!loading" />
  </button>
</template>

<script setup lang="ts">
import { computed } from 'vue'

type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger'
type ButtonSize = 'sm' | 'md' | 'lg'

interface Props {
  variant?: ButtonVariant
  size?: ButtonSize
  type?: 'button' | 'submit' | 'reset'
  disabled?: boolean
  loading?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  variant: 'primary',
  size: 'md',
  type: 'button',
  disabled: false,
  loading: false
})

defineEmits<{
  click: []
}>()

const sizeClasses = computed(() => {
  const sizes = {
    sm: 'px-3 py-1.5 text-sm',
    md: 'px-4 py-2 text-base',
    lg: 'px-6 py-3 text-lg'
  }
  return sizes[props.size]
})

const variantClasses = computed(() => {
  const variants = {
    primary: 'bg-primary text-white hover:bg-primary/90 active:bg-primary/80 shadow-sm hover:shadow-md',
    secondary: 'bg-secondary text-white hover:bg-secondary/90 active:bg-secondary/80 shadow-sm hover:shadow-md',
    ghost: 'text-slate-700 hover:bg-slate-100 active:bg-slate-200 border border-slate-200',
    danger: 'bg-red-600 text-white hover:bg-red-700 active:bg-red-800 shadow-sm hover:shadow-md'
  }
  return variants[props.variant]
})
</script>
