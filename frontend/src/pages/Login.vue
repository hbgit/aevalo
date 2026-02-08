<template>
  <div class="min-h-screen bg-slate-100 flex items-center justify-center px-4 py-10">
    <div class="w-full max-w-sm bg-white rounded-3xl shadow-xl overflow-hidden">
      <div class="bg-gradient-to-r from-orange-500 via-rose-500 to-purple-600 px-8 pt-10 pb-12 text-center text-white">
        <div class="mx-auto h-16 w-16 rounded-2xl bg-white/20 backdrop-blur flex items-center justify-center">
          <span class="text-2xl font-semibold">A</span>
        </div>
        <h1 class="mt-4 text-2xl font-semibold">Aevalo</h1>
        <p class="text-sm text-white/80">Avaliações inteligentes para seu negócio</p>
      </div>

      <div class="px-8 pb-8 -mt-8">
        <div class="bg-slate-100 rounded-full p-1 flex gap-2 shadow-inner">
          <button class="flex-1 rounded-full bg-white text-primary font-semibold text-sm py-2 shadow">Entrar</button>
          <button class="flex-1 rounded-full text-slate-500 font-semibold text-sm py-2">Cadastrar</button>
        </div>

        <div class="mt-6">
          <h2 class="text-xl font-semibold text-slate-900">Bem-vindo(a) de volta</h2>
          <p class="text-sm text-slate-500 mt-1">Entre com suas credenciais para continuar</p>
        </div>

        <!-- Error Message with Icon -->
        <div v-if="errorMessage" class="mt-4 p-4 rounded-lg text-sm flex gap-3" :class="errorStyles">
          <span class="flex-shrink-0">{{ errorIcon }}</span>
          <div class="flex-1">
            <p class="font-medium">{{ errorMessage }}</p>
            <p v-if="errorDetails" class="text-xs mt-1 opacity-90">{{ errorDetails }}</p>
            <div v-if="showRetryCountdown && retryCountdown > 0" class="mt-2 flex items-center gap-2 text-xs">
              <span>Tente novamente em {{ retryCountdown }}s</span>
              <div class="w-16 h-1 bg-current/20 rounded-full overflow-hidden">
                <div class="h-full bg-current transition-all" :style="{ width: (retryCountdown / 60) * 100 + '%' }"></div>
              </div>
            </div>
            <div v-if="showRetryButton" class="mt-3 flex gap-2">
              <button 
                type="button"
                @click="handleLogin"
                :disabled="isLoading"
                class="px-3 py-1 text-xs font-semibold rounded bg-current/20 hover:bg-current/30 disabled:opacity-50"
              >
                Tentar Novamente
              </button>
              <a v-if="statusPageUrl" :href="statusPageUrl" target="_blank" class="px-3 py-1 text-xs font-semibold rounded bg-current/20 hover:bg-current/30">
                Ver Status
              </a>
            </div>
          </div>
        </div>

        <form @submit.prevent="handleLogin" class="mt-6 space-y-4">
          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Email</label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M2.94 5.5 10 9.82l7.06-4.32A2 2 0 0015.82 4H4.18a2 2 0 00-1.24 1.5z" />
                  <path d="M18 8.08l-7.46 4.56a1 1 0 01-1.08 0L2 8.08V14a2 2 0 002 2h12a2 2 0 002-2V8.08z" />
                </svg>
              </span>
              <input
                v-model="email"
                type="email"
                placeholder="seu.email@exemplo.com"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-4 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
                required
              />
            </div>
          </div>

          <div>
            <label class="block text-sm font-medium text-slate-700 mb-2">Senha</label>
            <div class="relative">
              <span class="absolute left-3 top-1/2 -translate-y-1/2 text-slate-400">
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path fill-rule="evenodd" d="M5 8a5 5 0 1110 0v2h1a1 1 0 011 1v6a1 1 0 01-1 1H4a1 1 0 01-1-1v-6a1 1 0 011-1h1V8zm2 2h6V8a3 3 0 00-6 0v2z" clip-rule="evenodd" />
                </svg>
              </span>
              <input
                v-model="password"
                type="password"
                placeholder="••••••••"
                class="w-full border border-slate-200 rounded-xl pl-10 pr-10 py-2.5 text-sm focus:outline-none focus:ring-2 focus:ring-primary/30"
                required
              />
              <button 
                type="button" 
                @click="showPassword = !showPassword"
                class="absolute right-3 top-1/2 -translate-y-1/2 text-slate-400"
              >
                <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                  <path d="M10 3c4.418 0 8 4 8 7s-3.582 7-8 7-8-4-8-7 3.582-7 8-7zm0 2c-3.21 0-6 3-6 5s2.79 5 6 5 6-3 6-5-2.79-5-6-5zm0 2a3 3 0 110 6 3 3 0 010-6z" />
                </svg>
              </button>
            </div>
          </div>

          <div class="flex items-center justify-between text-sm">
            <label class="inline-flex items-center gap-2 text-slate-600">
              <input v-model="rememberMe" type="checkbox" class="h-4 w-4 rounded border-slate-300 text-primary focus:ring-primary" />
              Lembrar de mim
            </label>
            <a href="#" class="text-primary font-semibold">Esqueceu a senha?</a>
          </div>

          <button 
            type="submit" 
            :disabled="isLoading"
            class="w-full bg-orange-500 hover:bg-orange-600 disabled:bg-orange-300 text-white font-semibold py-3 rounded-xl flex items-center justify-center gap-2 transition-colors"
          >
            <span v-if="!isLoading">
              Entrar
              <svg class="h-4 w-4" viewBox="0 0 20 20" fill="currentColor">
                <path fill-rule="evenodd" d="M10.293 3.293a1 1 0 011.414 0l5 5a1 1 0 010 1.414l-5 5a1 1 0 01-1.414-1.414L13.586 11H4a1 1 0 110-2h9.586l-3.293-3.293a1 1 0 010-1.414z" clip-rule="evenodd" />
              </svg>
            </span>
            <span v-else class="flex items-center gap-2">
              <svg class="h-4 w-4 animate-spin" viewBox="0 0 24 24" fill="none" stroke="currentColor">
                <circle class="opacity-25" cx="12" cy="12" r="10" stroke-width="4"></circle>
                <path class="opacity-75" fill="currentColor" d="M4 12a8 8 0 018-8V0C5.373 0 0 5.373 0 12h4zm2 5.291A7.962 7.962 0 014 12H0c0 3.042 1.135 5.824 3 7.938l3-2.647z"></path>
              </svg>
              Entrando...
            </span>
          </button>
        </form>

        <div class="mt-6">
          <div class="flex items-center gap-3 text-xs text-slate-400">
            <span class="h-px flex-1 bg-slate-200"></span>
            ou continue com
            <span class="h-px flex-1 bg-slate-200"></span>
          </div>

          <div class="mt-4 grid grid-cols-2 gap-3">
            <button class="border border-slate-200 rounded-xl py-2.5 text-sm font-semibold text-slate-700 flex items-center justify-center gap-2">
              <span class="text-red-500">G</span>
              Google
            </button>
            <button class="border border-slate-200 rounded-xl py-2.5 text-sm font-semibold text-slate-700 flex items-center justify-center gap-2">
              <svg class="h-4 w-4" viewBox="0 0 24 24" fill="currentColor">
                <path d="M12 2C6.477 2 2 6.485 2 12.017c0 4.425 2.865 8.18 6.839 9.504.5.092.682-.217.682-.483 0-.237-.008-.868-.013-1.703-2.782.605-3.369-1.343-3.369-1.343-.454-1.161-1.11-1.47-1.11-1.47-.908-.62.069-.608.069-.608 1.003.07 1.531 1.032 1.531 1.032.892 1.53 2.341 1.088 2.91.832.092-.647.35-1.088.636-1.339-2.22-.253-4.555-1.114-4.555-4.957 0-1.094.39-1.988 1.029-2.688-.103-.253-.446-1.272.098-2.65 0 0 .84-.27 2.75 1.026A9.564 9.564 0 0112 6.844c.85.004 1.705.115 2.504.337 1.909-1.296 2.747-1.026 2.747-1.026.546 1.378.203 2.397.1 2.65.64.7 1.028 1.594 1.028 2.688 0 3.852-2.338 4.701-4.566 4.95.359.31.678.921.678 1.856 0 1.338-.012 2.418-.012 2.747 0 .268.18.58.688.482A10.02 10.02 0 0022 12.017C22 6.485 17.523 2 12 2z" />
              </svg>
              GitHub
            </button>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

const email = ref('')
const password = ref('')
const showPassword = ref(false)
const rememberMe = ref(false)
const isLoading = ref(false)
const errorMessage = ref('')
const errorDetails = ref('')
const errorType = ref<'auth' | 'rate-limit' | 'server' | 'maintenance' | 'network' | null>(null)
const retryCountdown = ref(0)
const showRetryCountdown = ref(false)
const showRetryButton = ref(false)
const statusPageUrl = ref('')
let retryTimer: number | null = null

const errorStyles = computed(() => {
  switch (errorType.value) {
    case 'auth':
      return 'bg-red-50 border border-red-200 text-red-700'
    case 'rate-limit':
      return 'bg-yellow-50 border border-yellow-200 text-yellow-700'
    case 'server':
    case 'maintenance':
      return 'bg-orange-50 border border-orange-200 text-orange-700'
    case 'network':
      return 'bg-slate-50 border border-slate-200 text-slate-700'
    default:
      return 'bg-red-50 border border-red-200 text-red-700'
  }
})

const errorIcon = computed(() => {
  switch (errorType.value) {
    case 'auth':
      return '🔐'
    case 'rate-limit':
      return '⏱️'
    case 'server':
      return '⚠️'
    case 'maintenance':
      return '🔧'
    case 'network':
      return '📡'
    default:
      return '❌'
  }
})

/**
 * Limpa o countdown quando componente é desmontado
 */
onMounted(() => {
  return () => {
    if (retryTimer) clearInterval(retryTimer)
  }
})

/**
 * Inicia countdown de retry (429 - rate limit)
 */
const startRetryCountdown = (seconds: number = 60) => {
  retryCountdown.value = seconds
  showRetryCountdown.value = true
  showRetryButton.value = false
  
  // Desabilita botão de login durante countdown
  isLoading.value = true
  
  retryTimer = window.setInterval(() => {
    retryCountdown.value--
    if (retryCountdown.value <= 0) {
      if (retryTimer) clearInterval(retryTimer)
      showRetryCountdown.value = false
      showRetryButton.value = true
      isLoading.value = false
    }
  }, 1000)
}

/**
 * Trata erro de resposta HTTP com códigos específicos
 */
const handleHttpError = async (response: Response, errorData: any) => {
  switch (response.status) {
    case 401:
      // Credenciais inválidas
      errorType.value = 'auth'
      errorMessage.value = 'Email ou senha incorretos. Tente novamente.'
      errorDetails.value = 'Verifique seus dados de acesso e tente novamente.'
      password.value = '' // Limpa campo de senha
      // Foca no campo de senha após erro
      setTimeout(() => {
        const passwordInput = document.querySelector('input[type="password"]')
        passwordInput?.focus()
      }, 100)
      break

    case 429:
      // Rate limit - muitas tentativas
      errorType.value = 'rate-limit'
      errorMessage.value = 'Muitas tentativas de login. Por favor, aguarde.'
      errorDetails.value = 'Sua conta foi temporariamente bloqueada por segurança.'
      startRetryCountdown(60) // 60 segundos
      break

    case 500:
      // Erro do servidor
      errorType.value = 'server'
      errorMessage.value = 'Nossos servidores estão temporariamente indisponíveis.'
      errorDetails.value = 'Já estamos trabalhando para resolver o problema. Tente novamente em instantes.'
      statusPageUrl.value = 'https://status.aevalo.app'
      showRetryButton.value = true
      break

    case 503:
      // Manutenção programada
      errorType.value = 'maintenance'
      errorMessage.value = 'Sistema em manutenção programada.'
      errorDetails.value = 'Retornaremos em breve com melhorias. Obrigado pela paciência!'
      showRetryButton.value = true
      break

    case 403:
      // Conta bloqueada ou email não verificado
      if (errorData?.code === 'EMAIL_NOT_VERIFIED') {
        errorType.value = 'auth'
        errorMessage.value = 'Email não verificado.'
        errorDetails.value = 'Verifique seu email para ativar sua conta. Não encontrou? Clique para reenviar.'
        showRetryButton.value = true
      } else {
        errorType.value = 'auth'
        errorMessage.value = 'Sua conta foi temporariamente bloqueada por segurança.'
        errorDetails.value = 'Verifique seu email para instruções de recuperação.'
        showRetryButton.value = false
      }
      break

    default:
      // Erro desconhecido
      errorType.value = null
      errorMessage.value = errorData?.error || 'Erro ao fazer login'
      errorDetails.value = `Código de erro: ${response.status}`
      showRetryButton.value = true
  }
}

/**
 * Trata erro de rede
 */
const handleNetworkError = () => {
  errorType.value = 'network'
  errorMessage.value = 'Sem conexão com a internet.'
  errorDetails.value = 'Verifique sua conexão de rede e tente novamente.'
  showRetryButton.value = true
}

/**
 * Validações antes do login
 */
const validateForm = (): boolean => {
  if (!email.value) {
    errorType.value = 'auth'
    errorMessage.value = 'Email é obrigatório.'
    errorDetails.value = ''
    return false
  }

  if (!password.value) {
    errorType.value = 'auth'
    errorMessage.value = 'Senha é obrigatória.'
    errorDetails.value = ''
    return false
  }

  // Validação básica de email
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(email.value)) {
    errorType.value = 'auth'
    errorMessage.value = 'Email inválido.'
    errorDetails.value = 'Por favor, insira um email válido.'
    return false
  }

  return true
}

/**
 * Handler principal de login
 */
const handleLogin = async () => {
  // Limpa erro anterior
  errorMessage.value = ''
  errorDetails.value = ''
  errorType.value = null
  showRetryButton.value = false

  // Valida formulário
  if (!validateForm()) {
    return
  }

  isLoading.value = true

  try {
    const response = await fetch('http://localhost:3000/auth/login', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json'
      },
      body: JSON.stringify({
        email: email.value,
        password: password.value
      }),
      signal: AbortSignal.timeout(10000) // 10 segundos de timeout
    })

    const data = await response.json()

    if (!response.ok) {
      await handleHttpError(response, data)
      return
    }

    // Sucesso - armazenar tokens
    localStorage.setItem('auth_token', data.token)
    localStorage.setItem('refresh_token', data.refresh_token)
    localStorage.setItem('token_expires_at', data.expires_at || Date.now() + 3600000) // 1 hora

    // Armazenar preferência de "lembrar de mim"
    if (rememberMe.value) {
      localStorage.setItem('remember_email', email.value)
    } else {
      localStorage.removeItem('remember_email')
    }

    // Redirecionar para dashboard
    await router.push('/dashboard')
  } catch (error) {
    // Tratamento de erros
    if (error instanceof TypeError && error.message.includes('Failed to fetch')) {
      // Erro de rede
      handleNetworkError()
    } else if (error instanceof DOMException && error.name === 'AbortError') {
      // Timeout
      errorType.value = 'server'
      errorMessage.value = 'Conexão expirou. Tente novamente.'
      errorDetails.value = 'A requisição demorou muito tempo. Verifique sua conexão.'
      showRetryButton.value = true
    } else {
      // Erro genérico
      console.error('Login error:', error)
      errorType.value = null
      errorMessage.value = 'Erro ao conectar com o servidor'
      errorDetails.value = error instanceof Error ? error.message : 'Tente novamente'
      showRetryButton.value = true
    }
  } finally {
    isLoading.value = false
  }
}

// Restaurar email se "lembrar de mim" estava ativo
const savedEmail = localStorage.getItem('remember_email')
if (savedEmail) {
  email.value = savedEmail
  rememberMe.value = true
}
</script>
