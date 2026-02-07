import { createApp } from 'vue'
import App from './App.vue'
import router from './router'
import { createPinia } from 'pinia'
import axios from 'axios'
import { useAuthStore } from './stores/auth'
import { useAuthInterceptor } from './composables/useAuthInterceptor'
import './styles/tailwind.css'

console.log('Starting Aevalo Frontend...')

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)
app.use(router)

// Configure Axios interceptors
const authInterceptor = useAuthInterceptor(axios)
authInterceptor.setupInterceptors()

// Set default API URL
axios.defaults.baseURL = import.meta.env.VITE_API_URL || 'http://localhost:3000'
axios.defaults.timeout = 30000

console.log('Axios configured with baseURL:', axios.defaults.baseURL)

// Restore session on app initialization
const authStore = useAuthStore()
authStore.restoreSession()

console.log('Session restored')

// Global error handler
app.config.errorHandler = (err, instance, info) => {
  console.error('Vue error:', err, info)
}

// Global warning handler
app.config.warnHandler = (message, instance, trace) => {
  console.warn('Vue warning:', message, trace)
}

app.mount('#app')

console.log('App mounted to #app')