<template>
  <div class="space-y-6">
    <div>
      <h4 class="text-sm font-semibold text-slate-900 dark:text-white mb-4">
        {{ question }}
      </h4>
      
      <div class="flex gap-2 justify-between">
        <button
          v-for="option in options"
          :key="option.value"
          @click="$emit('update:modelValue', option.value)"
          :class="[
            'flex-1 px-3 py-3 rounded-lg border-2 transition-all text-center',
            'dark:text-slate-300',
            modelValue === option.value
              ? 'border-primary bg-primary/10 dark:bg-primary/20 font-semibold text-primary'
              : 'border-slate-200 dark:border-slate-600 hover:border-slate-300 dark:hover:border-slate-500'
          ]"
        >
          <div class="text-sm">{{ option.label }}</div>
          <div v-if="option.description" class="text-xs text-slate-500 dark:text-slate-400 mt-1">
            {{ option.description }}
          </div>
        </button>
      </div>
    </div>

    <p v-if="hint" class="text-xs text-slate-500 dark:text-slate-400">
      {{ hint }}
    </p>
  </div>
</template>

<script setup lang="ts">
interface LikertOption {
  value: number | string
  label: string
  description?: string
}

interface Props {
  modelValue?: number | string
  question?: string
  options?: LikertOption[]
  hint?: string
}

withDefaults(defineProps<Props>(), {
  question: 'Como você avalia?',
  options: () => [
    { value: 1, label: 'Discordo\nfortemente', description: 'Muito insatisfeito' },
    { value: 2, label: 'Discordo', description: 'Insatisfeito' },
    { value: 3, label: 'Neutro', description: 'Nem satisfeito nem insatisfeito' },
    { value: 4, label: 'Concordo', description: 'Satisfeito' },
    { value: 5, label: 'Concordo\nfortemente', description: 'Muito satisfeito' }
  ]
})

defineEmits<{
  'update:modelValue': [value: number | string]
}>()
</script>
