<template>
  <div class="bg-slate-700/50 border border-slate-600 rounded-xl p-4">
    <!-- Header -->
    <div class="flex items-center justify-between mb-4">
      <h3 class="font-semibold text-slate-200 flex items-center gap-2">
        <span class="animate-pulse text-red-500">●</span>
        👥 Avaliadores Ativos
        <span class="ml-2 text-sm font-normal text-slate-400">({{ activeEvaluators.length }})</span>
      </h3>
      <button
        @click="refresh"
        class="text-xs px-2 py-1 rounded bg-slate-600 text-slate-300 hover:bg-slate-500 transition"
        :disabled="refreshing"
      >
        {{ refreshing ? '⟳ Atualizando...' : '⟳ Atualizar' }}
      </button>
    </div>

    <!-- Active Evaluators List -->
    <div v-if="activeEvaluators.length > 0" class="space-y-2">
      <div
        v-for="evaluator in activeEvaluators"
        :key="evaluator.id"
        class="p-3 rounded-lg bg-slate-800/50 border border-slate-600/50 flex items-center gap-3 animate-in slide-in"
      >
        <!-- Status Indicator (Green) -->
        <div class="w-2 h-2 rounded-full bg-green-500 animate-pulse flex-shrink-0"></div>

        <!-- Evaluator Name -->
        <div class="flex-1 min-w-0">
          <p class="text-sm font-medium text-slate-200 truncate">{{ evaluator.name }}</p>
          <p class="text-xs text-slate-400">
            Pergunta {{ evaluator.currentQuestion }} de {{ totalQuestions }}
          </p>
        </div>

        <!-- Time Badge -->
        <div class="text-xs text-slate-400 flex-shrink-0">
          {{ evaluator.lastActivityAgo }}
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="text-center py-8">
      <p class="text-slate-400 text-sm">Nenhum avaliador ativo no momento</p>
    </div>

    <!-- Stats -->
    <div class="mt-4 pt-4 border-t border-slate-600 space-y-2">
      <div class="flex justify-between text-xs text-slate-400">
        <span>Total de respostas:</span>
        <span class="font-semibold text-slate-200">{{ totalResponses }}</span>
      </div>
      <div class="flex justify-between text-xs text-slate-400">
        <span>Última resposta:</span>
        <span class="font-semibold text-slate-200">{{ lastResponseTime }}</span>
      </div>
      <div class="flex justify-between text-xs text-slate-400">
        <span>Taxa de conclusão:</span>
        <span class="font-semibold text-slate-200">{{ completionRate }}%</span>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

interface Props {
  evaluationId: string
  totalQuestions: number
}

interface Evaluator {
  id: string
  name: string
  currentQuestion: number
  lastActivity: Date
  lastActivityAgo: string
  isAnonymous: boolean
}

const props = defineProps<Props>()

const activeEvaluators = ref<Evaluator[]>([])
const totalResponses = ref(0)
const lastResponseTime = ref('--')
const completionRate = ref(0)
const refreshing = ref(false)
let refreshInterval: ReturnType<typeof setInterval> | null = null

const loadActiveEvaluators = async () => {
  try {
    refreshing.value = true

    // In production, use Supabase Realtime or WebSocket
    // For now, fetch from API
    const response = await fetch(`/api/evaluations/${props.evaluationId}/active-evaluators`)
    if (!response.ok) throw new Error('Erro ao carregar avaliadores ativos')

    const data = await response.json()
    activeEvaluators.value = data.activeEvaluators.map((e: any) => ({
      ...e,
      lastActivityAgo: formatTimeAgo(new Date(e.lastActivity))
    }))

    totalResponses.value = data.totalResponses
    completionRate.value = data.completionRate
    lastResponseTime.value = formatTimeAgo(new Date(data.lastResponseTime))
  } catch (err) {
    console.error('Erro ao carregar avaliadores:', err)
  } finally {
    refreshing.value = false
  }
}

const refresh = async () => {
  await loadActiveEvaluators()
}

const formatTimeAgo = (date: Date): string => {
  const now = new Date()
  const diffMs = now.getTime() - date.getTime()
  const diffMins = Math.floor(diffMs / 60000)

  if (diffMins < 1) return 'agora'
  if (diffMins === 1) return '1 minuto atrás'
  if (diffMins < 60) return `${diffMins} minutos atrás`

  const diffHours = Math.floor(diffMins / 60)
  if (diffHours === 1) return '1 hora atrás'
  if (diffHours < 24) return `${diffHours} horas atrás`

  return date.toLocaleDateString('pt-BR')
}

onMounted(() => {
  loadActiveEvaluators()

  // Refresh every 30 seconds
  refreshInterval = setInterval(() => {
    loadActiveEvaluators()
  }, 30000)
})

onUnmounted(() => {
  if (refreshInterval) {
    clearInterval(refreshInterval)
  }
})
</script>

<style scoped>
.animate-in {
  animation: slideIn 300ms ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}
</style>
