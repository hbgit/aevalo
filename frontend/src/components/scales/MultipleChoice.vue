<template>
  <div class="space-y-3">
    <div
      v-for="(option, index) in localOptions"
      :key="index"
      class="flex items-center gap-3 p-3 rounded-lg border-2 cursor-pointer transition-all duration-200"
      :class="[
        isSelected(option)
          ? 'border-purple-500 bg-slate-700/50'
          : 'border-slate-600 bg-slate-800/30 hover:border-slate-500'
      ]"
      @click="toggleOption(option)"
    >
      <!-- Custom Checkbox -->
      <div
        class="w-5 h-5 rounded border-2 transition-all duration-200 flex items-center justify-center flex-shrink-0"
        :class="[
          isSelected(option)
            ? 'border-purple-500 bg-gradient-to-r from-purple-600 to-purple-500'
            : 'border-slate-500 bg-transparent'
        ]"
      >
        <span v-if="isSelected(option)" class="text-white text-sm">✓</span>
      </div>

      <!-- Label -->
      <span class="flex-1 text-sm font-medium text-slate-200">{{ option }}</span>

      <!-- Optional: Show count if limited -->
      <span
        v-if="minSelections || maxSelections"
        class="text-xs text-slate-400"
        :class="isSelected(option) ? 'text-purple-400' : ''"
      >
        {{ isSelected(option) ? '✓' : '' }}
      </span>
    </div>

    <!-- Selection counter if limits defined -->
    <div
      v-if="(minSelections || maxSelections) && modelValue?.length"
      class="text-xs p-2 rounded-lg"
      :class="[
        isSelectorValid
          ? 'bg-green-900/20 text-green-400'
          : 'bg-orange-900/20 text-orange-400'
      ]"
    >
      {{ modelValue?.length || 0 }}
      <span v-if="maxSelections">de {{ maxSelections }}</span> selecionadas
      <span v-if="minSelections && !isSelectorValid">(mínimo {{ minSelections }})</span>
    </div>

    <!-- Validation message -->
    <p v-if="validationMessage" class="text-xs text-red-400 mt-2">
      {{ validationMessage }}
    </p>
  </div>
</template>

<script setup lang="ts">
import { computed, ref } from 'vue'

interface Props {
  modelValue?: string[]
  options?: string[]
  minSelections?: number
  maxSelections?: number
}

interface Emits {
  (e: 'update:modelValue', value: string[]): void
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: () => [],
  options: () => ['Opção 1', 'Opção 2', 'Opção 3'],
  minSelections: 0,
  maxSelections: 0
})

const emit = defineEmits<Emits>()

const localOptions = ref(props.options)

const isSelected = (option: string) => {
  return props.modelValue?.includes(option) || false
}

const isSelectorValid = computed(() => {
  const count = props.modelValue?.length || 0
  if (props.minSelections && count < props.minSelections) return false
  if (props.maxSelections && count > props.maxSelections) return false
  return true
})

const validationMessage = computed(() => {
  const count = props.modelValue?.length || 0
  if (props.minSelections && count < props.minSelections) {
    return `Selecione pelo menos ${props.minSelections} opção(ões)`
  }
  if (props.maxSelections && count > props.maxSelections) {
    return `Máximo de ${props.maxSelections} seleções atingido`
  }
  return ''
})

const toggleOption = (option: string) => {
  let newSelection = [...(props.modelValue || [])]

  if (isSelected(option)) {
    // Deselect
    newSelection = newSelection.filter(item => item !== option)
  } else {
    // Select (if not at max limit)
    if (!props.maxSelections || newSelection.length < props.maxSelections) {
      newSelection.push(option)
    }
  }

  emit('update:modelValue', newSelection)
}
</script>

<style scoped>
div {
  transition: all 200ms ease;
}
</style>
