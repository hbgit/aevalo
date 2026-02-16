<template>
  <div class="w-full">
    <label v-if="label" :for="id" class="block text-sm font-medium text-slate-700 dark:text-slate-300 mb-2">
      {{ label }}
      <span v-if="required" class="text-red-600">*</span>
    </label>
    
    <div class="relative">
      <span v-if="prefixIcon" class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-500">
        {{ prefixIcon }}
      </span>
      
      <input
        :id="id"
        :type="type"
        :placeholder="placeholder"
        :disabled="disabled"
        :required="required"
        :value="modelValue"
        :class="[
          'w-full px-4 py-2.5 border border-slate-200 rounded-lg bg-slate-50 text-sm text-slate-700',
          'focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition-all',
          'dark:bg-slate-700 dark:border-slate-600 dark:text-white dark:placeholder-slate-400',
          {
            'pl-10': prefixIcon,
            'pr-10': suffixIcon,
            'border-red-500 focus:ring-red-500/40 focus:border-red-500': error,
            'opacity-50 cursor-not-allowed': disabled
          }
        ]"
        @input="$emit('update:modelValue', ($event.target as HTMLInputElement).value)"
        @blur="$emit('blur')"
        @focus="$emit('focus')"
      />
      
      <span v-if="suffixIcon" class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-500">
        {{ suffixIcon }}
      </span>
    </div>
    
    <p v-if="error" class="mt-1 text-sm text-red-600 dark:text-red-400">
      {{ error }}
    </p>
    <p v-else-if="hint" class="mt-1 text-sm text-slate-500 dark:text-slate-400">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  modelValue?: string | number
  type?: string
  label?: string
  placeholder?: string
  hint?: string
  error?: string
  disabled?: boolean
  required?: boolean
  prefixIcon?: string
  suffixIcon?: string
}

withDefaults(defineProps<Props>(), {
  type: 'text',
  disabled: false,
  required: false,
  modelValue: ''
})

defineEmits<{
  'update:modelValue': [value: string | number]
  blur: []
  focus: []
}>()

const id = ref(`input-${Math.random().toString(36).substr(2, 9)}`)
</script>
