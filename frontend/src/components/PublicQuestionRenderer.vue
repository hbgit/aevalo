<template>
  <div>
    <component
      :is="questionComponent"
      :model-value="value"
      v-bind="questionProps"
      @update:model-value="$emit('update', $event)"
    />
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import LikertScale from './scales/LikertScale.vue'
import FrequencyScale from './scales/FrequencyScale.vue'
import PairedComparison from './scales/PairedComparison.vue'
import FixedSum from './scales/FixedSum.vue'
import OpenText from './scales/OpenText.vue'
import MultipleChoice from './scales/MultipleChoice.vue'
import SingleChoice from './scales/SingleChoice.vue'

interface Question {
  id: string
  text: string
  type: string
  required: boolean
  options?: string[]
  config?: Record<string, any>
}

interface Props {
  question: Question
  value?: any
}

const props = defineProps<Props>()
defineEmits<{
  update: [value: any]
}>()

const questionComponent = computed(() => {
  const map: Record<string, any> = {
    likert: LikertScale,
    frequency: FrequencyScale,
    paired: PairedComparison,
    fixed_sum: FixedSum,
    text: OpenText,
    multiple_choice: MultipleChoice,
    single_choice: SingleChoice
  }

  return map[props.question.type] || OpenText
})

const questionProps = computed(() => {
  const config = props.question.config || {}
  const baseProps = {
    options: props.question.options,
    ...config
  }

  // Type-specific props
  switch (props.question.type) {
    case 'likert':
      return {
        ...baseProps,
        scale: config.scale || 5,
        minLabel: config.minLabel || 'Discordo Totalmente',
        maxLabel: config.maxLabel || 'Concordo Totalmente'
      }
    case 'text':
      return {
        ...baseProps,
        maxLength: config.maxLength || 500,
        placeholder: 'Digite sua resposta aqui...'
      }
    case 'fixed_sum':
      return {
        ...baseProps,
        options: config.options || []
      }
    case 'multiple_choice':
      return {
        ...baseProps,
        options: props.question.options || [],
        minSelections: config.minSelections || 0,
        maxSelections: config.maxSelections || 0
      }
    case 'single_choice':
      return {
        ...baseProps,
        options: props.question.options || []
      }
    default:
      return baseProps
  }
})
</script>
