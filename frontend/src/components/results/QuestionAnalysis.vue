<template>
  <div>
    <!-- Likert/Frequency Analysis -->
    <div v-if="['likert', 'frequency'].includes(question.type)" class="space-y-4">
      <!-- Bar Chart -->
      <div class="space-y-3">
        <div
          v-for="(item, idx) in getBarChartData()"
          :key="idx"
          class="space-y-1"
        >
          <div class="flex items-center justify-between text-sm">
            <span class="text-slate-300">{{ item.label }}</span>
            <span class="text-slate-400">{{ item.percentage }}% ({{ item.count }})</span>
          </div>
          <div class="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-purple-600 to-purple-500 transition-all duration-300"
              :style="{ width: item.percentage + '%' }"
            ></div>
          </div>
        </div>
      </div>

      <!-- Statistics -->
      <div class="grid grid-cols-3 gap-3 pt-4 border-t border-slate-600">
        <div>
          <p class="text-xs text-slate-400">Média</p>
          <p class="text-lg font-bold text-white">{{ getStatistics().mean.toFixed(2) }}</p>
        </div>
        <div>
          <p class="text-xs text-slate-400">Desvio Padrão</p>
          <p class="text-lg font-bold text-white">{{ getStatistics().stdDev.toFixed(2) }}</p>
        </div>
        <div>
          <p class="text-xs text-slate-400">Moda</p>
          <p class="text-lg font-bold text-white">{{ getStatistics().mode }}</p>
        </div>
      </div>
    </div>

    <!-- Fixed Sum Analysis -->
    <div v-else-if="question.type === 'fixed_sum'" class="space-y-4">
      <!-- Distribution -->
      <div class="space-y-3">
        <div
          v-for="(item, idx) in getFixedSumData()"
          :key="idx"
          class="space-y-1"
        >
          <div class="flex items-center justify-between text-sm">
            <span class="text-slate-300">{{ item.option }}</span>
            <span class="text-slate-400">{{ item.average.toFixed(1) }} pts</span>
          </div>
          <div class="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-orange-600 to-orange-500"
              :style="{ width: (item.average / 100) * 100 + '%' }"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Multiple/Single Choice Analysis -->
    <div v-else-if="['multiple_choice', 'single_choice'].includes(question.type)" class="space-y-4">
      <div class="space-y-3">
        <div
          v-for="(item, idx) in getChoiceData()"
          :key="idx"
          class="space-y-1"
        >
          <div class="flex items-center justify-between text-sm">
            <span class="text-slate-300">{{ item.option }}</span>
            <span class="text-slate-400">{{ item.percentage }}% ({{ item.count }})</span>
          </div>
          <div class="w-full bg-slate-700 rounded-full h-2 overflow-hidden">
            <div
              class="h-full bg-gradient-to-r from-blue-600 to-blue-500"
              :style="{ width: item.percentage + '%' }"
            ></div>
          </div>
        </div>
      </div>
    </div>

    <!-- Open Text Analysis -->
    <div v-else-if="question.type === 'text'" class="space-y-4">
      <!-- Word Cloud -->
      <div class="mb-4">
        <p class="text-sm font-semibold text-slate-200 mb-3">Palavras mais frequentes</p>
        <div class="flex flex-wrap gap-2">
          <span
            v-for="(word, idx) in getWordCloud()"
            :key="idx"
            class="px-3 py-1 rounded-full bg-slate-700 text-slate-200 text-xs hover:bg-slate-600 cursor-pointer transition"
            :style="{ fontSize: Math.min(14 + word.count, 20) + 'px' }"
          >
            {{ word.text }}
          </span>
        </div>
      </div>

      <!-- Responses List -->
      <div class="space-y-2">
        <p class="text-sm font-semibold text-slate-200">Respostas ({{ textResponses.length }})</p>
        <div
          v-for="(response, idx) in textResponses.slice(0, 5)"
          :key="idx"
          class="p-3 bg-slate-700/30 rounded-lg border border-slate-600/30"
        >
          <p class="text-sm text-slate-300">{{ response }}</p>
        </div>
        <button
          v-if="textResponses.length > 5"
          class="text-xs text-purple-400 hover:text-purple-300 transition"
        >
          Ver todas as {{ textResponses.length }} respostas →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'

interface Question {
  id: string
  text: string
  type: string
  options?: string[]
  config?: Record<string, any>
}

interface Response {
  id: string
  questionIndex: number
  value: any
  completedAt: string
  timeSpent: number
}

interface Props {
  question: Question
  responses: Response[]
}

const props = defineProps<Props>()

const textResponses = computed(() => {
  return props.responses
    .filter(r => typeof r.value === 'string' && r.value.length > 0)
    .map(r => r.value)
})

const getBarChartData = () => {
  const counts: Record<number, number> = {}
  const total = props.responses.length

  props.responses.forEach(r => {
    const value = parseInt(r.value)
    counts[value] = (counts[value] || 0) + 1
  })

  return Object.entries(counts)
    .sort(([a], [b]) => parseInt(a) - parseInt(b))
    .map(([value, count]) => ({
      label: `${value}`,
      count,
      percentage: Math.round((count / total) * 100)
    }))
}

const getFixedSumData = () => {
  const options = props.question.config?.options || []
  const totals: Record<string, number[]> = {}

  options.forEach(opt => {
    totals[opt] = []
  })

  props.responses.forEach(r => {
    if (r.value && typeof r.value === 'object') {
      Object.entries(r.value).forEach(([opt, val]) => {
        if (totals[opt]) {
          totals[opt].push(Number(val))
        }
      })
    }
  })

  return options.map(opt => ({
    option: opt,
    average: totals[opt].length > 0
      ? totals[opt].reduce((a, b) => a + b) / totals[opt].length
      : 0
  }))
}

const getChoiceData = () => {
  const counts: Record<string, number> = {}
  const total = props.responses.length

  props.responses.forEach(r => {
    if (Array.isArray(r.value)) {
      r.value.forEach((choice: string) => {
        counts[choice] = (counts[choice] || 0) + 1
      })
    } else if (typeof r.value === 'string') {
      counts[r.value] = (counts[r.value] || 0) + 1
    }
  })

  return (props.question.options || []).map(opt => ({
    option: opt,
    count: counts[opt] || 0,
    percentage: Math.round(((counts[opt] || 0) / total) * 100)
  }))
}

const getStatistics = () => {
  const values = props.responses
    .map(r => parseFloat(r.value))
    .filter(v => !isNaN(v))

  if (values.length === 0) {
    return { mean: 0, stdDev: 0, mode: 0 }
  }

  const mean = values.reduce((a, b) => a + b) / values.length
  const variance = values.reduce((a, b) => a + Math.pow(b - mean, 2)) / values.length
  const stdDev = Math.sqrt(variance)

  const counts: Record<number, number> = {}
  values.forEach(v => {
    counts[v] = (counts[v] || 0) + 1
  })
  const mode = parseInt(
    Object.entries(counts).sort(([, a], [, b]) => b - a)[0]?.[0] || '0'
  )

  return { mean, stdDev, mode }
}

const getWordCloud = () => {
  const words: Record<string, number> = {}

  textResponses.value.forEach(text => {
    text.split(/\s+/).forEach(word => {
      const cleaned = word.toLowerCase().replace(/[^\w\u00C0-\u00FF]/g, '')
      if (cleaned.length > 3) {
        words[cleaned] = (words[cleaned] || 0) + 1
      }
    })
  })

  return Object.entries(words)
    .sort(([, a], [, b]) => b - a)
    .slice(0, 15)
    .map(([text, count]) => ({ text, count }))
}
</script>
