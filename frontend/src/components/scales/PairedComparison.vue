<template>
  <div class="space-y-4">
    <p class="text-sm text-slate-300 text-center font-medium">Qual opção você prefere?</p>
    
    <div class="grid grid-cols-2 gap-4 md:gap-6">
      <!-- Option A -->
      <button
        @click="selectValue('a')"
        class="group relative rounded-lg border-2 transition-all duration-200 p-4 flex flex-col items-center justify-center gap-3 hover:shadow-lg"
        :class="[
          modelValue === 'a'
            ? 'border-purple-500 bg-slate-700/50'
            : 'border-slate-600 bg-slate-800/50 hover:border-slate-500'
        ]"
      >
        <div class="text-center space-y-2 w-full">
          <p class="font-semibold text-slate-100 text-sm md:text-base">{{ optionALabel }}</p>
          <p v-if="optionADesc" class="text-xs text-slate-400">{{ optionADesc }}</p>
        </div>
        <div
          v-if="modelValue === 'a'"
          class="absolute inset-0 flex items-center justify-center rounded-lg pointer-events-none"
        >
          <div class="text-2xl">✓</div>
        </div>
      </button>

      <!-- Option B -->
      <button
        @click="selectValue('b')"
        class="group relative rounded-lg border-2 transition-all duration-200 p-4 flex flex-col items-center justify-center gap-3 hover:shadow-lg"
        :class="[
          modelValue === 'b'
            ? 'border-purple-500 bg-slate-700/50'
            : 'border-slate-600 bg-slate-800/50 hover:border-slate-500'
        ]"
      >
        <div class="text-center space-y-2 w-full">
          <p class="font-semibold text-slate-100 text-sm md:text-base">{{ optionBLabel }}</p>
          <p v-if="optionBDesc" class="text-xs text-slate-400">{{ optionBDesc }}</p>
        </div>
        <div
          v-if="modelValue === 'b'"
          class="absolute inset-0 flex items-center justify-center rounded-lg pointer-events-none"
        >
          <div class="text-2xl">✓</div>
        </div>
      </button>
    </div>

    <!-- No preference button -->
    <button
      @click="selectValue(null)"
      class="w-full px-4 py-2 rounded-lg text-sm text-slate-400 hover:text-slate-300 border border-slate-600 hover:border-slate-500 transition-colors"
    >
      Sem preferência
    </button>
  </div>
</template>

<script setup lang="ts">
interface Props {
  modelValue?: 'a' | 'b' | null
  optionALabel?: string
  optionBLabel?: string
  optionADesc?: string
  optionBDesc?: string
}

interface Emits {
  (e: 'update:modelValue', value: 'a' | 'b' | null): void
}

withDefaults(defineProps<Props>(), {
  modelValue: null,
  optionALabel: 'Opção A',
  optionBLabel: 'Opção B',
  optionADesc: '',
  optionBDesc: ''
})

const emit = defineEmits<Emits>()

const selectValue = (value: 'a' | 'b' | null) => {
  emit('update:modelValue', value)
}
</script>

<style scoped>
button {
  transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
}

button:active {
  transform: scale(0.98);
}
</style>
