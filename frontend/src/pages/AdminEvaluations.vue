<!-- Admin Evaluations Management Page -->
<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-slate-900">📋 Gestão de Avaliações</h1>
        <p class="text-sm text-slate-600 mt-1">Monitor e controle todas as avaliações da plataforma</p>
      </div>
      <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
        + Criar Avaliação
      </button>
    </div>

    <!-- Stats Overview -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <!-- Active Evaluations -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Ativas</p>
            <p class="text-3xl font-bold text-slate-900">127</p>
            <p class="text-xs text-slate-600 mt-2">↑ 8 esta semana</p>
          </div>
          <span class="text-2xl">📊</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-4/5 bg-gradient-to-r from-primary to-accent"></div>
        </div>
      </div>

      <!-- Completed -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Concluídas</p>
            <p class="text-3xl font-bold text-slate-900">3,764</p>
            <p class="text-xs text-slate-600 mt-2">↑ 23 hoje</p>
          </div>
          <span class="text-2xl">✅</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-full bg-gradient-to-r from-accent to-primary"></div>
        </div>
      </div>

      <!-- Avg Responses -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Resposta Média</p>
            <p class="text-3xl font-bold text-slate-900">12.7</p>
            <p class="text-xs text-slate-600 mt-2">↓ 0.5 vs semana</p>
          </div>
          <span class="text-2xl">📈</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-3/4 bg-gradient-to-r from-primary to-secondary"></div>
        </div>
      </div>

      <!-- Completion Rate -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Taxa Conclusão</p>
            <p class="text-3xl font-bold text-slate-900">87%</p>
            <p class="text-xs text-slate-600 mt-2">↑ 5% vs mês</p>
          </div>
          <span class="text-2xl">🎯</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-11/12 bg-gradient-to-r from-accent to-secondary"></div>
        </div>
      </div>
    </div>

    <!-- Search and Filters -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
      <div class="flex-1 relative">
        <input
          type="text"
          placeholder="🔍 Buscar avaliações..."
          class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary"
        />
      </div>
      <div class="flex items-center gap-2">
        <select class="px-3 py-2 rounded-lg border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40">
          <option>Status: Todos</option>
          <option>Ativo</option>
          <option>Pausado</option>
          <option>Concluído</option>
          <option>Rascunho</option>
        </select>
        <select class="px-3 py-2 rounded-lg border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40">
          <option>Tipo: Todos</option>
          <option>NPS</option>
          <option>CES</option>
          <option>CSAT</option>
          <option>Customizada</option>
        </select>
        <select class="px-3 py-2 rounded-lg border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40">
          <option>Ordenar: Recente</option>
          <option>Mais Popular</option>
          <option>Taxa Resposta</option>
          <option>Mais Antigo</option>
        </select>
      </div>
    </div>

    <!-- Evaluations List -->
    <div class="space-y-3">
      <!-- Evaluation Card 1 -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <h3 class="text-lg font-bold text-slate-900">NPS Q1 2026 - TechCorp</h3>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800">ATIVO</span>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-purple-100 text-purple-800">NPS</span>
            </div>
            <p class="text-sm text-slate-600">Avaliação trimestral de satisfação do cliente</p>
            <p class="text-xs text-slate-600 mt-1">Criado por <span class="font-semibold">João Silva</span> • 15 Jan 2026</p>
          </div>
          <div class="relative group">
            <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
              ⋮
            </button>
            <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                Ver Resultados
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Editar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Pausar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                Arquivar
              </button>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4 text-center">
          <div>
            <p class="text-2xl font-bold text-slate-900">1,000</p>
            <p class="text-xs text-slate-600">Respostas</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">78</p>
            <p class="text-xs text-slate-600">NPS Score</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">92%</p>
            <p class="text-xs text-slate-600">Taxa Resposta</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">2.3d</p>
            <p class="text-xs text-slate-600">Tempo Médio</p>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-4 pt-4 border-t border-slate-200">
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            📊 Ver Resultados
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            🔗 Compartilhar
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            📥 Exportar
          </button>
        </div>
      </div>

      <!-- Evaluation Card 2 -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <h3 class="text-lg font-bold text-slate-900">CES - Processo de Checkout</h3>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800">ATIVO</span>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-blue-100 text-blue-800">CES</span>
            </div>
            <p class="text-sm text-slate-600">Avaliação da facilidade de conclusão do checkout</p>
            <p class="text-xs text-slate-600 mt-1">Criado por <span class="font-semibold">Maria Santos</span> • 08 Feb 2026</p>
          </div>
          <div class="relative group">
            <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
              ⋮
            </button>
            <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                Ver Resultados
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Editar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Pausar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                Arquivar
              </button>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4 text-center">
          <div>
            <p class="text-2xl font-bold text-slate-900">487</p>
            <p class="text-xs text-slate-600">Respostas</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">4.1</p>
            <p class="text-xs text-slate-600">Facilidade (1-5)</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">78%</p>
            <p class="text-xs text-slate-600">Taxa Resposta</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">48h</p>
            <p class="text-xs text-slate-600">Tempo Médio</p>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-4 pt-4 border-t border-slate-200">
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            📊 Ver Resultados
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            🔗 Compartilhar
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            📥 Exportar
          </button>
        </div>
      </div>

      <!-- Evaluation Card 3 (Paused) -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition opacity-75">
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <h3 class="text-lg font-bold text-slate-900">CSAT - Atendimento ao Cliente</h3>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-yellow-100 text-yellow-800">PAUSADO</span>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-orange-100 text-orange-800">CSAT</span>
            </div>
            <p class="text-sm text-slate-600">Avaliação da satisfação com o suporte técnico</p>
            <p class="text-xs text-slate-600 mt-1">Criado por <span class="font-semibold">Pedro Costa</span> • 22 Dec 2025</p>
          </div>
          <div class="relative group">
            <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
              ⋮
            </button>
            <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                Ver Resultados
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Editar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Retomar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                Arquivar
              </button>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4 text-center">
          <div>
            <p class="text-2xl font-bold text-slate-900">312</p>
            <p class="text-xs text-slate-600">Respostas</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">3.8</p>
            <p class="text-xs text-slate-600">Satisfação (1-5)</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">62%</p>
            <p class="text-xs text-slate-600">Taxa Resposta</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">5.2d</p>
            <p class="text-xs text-slate-600">Tempo Médio</p>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-4 pt-4 border-t border-slate-200">
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            📊 Ver Resultados
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            🔗 Compartilhar
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            📥 Exportar
          </button>
        </div>
      </div>

      <!-- Evaluation Card 4 -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1">
            <div class="flex items-center gap-3 mb-2">
              <h3 class="text-lg font-bold text-slate-900">Pesquisa Customizada - Feedback Novo Produto</h3>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-green-100 text-green-800">ATIVO</span>
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-slate-100 text-slate-800">CUSTOMIZADO</span>
            </div>
            <p class="text-sm text-slate-600">Avaliação de feedback do novo produto lançado</p>
            <p class="text-xs text-slate-600 mt-1">Criado por <span class="font-semibold">Ana Costa</span> • 10 Feb 2026</p>
          </div>
          <div class="relative group">
            <button class="px-3 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
              ⋮
            </button>
            <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                Ver Resultados
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Editar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100 border-t border-slate-200">
                Pausar
              </button>
              <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                Arquivar
              </button>
            </div>
          </div>
        </div>

        <div class="grid grid-cols-4 gap-4 text-center">
          <div>
            <p class="text-2xl font-bold text-slate-900">156</p>
            <p class="text-xs text-slate-600">Respostas</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">4.3</p>
            <p class="text-xs text-slate-600">Satisfação (1-5)</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">85%</p>
            <p class="text-xs text-slate-600">Taxa Resposta</p>
          </div>
          <div>
            <p class="text-2xl font-bold text-slate-900">24h</p>
            <p class="text-xs text-slate-600">Tempo Médio</p>
          </div>
        </div>

        <div class="flex items-center gap-4 mt-4 pt-4 border-t border-slate-200">
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            📊 Ver Resultados
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            🔗 Compartilhar
          </button>
          <button class="px-4 py-2 text-sm font-semibold text-slate-600 hover:bg-slate-200 rounded transition">
            📥 Exportar
          </button>
        </div>
      </div>
    </div>

    <!-- Pagination -->
    <div class="flex items-center justify-between">
      <span class="text-sm text-slate-600">Mostrando 1-4 de 127 avaliações ativas</span>
      <div class="flex items-center gap-2">
        <button class="px-3 py-2 rounded-lg border border-slate-200 text-sm font-semibold hover:bg-slate-50 transition disabled:opacity-50" disabled>
          ← Anterior
        </button>
        <div class="flex items-center gap-1">
          <button class="h-8 w-8 rounded-lg bg-primary text-white font-semibold text-sm">1</button>
          <button class="h-8 w-8 rounded-lg border border-slate-200 text-slate-600 font-semibold text-sm hover:bg-slate-50 transition">2</button>
          <button class="h-8 w-8 rounded-lg border border-slate-200 text-slate-600 font-semibold text-sm hover:bg-slate-50 transition">3</button>
        </div>
        <button class="px-3 py-2 rounded-lg border border-slate-200 text-sm font-semibold hover:bg-slate-50 transition">
          Próxima →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Admin Evaluations Management Logic
</script>

<style scoped>
/* Admin Evaluations specific styles */
</style>
