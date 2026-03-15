<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900 py-8">
    <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
      <!-- Header -->
      <div class="mb-6">
        <div v-if="loading" class="animate-pulse space-y-4">
          <div class="h-10 bg-slate-700 rounded-lg w-1/2"></div>
          <div class="h-6 bg-slate-700 rounded-lg w-1/3"></div>
        </div>

        <div v-else-if="evaluation">
          <div class="flex flex-col lg:flex-row lg:items-start lg:justify-between gap-4">
            <div>
              <h1 class="text-3xl md:text-4xl font-bold text-white mb-2">{{ evaluation.title }}</h1>
              <p class="text-slate-400 mb-4">{{ evaluation.description }}</p>

              <div class="flex flex-wrap items-center gap-3 text-sm text-slate-400">
                <span>Aberta em {{ formatDate(evaluation.createdAt) }}</span>
                <span>•</span>
                <span v-if="evaluation.closedAt">Encerrada em {{ formatDate(evaluation.closedAt) }}</span>
                <span v-else>Em andamento</span>
                <span>•</span>
                <span class="text-white font-semibold">{{ totalResponses }} respostas</span>
                <span>•</span>
                <span class="text-emerald-400 font-medium">{{ completionRate }}% ({{ totalResponses }}/{{ startedResponses }} iniciadas)</span>
              </div>
            </div>

            <div class="flex flex-wrap gap-2">
              <button
                @click="exportData('xlsx')"
                class="px-3 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
              >
                📊 Excel (.xlsx)
              </button>
              <button
                @click="exportData('csv')"
                class="px-3 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
              >
                📄 CSV
              </button>
              <button
                @click="exportData('charts')"
                class="px-3 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm"
              >
                📈 Gráficos
              </button>
              <button
                @click="exportData('pdf')"
                class="px-3 py-2 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition text-sm"
              >
                📑 PDF
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Filters -->
      <div v-if="!loading && evaluation" class="mb-8 bg-slate-800/60 border border-slate-700 rounded-xl p-4">
        <div class="flex flex-col lg:flex-row gap-3">
          <div class="flex-1">
            <label class="text-xs text-slate-400 mb-1 block">Período da resposta</label>
            <div class="grid grid-cols-2 gap-2">
              <input
                v-model="filters.startDate"
                type="date"
                class="w-full px-3 py-2 rounded-lg bg-slate-700 border border-slate-600 text-slate-200 text-sm"
              />
              <input
                v-model="filters.endDate"
                type="date"
                class="w-full px-3 py-2 rounded-lg bg-slate-700 border border-slate-600 text-slate-200 text-sm"
              />
            </div>
          </div>

          <div>
            <label class="text-xs text-slate-400 mb-1 block">Dispositivo</label>
            <select
              v-model="filters.device"
              class="w-full lg:w-44 px-3 py-2 rounded-lg bg-slate-700 border border-slate-600 text-slate-200 text-sm"
            >
              <option value="all">Todos</option>
              <option value="desktop">Desktop</option>
              <option value="mobile">Mobile</option>
            </select>
          </div>

          <div>
            <label class="text-xs text-slate-400 mb-1 block">Tempo de conclusão</label>
            <select
              v-model="filters.speed"
              class="w-full lg:w-52 px-3 py-2 rounded-lg bg-slate-700 border border-slate-600 text-slate-200 text-sm"
            >
              <option value="all">Todas</option>
              <option value="fast">Rápidas (&lt; 2 min)</option>
              <option value="medium">Médias (2-5 min)</option>
              <option value="slow">Lentas (&gt; 5 min)</option>
            </select>
          </div>

          <div class="flex items-end gap-2">
            <button
              @click="applyFilter"
              class="px-4 py-2 rounded-lg bg-slate-600 text-white hover:bg-slate-500 transition text-sm"
            >
              Aplicar
            </button>
            <button
              @click="resetFilters"
              class="px-4 py-2 rounded-lg bg-transparent border border-slate-600 text-slate-300 hover:bg-slate-700 transition text-sm"
            >
              Limpar
            </button>
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
            :responses="getResponsesForQuestion(index, filteredResponses)"
          />
        </div>
      </div>

      <p v-if="exportMessage" class="mt-4 text-sm text-emerald-400">{{ exportMessage }}</p>

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
import { useRouter } from 'vue-router'
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
  device?: 'desktop' | 'mobile'
}

type ExportFormat = 'xlsx' | 'csv' | 'charts' | 'pdf'

const route = useRoute()
const router = useRouter()
const evaluationId = route.params.id as string

const loading = ref(true)
const error = ref('')
const evaluation = ref<Evaluation | null>(null)
const responses = ref<Response[]>([])
const exportMessage = ref('')

const filters = ref({
  startDate: '',
  endDate: '',
  device: 'all',
  speed: 'all',
})

const totalResponses = computed(() => evaluation.value?.totalResponses || 0)
const completionRate = computed(() => evaluation.value?.completionRate || 0)
const averageTime = computed(() => evaluation.value?.averageTime || '--')
const npsScore = computed(() => evaluation.value?.npsScore || 0)
const startedResponses = computed(() => {
  if (!evaluation.value) return 0
  return Math.max(totalResponses.value, Math.round((totalResponses.value * 100) / Math.max(completionRate.value, 1)))
})

const filteredResponses = computed(() => {
  return responses.value.filter((response) => {
    if (filters.value.startDate) {
      const start = new Date(filters.value.startDate)
      if (new Date(response.completedAt) < start) return false
    }
    if (filters.value.endDate) {
      const end = new Date(filters.value.endDate)
      end.setHours(23, 59, 59, 999)
      if (new Date(response.completedAt) > end) return false
    }
    if (filters.value.device !== 'all') {
      if (response.device !== filters.value.device) return false
    }
    if (filters.value.speed !== 'all') {
      if (filters.value.speed === 'fast' && response.timeSpent >= 120) return false
      if (filters.value.speed === 'medium' && (response.timeSpent < 120 || response.timeSpent > 300)) return false
      if (filters.value.speed === 'slow' && response.timeSpent <= 300) return false
    }
    return true
  })
})

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

const getResponsesForQuestion = (questionIndex: number, source: Response[] = responses.value): Response[] => {
  return source.filter(r => r.questionIndex === questionIndex)
}

const loadResults = async () => {
  try {
    loading.value = true
    error.value = ''

    const [evalResponse, respResponse] = await Promise.all([
      fetch(`/api/evaluations/${evaluationId}`),
      fetch(`/api/evaluations/${evaluationId}/responses`)
    ])

    if (evalResponse.status === 401 || respResponse.status === 401) {
      await router.push('/login')
      return
    }

    if (evalResponse.status === 403 || respResponse.status === 403) {
      throw new Error('Você não tem permissão para visualizar estes resultados')
    }

    if (evalResponse.status === 404 || respResponse.status === 404) {
      throw new Error('Avaliação não encontrada')
    }

    if (!evalResponse.ok || !respResponse.ok) {
      throw new Error('Erro ao carregar resultados')
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
  exportMessage.value = `Filtro aplicado: ${filteredResponses.value.length} respostas visíveis`
  setTimeout(() => {
    exportMessage.value = ''
  }, 2500)
}

const resetFilters = () => {
  filters.value = {
    startDate: '',
    endDate: '',
    device: 'all',
    speed: 'all',
  }
}

const downloadFile = (content: BlobPart, filename: string, type: string) => {
  const blob = new Blob([content], { type })
  const url = window.URL.createObjectURL(blob)
  const a = document.createElement('a')
  a.href = url
  a.download = filename
  document.body.appendChild(a)
  a.click()
  document.body.removeChild(a)
  window.URL.revokeObjectURL(url)
}

const exportData = async (format: ExportFormat) => {
  try {
    const safeTitle = (evaluation.value?.title || 'avaliacao').replace(/\s+/g, '-').toLowerCase()

    if (format === 'csv') {
      const header = 'response_id,question_index,value,completed_at,time_spent_seconds\n'
      const rows = filteredResponses.value.map((r) => {
        const escaped = JSON.stringify(r.value).replaceAll('"', '""')
        return `${r.id},${r.questionIndex},"${escaped}",${r.completedAt},${r.timeSpent}`
      }).join('\n')
      downloadFile(`${header}${rows}`, `${safeTitle}-resultados.csv`, 'text/csv;charset=utf-8')
      exportMessage.value = 'CSV exportado com sucesso'
      return
    }

    if (format === 'xlsx') {
      const header = 'response_id\tquestion_index\tvalue\tcompleted_at\ttime_spent_seconds\n'
      const rows = filteredResponses.value.map((r) => {
        const value = JSON.stringify(r.value)
        return `${r.id}\t${r.questionIndex}\t${value}\t${r.completedAt}\t${r.timeSpent}`
      }).join('\n')
      downloadFile(`${header}${rows}`, `${safeTitle}-resultados.xlsx`, 'application/vnd.openxmlformats-officedocument.spreadsheetml.sheet')
      exportMessage.value = 'Arquivo .xlsx gerado'
      return
    }

    if (format === 'charts') {
      const chartData = JSON.stringify({
        evaluationId,
        exportedAt: new Date().toISOString(),
        questions: evaluation.value?.questions || [],
        responses: filteredResponses.value,
      }, null, 2)
      downloadFile(chartData, `${safeTitle}-graficos.json`, 'application/json')
      exportMessage.value = 'Dados de gráficos exportados'
      return
    }

    if (format === 'pdf') {
      window.print()
      exportMessage.value = 'Use “Salvar como PDF” na janela de impressão'
      return
    }
  } catch (err) {
    console.error('Erro ao exportar:', err)
    exportMessage.value = 'Falha ao exportar resultados'
  } finally {
    setTimeout(() => {
      exportMessage.value = ''
    }, 2800)
  }
}

onMounted(() => {
  loadResults()
})
</script>
