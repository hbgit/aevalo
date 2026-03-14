<!-- Evaluation List Component -->
<template>
  <div class="evaluation-list rounded-2xl border border-slate-200 bg-white dark:bg-slate-800 dark:border-slate-700 p-6">
    <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4 mb-6">
      <div>
        <h2 class="text-lg font-bold text-slate-900 dark:text-white">Todas as Avaliações</h2>
        <p class="text-xs text-slate-500 dark:text-slate-400 mt-1">Mostrando {{ evaluations.length }} avaliações</p>
      </div>
      <button class="px-3 py-2 rounded-lg border border-slate-200 dark:border-slate-700 text-xs font-semibold text-slate-600 dark:text-slate-400 hover:border-primary hover:text-primary dark:hover:text-primary transition">
        Exportar CSV
      </button>
    </div>

    <div class="flex flex-col lg:flex-row gap-4 mb-5">
      <div class="flex-1 relative">
        <input
          type="text"
          placeholder="Buscar: category:NPS status:open"
          class="w-full rounded-lg border border-slate-200 dark:border-slate-700 bg-slate-50 dark:bg-slate-700 px-3 py-2 pl-10 text-xs text-slate-700 dark:text-white placeholder-slate-400 dark:placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary"
        />
        <svg class="absolute left-3 top-2.5 h-4 w-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
        </svg>
      </div>
      <select class="rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-700 px-3 py-2 text-xs text-slate-600 dark:text-slate-300 focus:outline-none focus:ring-2 focus:ring-primary/40">
        <option>Todos os Status</option>
        <option>Aberto</option>
        <option>Fechado</option>
        <option>Rascunho</option>
      </select>
      <select class="rounded-lg border border-slate-200 dark:border-slate-700 bg-white dark:bg-slate-700 px-3 py-2 text-xs text-slate-600 dark:text-slate-300 focus:outline-none focus:ring-2 focus:ring-primary/40">
        <option>Todas Categorias</option>
        <option>NPS</option>
        <option>CSAT</option>
        <option>CES</option>
        <option>Produto</option>
        <option>Equipe</option>
      </select>
    </div>

    <div class="overflow-x-auto">
      <table class="w-full text-xs">
        <thead class="text-left text-slate-500 dark:text-slate-400 font-semibold border-b border-slate-200 dark:border-slate-700">
          <tr class="text-xs uppercase tracking-wide">
            <th class="py-3 px-1">Título</th>
            <th class="py-3 px-1">Status</th>
            <th class="py-3 px-1">Categoria</th>
            <th class="py-3 px-1">Data</th>
            <th class="py-3 px-1">Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="evaluation in evaluations" :key="evaluation.id" class="border-b border-slate-200 dark:border-slate-700 hover:bg-slate-50 dark:hover:bg-slate-700/50 text-sm">
            <td class="py-4 px-1">
              <p class="font-semibold text-slate-900 dark:text-white">{{ evaluation.title }}</p>
              <p class="text-xs text-slate-500 dark:text-slate-400 mt-0.5">{{ evaluation.responses }} respostas coletadas</p>
            </td>
            <td class="py-4 px-1">
              <span :class="`inline-block px-2.5 py-1 rounded-full text-xs font-semibold ${getStatusColor(evaluation.status)}`">
                {{ evaluation.status }}
              </span>
            </td>
            <td class="py-4 px-1 text-slate-700 dark:text-slate-300">{{ evaluation.category }}</td>
            <td class="py-4 px-1 text-slate-700 dark:text-slate-300">{{ formatDate(evaluation.createdAt) }}</td>
            <td class="py-4 px-1">
              <div class="flex items-center gap-3 text-slate-400 dark:text-slate-500">
                <RouterLink :to="`/evaluation/${evaluation.id}`" class="hover:text-primary dark:hover:text-primary transition" title="Ver detalhes">
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M17 3a2 2 0 00-2-2H5a2 2 0 00-2 2v14l4-3h8a2 2 0 002-2V3z" />
                  </svg>
                </RouterLink>
                <button
                  v-if="evaluation.status === 'Aberto' || evaluation.status === 'open'"
                  @click="openShare(evaluation)"
                  class="hover:text-purple-400 dark:hover:text-purple-400 transition"
                  title="Compartilhar"
                >
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M15 8a3 3 0 10-2.977-2.63l-4.94 2.47a3 3 0 100 4.319l4.94 2.47a3 3 0 10.895-1.789l-4.94-2.47a3.027 3.027 0 000-.74l4.94-2.47C13.456 7.68 14.19 8 15 8z" />
                  </svg>
                </button>
                <RouterLink :to="`/evaluation/${evaluation.id}/results`" class="hover:text-primary dark:hover:text-primary transition" title="Resultados">
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zm6-4a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zm6-3a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z" />
                  </svg>
                </RouterLink>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="flex items-center justify-between text-xs text-slate-500 dark:text-slate-400 mt-6">
      <span>Mostrando 1-5 de 50 avaliações</span>
      <div class="flex items-center gap-2">
        <button class="h-8 w-8 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-primary hover:text-primary dark:hover:border-primary dark:hover:text-primary transition">&lt;</button>
        <button class="h-8 w-8 rounded-lg bg-primary text-white font-semibold">1</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-primary hover:text-primary dark:hover:border-primary dark:hover:text-primary transition">2</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-primary hover:text-primary dark:hover:border-primary dark:hover:text-primary transition">3</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 dark:border-slate-700 hover:border-primary hover:text-primary dark:hover:border-primary dark:hover:text-primary transition">&gt;</button>
      </div>
    </div>
  </div>

  <!-- Share Modal -->
  <ShareModal
    v-if="shareTarget"
    :evaluation-id="String(shareTarget.id)"
    :evaluation-title="shareTarget.title"
    @close="shareTarget = null"
  />
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { RouterLink } from 'vue-router'
import ShareModal from '@/components/modals/ShareModal.vue'
interface Evaluation {
  id: number | string
  title: string
  category: string
  status: string
  responses: number
  createdAt: string
}

const shareTarget = ref<Evaluation | null>(null)
const openShare = (evaluation: Evaluation) => {
  shareTarget.value = evaluation
}

const props = withDefaults(
  defineProps<{
    evaluations?: Evaluation[]
  }>(),
  {
    evaluations: () => []
  }
)

const getStatusColor = (status: string): string => {
  const colors: Record<string, string> = {
    open: 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400',
    closed: 'bg-slate-100 text-slate-800 dark:bg-slate-800 dark:text-slate-300',
    draft: 'bg-yellow-100 text-yellow-800 dark:bg-yellow-900/30 dark:text-yellow-400',
    archived: 'bg-blue-100 text-blue-800 dark:bg-blue-900/30 dark:text-blue-400'
  }
  return colors[status] || colors.draft
}

const formatDate = (date: string): string => {
  return new Date(date).toLocaleDateString('pt-BR', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  })
}
</script>
