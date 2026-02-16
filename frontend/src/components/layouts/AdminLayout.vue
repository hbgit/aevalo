<!-- Admin Layout Component -->
<template>
  <div class="flex h-screen bg-slate-100">
    <!-- Admin Sidebar -->
    <div class="w-64 bg-white border-r border-slate-200 flex flex-col overflow-y-auto">
      <!-- Logo -->
      <div class="px-6 py-6 border-b border-slate-200">
        <div class="flex items-center gap-2 mb-1">
          <span class="text-2xl">👑</span>
          <h1 class="text-xl font-bold text-slate-900">Admin</h1>
        </div>
        <p class="text-xs text-slate-600">Aevalo Platform</p>
      </div>

      <!-- Navigation -->
      <nav class="flex-1 px-3 py-6 space-y-2">
        <!-- Dashboard -->
        <router-link
          to="/admin"
          :class="[
            'px-4 py-3 rounded-lg font-semibold text-sm transition flex items-center gap-3',
            isActive('/admin')
              ? 'bg-primary text-white'
              : 'text-slate-700 hover:bg-slate-100'
          ]"
        >
          <span>📊</span>
          <span>Dashboard</span>
        </router-link>

        <!-- Users -->
        <router-link
          to="/admin/users"
          :class="[
            'px-4 py-3 rounded-lg font-semibold text-sm transition flex items-center gap-3',
            isActive('/admin/users')
              ? 'bg-primary text-white'
              : 'text-slate-700 hover:bg-slate-100'
          ]"
        >
          <span>👥</span>
          <span>Usuários</span>
          <span class="ml-auto text-xs bg-slate-200 text-slate-900 px-2 py-0.5 rounded-full">1.2k</span>
        </router-link>

        <!-- Evaluations -->
        <router-link
          to="/admin/evaluations"
          :class="[
            'px-4 py-3 rounded-lg font-semibold text-sm transition flex items-center gap-3',
            isActive('/admin/evaluations')
              ? 'bg-primary text-white'
              : 'text-slate-700 hover:bg-slate-100'
          ]"
        >
          <span>📋</span>
          <span>Avaliações</span>
          <span class="ml-auto text-xs bg-slate-200 text-slate-900 px-2 py-0.5 rounded-full">127</span>
        </router-link>

        <!-- Billing -->
        <router-link
          to="/admin/billing"
          :class="[
            'px-4 py-3 rounded-lg font-semibold text-sm transition flex items-center gap-3',
            isActive('/admin/billing')
              ? 'bg-primary text-white'
              : 'text-slate-700 hover:bg-slate-100'
          ]"
        >
          <span>💰</span>
          <span>Faturamento</span>
        </router-link>

        <!-- Settings -->
        <router-link
          to="/admin/settings"
          :class="[
            'px-4 py-3 rounded-lg font-semibold text-sm transition flex items-center gap-3',
            isActive('/admin/settings')
              ? 'bg-primary text-white'
              : 'text-slate-700 hover:bg-slate-100'
          ]"
        >
          <span>⚙️</span>
          <span>Configurações</span>
        </router-link>

        <!-- Divider -->
        <div class="my-4 border-t border-slate-200"></div>

        <!-- Back to Dashboard -->
        <router-link
          to="/dashboard"
          class="px-4 py-3 rounded-lg font-semibold text-sm transition text-slate-700 hover:bg-slate-100 flex items-center gap-3"
        >
          <span>🏠</span>
          <span>Dashboard Principal</span>
        </router-link>
      </nav>

      <!-- User Info -->
      <div class="border-t border-slate-200 px-4 py-4 space-y-3">
        <div class="flex items-center gap-3">
          <div class="h-10 w-10 rounded-full bg-gradient-to-br from-primary to-accent text-white flex items-center justify-center font-bold text-sm">
            AD
          </div>
          <div class="flex-1 min-w-0">
            <p class="text-sm font-semibold text-slate-900 truncate">Admin User</p>
            <p class="text-xs text-slate-600 truncate">admin@aevalo.com</p>
          </div>
        </div>
        <button class="w-full px-3 py-2 text-sm font-semibold text-slate-600 hover:text-slate-900 hover:bg-slate-100 rounded-lg transition text-left">
          🚪 Sair
        </button>
      </div>
    </div>

    <!-- Main Content -->
    <div class="flex-1 flex flex-col overflow-hidden">
      <!-- Header -->
      <div class="bg-white border-b border-slate-200 px-8 py-4 flex items-center justify-between">
        <div>
          <p class="text-xs text-slate-600">Painel Administrativo</p>
          <h1 class="text-lg font-bold text-slate-900">Área de Gestão</h1>
        </div>
        <div class="flex items-center gap-4">
          <!-- Last Updated -->
          <div class="text-right">
            <p class="text-xs text-slate-600">Última atualização</p>
            <p class="text-sm font-semibold text-slate-900">Há 2 min</p>
          </div>
          <!-- Help -->
          <button class="px-3 py-2 text-sm font-semibold text-slate-600 hover:text-primary transition flex items-center gap-1">
            <span>❓</span>
            <span>Ajuda</span>
          </button>
        </div>
      </div>

      <!-- Page Content -->
      <div class="flex-1 overflow-auto">
        <div class="p-8">
          <slot />
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()

const isActive = (path: string) => {
  return route.path === path || route.path.startsWith(path + '/')
}
</script>

<style scoped>
/* Admin Layout specific styles */
</style>
