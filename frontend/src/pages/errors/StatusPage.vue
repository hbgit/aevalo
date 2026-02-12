<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-50 to-blue-50 dark:from-slate-900 dark:to-slate-800 py-12 px-4 sm:px-6 lg:px-8">
    <div class="max-w-4xl mx-auto">
      <!-- Header -->
      <div class="text-center mb-12">
        <h1 class="text-4xl font-bold text-slate-900 dark:text-white mb-2">Aevalo System Status</h1>
        <p class="text-lg text-slate-600 dark:text-slate-300">Real-time monitoring and incident tracking</p>
      </div>

      <!-- Overall Status Card -->
      <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-8 mb-8 border-l-4 border-green-500">
        <div class="flex items-center justify-between mb-4">
          <div>
            <h2 class="text-2xl font-bold text-slate-900 dark:text-white flex items-center gap-3">
              <span class="inline-block w-3 h-3 rounded-full bg-green-500 animate-pulse"></span>
              All Systems Operational
            </h2>
            <p class="text-sm text-slate-600 dark:text-slate-400 mt-2">Last checked: {{ lastCheckTime }}</p>
          </div>
          <div class="text-right">
            <p class="text-3xl font-bold text-green-600 dark:text-green-400">99.98%</p>
            <p class="text-xs text-slate-500 dark:text-slate-400">Uptime (30 days)</p>
          </div>
        </div>
      </div>

      <!-- Components Status -->
      <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-8 mb-8">
        <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-6 flex items-center gap-2">
          <span>📊</span> System Components
        </h3>
        
        <div class="space-y-4">
          <div v-for="component in components" :key="component.id" class="flex items-center justify-between p-4 border border-slate-200 dark:border-slate-700 rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition">
            <div class="flex-1">
              <div class="flex items-center gap-3 mb-2">
                <span class="text-2xl">{{ component.icon }}</span>
                <div>
                  <h4 class="font-semibold text-slate-900 dark:text-white">{{ component.name }}</h4>
                  <p class="text-xs text-slate-500 dark:text-slate-400">{{ component.service }}</p>
                </div>
              </div>
              <div class="w-full bg-slate-200 dark:bg-slate-700 rounded-full h-1.5">
                <div 
                  class="bg-gradient-to-r from-green-400 to-green-500 h-1.5 rounded-full transition-all"
                  :style="{ width: component.uptime + '%' }"
                ></div>
              </div>
            </div>
            <div class="text-right ml-6 whitespace-nowrap">
              <p class="font-bold text-slate-900 dark:text-white">{{ component.uptime }}%</p>
              <p class="text-xs text-slate-500 dark:text-slate-400">uptime</p>
            </div>
          </div>
        </div>
      </div>

      <!-- Performance Metrics -->
      <div class="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6 border-t-4 border-blue-500">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-600 dark:text-slate-400 font-medium">Response Time (avg)</p>
              <p class="text-3xl font-bold text-slate-900 dark:text-white mt-2">89ms</p>
            </div>
            <span class="text-4xl">⚡</span>
          </div>
          <p class="text-xs text-green-600 dark:text-green-400 mt-4">↓ 5ms from yesterday</p>
        </div>

        <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6 border-t-4 border-blue-500">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-600 dark:text-slate-400 font-medium">Requests Processed</p>
              <p class="text-3xl font-bold text-slate-900 dark:text-white mt-2">1.2M</p>
            </div>
            <span class="text-4xl">📊</span>
          </div>
          <p class="text-xs text-green-600 dark:text-green-400 mt-4">↑ 12% from yesterday</p>
        </div>

        <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6 border-t-4 border-blue-500">
          <div class="flex items-center justify-between">
            <div>
              <p class="text-sm text-slate-600 dark:text-slate-400 font-medium">Error Rate</p>
              <p class="text-3xl font-bold text-slate-900 dark:text-white mt-2">0.02%</p>
            </div>
            <span class="text-4xl">❌</span>
          </div>
          <p class="text-xs text-green-600 dark:text-green-400 mt-4">↓ 0.5% from yesterday</p>
        </div>
      </div>

      <!-- Incident History -->
      <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-8 mb-8">
        <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-6 flex items-center gap-2">
          <span>📋</span> Incident History
        </h3>
        
        <div class="space-y-4">
          <div v-for="incident in incidents" :key="incident.id" class="border-l-4 p-4 rounded-r-lg" :class="incident.statusClass">
            <div class="flex items-start justify-between mb-2">
              <div class="flex items-center gap-3">
                <span class="text-2xl">{{ incident.icon }}</span>
                <div>
                  <h4 class="font-semibold text-slate-900 dark:text-white">{{ incident.title }}</h4>
                  <p class="text-sm text-slate-600 dark:text-slate-400">{{ incident.date }}</p>
                </div>
              </div>
              <span class="text-xs font-medium px-3 py-1 rounded-full" :class="incident.badgeClass">
                {{ incident.status }}
              </span>
            </div>
            <p class="text-sm text-slate-700 dark:text-slate-300 mb-2">{{ incident.description }}</p>
            <p class="text-xs text-slate-500 dark:text-slate-400">
              Duration: {{ incident.duration }} • Impact: {{ incident.impact }}
            </p>
          </div>
        </div>

        <button @click="showMoreIncidents = !showMoreIncidents" class="mt-6 text-blue-600 dark:text-blue-400 hover:text-blue-700 dark:hover:text-blue-300 font-medium text-sm">
          {{ showMoreIncidents ? '← Hide older incidents' : 'View older incidents →' }}
        </button>

        <!-- Older incidents (hidden by default) -->
        <transition name="fade">
          <div v-if="showMoreIncidents" class="mt-4 space-y-4 pt-4 border-t border-slate-200 dark:border-slate-700">
            <div v-for="incident in olderIncidents" :key="incident.id" class="border-l-4 p-4 rounded-r-lg" :class="incident.statusClass">
              <div class="flex items-start justify-between mb-2">
                <div class="flex items-center gap-3">
                  <span class="text-2xl">{{ incident.icon }}</span>
                  <div>
                    <h4 class="font-semibold text-slate-900 dark:text-white">{{ incident.title }}</h4>
                    <p class="text-sm text-slate-600 dark:text-slate-400">{{ incident.date }}</p>
                  </div>
                </div>
                <span class="text-xs font-medium px-3 py-1 rounded-full" :class="incident.badgeClass">
                  {{ incident.status }}
                </span>
              </div>
              <p class="text-sm text-slate-700 dark:text-slate-300 mb-2">{{ incident.description }}</p>
              <p class="text-xs text-slate-500 dark:text-slate-400">
                Duration: {{ incident.duration }} • Impact: {{ incident.impact }}
              </p>
            </div>
          </div>
        </transition>
      </div>

      <!-- Scheduled Maintenance -->
      <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-8 mb-8 border-l-4 border-yellow-500">
        <h3 class="text-xl font-bold text-slate-900 dark:text-white mb-6 flex items-center gap-2">
          <span>🔧</span> Scheduled Maintenance
        </h3>
        
        <div class="bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-200 dark:border-yellow-800 rounded-lg p-6">
          <div class="flex items-start gap-4">
            <span class="text-3xl">⏰</span>
            <div class="flex-1">
              <h4 class="font-semibold text-slate-900 dark:text-white mb-2">Infrastructure Upgrade</h4>
              <p class="text-sm text-slate-700 dark:text-slate-300 mb-3">
                <strong>Feb 15, 2026 · 02:00 - 04:00 BRT</strong>
              </p>
              <p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
                We will be performing a critical infrastructure upgrade to improve performance and reliability. 
                All services will be unavailable during this window.
              </p>
              <button @click="addToCalendar" class="text-sm font-medium text-yellow-700 dark:text-yellow-300 hover:text-yellow-800 dark:hover:text-yellow-200">
                📅 Add to Calendar
              </button>
            </div>
          </div>
        </div>
      </div>

      <!-- Subscribe & Resources -->
      <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6">
          <h3 class="text-lg font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
            <span>🔔</span> Stay Updated
          </h3>
          <p class="text-sm text-slate-600 dark:text-slate-400 mb-4">
            Subscribe to get real-time notifications about system incidents and scheduled maintenance.
          </p>
          <button @click="subscribeToUpdates" class="w-full bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white font-medium py-2 px-4 rounded-lg transition">
            🔔 Subscribe for Updates
          </button>
        </div>

        <div class="bg-white dark:bg-slate-800 rounded-lg shadow-lg p-6">
          <h3 class="text-lg font-bold text-slate-900 dark:text-white mb-4 flex items-center gap-2">
            <span>📊</span> Statistics
          </h3>
          <div class="space-y-3 text-sm">
            <div class="flex justify-between">
              <span class="text-slate-600 dark:text-slate-400">30-day uptime:</span>
              <span class="font-bold text-slate-900 dark:text-white">99.98%</span>
            </div>
            <div class="flex justify-between">
              <span class="text-slate-600 dark:text-slate-400">Last incident:</span>
              <span class="font-bold text-slate-900 dark:text-white">3 days ago</span>
            </div>
            <div class="flex justify-between">
              <span class="text-slate-600 dark:text-slate-400">Avg response time:</span>
              <span class="font-bold text-slate-900 dark:text-white">89ms</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Footer -->
      <div class="mt-12 pt-8 border-t border-slate-200 dark:border-slate-700 text-center">
        <p class="text-sm text-slate-600 dark:text-slate-400">
          For additional support, visit our <router-link to="/help" class="text-blue-600 dark:text-blue-400 hover:underline">Help Center</router-link> or 
          <a href="mailto:support@aevalo.app" class="text-blue-600 dark:text-blue-400 hover:underline">contact support</a>
        </p>
        <p class="text-xs text-slate-500 dark:text-slate-500 mt-4">
          Status page last updated: {{ updatedAt }}
        </p>
      </div>
    </div>

    <!-- Subscribe Modal -->
    <Teleport to="body">
      <Transition name="fade">
        <div v-if="showSubscribeModal" class="fixed inset-0 bg-black bg-opacity-50 dark:bg-opacity-70 flex items-center justify-center z-50 p-4">
          <div class="bg-white dark:bg-slate-800 rounded-lg shadow-2xl max-w-md w-full p-8 transform transition-all">
            <button @click="showSubscribeModal = false" class="absolute top-4 right-4 text-slate-500 hover:text-slate-700 dark:text-slate-400 dark:hover:text-slate-200">
              ✕
            </button>

            <h3 class="text-2xl font-bold text-slate-900 dark:text-white mb-2">Subscribe for Updates</h3>
            <p class="text-sm text-slate-600 dark:text-slate-400 mb-6">
              Receive notifications about system status changes and scheduled maintenance.
            </p>

            <div class="space-y-4 mb-6">
              <label class="flex items-center gap-3 p-3 border border-slate-200 dark:border-slate-700 rounded-lg cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-700 transition">
                <input type="checkbox" v-model="subscriptionSettings.email" class="w-4 h-4 rounded" />
                <span class="text-sm font-medium text-slate-900 dark:text-white">📧 Email Notifications</span>
              </label>

              <label class="flex items-center gap-3 p-3 border border-slate-200 dark:border-slate-700 rounded-lg cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-700 transition">
                <input type="checkbox" v-model="subscriptionSettings.slack" class="w-4 h-4 rounded" />
                <span class="text-sm font-medium text-slate-900 dark:text-white">💬 Slack Integration</span>
              </label>

              <label class="flex items-center gap-3 p-3 border border-slate-200 dark:border-slate-700 rounded-lg cursor-pointer hover:bg-slate-50 dark:hover:bg-slate-700 transition">
                <input type="checkbox" v-model="subscriptionSettings.push" class="w-4 h-4 rounded" />
                <span class="text-sm font-medium text-slate-900 dark:text-white">🔔 Push Notifications</span>
              </label>
            </div>

            <div class="flex gap-3">
              <button @click="showSubscribeModal = false" class="flex-1 px-4 py-2 border border-slate-300 dark:border-slate-600 text-slate-900 dark:text-white rounded-lg hover:bg-slate-50 dark:hover:bg-slate-700 transition font-medium">
                Cancel
              </button>
              <button @click="confirmSubscription" class="flex-1 px-4 py-2 bg-blue-600 hover:bg-blue-700 dark:bg-blue-700 dark:hover:bg-blue-600 text-white rounded-lg transition font-medium">
                Subscribe
              </button>
            </div>
          </div>
        </div>
      </Transition>
    </Teleport>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()

// State
const showMoreIncidents = ref(false)
const showSubscribeModal = ref(false)
const subscriptionSettings = ref({
  email: true,
  slack: false,
  push: false
})

// Computed
const lastCheckTime = computed(() => {
  const now = new Date()
  return now.toLocaleTimeString('pt-BR', { 
    hour: '2-digit', 
    minute: '2-digit'
  })
})

const updatedAt = computed(() => {
  const now = new Date()
  return now.toLocaleString('pt-BR', {
    day: '2-digit',
    month: '2-digit',
    year: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
})

// System Components Data
const components = ref([
  {
    id: 1,
    name: 'API Principal',
    service: 'Core API Services',
    icon: '🚀',
    uptime: 99.98
  },
  {
    id: 2,
    name: 'Dashboard Frontend',
    service: 'Web Application',
    icon: '🎨',
    uptime: 100
  },
  {
    id: 3,
    name: 'Database (Supabase)',
    service: 'PostgreSQL & Realtime',
    icon: '💾',
    uptime: 99.99
  },
  {
    id: 4,
    name: 'Gemini AI Integration',
    service: 'Generative AI',
    icon: '🤖',
    uptime: 98.5
  },
  {
    id: 5,
    name: 'Email Service',
    service: 'SendGrid Integration',
    icon: '📧',
    uptime: 99.95
  },
  {
    id: 6,
    name: 'Analytics Engine',
    service: 'Data Processing',
    icon: '📊',
    uptime: 100
  }
])

// Incidents Data
const incidents = ref([
  {
    id: 1,
    title: 'AI Generation Slowness',
    date: 'Feb 8, 2026',
    icon: '🟢',
    status: 'Resolved',
    description: 'Occasional delays in AI-powered question generation were resolved after optimizing Gemini API calls.',
    duration: '25 minutes',
    impact: 'Minor',
    statusClass: 'border-green-500 bg-green-50 dark:bg-green-900/20',
    badgeClass: 'bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-300'
  },
  {
    id: 2,
    title: 'Scheduled Maintenance',
    date: 'Feb 1, 2026',
    icon: '🟢',
    status: 'Resolved',
    description: 'Regular infrastructure maintenance and security updates were successfully completed.',
    duration: '2 hours',
    impact: 'Total',
    statusClass: 'border-green-500 bg-green-50 dark:bg-green-900/20',
    badgeClass: 'bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-300'
  }
])

const olderIncidents = ref([
  {
    id: 3,
    title: 'Database Connection Issues',
    date: 'Jan 25, 2026',
    icon: '🟢',
    status: 'Resolved',
    description: 'Temporary connectivity issues with Supabase were quickly resolved by their infrastructure team.',
    duration: '12 minutes',
    impact: 'Minor',
    statusClass: 'border-green-500 bg-green-50 dark:bg-green-900/20',
    badgeClass: 'bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-300'
  },
  {
    id: 4,
    title: 'Email Delivery Delays',
    date: 'Jan 18, 2026',
    icon: '🟢',
    status: 'Resolved',
    description: 'SendGrid experienced temporary queue processing delays affecting email notifications.',
    duration: '45 minutes',
    impact: 'Moderate',
    statusClass: 'border-green-500 bg-green-50 dark:bg-green-900/20',
    badgeClass: 'bg-green-100 dark:bg-green-900/50 text-green-800 dark:text-green-300'
  }
])

// Methods
const subscribeToUpdates = () => {
  showSubscribeModal.value = true
}

const confirmSubscription = () => {
  const settings = []
  if (subscriptionSettings.value.email) settings.push('Email')
  if (subscriptionSettings.value.slack) settings.push('Slack')
  if (subscriptionSettings.value.push) settings.push('Push')

  alert(`✅ Successfully subscribed! You will receive updates via: ${settings.join(', ')}`)
  showSubscribeModal.value = false
}

const addToCalendar = () => {
  // Generate iCal event
  const event = {
    title: 'Aevalo Infrastructure Upgrade',
    start: new Date('2026-02-15T02:00:00-03:00'),
    end: new Date('2026-02-15T04:00:00-03:00'),
    description: 'Critical infrastructure upgrade to improve performance and reliability.'
  }

  // For now, just show a confirmation
  alert('📅 Maintenance event added! (Integration with calendar apps coming soon)')
}

onMounted(() => {
  // Log page view
  console.log('[Status Page] Mounted - Monitoring system health')
  
  // Optionally fetch real status from API
  // fetchSystemStatus()
})
</script>

<style scoped>
/* Fade transition */
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.3s ease;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

/* Pulse animation for status indicator */
@keyframes pulse {
  0%, 100% {
    opacity: 1;
  }
  50% {
    opacity: 0.5;
  }
}

.animate-pulse {
  animation: pulse 2s cubic-bezier(0.4, 0, 0.6, 1) infinite;
}
</style>
