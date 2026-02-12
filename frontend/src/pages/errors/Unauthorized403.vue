<template>
  <div class="min-h-screen bg-gradient-to-br from-pink-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 flex items-center justify-center p-4">
    <div class="max-w-lg w-full">
      <!-- Animated Icons -->
      <div class="text-center mb-8">
        <div class="flex justify-center gap-4 mb-6">
          <div class="text-6xl">🔒</div>
          <div class="text-6xl animate-pulse">🚫</div>
        </div>
      </div>

      <!-- Main Content Card -->
      <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-xl p-8 space-y-6">
        <!-- Title -->
        <div class="text-center">
          <div class="text-6xl font-bold text-red-500 mb-3">403</div>
          <h1 class="text-3xl font-bold text-slate-900 dark:text-white mb-2">
            Acesso Negado
          </h1>
          <p class="text-slate-600 dark:text-slate-400 text-base">
            Você não tem permissão para acessar este recurso.
          </p>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Possible Reasons -->
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4">
          <h3 class="font-semibold text-slate-900 dark:text-white mb-3 flex items-center gap-2">
            <span>📋</span>
            <span>Possíveis motivos:</span>
          </h3>
          <ul class="space-y-2">
            <li class="flex items-start gap-3">
              <span class="text-red-500 font-bold mt-0.5">•</span>
              <span class="text-slate-700 dark:text-slate-300">Recurso pertence a outro usuário</span>
            </li>
            <li class="flex items-start gap-3">
              <span class="text-red-500 font-bold mt-0.5">•</span>
              <span class="text-slate-700 dark:text-slate-300">Sua função não permite esta ação</span>
            </li>
            <li class="flex items-start gap-3">
              <span class="text-red-500 font-bold mt-0.5">•</span>
              <span class="text-slate-700 dark:text-slate-300">Avaliação foi arquivada ou deletada</span>
            </li>
          </ul>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- What To Do Section -->
        <div class="space-y-3">
          <h3 class="font-semibold text-slate-900 dark:text-white flex items-center gap-2">
            <span>💡</span>
            <span>O que fazer:</span>
          </h3>
          <div class="grid grid-cols-1 gap-2">
            <button
              @click="goBack"
              class="px-4 py-2 text-left rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-400 transition-colors font-medium flex items-center gap-3"
            >
              <span class="text-xl">⬅️</span>
              <span>Voltar para área segura</span>
            </button>
            <button
              @click="navigateToDashboard"
              class="px-4 py-2 text-left rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-400 transition-colors font-medium flex items-center gap-3"
            >
              <span class="text-xl">🏠</span>
              <span>Dashboard inicial</span>
            </button>
            <button
              @click="showPermissionsInfo"
              class="px-4 py-2 text-left rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-400 transition-colors font-medium flex items-center gap-3"
            >
              <span class="text-xl">📖</span>
              <span>Ver permissões da sua conta</span>
            </button>
            <button
              @click="contactAdmin"
              class="px-4 py-2 text-left rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-slate-400 transition-colors font-medium flex items-center gap-3"
            >
              <span class="text-xl">💬</span>
              <span>Falar com administrador</span>
            </button>
          </div>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Contact Section -->
        <div class="bg-gradient-to-br from-blue-50 to-blue-100 dark:from-blue-900/20 dark:to-blue-800/20 border border-blue-200 dark:border-blue-800 rounded-lg p-4 text-center">
          <p class="text-sm text-slate-700 dark:text-slate-300 mb-3">
            Se acredita que isso é um erro,
          </p>
          <a
            href="mailto:support@aevalo.app"
            class="inline-flex items-center gap-2 px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-lg transition-colors"
          >
            <span>📧</span>
            <span>Entre em contato conosco</span>
          </a>
        </div>

        <!-- Error Details (Admin Info) -->
        <div v-if="showDetails" class="bg-slate-100 dark:bg-slate-700 rounded-lg p-3 text-xs font-mono">
          <p class="text-slate-600 dark:text-slate-400 mb-2">Detalhes do erro:</p>
          <p class="text-slate-700 dark:text-slate-300">
            <span class="text-slate-500">Acesso negado em:</span> {{ currentPath }}
          </p>
          <p class="text-slate-700 dark:text-slate-300">
            <span class="text-slate-500">Timestamp:</span> {{ currentTime }}
          </p>
        </div>
      </div>

      <!-- Footer Info -->
      <div class="mt-6 text-center">
        <p class="text-sm text-slate-600 dark:text-slate-400">
          Código de erro: <span class="font-semibold text-red-600 dark:text-red-400">403 FORBIDDEN</span>
        </p>
      </div>
    </div>

    <!-- Admin Contact Modal -->
    <Teleport to="body">
      <Transition name="fade">
        <div v-if="showAdminModal" class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div class="bg-white dark:bg-slate-800 rounded-xl shadow-2xl max-w-md w-full p-6 space-y-4">
            <div class="text-center">
              <div class="text-4xl mb-3">👤</div>
              <h2 class="text-xl font-bold text-slate-900 dark:text-white">
                Contato com Administrador
              </h2>
            </div>

            <div class="space-y-3">
              <p class="text-sm text-slate-600 dark:text-slate-400">
                Para solicitar acesso ou esclarecer dúvidas sobre suas permissões, entre em contato com o administrador:
              </p>

              <div class="bg-slate-50 dark:bg-slate-700 rounded-lg p-3 space-y-2">
                <p class="text-sm">
                  <span class="font-semibold text-slate-700 dark:text-slate-300">Email:</span>
                  <a
                    href="mailto:admin@aevalo.app"
                    class="text-blue-600 dark:text-blue-400 hover:underline"
                  >
                    admin@aevalo.app
                  </a>
                </p>
                <p class="text-sm">
                  <span class="font-semibold text-slate-700 dark:text-slate-300">Suporte:</span>
                  <a
                    href="mailto:support@aevalo.app"
                    class="text-blue-600 dark:text-blue-400 hover:underline"
                  >
                    support@aevalo.app
                  </a>
                </p>
              </div>
            </div>

            <div class="flex gap-3 pt-4">
              <button
                @click="showAdminModal = false"
                class="flex-1 px-4 py-2 border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 font-semibold rounded-lg transition-colors"
              >
                Fechar
              </button>
              <a
                href="mailto:admin@aevalo.app"
                class="flex-1 px-4 py-2 bg-gradient-to-r from-primary to-accent hover:from-accent hover:to-primary text-white font-semibold rounded-lg transition-all text-center"
              >
                Enviar Email
              </a>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>

    <!-- Permissions Info Modal -->
    <Teleport to="body">
      <Transition name="fade">
        <div v-if="showPermissionsModal" class="fixed inset-0 bg-black/50 flex items-center justify-center p-4 z-50">
          <div class="bg-white dark:bg-slate-800 rounded-xl shadow-2xl max-w-md w-full p-6 space-y-4">
            <div class="text-center">
              <div class="text-4xl mb-3">🔐</div>
              <h2 class="text-xl font-bold text-slate-900 dark:text-white">
                Suas Permissões
              </h2>
            </div>

            <div class="space-y-3">
              <p class="text-sm text-slate-600 dark:text-slate-400">
                Para visualizar suas permissões e função na plataforma:
              </p>

              <div class="space-y-2">
                <button
                  @click="navigateToSettings"
                  class="w-full px-4 py-2 bg-blue-500 hover:bg-blue-600 text-white font-semibold rounded-lg transition-colors"
                >
                  Ir para Configurações
                </button>
                <button
                  @click="navigateToTeamSettings"
                  class="w-full px-4 py-2 border-2 border-blue-500 text-blue-600 dark:text-blue-400 hover:bg-blue-50 dark:hover:bg-blue-900/20 font-semibold rounded-lg transition-colors"
                >
                  Gerenciamento de Equipe
                </button>
              </div>

              <div class="bg-slate-50 dark:bg-slate-700 rounded-lg p-3 text-xs">
                <p class="text-slate-600 dark:text-slate-400 mb-2">
                  <strong>Nota:</strong> Se você foi adicionado recentemente à equipe, aguarde alguns minutos para suas permissões serem atualizadas.
                </p>
              </div>
            </div>

            <button
              @click="showPermissionsModal = false"
              class="w-full px-4 py-2 border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 font-semibold rounded-lg transition-colors"
            >
              Fechar
            </button>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

const showAdminModal = ref(false)
const showPermissionsModal = ref(false)
const showDetails = ref(false)

const currentPath = computed(() => route.path)
const currentTime = computed(() => new Date().toLocaleString('pt-BR'))

onMounted(() => {
  // Log error for monitoring
  console.error('Access Forbidden (403)', {
    timestamp: new Date().toISOString(),
    path: route.path,
    referrer: document.referrer,
    url: window.location.href,
  })

  // Show details if user is admin (in development)
  if (import.meta.env.DEV) {
    showDetails.value = true
  }
})

const goBack = () => {
  router.go(-1)
}

const navigateToDashboard = () => {
  router.push({ name: 'Dashboard' })
}

const showPermissionsInfo = () => {
  showPermissionsModal.value = true
}

const contactAdmin = () => {
  showAdminModal.value = true
}

const navigateToSettings = () => {
  showPermissionsModal.value = false
  router.push({ path: '/settings/account' }).catch(() => {
    // Fallback if settings page doesn't exist
    alert('Página de configurações não disponível. Verifique com o administrador.')
  })
}

const navigateToTeamSettings = () => {
  showPermissionsModal.value = false
  router.push({ path: '/settings/team' }).catch(() => {
    // Fallback if settings page doesn't exist
    alert('Página de equipe não disponível. Verifique com o administrador.')
  })
}
</script>

<style scoped>
.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}

@keyframes pulse {
  0%,
  100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}
</style>
