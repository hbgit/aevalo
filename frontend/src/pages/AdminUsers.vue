<!-- Admin Users Management Page -->
<template>
  <div class="space-y-6">
    <!-- Page Header -->
    <div class="flex items-center justify-between">
      <div>
        <h1 class="text-3xl font-bold text-slate-900">👥 Gestão de Usuários</h1>
        <p class="text-sm text-slate-600 mt-1">Gerenciar usuários, permissões e assinaturas</p>
      </div>
      <button class="px-4 py-2 rounded-lg bg-primary text-white text-sm font-semibold hover:bg-accent transition">
        + Adicionar Usuário
      </button>
    </div>

    <!-- Search and Filters -->
    <div class="flex flex-col sm:flex-row items-start sm:items-center gap-4">
      <div class="flex-1 relative">
        <input
          type="text"
          placeholder="🔍 Buscar usuários..."
          class="w-full px-4 py-2 rounded-lg border border-slate-200 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary"
        />
      </div>
      <div class="flex items-center gap-2">
        <select class="px-3 py-2 rounded-lg border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40">
          <option>Todos os Planos</option>
          <option>Gratuito</option>
          <option>Pro</option>
          <option>Team</option>
        </select>
        <select class="px-3 py-2 rounded-lg border border-slate-200 text-sm focus:outline-none focus:ring-2 focus:ring-primary/40">
          <option>Status: Todos</option>
          <option>Ativo</option>
          <option>Inativo</option>
          <option>Suspenso</option>
        </select>
      </div>
    </div>

    <!-- Stats Bar -->
    <div class="bg-white rounded-lg border border-slate-200 px-6 py-3 flex items-center justify-between text-sm">
      <span class="text-slate-600">
        <span class="font-bold text-slate-900">1,247</span> usuários · Exibindo 25 por página
      </span>
      <div class="flex items-center gap-2">
        <button class="px-3 py-1 text-xs font-semibold text-slate-600 hover:text-primary transition">
          Exportar CSV
        </button>
        <button class="px-3 py-1 text-xs font-semibold text-slate-600 hover:text-primary transition">
          Enviar Email
        </button>
      </div>
    </div>

    <!-- Users Table -->
    <div class="bg-white rounded-2xl border border-slate-200 overflow-hidden">
      <table class="w-full">
        <thead class="bg-slate-50 border-b border-slate-200">
          <tr>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">
              <input type="checkbox" class="rounded" />
            </th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Nome</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Email</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Plano</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Status</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Membro Desde</th>
            <th class="px-6 py-3 text-left text-xs font-semibold text-slate-700 uppercase tracking-wider">Ações</th>
          </tr>
        </thead>
        <tbody class="divide-y divide-slate-200">
          <!-- User Row 1 -->
          <tr class="hover:bg-slate-50 transition">
            <td class="px-6 py-4">
              <input type="checkbox" class="rounded" />
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-primary to-accent text-white flex items-center justify-center font-bold text-sm">
                  JA
                </div>
                <div>
                  <p class="font-semibold text-slate-900">João Silva</p>
                  <p class="text-xs text-slate-600">usr_123</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">joao@empresa.com</td>
            <td class="px-6 py-4">
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-purple-100 text-purple-800">PRO</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-green-500"></span>
                <span class="text-sm text-slate-600">Ativo</span>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">15 Jan 2026</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <button class="px-2 py-1 text-xs text-primary hover:bg-primary/10 rounded transition">
                  Ver
                </button>
                <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                  Editar
                </button>
                <div class="relative group">
                  <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                    ⋮
                  </button>
                  <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
                    <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                      Suspender
                    </button>
                    <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                      Deletar
                    </button>
                  </div>
                </div>
              </div>
            </td>
          </tr>

          <!-- User Row 2 -->
          <tr class="hover:bg-slate-50 transition">
            <td class="px-6 py-4">
              <input type="checkbox" class="rounded" />
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-secondary to-accent text-white flex items-center justify-center font-bold text-sm">
                  MS
                </div>
                <div>
                  <p class="font-semibold text-slate-900">Maria Santos</p>
                  <p class="text-xs text-slate-600">usr_456</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">maria@empresa.com</td>
            <td class="px-6 py-4">
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-blue-100 text-blue-800">TEAM</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-green-500"></span>
                <span class="text-sm text-slate-600">Ativo</span>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">03 Fev 2026</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <button class="px-2 py-1 text-xs text-primary hover:bg-primary/10 rounded transition">
                  Ver
                </button>
                <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                  Editar
                </button>
                <div class="relative group">
                  <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                    ⋮
                  </button>
                  <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
                    <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                      Suspender
                    </button>
                    <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                      Deletar
                    </button>
                  </div>
                </div>
              </div>
            </td>
          </tr>

          <!-- User Row 3 (Suspended) -->
          <tr class="hover:bg-slate-50 transition opacity-75">
            <td class="px-6 py-4">
              <input type="checkbox" class="rounded" />
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-slate-400 to-slate-500 text-white flex items-center justify-center font-bold text-sm">
                  PC
                </div>
                <div>
                  <p class="font-semibold text-slate-900">Pedro Costa</p>
                  <p class="text-xs text-slate-600">usr_789</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">pedro@empresa.com</td>
            <td class="px-6 py-4">
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-slate-100 text-slate-800">PRO</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-red-500"></span>
                <span class="text-sm text-slate-600">Suspenso</span>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">20 Dez 2025</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <button class="px-2 py-1 text-xs text-primary hover:bg-primary/10 rounded transition">
                  Ver
                </button>
                <button class="px-2 py-1 text-xs text-green-600 hover:bg-green-50 rounded transition">
                  Ativar
                </button>
                <div class="relative group">
                  <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                    ⋮
                  </button>
                  <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
                    <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                      Editar
                    </button>
                    <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                      Deletar
                    </button>
                  </div>
                </div>
              </div>
            </td>
          </tr>

          <!-- User Row 4 -->
          <tr class="hover:bg-slate-50 transition">
            <td class="px-6 py-4">
              <input type="checkbox" class="rounded" />
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-accent to-primary text-white flex items-center justify-center font-bold text-sm">
                  AC
                </div>
                <div>
                  <p class="font-semibold text-slate-900">Ana Costa</p>
                  <p class="text-xs text-slate-600">usr_101</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">ana@empresa.com</td>
            <td class="px-6 py-4">
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-slate-100 text-slate-800">GRÁTIS</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-green-500"></span>
                <span class="text-sm text-slate-600">Ativo</span>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">08 Fev 2026</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <button class="px-2 py-1 text-xs text-primary hover:bg-primary/10 rounded transition">
                  Ver
                </button>
                <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                  Editar
                </button>
                <div class="relative group">
                  <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                    ⋮
                  </button>
                  <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
                    <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                      Suspender
                    </button>
                    <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                      Deletar
                    </button>
                  </div>
                </div>
              </div>
            </td>
          </tr>

          <!-- User Row 5 -->
          <tr class="hover:bg-slate-50 transition">
            <td class="px-6 py-4">
              <input type="checkbox" class="rounded" />
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-3">
                <div class="h-10 w-10 rounded-full bg-gradient-to-br from-primary to-secondary text-white flex items-center justify-center font-bold text-sm">
                  RB
                </div>
                <div>
                  <p class="font-semibold text-slate-900">Roberto Barros</p>
                  <p class="text-xs text-slate-600">usr_202</p>
                </div>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">roberto@empresa.com</td>
            <td class="px-6 py-4">
              <span class="px-2.5 py-1 text-xs font-semibold rounded-full bg-purple-100 text-purple-800">PRO</span>
            </td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <span class="h-2 w-2 rounded-full bg-green-500"></span>
                <span class="text-sm text-slate-600">Ativo</span>
              </div>
            </td>
            <td class="px-6 py-4 text-sm text-slate-600">01 Fev 2026</td>
            <td class="px-6 py-4">
              <div class="flex items-center gap-2">
                <button class="px-2 py-1 text-xs text-primary hover:bg-primary/10 rounded transition">
                  Ver
                </button>
                <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                  Editar
                </button>
                <div class="relative group">
                  <button class="px-2 py-1 text-xs text-slate-600 hover:bg-slate-200 rounded transition">
                    ⋮
                  </button>
                  <div class="hidden group-hover:block absolute right-0 mt-1 bg-white border border-slate-200 rounded-lg shadow-lg z-10">
                    <button class="block w-full text-left px-4 py-2 text-sm text-slate-700 hover:bg-slate-100">
                      Suspender
                    </button>
                    <button class="block w-full text-left px-4 py-2 text-sm text-red-600 hover:bg-red-50 border-t border-slate-200">
                      Deletar
                    </button>
                  </div>
                </div>
              </div>
            </td>
          </tr>
        </tbody>
      </table>
    </div>

    <!-- Pagination -->
    <div class="flex items-center justify-between">
      <span class="text-sm text-slate-600">Mostrando 1-25 de 1,247 usuários</span>
      <div class="flex items-center gap-2">
        <button class="px-3 py-2 rounded-lg border border-slate-200 text-sm font-semibold hover:bg-slate-50 transition disabled:opacity-50" disabled>
          ← Anterior
        </button>
        <div class="flex items-center gap-1">
          <button class="h-8 w-8 rounded-lg bg-primary text-white font-semibold text-sm">1</button>
          <button class="h-8 w-8 rounded-lg border border-slate-200 text-slate-600 font-semibold text-sm hover:bg-slate-50 transition">2</button>
          <button class="h-8 w-8 rounded-lg border border-slate-200 text-slate-600 font-semibold text-sm hover:bg-slate-50 transition">3</button>
          <span class="text-slate-600">...</span>
          <button class="h-8 w-8 rounded-lg border border-slate-200 text-slate-600 font-semibold text-sm hover:bg-slate-50 transition">50</button>
        </div>
        <button class="px-3 py-2 rounded-lg border border-slate-200 text-sm font-semibold hover:bg-slate-50 transition">
          Próxima →
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
// Admin Users Management Logic
</script>

<style scoped>
/* Admin Users specific styles */
</style>
