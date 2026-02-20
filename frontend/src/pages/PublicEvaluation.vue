<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-slate-800 to-slate-900">
    <!-- Header -->
    <header class="sticky top-0 z-40 bg-slate-800/80 backdrop-blur-lg border-b border-slate-700">
      <div class="max-w-4xl mx-auto px-4 sm:px-6 lg:px-8 py-4 flex items-center justify-between">
        <div class="flex items-center gap-3">
          <div class="h-8 w-8 rounded-lg bg-gradient-to-br from-purple-600 to-purple-500 text-white font-bold flex items-center justify-center text-sm">
            A
          </div>
          <span class="text-lg font-bold text-white hidden sm:inline">Aevalo</span>
        </div>
        <div class="text-sm text-slate-400">
          {{ currentQuestion + 1 }} de {{ evaluationData?.questions.length || 0 }}
        </div>
      </div>

      <!-- Progress Bar -->
      <div class="h-1 bg-slate-700">
        <div
          class="h-full bg-gradient-to-r from-purple-600 to-purple-500 transition-all duration-300"
          :style="{ width: progressPercentage + '%' }"
        ></div>
      </div>
    </header>

    <!-- Main Content -->
    <main class="flex-1">
      <!-- Status Messages -->
      <div class="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
        <!-- Draft State -->
        <div v-if="evaluationData?.status === 'draft'" class="py-20 text-center">
          <div class="text-6xl mb-4">📝</div>
          <h1 class="text-2xl font-bold text-white mb-3">Esta avaliação ainda não foi publicada</h1>
          <p class="text-slate-400 mb-6">O criador da avaliação ainda está preparando o conteúdo.</p>
          <button
            @click="$router.back()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            ← Voltar
          </button>
        </div>

        <!-- Closed State -->
        <div v-else-if="evaluationData?.status === 'closed'" class="py-20 text-center">
          <div class="text-6xl mb-4">🔒</div>
          <h1 class="text-2xl font-bold text-white mb-3">Esta avaliação foi encerrada</h1>
          <p class="text-slate-400 mb-2">Data de encerramento: {{ formatDate(evaluationData?.closedAt) }}</p>
          <p class="text-slate-400 mb-6">Obrigado por seu interesse em participar!</p>
          <button
            @click="$router.back()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            ← Voltar
          </button>
        </div>

        <!-- Expired State -->
        <div v-else-if="evaluationData?.status === 'expired'" class="py-20 text-center">
          <div class="text-6xl mb-4">⏰</div>
          <h1 class="text-2xl font-bold text-white mb-3">O prazo para responder expirou</h1>
          <p class="text-slate-400 mb-6">Expirou em: {{ formatDate(evaluationData?.expiredAt) }}</p>
          <button
            @click="$router.back()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            ← Voltar
          </button>
        </div>

        <!-- Limit Reached State -->
        <div v-else-if="evaluationData?.status === 'limit_reached'" class="py-20 text-center">
          <div class="text-6xl mb-4">✅</div>
          <h1 class="text-2xl font-bold text-white mb-3">Limite de respostas atingido</h1>
          <p class="text-slate-400 mb-6">Obrigado por seu interesse! Já temos todas as respostas que precisávamos.</p>
          <button
            @click="$router.back()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            ← Voltar
          </button>
        </div>

        <!-- Loading State -->
        <div v-else-if="loading" class="py-20 text-center">
          <div class="inline-flex items-center justify-center">
            <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-purple-600"></div>
          </div>
          <p class="text-slate-400 mt-4">Carregando avaliação...</p>
        </div>

        <!-- Error State -->
        <div v-else-if="error" class="py-20 text-center">
          <div class="text-6xl mb-4">⚠️</div>
          <h1 class="text-2xl font-bold text-white mb-3">Link inválido ou expirado</h1>
          <p class="text-slate-400 mb-6">{{ error }}</p>
          <button
            @click="$router.back()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
          >
            ← Voltar
          </button>
        </div>

        <!-- Open State - Evaluation Questions -->
        <div v-else-if="evaluationData?.status === 'open'" class="space-y-8">
          <!-- Validation Errors Alert -->
          <div v-if="validationErrors.length" class="p-4 rounded-lg bg-red-500/10 border border-red-500/30">
            <p class="font-semibold text-red-400 mb-2">{{ validationErrors.length }} pergunta(s) precisa(m) de atenção:</p>
            <ul class="space-y-1 text-sm text-red-400">
              <li v-for="(error, idx) in validationErrors" :key="idx">• {{ error }}</li>
            </ul>
          </div>

          <!-- Current Question -->
          <div class="space-y-6">
            <!-- Question Title -->
            <div>
              <h2 class="text-3xl font-bold text-white mb-2">{{ currentQuestionData?.text }}</h2>
              <p v-if="currentQuestionData?.required" class="text-sm text-red-400">* Obrigatória</p>
            </div>

            <!-- Question Responses Component -->
            <div class="bg-slate-800/50 border border-slate-700 rounded-xl p-6">
              <PublicQuestionRenderer
                :question="currentQuestionData"
                :value="responses[currentQuestion]"
                @update="updateResponse"
              />
            </div>
          </div>

          <!-- Navigation Buttons -->
          <div class="flex items-center justify-between gap-4 pt-6">
            <button
              @click="previousQuestion"
              :disabled="currentQuestion === 0"
              class="px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 disabled:opacity-50 disabled:cursor-not-allowed transition"
            >
              ← Anterior
            </button>

            <div class="text-sm text-slate-400">
              {{ currentQuestion + 1 }} / {{ evaluationData?.questions.length }}
            </div>

            <button
              v-if="currentQuestion < (evaluationData?.questions.length || 0) - 1"
              @click="nextQuestion"
              class="px-6 py-2 rounded-lg bg-purple-600 text-white hover:bg-purple-700 transition"
            >
              Próxima →
            </button>

            <button
              v-else
              @click="submitResponses"
              :disabled="submitting"
              class="px-6 py-2 rounded-lg bg-gradient-to-r from-purple-600 to-purple-500 text-white hover:shadow-lg hover:shadow-purple-600/50 disabled:opacity-50 disabled:cursor-not-allowed transition"
            >
              <span v-if="submitting" class="flex items-center gap-2">
                <span class="animate-spin">⏳</span>
                Enviando...
              </span>
              <span v-else>✓ Enviar Respostas</span>
            </button>
          </div>
        </div>
      </div>
    </main>

    <!-- Success Modal -->
    <SuccessModal v-if="showSuccessModal" @close="handleSuccessClose" />

    <!-- Error Modal -->
    <ErrorModal v-if="showErrorModal" :error="errorMessage" @close="showErrorModal = false" @retry="submitResponses" />

    <!-- Footer -->
    <footer class="border-t border-slate-700 py-6 px-4 text-center text-xs text-slate-500">
      <p>Powered by <a href="https://aevalo.app" target="_blank" rel="noopener" class="text-purple-400 hover:text-purple-300">Aevalo</a></p>
      <p class="mt-2 space-x-4">
        <a href="#" class="hover:text-slate-400">Política de Privacidade</a>
        <span>•</span>
        <a href="#" class="hover:text-slate-400">Suporte</a>
      </p>
    </footer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'
import PublicQuestionRenderer from '@/components/PublicQuestionRenderer.vue'
import SuccessModal from '@/components/modals/SuccessModal.vue'
import ErrorModal from '@/components/modals/ErrorModal.vue'

interface Question {
  id: string
  text: string
  type: string
  required: boolean
  options?: string[]
  config?: Record<string, any>
}

interface EvaluationData {
  id: string
  title: string
  description: string
  status: 'draft' | 'open' | 'closed' | 'expired' | 'limit_reached'
  questions: Question[]
  closedAt?: string
  expiredAt?: string
  responseLimit?: number
  currentResponseCount?: number
}

const route = useRoute()
const router = useRouter()

const evaluationId = route.params.id as string
const loading = ref(true)
const error = ref('')
const evaluationData = ref<EvaluationData | null>(null)
const currentQuestion = ref(0)
const responses = ref<Record<number, any>>({})
const validationErrors = ref<string[]>([])
const submitting = ref(false)
const showSuccessModal = ref(false)
const showErrorModal = ref(false)
const errorMessage = ref('')

const currentQuestionData = computed(() => {
  return evaluationData.value?.questions[currentQuestion.value]
})

const progressPercentage = computed(() => {
  if (!evaluationData.value?.questions.length) return 0
  return ((currentQuestion.value + 1) / evaluationData.value.questions.length) * 100
})

const formatDate = (date?: string) => {
  if (!date) return ''
  return new Date(date).toLocaleDateString('pt-BR', {
    year: 'numeric',
    month: 'long',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

const loadEvaluation = async () => {
  try {
    loading.value = true
    error.value = ''

    // Substituir por chamada real à API
    const response = await fetch(`/api/evaluations/${evaluationId}/public`)
    if (!response.ok) {
      throw new Error('Avaliação não encontrada')
    }

    evaluationData.value = await response.json()
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Erro ao carregar avaliação'
  } finally {
    loading.value = false
  }
}

const updateResponse = (value: any) => {
  responses.value[currentQuestion.value] = value
  validationErrors.value = validationErrors.value.filter(err => !err.includes(`Pergunta ${currentQuestion.value + 1}`))
}

const nextQuestion = () => {
  if (currentQuestion.value < (evaluationData.value?.questions.length || 0) - 1) {
    currentQuestion.value++
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

const previousQuestion = () => {
  if (currentQuestion.value > 0) {
    currentQuestion.value--
    window.scrollTo({ top: 0, behavior: 'smooth' })
  }
}

const validateResponses = (): boolean => {
  validationErrors.value = []
  const errors: string[] = []

  evaluationData.value?.questions.forEach((question, index) => {
    const response = responses.value[index]

    // Check required fields
    if (question.required && !response) {
      errors.push(`Pergunta ${index + 1}: "${question.text}" é obrigatória`)
      return
    }

    // Validate Fixed Sum
    if (question.type === 'fixed_sum' && response) {
      const sum = Object.values(response as Record<string, number>).reduce((a: number, b: number) => a + b, 0)
      if (sum !== 100) {
        errors.push(`Pergunta ${index + 1}: A soma deve ser exatamente 100`)
      }
    }

    // Validate Text Length
    if (question.type === 'text' && response) {
      const maxLength = question.config?.maxLength || 500
      if (response.length > maxLength) {
        errors.push(`Pergunta ${index + 1}: Máximo ${maxLength} caracteres`)
      }
    }

    // Validate Multiple Choice Min/Max
    if (question.type === 'multiple_choice' && response) {
      const minSelections = question.config?.minSelections || 0
      const maxSelections = question.config?.maxSelections || 999
      const count = Array.isArray(response) ? response.length : 0

      if (count < minSelections) {
        errors.push(`Pergunta ${index + 1}: Selecione pelo menos ${minSelections} opção(ões)`)
      }
      if (count > maxSelections) {
        errors.push(`Pergunta ${index + 1}: Máximo ${maxSelections} opção(ões)`)
      }
    }
  })

  validationErrors.value = errors
  return errors.length === 0
}

const submitResponses = async () => {
  // Validate all responses
  if (!validateResponses()) {
    // Scroll to first error
    window.scrollTo({ top: 0, behavior: 'smooth' })
    return
  }

  try {
    submitting.value = true

    // Prepare payload
    const payload = {
      evaluationId,
      responses: responses.value,
      submittedAt: new Date().toISOString()
    }

    // Make API call
    const response = await fetch(`/api/evaluations/${evaluationId}/responses`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify(payload)
    })

    if (!response.ok) {
      const data = await response.json()
      throw new Error(data.message || 'Erro ao enviar respostas')
    }

    showSuccessModal.value = true
  } catch (err) {
    errorMessage.value = err instanceof Error ? err.message : 'Erro ao enviar respostas'
    showErrorModal.value = true
  } finally {
    submitting.value = false
  }
}

const handleSuccessClose = () => {
  showSuccessModal.value = false
  router.push('/')
}

onMounted(() => {
  loadEvaluation()
})
</script>

<style scoped>
/* Smooth transitions */
:deep(.question-transition) {
  transition: opacity 300ms ease, transform 300ms ease;
}
</style>
