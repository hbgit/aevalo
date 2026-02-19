<template>
  <div class="space-y-2">
    <button
      v-for="(option, index) in localOptions"
      :key="index"
      @click="selectOption(option)"
      class="w-full p-3 rounded-lg border-2 text-left transition-all duration-200"
      :class="[
        modelValue === option
          ? 'border-purple-500 bg-slate-700/50 text-white'
          : 'border-slate-600 bg-slate-800/30 text-slate-200 hover:border-slate-500 hover:bg-slate-800/50'
      ]"
    >
      <div class="flex items-center gap-3">
        <!-- Custom Radio Button -->
        <div
          class="w-5 h-5 rounded-full border-2 flex items-center justify-center flex-shrink-0 transition-all"
          :class="[
            modelValue === option
              ? 'border-purple-500 bg-gradient-to-r from-purple-600 to-purple-500'
              : 'border-slate-400'
          ]"
        >
          <span v-if="modelValue === option" class="text-white text-xs">●</span>
        </div>

        <!-- Label -->
        <span class="flex-1 font-medium text-sm">{{ option }}</span>

        <!-- Checkmark if selected -->
        <span v-if="modelValue === option" class="text-purple-400 text-lg">✓</span>
      </div>
    </button>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

interface Props {
  modelValue?: string | null
  options?: string[]
}

interface Emits {
  (e: 'update:modelValue', value: string): void
}

withDefaults(defineProps<Props>(), {
  modelValue: null,
  options: () => ['Opção 1', 'Opção 2', 'Opção 3']
})

const emit = defineEmits<Emits>()

const props = defineProps<Props>()

const localOptions = ref(props.options)

const selectOption = (option: string) => {
  emit('update:modelValue', option)
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
