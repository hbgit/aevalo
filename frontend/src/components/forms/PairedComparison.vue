<template>
  <div class="space-y-6">
    <div>
      <h4 class="text-sm font-semibold text-slate-900 dark:text-white mb-4">
        {{ question }}
      </h4>
      <p class="text-xs text-slate-600 dark:text-slate-400 mb-4">
        Selecione a opção que melhor representa sua escolha
      </p>

      <div class="space-y-2">
        <div
          v-for="(pair, index) in pairs"
          :key="index"
          class="grid grid-cols-3 gap-2 items-center"
        >
          <!-- Left option -->
          <button
            @click="selectPair(index, 'left')"
            :class="[
              'px-4 py-3 rounded-lg border-2 transition-all text-left text-sm',
              'dark:text-slate-300',
              modelValue?.[index] === 'left'
                ? 'border-primary bg-primary/10 dark:bg-primary/20 font-semibold text-primary'
                : 'border-slate-200 dark:border-slate-600 hover:border-slate-300 dark:hover:border-slate-500'
            ]"
          >
            {{ pair.left }}
          </button>

          <!-- VS indicator -->
          <div class="text-center text-xs font-bold text-slate-500 dark:text-slate-400">
            VS
          </div>

          <!-- Right option -->
          <button
            @click="selectPair(index, 'right')"
            :class="[
              'px-4 py-3 rounded-lg border-2 transition-all text-right text-sm',
              'dark:text-slate-300',
              modelValue?.[index] === 'right'
                ? 'border-primary bg-primary/10 dark:bg-primary/20 font-semibold text-primary'
                : 'border-slate-200 dark:border-slate-600 hover:border-slate-300 dark:hover:border-slate-500'
            ]"
          >
            {{ pair.right }}
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface PairedComparison {
  left: string
  right: string
}

interface Props {
  modelValue?: ('left' | 'right')[]
  question?: string
  pairs?: PairedComparison[]
}

withDefaults(defineProps<Props>(), {
  question: 'Qual você prefere?',
  pairs: () => [
    { left: 'Opção A', right: 'Opção B' },
    { left: 'Opção A', right: 'Opção C' },
    { left: 'Opção B', right: 'Opção C' }
  ]
})

defineEmits<{
  'update:modelValue': [value: ('left' | 'right')[]]
}>()

const selectPair = (index: number, choice: 'left' | 'right') => {
  const updated = [...(defineProps().modelValue || [])]
  updated[index] = choice
}
</script>
