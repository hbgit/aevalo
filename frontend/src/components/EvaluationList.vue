<!-- Evaluation List Component -->
<template>
  <div class="evaluation-list rounded-2xl border border-slate-200 bg-white p-6">
    <div class="flex flex-col lg:flex-row lg:items-center lg:justify-between gap-4 mb-6">
      <div>
        <h2 class="text-lg font-bold text-slate-900">Todas as Avaliações</h2>
        <p class="text-xs text-slate-500 mt-1">Mostrando {{ evaluations.length }} avaliações</p>
      </div>
      <button class="px-3 py-2 rounded-lg border border-slate-200 text-xs font-semibold text-slate-600 hover:border-primary hover:text-primary transition">
        Exportar CSV
      </button>
    </div>

    <div class="flex flex-col lg:flex-row gap-4 mb-5">
      <div class="flex-1 relative">
        <input
          type="text"
          placeholder="Buscar: category:NPS status:open"
          class="w-full rounded-lg border border-slate-200 bg-slate-50 px-3 py-2 pl-10 text-xs text-slate-700 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary"
        />
        <svg class="absolute left-3 top-2.5 h-4 w-4 text-slate-400" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z"></path>
        </svg>
      </div>
      <select class="rounded-lg border border-slate-200 bg-white px-3 py-2 text-xs text-slate-600">
        <option>Todos os Status</option>
        <option>Aberto</option>
        <option>Fechado</option>
        <option>Rascunho</option>
      </select>
      <select class="rounded-lg border border-slate-200 bg-white px-3 py-2 text-xs text-slate-600">
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
        <thead class="text-left text-slate-500 font-semibold border-b border-slate-200">
          <tr class="text-xs uppercase tracking-wide">
            <th class="py-3 px-1">Título</th>
            <th class="py-3 px-1">Status</th>
            <th class="py-3 px-1">Categoria</th>
            <th class="py-3 px-1">Data</th>
            <th class="py-3 px-1">Ações</th>
          </tr>
        </thead>
        <tbody>
          <tr v-for="evaluation in evaluations" :key="evaluation.id" class="border-b hover:bg-slate-50 text-sm">
            <td class="py-4 px-1">
              <p class="font-semibold text-slate-900">{{ evaluation.title }}</p>
              <p class="text-xs text-slate-500 mt-0.5">{{ evaluation.responses }} respostas coletadas</p>
            </td>
            <td class="py-4 px-1">
              <span :class="`inline-block px-2.5 py-1 rounded-full text-xs font-semibold ${getStatusColor(evaluation.status)}`">
                {{ evaluation.status }}
              </span>
            </td>
            <td class="py-4 px-1 text-slate-700">{{ evaluation.category }}</td>
            <td class="py-4 px-1 text-slate-700">{{ formatDate(evaluation.createdAt) }}</td>
            <td class="py-4 px-1">
              <div class="flex items-center gap-3 text-slate-400">
                <RouterLink :to="`/evaluation/${evaluation.id}`" class="hover:text-primary transition">
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M17 3a2 2 0 00-2-2H5a2 2 0 00-2 2v14l4-3h8a2 2 0 002-2V3z" />
                  </svg>
                </RouterLink>
                <button class="hover:text-primary transition">
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M2.003 9.25l7.071-7.071a3 3 0 114.242 4.243L6.243 13.493 2 14l.503-4.75z" />
                    <path d="M12 5l3 3" />
                  </svg>
                </button>
                <button class="hover:text-primary transition">
                  <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                    <path d="M6 10a2 2 0 114 0 2 2 0 01-4 0zm6-2a2 2 0 100 4 2 2 0 000-4zm-2 2a2 2 0 114 0 2 2 0 01-4 0z" />
                  </svg>
                </button>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <div class="flex items-center justify-between text-xs text-slate-500 mt-6">
      <span>Mostrando 1-5 de 50 avaliações</span>
      <div class="flex items-center gap-2">
        <button class="h-8 w-8 rounded-lg border border-slate-200 hover:border-primary hover:text-primary transition">&lt;</button>
        <button class="h-8 w-8 rounded-lg bg-primary text-white font-semibold">1</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 hover:border-primary hover:text-primary transition">2</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 hover:border-primary hover:text-primary transition">3</button>
        <button class="h-8 w-8 rounded-lg border border-slate-200 hover:border-primary hover:text-primary transition">&gt;</button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'

interface Evaluation {
  id: string
  title: string
  category: string
  status: string
  responses: number
  createdAt: string
}

defineProps<{
  evaluations: Evaluation[]
}>()

const getStatusColor = (status: string) => {
  const colors: Record<string, string> = {
    'Aberto': 'bg-emerald-100 text-emerald-700',
    'Rascunho': 'bg-amber-100 text-amber-700',
    'Fechado': 'bg-slate-100 text-slate-600',
  }
  return colors[status] || 'bg-slate-100 text-slate-600'
}

const formatDate = (date: string) => {
  return new Date(date).toLocaleDateString('pt-BR')
}
</script>
