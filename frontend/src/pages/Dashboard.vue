<!-- Dashboard Page Component -->
<template>
  <div class="space-y-8">
    <!-- Hero Overview -->
    <section class="rounded-2xl bg-gradient-to-r from-primary via-purple-600 to-accent text-white p-8 shadow-lg">
      <div class="flex flex-col xl:flex-row xl:items-start xl:justify-between gap-8">
        <div class="space-y-2">
          <p class="text-xs uppercase tracking-widest text-purple-200 font-medium">Avaliações Ativas</p>
          <h1 class="text-3xl font-bold mt-2">Acompanhe suas avaliações em andamento</h1>
          <div class="flex flex-wrap gap-3 mt-6">
            <button class="px-4 py-2 rounded-lg bg-secondary text-white font-semibold shadow hover:shadow-md transition">
              + Nova Avaliação
            </button>
            <button class="px-4 py-2 rounded-lg bg-white/10 text-white font-semibold border border-white/20 hover:bg-white/20 transition">
              Ver Todas Pendências
            </button>
          </div>
        </div>

        <div class="flex flex-wrap gap-4">
          <div class="bg-white/10 rounded-xl px-4 py-3 w-48">
            <div class="flex items-center justify-between">
              <p class="text-[11px] uppercase tracking-wide text-purple-200">Prazo mais próximo</p>
              <span class="text-[10px] px-2 py-0.5 rounded-full bg-secondary/80">Urgente</span>
            </div>
            <div class="flex items-center gap-2 mt-3">
              <span class="text-2xl">⏰</span>
              <p class="text-3xl font-bold">3</p>
              <span class="text-lg text-purple-100">dias</span>
            </div>
          </div>
          <div class="bg-white/10 rounded-xl px-4 py-3 w-48">
            <p class="text-[11px] uppercase tracking-wide text-purple-200">Avaliadores ativos</p>
            <div class="flex items-center gap-2 mt-3">
              <span class="text-2xl">👥</span>
              <p class="text-3xl font-bold">47</p>
            </div>
          </div>
          <div class="bg-white/10 rounded-xl px-4 py-3 w-48">
            <p class="text-[11px] uppercase tracking-wide text-purple-200">Taxa de conclusão</p>
            <div class="flex items-center gap-2 mt-3">
              <span class="text-2xl">📊</span>
              <p class="text-3xl font-bold">89%</p>
            </div>
          </div>
        </div>

        <div class="bg-white/15 rounded-xl px-4 py-3 w-28 text-center">
          <p class="text-[11px] uppercase tracking-wide text-purple-200 font-medium">Abertas</p>
          <p class="text-4xl font-bold mt-2">12</p>
        </div>
      </div>
    </section>

    <!-- Category Summary -->
    <section class="space-y-4">
      <div class="flex items-center justify-between px-1">
        <h2 class="text-sm font-bold text-slate-800">Resumo por Categoria</h2>
        <button class="text-xs font-semibold text-primary hover:text-accent">Ver detalhes</button>
      </div>
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-5 gap-4">
        <div
          v-for="category in categoriesSummary"
          :key="category.name"
          class="rounded-2xl border border-slate-150 bg-white p-5 hover:shadow-md transition"
        >
          <div class="flex items-center justify-between mb-3">
            <span :class="`h-9 w-9 rounded-lg flex items-center justify-center ${category.badgeBg}`">
              <span class="text-lg">{{ category.icon }}</span>
            </span>
            <span class="text-2xl font-bold text-slate-800">{{ category.value }}</span>
          </div>
          <p class="text-sm font-bold text-slate-800">{{ category.name }}</p>
          <p class="text-xs text-slate-500 mt-1">{{ category.label }}</p>
        </div>
      </div>
    </section>

    <!-- Main Content -->
    <div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
      <div class="xl:col-span-2">
        <EvaluationList :evaluations="evaluations" />
      </div>
      <div class="space-y-4">
        <AnalyticsChart :data="analyticsData" />
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import EvaluationList from '../components/EvaluationList.vue'
import AnalyticsChart from '../components/AnalyticsChart.vue'

const evaluations = ref([
  {
    id: '1',
    title: 'Pesquisa de Satisfação Q1 2024',
    category: 'NPS',
    status: 'Aberto',
    responses: 23,
    createdAt: '2024-01-15',
  },
  {
    id: '2',
    title: 'Avaliação de Produto - Novo App',
    category: 'Produto',
    status: 'Aberto',
    responses: 156,
    createdAt: '2024-01-12',
  },
  {
    id: '3',
    title: 'CSAT Atendimento ao Cliente',
    category: 'CSAT',
    status: 'Fechado',
    responses: 89,
    createdAt: '2024-01-08',
  },
  {
    id: '4',
    title: 'Feedback Interno - Equipe Dev',
    category: 'Equipe',
    status: 'Aberto',
    responses: 12,
    createdAt: '2024-01-05',
  },
  {
    id: '5',
    title: 'CES - Onboarding Experience',
    category: 'CES',
    status: 'Rascunho',
    responses: 45,
    createdAt: '2024-01-02',
  },
])

const categoriesSummary = ref([
  { name: 'NPS', label: 'Net Promoter Score', value: 8, badgeBg: 'bg-purple-50', iconColor: 'text-purple-600', icon: '★' },
  { name: 'CSAT', label: 'Satisfação do Cliente', value: 15, badgeBg: 'bg-orange-50', iconColor: 'text-orange-600', icon: '☺️' },
  { name: 'CES', label: 'Customer Effort Score', value: 6, badgeBg: 'bg-blue-50', iconColor: 'text-blue-600', icon: '💧' },
  { name: 'Produto', label: 'Avaliações de produto', value: 12, badgeBg: 'bg-emerald-50', iconColor: 'text-emerald-600', icon: '🎁' },
  { name: 'Equipe', label: 'Avaliações internas', value: 9, badgeBg: 'bg-pink-50', iconColor: 'text-pink-600', icon: '👥' },
])

const analyticsData = ref({
  categories: [
    { label: 'NPS', value: 8, color: 'bg-purple-500' },
    { label: 'CSAT', value: 15, color: 'bg-orange-500' },
    { label: 'CES', value: 6, color: 'bg-blue-500' },
    { label: 'Produto', value: 12, color: 'bg-emerald-500' },
    { label: 'Equipe', value: 9, color: 'bg-pink-500' },
  ],
  statuses: [
    { label: 'Aberto', value: 12, total: 50, color: 'bg-emerald-500' },
    { label: 'Fechado', value: 35, total: 50, color: 'bg-slate-500' },
    { label: 'Rascunho', value: 3, total: 50, color: 'bg-amber-400' },
  ],
  insight: 'Suas avaliações NPS estão com 15% mais respostas este mês. Continue engajando seus usuários!'
})

onMounted(async () => {
  // TODO: Fetch dashboard metrics via GraphQL
})
</script>
