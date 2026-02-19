<template>
  <div class="flex gap-2 overflow-x-auto pb-2">
    <button
      v-for="option in options"
      :key="option.value"
      @click="selectValue(option.value)"
      class="px-4 py-2 rounded-full whitespace-nowrap transition-all duration-200 text-sm font-medium flex-shrink-0"
      :class="[
        option.value === modelValue
          ? 'bg-gradient-to-r from-orange-500 to-purple-600 text-white shadow-lg'
          : 'bg-slate-700 text-slate-300 hover:bg-slate-600'
      ]"
    >
      {{ option.label }}
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  modelValue?: string | null
}

interface Emits {
  (e: 'update:modelValue', value: string): void
}

withDefaults(defineProps<Props>(), {
  modelValue: null
})

const emit = defineEmits<Emits>()

const options = ref([
  { label: 'Nunca', value: 'never' },
  { label: 'Raramente', value: 'rarely' },
  { label: 'Às vezes', value: 'sometimes' },
  { label: 'Frequentemente', value: 'frequently' },
  { label: 'Sempre', value: 'always' }
])

const selectValue = (value: string) => {
  emit('update:modelValue', value)
}
</script>

<style scoped>
::-webkit-scrollbar {
  height: 4px;
}

::-webkit-scrollbar-track {
  background: transparent;
}

::-webkit-scrollbar-thumb {
  background: rgba(100, 116, 139, 0.5);
  border-radius: 2px;
}
</style>
