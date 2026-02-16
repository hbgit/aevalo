<template>
  <div class="space-y-6">
    <div>
      <h4 class="text-sm font-semibold text-slate-900 dark:text-white mb-2">
        {{ question }}
      </h4>
      <p class="text-xs text-slate-600 dark:text-slate-400 mb-4">
        Distribua {{ total }} pontos entre as opções (soma deve ser exatamente {{ total }})
      </p>

      <div class="space-y-3">
        <div v-for="(option, index) in options" :key="index" class="flex items-center gap-3">
          <label class="flex-1 text-sm text-slate-700 dark:text-slate-300">
            {{ option }}
          </label>
          
          <input
            type="number"
            :value="modelValue[index] || 0"
            @input="updateValue(index, $event)"
            min="0"
            :max="total"
            class="w-20 px-3 py-2 border border-slate-200 dark:border-slate-600 rounded-lg text-center bg-slate-50 dark:bg-slate-700 text-slate-900 dark:text-white"
          />
        </div>
      </div>

      <!-- Sum indicator -->
      <div class="mt-4 p-3 rounded-lg" :class="sumValid ? 'bg-green-100 dark:bg-green-900/30 text-green-700 dark:text-green-300' : 'bg-red-100 dark:bg-red-900/30 text-red-700 dark:text-red-300'">
        <p class="text-xs font-medium">
          Soma total: {{ currentSum }}/{{ total }}
          <span v-if="sumValid">✅ Correto!</span>
          <span v-else>❌ Ajuste os valores</span>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  modelValue?: number[]
  question?: string
  options?: string[]
  total?: number
}

const props = withDefaults(defineProps<Props>(), {
  question: 'Distribua os pontos',
  options: () => ['Opção 1', 'Opção 2', 'Opção 3'],
  total: 100,
  modelValue: () => [0, 0, 0]
})

defineEmits<{
  'update:modelValue': [value: number[]]
}>()

const currentSum = computed(() => {
  return (props.modelValue || []).reduce((sum, val) => sum + (val || 0), 0)
})

const sumValid = computed(() => {
  return currentSum.value === props.total
})

const updateValue = (index: number, event: Event) => {
  const input = event.target as HTMLInputElement
  const newValue = parseInt(input.value) || 0
  const updated = [...(props.modelValue || [])]
  updated[index] = Math.min(newValue, props.total)
  
  // Prevent sum from exceeding total
  const total = updated.reduce((sum, val) => sum + val, 0)
  if (total > props.total) {
    updated[index] = Math.max(0, props.total - (total - updated[index]))
  }
  
  $emit('update:modelValue', updated)
}

const $emit = defineEmits<{
  'update:modelValue': [value: number[]]
}>()
</script>
