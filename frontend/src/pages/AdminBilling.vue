<!-- Admin Billing Management Page -->
<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-slate-900">💰 Faturamento e Assinaturas</h1>
        <p class="text-sm text-slate-600 mt-1">Gerenciar pagamentos, planos e receitas</p>
      </div>
      <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
        📊 Relatório Financeiro
      </button>
    </div>

    <!-- Revenue Summary -->
    <div class="grid grid-cols-1 md:grid-cols-4 gap-4">
      <!-- Monthly Recurring Revenue -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Receita Recorrente Mensal</p>
            <p class="text-3xl font-bold text-slate-900">R$ 38.4k</p>
            <p class="text-xs text-slate-600 mt-2">↑ 12% vs mês anterior</p>
          </div>
          <span class="text-2xl">💵</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-5/6 bg-gradient-to-r from-primary to-accent"></div>
        </div>
      </div>

      <!-- Total Customers -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Clientes Pagos</p>
            <p class="text-3xl font-bold text-slate-900">287</p>
            <p class="text-xs text-slate-600 mt-2">↑ 18 esta semana</p>
          </div>
          <span class="text-2xl">👥</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-3/4 bg-gradient-to-r from-accent to-primary"></div>
        </div>
      </div>

      <!-- Churn Rate -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Taxa de Cancelamento</p>
            <p class="text-3xl font-bold text-slate-900">2.1%</p>
            <p class="text-xs text-slate-600 mt-2">↓ 0.3% vs mês</p>
          </div>
          <span class="text-2xl">📉</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-1/4 bg-gradient-to-r from-red-400 to-red-500"></div>
        </div>
      </div>

      <!-- Arr (Annual) -->
      <div class="bg-white rounded-2xl border border-slate-200 p-6 hover:shadow-md transition">
        <div class="flex items-start justify-between">
          <div>
            <p class="text-sm font-semibold text-slate-600 mb-1">Receita Anualizada</p>
            <p class="text-3xl font-bold text-slate-900">R$ 460.8k</p>
            <p class="text-xs text-slate-600 mt-2">↑ 14% vs ano</p>
          </div>
          <span class="text-2xl">📈</span>
        </div>
        <div class="h-1 bg-slate-100 rounded-full overflow-hidden mt-4">
          <div class="h-full w-5/6 bg-gradient-to-r from-secondary to-accent"></div>
        </div>
      </div>
    </div>

    <!-- Plan Distribution Chart -->
    <div class="bg-white rounded-2xl border border-slate-200 p-6">
      <h3 class="text-lg font-bold text-slate-900 mb-4">📊 Distribuição de Planos</h3>
      <div class="grid grid-cols-3 gap-6">
        <!-- Free Plan -->
        <div class="text-center">
          <div class="w-24 h-24 rounded-full bg-gradient-to-br from-slate-100 to-slate-200 flex items-center justify-center mx-auto mb-3">
            <div class="text-center">
              <p class="text-2xl font-bold text-slate-900">960</p>
              <p class="text-xs text-slate-600">Gratuitos</p>
            </div>
          </div>
          <p class="font-semibold text-slate-900">77%</p>
          <p class="text-xs text-slate-600">Plano Gratuito</p>
        </div>

        <!-- Pro Plan -->
        <div class="text-center">
          <div class="w-24 h-24 rounded-full bg-gradient-to-br from-purple-400 to-purple-600 flex items-center justify-center mx-auto mb-3">
            <div class="text-center text-white">
              <p class="text-2xl font-bold">187</p>
              <p class="text-xs">Pro</p>
            </div>
          </div>
          <p class="font-semibold text-slate-900">15%</p>
          <p class="text-xs text-slate-600">Plano Pro</p>
        </div>

        <!-- Team Plan -->
        <div class="text-center">
          <div class="w-24 h-24 rounded-full bg-gradient-to-br from-blue-400 to-blue-600 flex items-center justify-center mx-auto mb-3">
            <div class="text-center text-white">
              <p class="text-2xl font-bold">100</p>
              <p class="text-xs">Team</p>
            </div>
          </div>
          <p class="font-semibold text-slate-900">8%</p>
          <p class="text-xs text-slate-600">Plano Team</p>
        </div>
      </div>
    </div>

    <!-- Recent Transactions -->
    <div class="bg-white rounded-2xl border border-slate-200 p-6">
      <div class="flex items-center justify-between mb-4">
        <h3 class="text-lg font-bold text-slate-900">📋 Últimas Transações</h3>
        <button class="text-sm text-primary hover:underline">Ver Todas →</button>
      </div>

      <div class="space-y-3">
        <!-- Transaction 1 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center">
              <span>✅</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Pagamento Recebido - TechCorp</p>
              <p class="text-xs text-slate-600">PRO Plan · Renovação Anual</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-slate-900">+R$ 588,00</p>
            <p class="text-xs text-slate-600">Hoje, 14:32</p>
          </div>
        </div>

        <!-- Transaction 2 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center">
              <span>✅</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Pagamento Recebido - StartupXYZ</p>
              <p class="text-xs text-slate-600">TEAM Plan · Renovação Mensal</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-slate-900">+R$ 149,00</p>
            <p class="text-xs text-slate-600">Hoje, 10:15</p>
          </div>
        </div>

        <!-- Transaction 3 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-yellow-100 flex items-center justify-center">
              <span>⏳</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Pagamento Pendente - DataCorp</p>
              <p class="text-xs text-slate-600">TEAM Plan · Vencimento 15 Fev</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-yellow-700">R$ 299,00</p>
            <p class="text-xs text-slate-600">Ontem, 09:00</p>
          </div>
        </div>

        <!-- Transaction 4 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center">
              <span>✅</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Pagamento Recebido - Empresa ABC</p>
              <p class="text-xs text-slate-600">PRO Plan · Atualização Upgrade</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-slate-900">+R$ 49,00</p>
            <p class="text-xs text-slate-600">12 Fev 2026</p>
          </div>
        </div>

        <!-- Transaction 5 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-red-100 flex items-center justify-center">
              <span>❌</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Cancelamento de Assinatura - ClienteX</p>
              <p class="text-xs text-slate-600">TEAM Plan · Pedido de Cancelamento</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-red-600">-R$ 149,00</p>
            <p class="text-xs text-slate-600">10 Fev 2026</p>
          </div>
        </div>

        <!-- Transaction 6 -->
        <div class="flex items-center justify-between py-3 border-b border-slate-200 last:border-b-0">
          <div class="flex items-center gap-4 flex-1">
            <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center">
              <span>✅</span>
            </div>
            <div class="flex-1">
              <p class="font-semibold text-slate-900">Pagamento Recebido - AgenciaZ</p>
              <p class="text-xs text-slate-600">PRO Plan · Renovação Mensal</p>
            </div>
          </div>
          <div class="text-right">
            <p class="font-bold text-slate-900">+R$ 49,00</p>
            <p class="text-xs text-slate-600">09 Fev 2026</p>
          </div>
        </div>
      </div>
    </div>

    <!-- Active Subscriptions by Plan -->
    <div class="bg-white rounded-2xl border border-slate-200 p-6">
      <h3 class="text-lg font-bold text-slate-900 mb-4">🎯 Assinaturas Ativas</h3>

      <div class="space-y-4">
        <!-- Free Plan Row -->
        <div class="flex items-center justify-between p-4 bg-slate-50 rounded-lg">
          <div class="flex-1">
            <p class="font-semibold text-slate-900">Plano Gratuito</p>
            <p class="text-xs text-slate-600">Sem limite de assinaturas</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">960</p>
            <p class="text-xs text-slate-600">assinaturas</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">R$ 0</p>
            <p class="text-xs text-slate-600">receita</p>
          </div>
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            Gerenciar
          </button>
        </div>

        <!-- Pro Plan Row -->
        <div class="flex items-center justify-between p-4 bg-purple-50 rounded-lg border border-purple-200">
          <div class="flex-1">
            <p class="font-semibold text-slate-900">Plano Pro</p>
            <p class="text-xs text-slate-600">R$ 49/mês · Faturamento mensal</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">187</p>
            <p class="text-xs text-slate-600">assinaturas</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">R$ 9.2k</p>
            <p class="text-xs text-slate-600">receita/mês</p>
          </div>
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            Gerenciar
          </button>
        </div>

        <!-- Team Plan Row -->
        <div class="flex items-center justify-between p-4 bg-blue-50 rounded-lg border border-blue-200">
          <div class="flex-1">
            <p class="font-semibold text-slate-900">Plano Team</p>
            <p class="text-xs text-slate-600">R$ 149/mês · Faturamento mensal</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">100</p>
            <p class="text-xs text-slate-600">assinaturas</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">R$ 14.9k</p>
            <p class="text-xs text-slate-600">receita/mês</p>
          </div>
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            Gerenciar
          </button>
        </div>

        <!-- Team Annual Plan Row -->
        <div class="flex items-center justify-between p-4 bg-accent/10 rounded-lg border border-secondary/50">
          <div class="flex-1">
            <p class="font-semibold text-slate-900">Plano Team (Anual)</p>
            <p class="text-xs text-slate-600">R$ 1.790/ano · Faturamento anual (com desconto)</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">28</p>
            <p class="text-xs text-slate-600">assinaturas</p>
          </div>
          <div class="text-center">
            <p class="text-2xl font-bold text-slate-900">R$ 14.2k</p>
            <p class="text-xs text-slate-600">receita/ano</p>
          </div>
          <button class="px-4 py-2 text-sm font-semibold text-primary hover:bg-primary/10 rounded transition">
            Gerenciar
          </button>
        </div>
      </div>
    </div>

    <!-- Failed Payments Alert -->
    <div class="bg-red-50 border border-red-200 rounded-2xl p-6">
      <div class="flex items-start gap-4">
        <div class="text-2xl">⚠️</div>
        <div class="flex-1">
          <h4 class="font-bold text-red-900 mb-2">Pagamentos Falhados</h4>
          <p class="text-sm text-red-800 mb-3">3 pagamentos falharam e precisam de ação imediata</p>
          <button class="px-4 py-2 text-sm font-semibold rounded bg-red-600 text-white hover:bg-red-700 transition">
            Ver Detalhes
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Admin Billing Management Logic
</script>

<style scoped>
/* Admin Billing specific styles */
</style>
