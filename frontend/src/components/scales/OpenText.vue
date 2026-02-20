<template>
  <div class="space-y-2">
    <textarea
      :value="modelValue || ''"
      @input="(e: any) => emit('update:modelValue', e.target.value)"
      :placeholder="placeholder"
      :maxlength="maxLength"
      rows="4"
      class="w-full px-4 py-3 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-slate-400 focus:border-purple-500 focus:outline-none resize-none transition-colors"
    />

    <!-- Character counter -->
    <div class="flex justify-between items-center">
      <p v-if="hint" class="text-xs text-slate-400">{{ hint }}</p>
      <div class="text-xs transition-colors" :class="charCounterColor">
        {{ currentLength }}/{{ maxLength }} caracteres
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Props {
  modelValue?: string | null
  maxLength?: number
  placeholder?: string
  hint?: string
}

interface Emits {
  (e: 'update:modelValue', value: string): void
}

const props = withDefaults(defineProps<Props>(), {
  modelValue: '',
  maxLength: 500,
  placeholder: 'Digite sua resposta aqui...',
  hint: ''
})

const emit = defineEmits<Emits>()

const currentLength = computed(() => (props.modelValue || '').length)

const charCounterColor = computed(() => {
  const ratio = currentLength.value / (props.maxLength || 500)
  if (ratio >= 0.9) return 'text-red-400'
  if (ratio >= 0.75) return 'text-orange-400'
  return 'text-slate-400'
})
</script>

<style scoped>
textarea {
  transition: border-color 200ms ease;
}

textarea:focus {
  box-shadow: 0 0 0 3px rgba(147, 51, 234, 0.1);
}
</style>
