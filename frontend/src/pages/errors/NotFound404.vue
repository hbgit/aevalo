<template>
  <div class="min-h-screen bg-gradient-to-br from-purple-50 via-white to-orange-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 flex items-center justify-center px-4">
    <div class="max-w-2xl w-full">
      <!-- Card Container -->
      <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-200 dark:border-gray-700 p-8 md:p-12 text-center">
        
        <!-- Illustration -->
        <div class="mb-8 flex justify-center">
          <div class="relative">
            <!-- Main Icon -->
            <div class="text-8xl animate-bounce">
              🔍
            </div>
            <!-- Question Mark -->
            <div class="absolute -right-4 -top-2 text-5xl animate-pulse">
              ❓
            </div>
          </div>
        </div>

        <!-- Error Code -->
        <h1 class="text-8xl md:text-9xl font-bold text-primary dark:text-purple-400 mb-4">
          404
        </h1>

        <!-- Title -->
        <h2 class="text-2xl md:text-3xl font-semibold text-gray-900 dark:text-white mb-4">
          Página não encontrada
        </h2>

        <!-- Description -->
        <p class="text-gray-600 dark:text-gray-300 mb-8 max-w-md mx-auto">
          A página que você procura não existe ou foi movida para outro endereço.
        </p>

        <!-- Possible Reasons -->
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-6 mb-8 text-left">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-3">
            Possíveis motivos:
          </h3>
          <ul class="space-y-2 text-sm text-gray-600 dark:text-gray-300">
            <li class="flex items-start">
              <span class="text-orange-500 mr-2">•</span>
              <span>Link quebrado ou desatualizado</span>
            </li>
            <li class="flex items-start">
              <span class="text-orange-500 mr-2">•</span>
              <span>URL digitada incorretamente</span>
            </li>
            <li class="flex items-start">
              <span class="text-orange-500 mr-2">•</span>
              <span>Recurso foi removido</span>
            </li>
          </ul>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-3 mb-8">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
            Sugestões:
          </h3>
          
          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <!-- Back Button -->
            <button
              @click="goBack"
              class="inline-flex items-center justify-center px-6 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18" />
              </svg>
              Voltar
            </button>

            <!-- Dashboard Button -->
            <button
              @click="goToDashboard"
              class="inline-flex items-center justify-center px-6 py-3 bg-gradient-to-r from-primary to-accent hover:from-purple-700 hover:to-purple-600 text-white rounded-lg shadow-lg hover:shadow-xl transition-all duration-200"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6" />
              </svg>
              Ir para Dashboard
            </button>
          </div>

          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <!-- Search Button -->
            <button
              @click="openSearch"
              class="inline-flex items-center justify-center px-6 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M21 21l-6-6m2-5a7 7 0 11-14 0 7 7 0 0114 0z" />
              </svg>
              Buscar
            </button>

            <!-- Report Button -->
            <button
              @click="reportIssue"
              class="inline-flex items-center justify-center px-6 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 12h.01M12 12h.01M16 12h.01M21 12c0 4.418-4.03 8-9 8a9.863 9.863 0 01-4.255-.949L3 20l1.395-3.72C3.512 15.042 3 13.574 3 12c0-4.418 4.03-8 9-8s9 3.582 9 8z" />
              </svg>
              Reportar
            </button>
          </div>
        </div>

        <!-- Divider -->
        <div class="border-t border-gray-200 dark:border-gray-700 my-8"></div>

        <!-- Popular Pages -->
        <div class="text-left">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
            Páginas populares:
          </h3>
          <ul class="space-y-3">
            <li>
              <router-link
                to="/dashboard"
                class="text-accent hover:text-purple-700 dark:hover:text-purple-400 transition-colors duration-200 inline-flex items-center"
              >
                <span class="mr-2">•</span>
                Dashboard
              </router-link>
            </li>
            <li>
              <router-link
                to="/evaluations/create"
                class="text-accent hover:text-purple-700 dark:hover:text-purple-400 transition-colors duration-200 inline-flex items-center"
              >
                <span class="mr-2">•</span>
                Criar Nova Avaliação
              </router-link>
            </li>
            <li>
              <router-link
                to="/templates"
                class="text-accent hover:text-purple-700 dark:hover:text-purple-400 transition-colors duration-200 inline-flex items-center"
              >
                <span class="mr-2">•</span>
                Templates
              </router-link>
            </li>
            <li>
              <router-link
                to="/analytics"
                class="text-accent hover:text-purple-700 dark:hover:text-purple-400 transition-colors duration-200 inline-flex items-center"
              >
                <span class="mr-2">•</span>
                Analytics
              </router-link>
            </li>
          </ul>
        </div>

      </div>

      <!-- Footer Info -->
      <div class="mt-6 text-center text-sm text-gray-500 dark:text-gray-400">
        <p>Erro registrado em: {{ errorTimestamp }}</p>
        <p class="mt-1">URL tentada: <code class="text-xs bg-gray-100 dark:bg-gray-700 px-2 py-1 rounded">{{ attemptedUrl }}</code></p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

// Error tracking
const errorTimestamp = ref('')
const attemptedUrl = ref('')

onMounted(() => {
  // Log error for analytics
  errorTimestamp.value = new Date().toLocaleString('pt-BR')
  attemptedUrl.value = window.location.pathname
  
  // Log to console for debugging
  console.warn('404 Error:', {
    path: route.path,
    fullPath: route.fullPath,
    timestamp: errorTimestamp.value,
    referrer: document.referrer
  })

  // Optional: Send to error tracking service (Sentry, etc)
  // trackError('404', { path: route.path, referrer: document.referrer })
})

// Navigation methods
const goBack = () => {
  if (window.history.length > 1) {
    router.go(-1)
  } else {
    router.push('/dashboard')
  }
}

const goToDashboard = () => {
  router.push('/dashboard')
}

const openSearch = () => {
  // Trigger global search (Cmd+K)
  // This would emit an event or call a composable
  console.log('Open search triggered')
  // Example: emit('openGlobalSearch')
  router.push('/dashboard') // Fallback
}

const reportIssue = () => {
  // Open support/contact form
  // Could open a modal or navigate to contact page
  console.log('Report issue triggered')
  // Example: open modal with pre-filled 404 error details
  router.push('/help/contact')
}
</script>

<style scoped>
/* Custom animations */
@keyframes bounce {
  0%, 100% {
    transform: translateY(0);
  }
  50% {
    transform: translateY(-10px);
  }
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.animate-bounce {
  animation: bounce 2s infinite;
}

.animate-pulse {
  animation: pulse 2s infinite;
}

/* Smooth transitions */
button {
  transition: all 0.2s ease;
}

button:active {
  transform: scale(0.98);
}
</style>
