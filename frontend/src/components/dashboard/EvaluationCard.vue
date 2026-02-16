<template>
  <div class="rounded-2xl border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-800 shadow-sm hover:shadow-md transition-shadow overflow-hidden">
    <!-- Card Header -->
    <div class="p-4 border-b border-slate-200 dark:border-slate-700">
      <div class="flex items-start justify-between">
        <div class="flex-1">
          <div class="flex items-center gap-2 mb-1">
            <span class="text-lg">{{ categoryIcon }}</span>
            <h3 class="text-sm font-semibold text-slate-900 dark:text-white truncate">
              {{ title }}
            </h3>
          </div>
          <p class="text-xs text-slate-500 dark:text-slate-400">
            {{ category }}
          </p>
        </div>
        
        <!-- Status badge -->
        <Badge :variant="statusBadgeVariant" class="flex-shrink-0">
          {{ status }}
        </Badge>
      </div>
    </div>

    <!-- Card Body -->
    <div class="p-4 space-y-3">
      <!-- Meta info -->
      <div class="flex items-center justify-between text-xs text-slate-600 dark:text-slate-400">
        <span>Criada em {{ createdDate }}</span>
        <span>Atualizada {{ updatedAt }}</span>
      </div>

      <!-- Progress -->
      <div>
        <div class="flex items-center justify-between mb-1">
          <span class="text-xs font-medium text-slate-700 dark:text-slate-300">
            {{ responses }}/{{ totalResponses }}
          </span>
          <span class="text-xs text-slate-500 dark:text-slate-400">
            {{ responsePercentage }}%
          </span>
        </div>
        <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-1.5">
          <div
            class="bg-gradient-to-r from-primary to-accent h-1.5 rounded-full transition-all"
            :style="{ width: responsePercentage + '%' }"
          />
        </div>
      </div>

      <!-- Quick actions -->
      <div class="flex gap-2 pt-2">
        <button
          @click="$emit('view')"
          class="flex-1 text-xs font-medium text-primary hover:bg-primary/5 py-2 rounded-lg transition"
        >
          📊 Ver Resultados
        </button>
        <button
          @click="$emit('edit')"
          class="flex-1 text-xs font-medium text-slate-700 dark:text-slate-300 hover:bg-slate-100 dark:hover:bg-slate-700 py-2 rounded-lg transition"
        >
          ✏️ Editar
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue'
import { Badge } from '../ui/index'

type EvaluationStatus = 'open' | 'closed' | 'draft' | 'archived'

interface Props {
  title?: string
  category?: string
  categoryIcon?: string
  status?: EvaluationStatus
  responses?: number
  totalResponses?: number
  createdDate?: string
  updatedAt?: string
}

const props = withDefaults(defineProps<Props>(), {
  title: 'Avaliação Sem Título',
  category: 'Geral',
  categoryIcon: '📝',
  status: 'open',
  responses: 5,
  totalResponses: 20,
  createdDate: '15/02/2026',
  updatedAt: 'há 2 dias'
})

defineEmits<{
  view: []
  edit: []
}>()

const statusBadgeVariant = computed(() => {
  const variants = {
    open: 'success',
    closed: 'secondary',
    draft: 'warning',
    archived: 'info'
  } as Record<EvaluationStatus, string>
  return variants[props.status] || 'secondary'
})

const responsePercentage = computed(() => {
  if (props.totalResponses === 0) return 0
  return Math.round((props.responses / props.totalResponses) * 100)
})
</script>
