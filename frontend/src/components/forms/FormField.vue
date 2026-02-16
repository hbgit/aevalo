<template>
  <div class="space-y-2">
    <label v-if="label" class="block text-sm font-medium text-slate-700 dark:text-slate-300">
      {{ label }}
      <span v-if="required" class="text-red-600">*</span>
    </label>

    <div class="relative">
      <component
        :is="fieldComponent"
        :model-value="modelValue"
        :error="error"
        v-bind="fieldProps"
        @update:model-value="$emit('update:modelValue', $event)"
        @blur="$emit('blur')"
        @focus="$emit('focus')"
      />
    </div>

    <p v-if="error" class="text-xs text-red-600 dark:text-red-400">
      {{ error }}
    </p>
    <p v-else-if="hint" class="text-xs text-slate-500 dark:text-slate-400">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import Input from '../ui/Input.vue'
import LikertScale from './LikertScale.vue'
import FixedSumInput from './FixedSumInput.vue'
import PairedComparison from './PairedComparison.vue'

type FieldType = 'text' | 'email' | 'number' | 'likert' | 'fixed_sum' | 'paired_comparison'

interface Props {
  modelValue?: any
  label?: string
  hint?: string
  error?: string
  required?: boolean
  type?: FieldType
  fieldProps?: Record<string, any>
}

const props = withDefaults(defineProps<Props>(), {
  type: 'text',
  fieldProps: () => ({})
})

defineEmits<{
  'update:modelValue': [value: any]
  blur: []
  focus: []
}>()

const fieldComponent = computed(() => {
  const components = {
    text: Input,
    email: Input,
    number: Input,
    likert: LikertScale,
    fixed_sum: FixedSumInput,
    paired_comparison: PairedComparison
  }
  return components[props.type]
})
</script>
