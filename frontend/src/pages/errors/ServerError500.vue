<template>
  <div class="min-h-screen bg-gradient-to-br from-red-50 via-white to-orange-50 dark:from-gray-900 dark:via-gray-800 dark:to-gray-900 flex items-center justify-center px-4">
    <div class="max-w-2xl w-full">
      <!-- Card Container -->
      <div class="bg-white dark:bg-gray-800 rounded-2xl shadow-xl border border-gray-200 dark:border-gray-700 p-8 md:p-12 text-center">
        
        <!-- Illustration -->
        <div class="mb-8 flex justify-center">
          <div class="relative">
            <!-- Main Icon (Gear) -->
            <div class="text-8xl animate-spin-slow">
              ⚙️
            </div>
            <!-- Explosion Icon -->
            <div class="absolute -right-4 -top-2 text-5xl animate-pulse">
              💥
            </div>
          </div>
        </div>

        <!-- Error Code -->
        <h1 class="text-8xl md:text-9xl font-bold text-error dark:text-red-400 mb-4">
          500
        </h1>

        <!-- Title -->
        <h2 class="text-2xl md:text-3xl font-semibold text-gray-900 dark:text-white mb-4">
          Algo deu errado
        </h2>

        <!-- Description -->
        <p class="text-gray-600 dark:text-gray-300 mb-6 max-w-lg mx-auto">
          Nossos servidores encontraram um erro inesperado. Já fomos notificados e estamos trabalhando na solução.
        </p>

        <!-- Error ID -->
        <div class="bg-red-50 dark:bg-red-900/20 border border-red-200 dark:border-red-800 rounded-lg p-4 mb-8 inline-block">
          <p class="text-sm font-medium text-gray-700 dark:text-gray-200 mb-1">
            ID do Erro:
          </p>
          <p class="text-lg font-mono font-bold text-error dark:text-red-400">
            {{ errorId }}
          </p>
          <p class="text-xs text-gray-500 dark:text-gray-400 mt-1">
            (Guarde este código para referência)
          </p>
        </div>

        <!-- Action Buttons -->
        <div class="space-y-3 mb-8">
          <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200 mb-4">
            O que você pode fazer:
          </h3>
          
          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <!-- Retry Button (Primary) -->
            <button
              @click="retryRequest"
              :disabled="isRetrying"
              class="inline-flex items-center justify-center px-6 py-3 bg-gradient-to-r from-error to-orange-600 hover:from-red-600 hover:to-orange-700 disabled:from-gray-400 disabled:to-gray-500 text-white rounded-lg shadow-lg hover:shadow-xl disabled:shadow-none transition-all duration-200 disabled:cursor-not-allowed"
            >
              <svg 
                class="w-5 h-5 mr-2" 
                :class="{ 'animate-spin': isRetrying }"
                fill="none" 
                stroke="currentColor" 
                viewBox="0 0 24 24"
              >
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15" />
              </svg>
              {{ isRetrying ? 'Tentando...' : 'Tentar Novamente' }}
            </button>

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
          </div>

          <div class="flex flex-col sm:flex-row gap-3 justify-center">
            <!-- Status Button -->
            <button
              @click="openStatusPage"
              class="inline-flex items-center justify-center px-6 py-3 border border-gray-300 dark:border-gray-600 rounded-lg text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-700 transition-colors duration-200"
            >
              <svg class="w-5 h-5 mr-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 19v-6a2 2 0 00-2-2H5a2 2 0 00-2 2v6a2 2 0 002 2h2a2 2 0 002-2zm0 0V9a2 2 0 012-2h2a2 2 0 012 2v10m-6 0a2 2 0 002 2h2a2 2 0 002-2m0 0V5a2 2 0 012-2h2a2 2 0 012 2v14a2 2 0 01-2 2h-2a2 2 0 01-2-2z" />
              </svg>
              Status
            </button>

            <!-- Report Button -->
            <button
              @click="reportError"
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

        <!-- System Status -->
        <div class="bg-gray-50 dark:bg-gray-700/50 rounded-lg p-6 text-left">
          <div class="flex items-center justify-between mb-4">
            <h3 class="text-sm font-semibold text-gray-700 dark:text-gray-200">
              Status do Sistema:
            </h3>
            <span class="inline-flex items-center px-3 py-1 rounded-full text-sm font-medium"
                  :class="systemStatus.operational ? 'bg-green-100 text-green-800 dark:bg-green-900/30 dark:text-green-400' : 'bg-red-100 text-red-800 dark:bg-red-900/30 dark:text-red-400'">
              <span class="w-2 h-2 rounded-full mr-2" :class="systemStatus.operational ? 'bg-green-500' : 'bg-red-500'"></span>
              {{ systemStatus.operational ? 'Operacional' : 'Problemas Detectados' }}
            </span>
          </div>
          
          <div class="space-y-2 text-sm text-gray-600 dark:text-gray-300">
            <div class="flex items-center justify-between">
              <span>Servidores:</span>
              <span class="font-medium">{{ systemStatus.servers }}</span>
            </div>
            <div class="flex items-center justify-between">
              <span>Última verificação:</span>
              <span class="font-medium">{{ systemStatus.lastCheck }}</span>
            </div>
          </div>

          <button
            @click="openStatusPage"
            class="mt-4 w-full inline-flex items-center justify-center px-4 py-2 text-sm font-medium text-accent hover:text-purple-700 dark:hover:text-purple-400 transition-colors duration-200"
          >
            Ver Status Detalhado
            <svg class="w-4 h-4 ml-2" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 5l7 7-7 7" />
            </svg>
          </button>
        </div>

        <!-- Auto-retry countdown -->
        <div v-if="autoRetryEnabled && retryCountdown > 0" class="mt-6">
          <p class="text-sm text-gray-500 dark:text-gray-400">
            Tentando novamente automaticamente em {{ retryCountdown }}s...
          </p>
          <div class="mt-2 w-full bg-gray-200 dark:bg-gray-700 rounded-full h-1">
            <div 
              class="bg-accent h-1 rounded-full transition-all duration-1000"
              :style="{ width: `${(retryCountdown / 5) * 100}%` }"
            ></div>
          </div>
        </div>

      </div>

      <!-- Footer Info -->
      <div class="mt-6 text-center text-sm text-gray-500 dark:text-gray-400">
        <p>Erro registrado em: {{ errorTimestamp }}</p>
        <p class="mt-1">Estamos monitorando e investigando o problema</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

// Error tracking
const errorId = ref('')
const errorTimestamp = ref('')
const isRetrying = ref(false)

// System status (mock - should come from API)
const systemStatus = ref({
  operational: true,
  servers: '3/3 online',
  lastCheck: 'há 30 segundos'
})

// Auto-retry configuration
const autoRetryEnabled = ref(true)
const retryCountdown = ref(5)
let countdownInterval: NodeJS.Timeout | null = null
let retryAttempts = ref(0)
const maxRetryAttempts = 3

onMounted(() => {
  // Generate unique error ID
  errorId.value = generateErrorId()
  errorTimestamp.value = new Date().toLocaleString('pt-BR')
  
  // Log error for analytics and error tracking
  logError({
    errorId: errorId.value,
    path: route.path,
    fullPath: route.fullPath,
    timestamp: new Date().toISOString(),
    referrer: document.referrer,
    userAgent: navigator.userAgent
  })

  // Start auto-retry countdown if enabled
  if (autoRetryEnabled.value && retryAttempts.value < maxRetryAttempts) {
    startAutoRetryCountdown()
  }

  // Check system status
  checkSystemStatus()
})

onUnmounted(() => {
  // Clear countdown interval on component unmount
  if (countdownInterval) {
    clearInterval(countdownInterval)
  }
})

// Generate unique error ID
const generateErrorId = (): string => {
  const date = new Date()
  const year = date.getFullYear()
  const month = String(date.getMonth() + 1).padStart(2, '0')
  const day = String(date.getDate()).padStart(2, '0')
  const random = Math.floor(Math.random() * 10000).toString().padStart(4, '0')
  return `#ERR-${year}-${month}-${day}-${random}`
}

// Log error to tracking service
const logError = (errorData: any) => {
  console.error('500 Server Error:', errorData)
  
  // TODO: Send to error tracking service (Sentry, etc)
  // Example:
  // Sentry.captureException(new Error('Server Error 500'), {
  //   tags: { errorId: errorData.errorId },
  //   extra: errorData
  // })
}

// Start auto-retry countdown
const startAutoRetryCountdown = () => {
  countdownInterval = setInterval(() => {
    retryCountdown.value--
    
    if (retryCountdown.value <= 0) {
      if (countdownInterval) {
        clearInterval(countdownInterval)
      }
      retryRequest()
    }
  }, 1000)
}

// Navigation methods
const goBack = () => {
  if (window.history.length > 1) {
    router.go(-1)
  } else {
    router.push('/dashboard')
  }
}

const retryRequest = async () => {
  if (isRetrying.value) return
  
  isRetrying.value = true
  retryAttempts.value++
  
  // Stop countdown if running
  if (countdownInterval) {
    clearInterval(countdownInterval)
    countdownInterval = null
  }

  try {
    // Simulate retry delay
    await new Promise(resolve => setTimeout(resolve, 1000))
    
    // Attempt to reload the previous page or go to dashboard
    if (window.history.length > 1) {
      window.location.reload()
    } else {
      router.push('/dashboard')
    }
  } catch (error) {
    console.error('Retry failed:', error)
    isRetrying.value = false
    
    // If max retries not reached, restart countdown
    if (retryAttempts.value < maxRetryAttempts) {
      retryCountdown.value = 5
      startAutoRetryCountdown()
    } else {
      autoRetryEnabled.value = false
    }
  }
}

const openStatusPage = () => {
  // Open status page in new tab
  window.open('https://status.aevalo.app', '_blank')
}

const reportError = () => {
  // Open support/contact form with pre-filled error details
  const subject = encodeURIComponent(`Erro 500 - ${errorId.value}`)
  const body = encodeURIComponent(`
ID do Erro: ${errorId.value}
Timestamp: ${errorTimestamp.value}
URL: ${window.location.href}
Navegador: ${navigator.userAgent}

Descrição do problema:
[Descreva o que você estava fazendo quando o erro ocorreu]
  `)
  
  // Option 1: Open email client
  // window.location.href = `mailto:support@aevalo.app?subject=${subject}&body=${body}`
  
  // Option 2: Open support page
  router.push({
    path: '/help/contact',
    query: { errorId: errorId.value }
  })
}

const checkSystemStatus = async () => {
  try {
    // TODO: Implement actual system status check
    // const response = await fetch('/api/health')
    // const data = await response.json()
    // systemStatus.value = data
    
    // Mock status check
    systemStatus.value = {
      operational: true,
      servers: '3/3 online',
      lastCheck: 'há 30 segundos'
    }
  } catch (error) {
    console.error('Failed to check system status:', error)
    systemStatus.value = {
      operational: false,
      servers: '0/3 online',
      lastCheck: 'falhou'
    }
  }
}
</script>

<style scoped>
/* Custom animations */
@keyframes spin-slow {
  from {
    transform: rotate(0deg);
  }
  to {
    transform: rotate(360deg);
  }
}

.animate-spin-slow {
  animation: spin-slow 3s linear infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.animate-pulse {
  animation: pulse 2s infinite;
}

/* Smooth transitions */
button {
  transition: all 0.2s ease;
}

button:not(:disabled):active {
  transform: scale(0.98);
}

/* Progress bar animation */
.transition-all {
  transition: all 1s ease-out;
}
</style>
