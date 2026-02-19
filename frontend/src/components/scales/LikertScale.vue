<template>
  <div class="space-y-3">
    <!-- Desktop View -->
    <div class="hidden md:flex items-center justify-center gap-3">
      <span class="text-xs text-slate-400 w-32 text-right">{{ minLabel }}</span>
      <div class="flex gap-2">
        <button
          v-for="value in scaleValues"
          :key="value"
          @click="selectValue(value)"
          class="w-10 h-10 rounded-full transition-all duration-200 flex items-center justify-center text-sm font-semibold"
          :class="[
            value === modelValue
              ? 'bg-gradient-to-r from-purple-600 to-purple-500 text-white scale-110 shadow-lg'
              : 'bg-slate-700 text-slate-300 hover:bg-slate-600 hover:scale-105'
          ]"
        >
          {{ value }}
        </button>
      </div>
      <span class="text-xs text-slate-400 w-32 text-left">{{ maxLabel }}</span>
    </div>

    <!-- Mobile View -->
    <div class="md:hidden space-y-2">
      <div class="flex flex-col gap-2">
        <button
          v-for="value in scaleValues"
          :key="value"
          @click="selectValue(value)"
          class="px-4 py-2 rounded-lg transition-all duration-200 text-sm font-medium"
          :class="[
            value === modelValue
              ? 'bg-gradient-to-r from-purple-600 to-purple-500 text-white'
              : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
          ]"
        >
          {{ value }}
        </button>
      </div>
    </div>

    <!-- Labels for mobile -->
    <div class="md:hidden flex justify-between text-xs text-slate-400">
      <span>{{ minLabel }}</span>
      <span>{{ maxLabel }}</span>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  modelValue?: number | null
  scale?: 5 | 7 | 10
  minLabel?: string
  maxLabel?: string
}

interface Emits {
  (e: 'update:modelValue', value: number): void
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: null,
  scale: 5,
  minLabel: 'Discordo Totalmente',
  maxLabel: 'Concordo Totalmente'
})

const emit = defineEmits<Emits>()

const scaleValues = computed(() => {
  const start = 1
  const end = props.scale
  return Array.from({ length: end - start + 1 }, (_, i) => start + i)
})

const selectValue = (value: number) => {
  emit('update:modelValue', value)
}
</script>

<style scoped>
button {
  transition: all 200ms cubic-bezier(0.4, 0, 0.2, 1);
}

button:active {
  transform: scale(0.95);
}
</style>
