<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
    <div class="max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 py-8 space-y-6">

      <!-- Back Link -->
      <RouterLink
        to="/dashboard"
        class="inline-flex items-center gap-2 text-sm text-slate-400 hover:text-white transition"
      >
        ← Voltar ao Dashboard
      </RouterLink>

      <!-- Loading -->
      <div v-if="loading" class="space-y-4">
        <div class="animate-pulse h-10 bg-slate-700 rounded-lg w-2/3"></div>
        <div class="animate-pulse h-6 bg-slate-700 rounded-lg w-1/3"></div>
      </div>

      <!-- Error -->
      <div v-else-if="error" class="text-center py-24">
        <p class="text-slate-400 mb-4">{{ error }}</p>
        <button @click="loadEvaluation" class="px-4 py-2 bg-purple-600 text-white rounded-lg hover:bg-purple-700 transition">
          Tentar novamente
        </button>
      </div>

      <template v-else-if="evaluation">
        <!-- Header -->
        <div class="bg-slate-800/60 border border-slate-700 rounded-2xl p-6">
          <div class="flex flex-col md:flex-row md:items-start gap-4 justify-between">
            <div class="flex-1 min-w-0">
              <div class="flex items-center gap-3 mb-2 flex-wrap">
                <h1 class="text-2xl font-bold text-white">{{ evaluation.title }}</h1>
                <span
                  class="px-3 py-1 rounded-full text-xs font-semibold"
                  :class="statusClass"
                >
                  {{ statusLabel }}
                </span>
              </div>
              <p class="text-slate-400 text-sm mb-4">{{ evaluation.description }}</p>

              <!-- Meta info -->
              <div class="flex flex-wrap gap-4 text-xs text-slate-400">
                <span>📅 Criada em {{ formatDate(evaluation.createdAt) }}</span>
                <span v-if="evaluation.closedAt">🔒 Encerrada em {{ formatDate(evaluation.closedAt) }}</span>
                <span>📋 {{ evaluation.questions?.length || 0 }} perguntas</span>
                <span>💬 {{ evaluation.responseCount || 0 }} respostas</span>
              </div>
            </div>

            <!-- Action Buttons -->
            <div class="flex flex-wrap gap-2 flex-shrink-0">
              <button
                @click="showShareModal = true"
                class="px-4 py-2 rounded-lg bg-purple-600/20 text-purple-400 border border-purple-600/30 hover:bg-purple-600/30 transition text-sm font-medium flex items-center gap-2"
              >
                🔗 Compartilhar
              </button>
              <RouterLink
                :to="`/evaluation/${evaluation.id}/results`"
                class="px-4 py-2 rounded-lg bg-slate-700 text-slate-200 hover:bg-slate-600 transition text-sm font-medium flex items-center gap-2"
              >
                📊 Resultados
              </RouterLink>
              <button
                v-if="evaluation.status === 'open'"
                @click="showCloseModal = true"
                class="px-4 py-2 rounded-lg bg-red-600/20 text-red-400 border border-red-600/30 hover:bg-red-600/30 transition text-sm font-medium flex items-center gap-2"
              >
                🔒 Encerrar
              </button>
            </div>
          </div>
        </div>

        <!-- Public Link Banner (only when open) -->
        <div v-if="evaluation.status === 'open'" class="bg-slate-800/40 border border-slate-700 rounded-xl p-4">
          <div class="flex flex-col sm:flex-row items-start sm:items-center gap-3">
            <div class="flex-1 min-w-0">
              <p class="text-xs font-semibold text-slate-400 uppercase mb-1">Link Público</p>
              <p class="text-sm text-white font-mono truncate">{{ publicLink }}</p>
            </div>
            <button
              @click="copyLink"
              class="flex-shrink-0 px-4 py-2 rounded-lg transition text-sm font-medium"
              :class="linkCopied ? 'bg-green-600 text-white' : 'bg-slate-700 text-slate-200 hover:bg-slate-600'"
            >
              {{ linkCopied ? '✓ Copiado!' : '📋 Copiar Link' }}
            </button>
          </div>
        </div>

        <!-- Stats + Widget Grid -->
        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Stats Cards -->
          <div class="lg:col-span-2 grid grid-cols-2 sm:grid-cols-4 gap-4">
            <div class="bg-slate-800/60 border border-slate-700 rounded-xl p-4">
              <p class="text-xs text-slate-400 mb-1">Respostas</p>
              <p class="text-3xl font-bold text-white">{{ evaluation.responseCount || 0 }}</p>
            </div>
            <div class="bg-slate-800/60 border border-slate-700 rounded-xl p-4">
              <p class="text-xs text-slate-400 mb-1">Conclusão</p>
              <p class="text-3xl font-bold text-white">{{ evaluation.completionRate || 0 }}%</p>
            </div>
            <div class="bg-slate-800/60 border border-slate-700 rounded-xl p-4">
              <p class="text-xs text-slate-400 mb-1">Tempo Médio</p>
              <p class="text-2xl font-bold text-white">{{ evaluation.avgTime || '--' }}</p>
            </div>
            <div class="bg-slate-800/60 border border-slate-700 rounded-xl p-4">
              <p class="text-xs text-slate-400 mb-1">Perguntas</p>
              <p class="text-3xl font-bold text-white">{{ evaluation.questions?.length || 0 }}</p>
            </div>

            <!-- Response progress bar -->
            <div v-if="evaluation.responseLimit" class="col-span-full bg-slate-800/60 border border-slate-700 rounded-xl p-4">
              <div class="flex justify-between text-xs text-slate-400 mb-2">
                <span>Progresso de Respostas</span>
                <span>{{ evaluation.responseCount }} / {{ evaluation.responseLimit }}</span>
              </div>
              <div class="w-full bg-slate-700 rounded-full h-2">
                <div
                  class="h-full rounded-full bg-gradient-to-r from-purple-600 to-purple-400 transition-all"
                  :style="{ width: responseProgress + '%' }"
                ></div>
              </div>
            </div>
          </div>

          <!-- Active Evaluators Widget -->
          <ActiveEvaluatorsWidget
            :evaluation-id="evaluation.id"
            :total-questions="evaluation.questions?.length || 0"
          />
        </div>

        <!-- Questions List -->
        <div class="bg-slate-800/60 border border-slate-700 rounded-2xl p-6">
          <h2 class="text-lg font-bold text-white mb-4">Perguntas ({{ evaluation.questions?.length || 0 }})</h2>
          <div v-if="evaluation.questions?.length" class="space-y-3">
            <div
              v-for="(question, idx) in evaluation.questions"
              :key="question.id"
              class="flex items-start gap-4 p-4 rounded-xl bg-slate-700/40 border border-slate-600/50"
            >
              <span class="flex-shrink-0 w-7 h-7 rounded-full bg-purple-600/20 text-purple-400 text-xs font-bold flex items-center justify-center">
                {{ idx + 1 }}
              </span>
              <div class="flex-1 min-w-0">
                <p class="text-sm font-medium text-white">{{ question.text }}</p>
                <p class="text-xs text-slate-400 mt-1">{{ getScaleLabel(question.type) }}</p>
              </div>
              <span v-if="question.required" class="text-xs px-2 py-0.5 rounded bg-red-500/20 text-red-400 flex-shrink-0">Obrigatória</span>
            </div>
          </div>
          <div v-else class="text-center py-8 text-slate-400">
            <p>Nenhuma pergunta cadastrada</p>
          </div>
        </div>
      </template>
    </div>

    <!-- Share Modal -->
    <ShareModal
      v-if="showShareModal && evaluation"
      :evaluation-id="evaluation.id"
      :evaluation-title="evaluation.title"
      @close="showShareModal = false"
    />

    <!-- Close Evaluation Modal -->
    <CloseEvaluationModal
      v-if="showCloseModal && evaluation"
      :evaluation-id="evaluation.id"
      :evaluation-title="evaluation.title"
      :response-count="evaluation.responseCount || 0"
      @close="showCloseModal = false"
      @confirmed="handleEvaluationClosed"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter, RouterLink } from 'vue-router'
import ShareModal from '@/components/modals/ShareModal.vue'
import CloseEvaluationModal from '@/components/modals/CloseEvaluationModal.vue'
import ActiveEvaluatorsWidget from '@/components/widgets/ActiveEvaluatorsWidget.vue'

interface Question {
  id: string
  text: string
  type: string
  required: boolean
}

interface Evaluation {
  id: string
  title: string
  description: string
  status: 'draft' | 'open' | 'closed' | 'expired'
  questions: Question[]
  responseCount: number
  responseLimit?: number
  completionRate: number
  avgTime: string
  createdAt: string
  closedAt?: string
}

const route = useRoute()
const router = useRouter()
const evaluationId = route.params.id as string

const loading = ref(true)
const error = ref('')
const evaluation = ref<Evaluation | null>(null)
const showShareModal = ref(false)
const showCloseModal = ref(false)
const linkCopied = ref(false)

const publicLink = computed(() =>
  `${window.location.origin}/e/${evaluationId}`
)

const responseProgress = computed(() => {
  if (!evaluation.value?.responseLimit) return 0
  return Math.min(100, Math.round((evaluation.value.responseCount / evaluation.value.responseLimit) * 100))
})

const statusClass = computed(() => {
  const map: Record<string, string> = {
    open: 'bg-green-500/20 text-green-400',
    draft: 'bg-slate-500/20 text-slate-400',
    closed: 'bg-red-500/20 text-red-400',
    expired: 'bg-orange-500/20 text-orange-400',
  }
  return map[evaluation.value?.status || 'draft']
})

const statusLabel = computed(() => {
  const map: Record<string, string> = {
    open: '🟢 Aberta',
    draft: '📝 Rascunho',
    closed: '🔒 Encerrada',
    expired: '⏰ Expirada',
  }
  return map[evaluation.value?.status || 'draft']
})

const formatDate = (date?: string) => {
  if (!date) return '--'
  return new Date(date).toLocaleDateString('pt-BR', {
    day: '2-digit',
    month: 'short',
    year: 'numeric',
  })
}

const getScaleLabel = (type: string) => {
  const map: Record<string, string> = {
    likert: 'Escala Likert',
    frequency: 'Frequência',
    paired: 'Comparação Pareada',
    fixed_sum: 'Distribuir 100 Pontos',
    text: 'Texto Aberto',
    multiple_choice: 'Múltipla Escolha',
    single_choice: 'Escolha Única',
  }
  return map[type] || type
}

const copyLink = async () => {
  try {
    await navigator.clipboard.writeText(publicLink.value)
    linkCopied.value = true
    setTimeout(() => { linkCopied.value = false }, 2500)
  } catch {
    // fallback
  }
}

const loadEvaluation = async () => {
  try {
    loading.value = true
    error.value = ''
    const res = await fetch(`/api/evaluations/${evaluationId}`)
    if (!res.ok) throw new Error('Avaliação não encontrada')
    evaluation.value = await res.json()
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erro ao carregar'
  } finally {
    loading.value = false
  }
}

const handleEvaluationClosed = () => {
  showCloseModal.value = false
  if (evaluation.value) {
    evaluation.value.status = 'closed'
    evaluation.value.closedAt = new Date().toISOString()
  }
  router.push(`/evaluation/${evaluationId}/results`)
}

onMounted(loadEvaluation)
</script>
