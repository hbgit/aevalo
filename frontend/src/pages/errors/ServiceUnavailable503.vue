<template>
  <div class="min-h-screen bg-gradient-to-br from-purple-50 to-slate-100 dark:from-slate-900 dark:to-slate-800 flex items-center justify-center p-4">
    <div class="max-w-lg w-full">
      <!-- Animated Icons -->
      <div class="text-center mb-8">
        <div class="flex justify-center gap-4 mb-6">
          <div class="text-6xl animate-spin-slow">🔧</div>
          <div class="text-6xl animate-pulse">⏰</div>
        </div>
      </div>

      <!-- Main Content Card -->
      <div class="bg-white dark:bg-slate-800 rounded-2xl shadow-xl p-8 space-y-6">
        <!-- Title -->
        <div class="text-center">
          <h1 class="text-4xl font-bold text-slate-900 dark:text-white mb-2">
            Manutenção em Andamento
          </h1>
          <p class="text-slate-600 dark:text-slate-400 text-base">
            Estamos realizando melhorias no sistema para oferecer uma experiência ainda melhor.
          </p>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Maintenance Details -->
        <div class="space-y-4">
          <div class="flex items-start gap-3">
            <span class="text-xl mt-0.5">📅</span>
            <div class="flex-1">
              <p class="text-sm text-slate-600 dark:text-slate-400">Início</p>
              <p class="font-semibold text-slate-900 dark:text-white">
                {{ formatDate(maintenanceStart) }}
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <span class="text-xl mt-0.5">⏱️</span>
            <div class="flex-1">
              <p class="text-sm text-slate-600 dark:text-slate-400">Previsão de retorno</p>
              <p class="font-semibold text-slate-900 dark:text-white">
                {{ formatDate(maintenanceEnd) }}
              </p>
            </div>
          </div>

          <div class="flex items-start gap-3">
            <span class="text-xl mt-0.5">⏳</span>
            <div class="flex-1">
              <p class="text-sm text-slate-600 dark:text-slate-400">Tempo restante</p>
              <p class="font-semibold text-slate-900 dark:text-white">
                {{ timeRemaining }}
              </p>
            </div>
          </div>
        </div>

        <!-- Countdown Progress Bar -->
        <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-2 overflow-hidden">
          <div
            class="h-full bg-gradient-to-r from-orange-400 to-accent transition-all duration-1000 ease-linear"
            :style="{ width: `${progressPercentage}%` }"
          ></div>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Maintenance Activities -->
        <div class="bg-gradient-to-br from-purple-50 to-blue-50 dark:from-slate-700 dark:to-slate-600 rounded-lg p-4">
          <h3 class="font-semibold text-slate-900 dark:text-white mb-3 flex items-center gap-2">
            <span>Durante a manutenção:</span>
          </h3>
          <ul class="space-y-2">
            <li class="flex items-start gap-3">
              <span class="text-green-500 font-bold mt-0.5">✓</span>
              <span class="text-slate-700 dark:text-slate-300">Upgrade de infraestrutura</span>
            </li>
            <li class="flex items-start gap-3">
              <span class="text-green-500 font-bold mt-0.5">✓</span>
              <span class="text-slate-700 dark:text-slate-300">Melhorias de performance</span>
            </li>
            <li class="flex items-start gap-3">
              <span class="text-green-500 font-bold mt-0.5">✓</span>
              <span class="text-slate-700 dark:text-slate-300">Novos recursos sendo implantados</span>
            </li>
          </ul>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Social/Status Links -->
        <div class="space-y-3">
          <p class="text-sm text-slate-600 dark:text-slate-400 text-center">Fique por dentro:</p>
          <div class="flex gap-3 justify-center">
            <a
              href="https://twitter.com/aevalo"
              target="_blank"
              rel="noopener noreferrer"
              class="px-4 py-2 rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-orange-400 dark:hover:border-orange-400 transition-colors font-medium flex items-center gap-2"
            >
              <span>🐦</span>
              <span>Twitter</span>
            </a>
            <a
              href="https://status.aevalo.app"
              target="_blank"
              rel="noopener noreferrer"
              class="px-4 py-2 rounded-lg border-2 border-slate-300 dark:border-slate-600 text-slate-700 dark:text-slate-300 hover:bg-slate-50 dark:hover:bg-slate-700 hover:border-orange-400 dark:hover:border-orange-400 transition-colors font-medium flex items-center gap-2"
            >
              <span>💬</span>
              <span>Status Page</span>
            </a>
          </div>
        </div>

        <!-- Divider -->
        <div class="border-t border-slate-200 dark:border-slate-700"></div>

        <!-- Action Buttons -->
        <div class="space-y-3 pt-2">
          <button
            @click="reloadPage"
            class="w-full px-6 py-3 bg-gradient-to-r from-primary to-accent hover:from-accent hover:to-primary text-white font-bold rounded-lg transition-all duration-200 transform hover:scale-105 active:scale-95 flex items-center justify-center gap-2"
          >
            <span>🔄</span>
            <span>Recarregar Página</span>
          </button>

          <p class="text-center text-sm text-slate-600 dark:text-slate-400">
            Obrigado pela compreensão!
          </p>
        </div>

        <!-- Auto-Reload Status -->
        <div class="text-center">
          <p class="text-xs text-slate-500 dark:text-slate-500">
            A página será recarregada automaticamente em {{ autoReloadCountdown }}s
          </p>
        </div>
      </div>

      <!-- Footer Info -->
      <div class="mt-6 text-center">
        <p class="text-sm text-slate-600 dark:text-slate-400">
          Precisa de suporte? <a href="mailto:support@aevalo.app" class="text-primary hover:underline font-semibold">Contato</a>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'

// Maintenance window (simulated - adjust as needed)
const maintenanceStart = new Date('2026-02-15T02:00:00-03:00')
const maintenanceEnd = new Date('2026-02-15T04:00:00-03:00')

const timeRemaining = ref('')
const progressPercentage = ref(100)
const autoReloadCountdown = ref(60)

let timerInterval: ReturnType<typeof setInterval> | null = null
let reloadCountdownInterval: ReturnType<typeof setInterval> | null = null
let autoReloadTimeout: ReturnType<typeof setTimeout> | null = null

onMounted(() => {
  // Log error for monitoring
  console.error('Service Unavailable (503)', {
    timestamp: new Date().toISOString(),
    maintenanceStart: maintenanceStart.toISOString(),
    maintenanceEnd: maintenanceEnd.toISOString(),
    url: window.location.href,
  })

  // Update countdown and progress bar
  const updateCountdown = () => {
    const now = new Date().getTime()
    const endTime = maintenanceEnd.getTime()
    const startTime = maintenanceStart.getTime()
    const totalDuration = endTime - startTime
    const elapsed = now - startTime

    if (now < startTime) {
      // Maintenance hasn't started yet
      const timeUntilStart = startTime - now
      timeRemaining.value = formatTimeRemaining(timeUntilStart)
      progressPercentage.value = 0
    } else if (now < endTime) {
      // Maintenance is ongoing
      const remaining = endTime - now
      timeRemaining.value = formatTimeRemaining(remaining)
      progressPercentage.value = Math.max(0, ((totalDuration - elapsed) / totalDuration) * 100)
    } else {
      // Maintenance is over - auto reload
      timeRemaining.value = 'Retornando...'
      progressPercentage.value = 0
      window.location.reload()
    }
  }

  // Update auto-reload countdown
  const updateAutoReloadCountdown = () => {
    autoReloadCountdown.value--
    if (autoReloadCountdown.value <= 0) {
      autoReloadCountdown.value = 60
      window.location.reload()
    }
  }

  // Initial update
  updateCountdown()
  updateAutoReloadCountdown()

  // Set intervals
  timerInterval = setInterval(updateCountdown, 1000)
  reloadCountdownInterval = setInterval(updateAutoReloadCountdown, 1000)

  // Auto-refresh every 60 seconds
  autoReloadTimeout = setTimeout(() => {
    window.location.reload()
  }, 60000)
})

onUnmounted(() => {
  if (timerInterval) clearInterval(timerInterval)
  if (reloadCountdownInterval) clearInterval(reloadCountdownInterval)
  if (autoReloadTimeout) clearTimeout(autoReloadTimeout)
})

const formatTimeRemaining = (ms: number): string => {
  if (ms < 0) return 'Em breve'

  const totalSeconds = Math.floor(ms / 1000)
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60

  if (hours > 0) {
    return `${hours}h ${minutes}m`
  } else if (minutes > 0) {
    return `${minutes}m ${seconds}s`
  } else {
    return `${seconds}s`
  }
}

const formatDate = (date: Date): string => {
  return date.toLocaleString('pt-BR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit',
    timeZoneName: 'short',
  })
}

const reloadPage = () => {
  window.location.reload()
}
</script>

<style scoped>
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
</style>
