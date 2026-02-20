<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 py-8">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Header -->
      <div class="mb-8">
        <div v-if="loading" class="animate-pulse space-y-4">
          <div class="h-10 bg-slate-700 rounded-lg w-1/2"></div>
          <div class="h-6 bg-slate-700 rounded-lg w-1/3"></div>
        </div>

        <div v-else-if="evaluation">
          <h1 class="text-4xl font-bold text-white mb-2">{{ evaluation.title }}</h1>
          <p class="text-slate-400 mb-4">{{ evaluation.description }}</p>

          <!-- Timeline -->
          <div class="flex items-center gap-4 text-sm text-slate-400">
            <span>Aberta em {{ formatDate(evaluation.createdAt) }}</span>
            <span>•</span>
            <span>Encerrada em {{ formatDate(evaluation.closedAt) }}</span>
            <span>•</span>
            <span class="text-white font-semibold">{{ totalResponses }} respostas</span>
          </div>
        </div>
      </div>

      <!-- KPIs Grid -->
      <div v-if="!loading && evaluation" class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-8">
        <KPICard
          title="Total de Respostas"
          :value="totalResponses"
          icon="📊"
        />
        <KPICard
          title="Taxa de Conclusão"
          :value="`${completionRate}%`"
          icon="✓"
        />
        <KPICard
          title="Tempo Médio"
          :value="averageTime"
          icon="⏱️"
        />
        <KPICard
          title="NPS Score"
          :value="npsScore"
          icon="👍"
          :highlight="true"
        />
      </div>

      <!-- Questions Analysis -->
      <div v-if="!loading && evaluation" class="space-y-6">
        <div
          v-for="(question, index) in evaluation.questions"
          :key="question.id"
          class="bg-slate-800/50 border border-slate-700 rounded-xl p-6"
        >
          <!-- Question Header -->
          <div class="mb-6">
            <h2 class="text-lg font-semibold text-white mb-2">
              {{ index + 1 }}. {{ question.text }}
            </h2>
            <p class="text-sm text-slate-400">{{ getQuestionTypeName(question.type) }}</p>
          </div>

          <!-- Analysis by Type -->
          <QuestionAnalysis
            :question="question"
            :responses="getResponsesForQuestion(index)"
          />
        </div>
      </div>

      <!-- Filters and Export -->
      <div v-if="!loading && evaluation" class="mt-8 flex gap-4">
        <button
          @click="applyFilter"
          class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
        >
          🔍 Filtrar Resultados
        </button>
        <button
          @click="exportData"
          class="px-4 py-2 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition text-sm flex items-center gap-2"
        >
          ⬇️ Exportar CSV
        </button>
      </div>

      <!-- Loading State -->
      <div v-if="loading" class="flex justify-center py-12">
        <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
      </div>

      <!-- Error State -->
      <div v-if="error" class="text-center py-12">
        <p class="text-red-400 mb-4">{{ error }}</p>
        <button
          @click="loadResults"
          class="px-4 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
        >
          Tentar novamente
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute } from 'vue-router'
import KPICard from '@/components/results/KPICard.vue'
import QuestionAnalysis from '@/components/results/QuestionAnalysis.vue'

interface Question {
  id: string
  text: string
  type: string
  required: boolean
  options?: string[]
  config?: Record<string, any>
}

interface Evaluation {
  id: string
  title: string
  description: string
  status: string
  questions: Question[]
  createdAt: string
  closedAt: string
  totalResponses: number
  completionRate: number
  averageTime: string
  npsScore: number
}

interface Response {
  id: string
  evaluationId: string
  questionIndex: number
  value: any
  respondentName?: string
  completedAt: string
  timeSpent: number
}

const route = useRoute()
const evaluationId = route.params.id as string

const loading = ref(true)
const error = ref('')
const evaluation = ref<Evaluation | null>(null)
const responses = ref<Response[]>([])

const totalResponses = computed(() => evaluation.value?.totalResponses || 0)
const completionRate = computed(() => evaluation.value?.completionRate || 0)
const averageTime = computed(() => evaluation.value?.averageTime || '--')
const npsScore = computed(() => evaluation.value?.npsScore || 0)

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('pt-BR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const getQuestionTypeName = (type: string): string => {
  const names: Record<string, string> = {
    likert: 'Escala Likert',
    frequency: 'Escala de Frequência',
    paired: 'Comparação Pareada',
    fixed_sum: 'Distribuição de Pontos',
    text: 'Texto Aberto',
    multiple_choice: 'Múltipla Escolha',
    single_choice: 'Escolha Única'
  }
  return names[type] || type
}

const getResponsesForQuestion = (questionIndex: number): Response[] => {
  return responses.value.filter(r => r.questionIndex === questionIndex)
}

const loadResults = async () => {
  try {
    loading.value = true
    error.value = ''

    const [evalResponse, respResponse] = await Promise.all([
      fetch(`/api/evaluations/${evaluationId}`),
      fetch(`/api/evaluations/${evaluationId}/responses`)
    ])

    if (!evalResponse.ok || !respResponse.ok) {
      throw new Error('Erro ao carregar dados')
    }

    evaluation.value = await evalResponse.json()
    responses.value = await respResponse.json()
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erro ao carregar resultados'
  } finally {
    loading.value = false
  }
}

const applyFilter = () => {
  // Implement filtering logic
  console.log('Aplicar filtro')
}

const exportData = async () => {
  try {
    const response = await fetch(`/api/evaluations/${evaluationId}/export/csv`)
    if (!response.ok) throw new Error('Erro ao exportar')

    const blob = await response.blob()
    const url = window.URL.createObjectURL(blob)
    const a = document.createElement('a')
    a.href = url
    a.download = `${evaluation.value?.title || 'avaliacao'}-resultados.csv`
    document.body.appendChild(a)
    a.click()
    window.URL.revokeObjectURL(url)
    document.body.removeChild(a)
  } catch (err) {
    console.error('Erro ao exportar:', err)
  }
}

onMounted(() => {
  loadResults()
})
</script>
