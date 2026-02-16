<!-- Admin Dashboard - Area Administrativa -->
<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-slate-900">👑 Admin Dashboard</h1>
        <p class="text-sm text-slate-600 mt-1">Monitoramento do sistema e métricas de negócio</p>
      </div>
      <div class="flex items-center gap-3">
        <button class="px-4 py-2 rounded-lg border border-slate-200 text-sm font-semibold hover:bg-slate-50 transition">
          🔄 Atualizar
        </button>
        <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
          ⚙️ Configurações
        </button>
      </div>
    </div>

    <!-- System Health Section -->
    <section class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-bold text-slate-900">System Health</h2>
        <div class="flex items-center gap-2 text-sm text-green-600">
          <span class="h-2 w-2 rounded-full bg-green-500 animate-pulse"></span>
          Todos os sistemas operacionais
        </div>
      </div>

      <!-- Health Cards Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Uptime Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Uptime</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">99.98%</p>
            </div>
            <span class="text-2xl">📈</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12 7a1 1 0 110-2h.01a1 1 0 110 2H12zm-4 3a1 1 0 100-2 1 1 0 000 2zm5.465-4.066l-1.414 1.414A2 2 0 1010 8.172V5a1 1 0 112 0v3.172a2 2 0 01.535 1.414l1.414-1.414a1 1 0 111.414 1.414l-2 2a1 1 0 01-1.414 0l-2-2a1 1 0 111.414-1.414z" clip-rule="evenodd" />
            </svg>
            <span>Estável</span>
          </div>
        </div>

        <!-- Requests Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Requests/dia</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">1.2M</p>
            </div>
            <span class="text-2xl">📊</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M13 6a3 3 0 11-6 0 3 3 0 016 0zM18 8a2 2 0 11-4 0 2 2 0 014 0zM14 15a4 4 0 00-8 0v3h8v-3zM6 8a2 2 0 11-4 0 2 2 0 014 0zM16 18v-3a5.972 5.972 0 00-.75-2.906A3.005 3.005 0 0119 15v3h-3zM4.75 12.094A5.973 5.973 0 004 15v3H1v-3a3 3 0 013.75-2.906z" />
            </svg>
            <span>↑ 12%</span>
          </div>
        </div>

        <!-- Errors Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Error Rate</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">0.02%</p>
            </div>
            <span class="text-2xl">✅</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12 7a1 1 0 110-2h.01a1 1 0 110 2H12zm-4 3a1 1 0 100-2 1 1 0 000 2zm5.465-4.066l-1.414 1.414A2 2 0 1010 8.172V5a1 1 0 112 0v3.172a2 2 0 01.535 1.414l1.414-1.414a1 1 0 111.414 1.414l-2 2a1 1 0 01-1.414 0l-2-2a1 1 0 111.414-1.414z" clip-rule="evenodd" />
            </svg>
            <span>↓ 0.5%</span>
          </div>
        </div>

        <!-- Latency Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Latência (avg)</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">89ms</p>
            </div>
            <span class="text-2xl">⚡</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M12 7a1 1 0 110-2h.01a1 1 0 110 2H12zm-4 3a1 1 0 100-2 1 1 0 000 2zm5.465-4.066l-1.414 1.414A2 2 0 1010 8.172V5a1 1 0 112 0v3.172a2 2 0 01.535 1.414l1.414-1.414a1 1 0 111.414 1.414l-2 2a1 1 0 01-1.414 0l-2-2a1 1 0 111.414-1.414z" clip-rule="evenodd" />
            </svg>
            <span>↓ 5ms</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Business Metrics Section -->
    <section class="space-y-4">
      <h2 class="text-lg font-bold text-slate-900">Métricas de Negócio (Últimos 30 dias)</h2>

      <!-- Metrics Cards Grid -->
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <!-- Usuários Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Usuários</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">1,247</p>
            </div>
            <span class="text-2xl">👥</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z" />
            </svg>
            <span>↑ 15%</span>
          </div>
          <div class="w-full h-1 bg-slate-100 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-primary to-accent" style="width: 85%"></div>
          </div>
        </div>

        <!-- Avaliações Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Avaliações</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">3,891</p>
            </div>
            <span class="text-2xl">📋</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z" />
            </svg>
            <span>↑ 23%</span>
          </div>
          <div class="w-full h-1 bg-slate-100 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-accent to-primary" style="width: 92%"></div>
          </div>
        </div>

        <!-- Respostas Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">Respostas</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">47,832</p>
            </div>
            <span class="text-2xl">📊</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z" />
            </svg>
            <span>↑ 31%</span>
          </div>
          <div class="w-full h-1 bg-slate-100 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-primary to-secondary" style="width: 88%"></div>
          </div>
        </div>

        <!-- MRR Card -->
        <div class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
          <div class="flex items-start justify-between">
            <div>
              <p class="text-sm text-slate-600">MRR (Revenue)</p>
              <p class="text-3xl font-bold text-slate-900 mt-1">R$ 12.3k</p>
            </div>
            <span class="text-2xl">💰</span>
          </div>
          <div class="flex items-center gap-2 text-green-600 text-sm">
            <svg class="h-4 w-4" fill="currentColor" viewBox="0 0 20 20">
              <path d="M2 11a1 1 0 011-1h2a1 1 0 011 1v5a1 1 0 01-1 1H3a1 1 0 01-1-1v-5zM8 7a1 1 0 011-1h2a1 1 0 011 1v9a1 1 0 01-1 1H9a1 1 0 01-1-1V7zM14 4a1 1 0 011-1h2a1 1 0 011 1v12a1 1 0 01-1 1h-2a1 1 0 01-1-1V4z" />
            </svg>
            <span>↑ 18%</span>
          </div>
          <div class="w-full h-1 bg-slate-100 rounded-full overflow-hidden">
            <div class="h-full bg-gradient-to-r from-green-500 to-emerald-500" style="width: 75%"></div>
          </div>
        </div>
      </div>
    </section>

    <!-- Growth Chart -->
    <section class="bg-white rounded-2xl border border-slate-200 p-6 space-y-4">
      <h3 class="font-bold text-slate-900">Crescimento de Usuários (30 dias)</h3>
      <!-- Mock Chart -->
      <div class="flex items-end justify-between gap-2 h-48">
        <div v-for="i in 30" :key="i" class="flex-1 bg-slate-100 rounded-t-lg hover:bg-primary/20 transition" :style="`height: ${Math.random() * 100 + 30}%`"></div>
      </div>
      <div class="flex justify-between text-xs text-slate-600">
        <span>10 dias atrás</span>
        <span>Hoje</span>
      </div>
    </section>

    <!-- Latest Activities Section -->
    <section class="space-y-4">
      <div class="flex items-center justify-between">
        <h2 class="text-lg font-bold text-slate-900">Últimas Atividades</h2>
        <RouterLink to="/admin/activity-log" class="text-sm font-semibold text-primary hover:text-accent transition">
          Ver todas →
        </RouterLink>
      </div>

      <div class="space-y-3">
        <!-- Activity Item 1: New User -->
        <div class="bg-white rounded-xl border border-slate-200 p-4 space-y-2">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center text-lg">
                🟢
              </div>
              <div class="flex-1">
                <p class="font-semibold text-slate-900">Novo usuário registrado</p>
                <p class="text-sm text-slate-600">ana@techcorp.com · PRO Plan</p>
              </div>
            </div>
            <span class="text-xs text-slate-500">há 5 min</span>
          </div>
          <div class="flex items-center gap-2 ml-13">
            <button class="px-3 py-1 text-xs rounded-lg border border-slate-200 hover:bg-slate-50 transition">
              Ver Perfil
            </button>
          </div>
        </div>

        <!-- Activity Item 2: Error -->
        <div class="bg-white rounded-xl border border-slate-200 p-4 space-y-2">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <div class="h-10 w-10 rounded-full bg-red-100 flex items-center justify-center text-lg">
                🔴
              </div>
              <div class="flex-1">
                <p class="font-semibold text-slate-900">Erro crítico detectado</p>
                <p class="text-sm text-slate-600">Gemini API timeout · Server 3</p>
              </div>
            </div>
            <span class="text-xs text-slate-500">há 12 min</span>
          </div>
          <div class="flex items-center gap-2 ml-13">
            <button class="px-3 py-1 text-xs rounded-lg border border-red-200 text-red-600 hover:bg-red-50 transition">
              Ver Detalhes
            </button>
          </div>
        </div>

        <!-- Activity Item 3: Payment -->
        <div class="bg-white rounded-xl border border-slate-200 p-4 space-y-2">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <div class="h-10 w-10 rounded-full bg-green-100 flex items-center justify-center text-lg">
                🟢
              </div>
              <div class="flex-1">
                <p class="font-semibold text-slate-900">Pagamento recebido</p>
                <p class="text-sm text-slate-600">R$ 149,00 · TEAM Plan · TechCorp</p>
              </div>
            </div>
            <span class="text-xs text-slate-500">há 1h</span>
          </div>
        </div>

        <!-- Activity Item 4: Maintenance -->
        <div class="bg-white rounded-xl border border-slate-200 p-4 space-y-2">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <div class="h-10 w-10 rounded-full bg-blue-100 flex items-center justify-center text-lg">
                🔧
              </div>
              <div class="flex-1">
                <p class="font-semibold text-slate-900">Manutenção programada</p>
                <p class="text-sm text-slate-600">Upgrade de infraestrutura · Planejado para 15 Fev</p>
              </div>
            </div>
            <span class="text-xs text-slate-500">há 2h</span>
          </div>
          <div class="flex items-center gap-2 ml-13">
            <button class="px-3 py-1 text-xs rounded-lg border border-slate-200 hover:bg-slate-50 transition">
              Adicionar Calendário
            </button>
          </div>
        </div>

        <!-- Activity Item 5: User Action -->
        <div class="bg-white rounded-xl border border-slate-200 p-4 space-y-2">
          <div class="flex items-start justify-between">
            <div class="flex items-start gap-3 flex-1">
              <div class="h-10 w-10 rounded-full bg-purple-100 flex items-center justify-center text-lg">
                ✨
              </div>
              <div class="flex-1">
                <p class="font-semibold text-slate-900">Limite de respostas atingido</p>
                <p class="text-sm text-slate-600">Pesquisa "NPS Q1 2026" atingiu 1000 respostas</p>
              </div>
            </div>
            <span class="text-xs text-slate-500">ontem</span>
          </div>
        </div>
      </div>
    </section>

    <!-- Navigation to Other Admin Pages -->
    <section class="bg-slate-50 rounded-2xl border border-slate-200 p-6 space-y-4">
      <h3 class="font-bold text-slate-900">Seções Administrativas</h3>
      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4">
        <RouterLink to="/admin/users" class="p-4 rounded-lg bg-white border border-slate-200 hover:border-primary hover:shadow-md transition text-center space-y-2">
          <div class="text-2xl">👥</div>
          <p class="font-semibold text-slate-900">Usuários</p>
          <p class="text-xs text-slate-600">1,247 ativos</p>
        </RouterLink>

        <RouterLink to="/admin/evaluations" class="p-4 rounded-lg bg-white border border-slate-200 hover:border-primary hover:shadow-md transition text-center space-y-2">
          <div class="text-2xl">📋</div>
          <p class="font-semibold text-slate-900">Avaliações</p>
          <p class="text-xs text-slate-600">3,891 total</p>
        </RouterLink>

        <RouterLink to="/admin/billing" class="p-4 rounded-lg bg-white border border-slate-200 hover:border-primary hover:shadow-md transition text-center space-y-2">
          <div class="text-2xl">💰</div>
          <p class="font-semibold text-slate-900">Faturamento</p>
          <p class="text-xs text-slate-600">R$ 12.3k MRR</p>
        </RouterLink>

        <RouterLink to="/admin/settings" class="p-4 rounded-lg bg-white border border-slate-200 hover:border-primary hover:shadow-md transition text-center space-y-2">
          <div class="text-2xl">⚙️</div>
          <p class="font-semibold text-slate-900">Configurações</p>
          <p class="text-xs text-slate-600">Sistema</p>
        </RouterLink>
      </div>
    </section>
  </div>
</template>

<script setup lang="ts">
import { RouterLink } from 'vue-router'
</script>

<style scoped>
/* Admin-specific styles */
</style>
