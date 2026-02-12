<template>
  <Teleport to="body">
    <div v-if="isDev" class="dev-menu-container">
      <!-- Toggle Button -->
      <button
        @click="isOpen = !isOpen"
        class="dev-menu-toggle"
        :class="{ 'menu-open': isOpen }"
        title="Dev Menu (Alt+D)"
      >
        <span class="toggle-icon">🛠️</span>
        <span v-if="isOpen" class="toggle-text">✕</span>
      </button>

      <!-- Menu Panel -->
      <Transition name="slide-fade">
        <div v-if="isOpen" class="dev-menu-panel">
          <!-- Header -->
          <div class="menu-header">
            <h3 class="menu-title">
              <span class="title-icon">⚡</span>
              Dev Tools
            </h3>
            <button @click="isOpen = false" class="close-btn">
              ✕
            </button>
          </div>

          <!-- Sections -->
          <div class="menu-content">
            <!-- Error Pages Section -->
            <div class="menu-section">
              <h4 class="section-title">🚨 Error Pages</h4>
              <div class="button-grid">
                <button @click="navigateTo('ServerError')" class="menu-btn error-500">
                  <span class="btn-icon">💥</span>
                  <span class="btn-label">500 Error</span>
                </button>
                <button @click="navigateTo('NotFound')" class="menu-btn error-404">
                  <span class="btn-icon">🔍</span>
                  <span class="btn-label">404 Error</span>
                </button>
                <button @click="navigateTo('ServiceUnavailable')" class="menu-btn error-503">
                  <span class="btn-icon">🔧</span>
                  <span class="btn-label">503 Error</span>
                </button>
                <button @click="navigateTo('Unauthorized')" class="menu-btn error-403">
                  <span class="btn-icon">🔒</span>
                  <span class="btn-label">403 Error</span>
                </button>
              </div>
            </div>

            <!-- Quick Actions Section -->
            <div class="menu-section">
              <h4 class="section-title">⚡ Quick Actions</h4>
              <div class="button-grid">
                <button @click="clearLocalStorage" class="menu-btn action-clear">
                  <span class="btn-icon">🗑️</span>
                  <span class="btn-label">Clear Storage</span>
                </button>
                <button @click="toggleDarkMode" class="menu-btn action-theme">
                  <span class="btn-icon">{{ isDarkMode ? '☀️' : '🌙' }}</span>
                  <span class="btn-label">Toggle Theme</span>
                </button>
                <button @click="reloadPage" class="menu-btn action-reload">
                  <span class="btn-icon">🔄</span>
                  <span class="btn-label">Hard Reload</span>
                </button>
                <button @click="copyCurrentUrl" class="menu-btn action-copy">
                  <span class="btn-icon">📋</span>
                  <span class="btn-label">Copy URL</span>
                </button>
              </div>
            </div>

            <!-- Navigation Section -->
            <div class="menu-section">
              <h4 class="section-title">🧭 Navigation</h4>
              <div class="button-grid">
                <button @click="navigateTo('Dashboard')" class="menu-btn nav-btn">
                  <span class="btn-icon">🏠</span>
                  <span class="btn-label">Dashboard</span>
                </button>
                <button @click="navigateTo('CreateEvaluation')" class="menu-btn nav-btn">
                  <span class="btn-icon">➕</span>
                  <span class="btn-label">Create</span>
                </button>
                <button @click="navigateTo('Login')" class="menu-btn nav-btn">
                  <span class="btn-icon">🔐</span>
                  <span class="btn-label">Login</span>
                </button>
                <button @click="goBack" class="menu-btn nav-btn">
                  <span class="btn-icon">⬅️</span>
                  <span class="btn-label">Go Back</span>
                </button>
              </div>
            </div>

            <!-- Info Section -->
            <div class="menu-section info-section">
              <div class="info-item">
                <span class="info-label">Route:</span>
                <code class="info-value">{{ currentRoute }}</code>
              </div>
              <div class="info-item">
                <span class="info-label">Mode:</span>
                <code class="info-value">{{ mode }}</code>
              </div>
            </div>
          </div>

          <!-- Footer -->
          <div class="menu-footer">
            <span class="footer-text">Press <kbd>Alt + D</kbd> to toggle</span>
          </div>
        </div>
      </Transition>
    </div>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter, useRoute } from 'vue-router'

const router = useRouter()
const route = useRoute()

// Show DevMenu sempre (incluindo produção para testes)
const isDev = true
const isOpen = ref(false)

const currentRoute = computed(() => route.path)
const mode = computed(() => import.meta.env.MODE)

// Dark mode state (simplified - adjust based on your theme implementation)
const isDarkMode = ref(false)

onMounted(() => {
  // Check initial dark mode state
  isDarkMode.value = document.documentElement.classList.contains('dark')
  
  // Keyboard shortcut: Alt + D
  window.addEventListener('keydown', handleKeyboard)
})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyboard)
})

const handleKeyboard = (e: KeyboardEvent) => {
  // Alt + D (aceita tanto 'd' quanto 'D')
  if (e.altKey && (e.key === 'd' || e.key === 'D')) {
    e.preventDefault()
    isOpen.value = !isOpen.value
    return
  }
  
  // ESC to close
  if (e.key === 'Escape' && isOpen.value) {
    e.preventDefault()
    isOpen.value = false
  }
}

const navigateTo = (routeNameOrPath: string) => {
  if (routeNameOrPath.startsWith('/')) {
    router.push(routeNameOrPath)
  } else {
    router.push({ name: routeNameOrPath })
  }
  isOpen.value = false
}

const goBack = () => {
  router.go(-1)
  isOpen.value = false
}

const clearLocalStorage = () => {
  if (confirm('Clear all localStorage data?')) {
    localStorage.clear()
    sessionStorage.clear()
    alert('✅ Storage cleared!')
  }
}

const toggleDarkMode = () => {
  document.documentElement.classList.toggle('dark')
  isDarkMode.value = document.documentElement.classList.contains('dark')
}

const reloadPage = () => {
  window.location.reload()
}

const copyCurrentUrl = async () => {
  try {
    await navigator.clipboard.writeText(window.location.href)
    alert('✅ URL copied to clipboard!')
  } catch (err) {
    console.error('Failed to copy:', err)
  }
}
</script>

<style scoped>
/* Container */
.dev-menu-container {
  position: fixed;
  bottom: 24px;
  right: 24px;
  z-index: 9999;
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
}

/* Toggle Button */
.dev-menu-toggle {
  width: 56px;
  height: 56px;
  border-radius: 50%;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  border: none;
  color: white;
  font-size: 24px;
  cursor: pointer;
  box-shadow: 0 4px 12px rgba(102, 126, 234, 0.4);
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.dev-menu-toggle:hover {
  transform: scale(1.1);
  box-shadow: 0 6px 20px rgba(102, 126, 234, 0.6);
}

.dev-menu-toggle:active {
  transform: scale(0.95);
}

.dev-menu-toggle.menu-open {
  background: linear-gradient(135deg, #f093fb 0%, #f5576c 100%);
}

.toggle-icon {
  display: block;
  transition: opacity 0.2s;
}

.menu-open .toggle-icon {
  display: none;
}

.toggle-text {
  display: none;
  font-size: 28px;
}

.menu-open .toggle-text {
  display: block;
}

/* Menu Panel */
.dev-menu-panel {
  position: absolute;
  bottom: 70px;
  right: 0;
  width: 380px;
  max-height: 80vh;
  background: white;
  border-radius: 16px;
  box-shadow: 0 20px 60px rgba(0, 0, 0, 0.3);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

@media (prefers-color-scheme: dark) {
  .dev-menu-panel {
    background: #1f2937;
    color: #f3f4f6;
  }
}

/* Header */
.menu-header {
  padding: 20px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.menu-title {
  margin: 0;
  font-size: 18px;
  font-weight: 700;
  display: flex;
  align-items: center;
  gap: 8px;
}

.title-icon {
  font-size: 20px;
}

.close-btn {
  background: rgba(255, 255, 255, 0.2);
  border: none;
  color: white;
  width: 32px;
  height: 32px;
  border-radius: 8px;
  cursor: pointer;
  font-size: 20px;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: background 0.2s;
}

.close-btn:hover {
  background: rgba(255, 255, 255, 0.3);
}

/* Content */
.menu-content {
  padding: 16px;
  overflow-y: auto;
  flex: 1;
}

.menu-section {
  margin-bottom: 24px;
}

.menu-section:last-child {
  margin-bottom: 0;
}

.section-title {
  font-size: 14px;
  font-weight: 600;
  margin: 0 0 12px 0;
  color: #6b7280;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

@media (prefers-color-scheme: dark) {
  .section-title {
    color: #9ca3af;
  }
}

/* Button Grid */
.button-grid {
  display: grid;
  grid-template-columns: repeat(2, 1fr);
  gap: 8px;
}

.menu-btn {
  padding: 12px 16px;
  border: 2px solid #e5e7eb;
  border-radius: 12px;
  background: #f9fafb;
  cursor: pointer;
  transition: all 0.2s ease;
  display: flex;
  align-items: center;
  gap: 8px;
  position: relative;
  font-size: 14px;
  font-weight: 500;
}

@media (prefers-color-scheme: dark) {
  .menu-btn {
    background: #374151;
    border-color: #4b5563;
    color: #f3f4f6;
  }
}

.menu-btn:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.menu-btn:active:not(:disabled) {
  transform: translateY(0);
}

.menu-btn:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-icon {
  font-size: 18px;
}

.btn-label {
  flex: 1;
  text-align: left;
}

.badge {
  position: absolute;
  top: 4px;
  right: 4px;
  background: #fbbf24;
  color: #78350f;
  font-size: 10px;
  padding: 2px 6px;
  border-radius: 6px;
  font-weight: 700;
}

/* Error Buttons */
.error-500:hover:not(:disabled) {
  border-color: #ef4444;
  background: #fee2e2;
}

.error-404:hover:not(:disabled) {
  border-color: #f97316;
  background: #ffedd5;
}

.error-503:hover:not(:disabled) {
  border-color: #eab308;
  background: #fef9c3;
}

.error-403:hover:not(:disabled) {
  border-color: #ec4899;
  background: #fce7f3;
}

/* Action Buttons */
.action-clear:hover:not(:disabled) {
  border-color: #ef4444;
  background: #fee2e2;
}

.action-theme:hover:not(:disabled) {
  border-color: #8b5cf6;
  background: #ede9fe;
}

.action-reload:hover:not(:disabled) {
  border-color: #3b82f6;
  background: #dbeafe;
}

.action-copy:hover:not(:disabled) {
  border-color: #10b981;
  background: #d1fae5;
}

/* Info Section */
.info-section {
  background: #f3f4f6;
  border-radius: 12px;
  padding: 12px;
}

@media (prefers-color-scheme: dark) {
  .info-section {
    background: #374151;
  }
}

.info-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 8px;
  font-size: 13px;
}

.info-item:last-child {
  margin-bottom: 0;
}

.info-label {
  font-weight: 600;
  color: #6b7280;
}

@media (prefers-color-scheme: dark) {
  .info-label {
    color: #9ca3af;
  }
}

.info-value {
  background: white;
  padding: 4px 8px;
  border-radius: 6px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 12px;
  color: #4b5563;
}

@media (prefers-color-scheme: dark) {
  .info-value {
    background: #1f2937;
    color: #d1d5db;
  }
}

/* Footer */
.menu-footer {
  padding: 12px 20px;
  background: #f9fafb;
  border-top: 1px solid #e5e7eb;
  text-align: center;
}

@media (prefers-color-scheme: dark) {
  .menu-footer {
    background: #374151;
    border-top-color: #4b5563;
  }
}

.footer-text {
  font-size: 12px;
  color: #6b7280;
}

@media (prefers-color-scheme: dark) {
  .footer-text {
    color: #9ca3af;
  }
}

kbd {
  background: white;
  border: 1px solid #d1d5db;
  border-radius: 4px;
  padding: 2px 6px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 11px;
  font-weight: 600;
  color: #4b5563;
}

@media (prefers-color-scheme: dark) {
  kbd {
    background: #1f2937;
    border-color: #4b5563;
    color: #d1d5db;
  }
}

/* Transitions */
.slide-fade-enter-active {
  transition: all 0.3s ease-out;
}

.slide-fade-leave-active {
  transition: all 0.2s ease-in;
}

.slide-fade-enter-from {
  transform: translateY(20px);
  opacity: 0;
}

.slide-fade-leave-to {
  transform: translateY(20px);
  opacity: 0;
}

/* Scrollbar */
.menu-content::-webkit-scrollbar {
  width: 8px;
}

.menu-content::-webkit-scrollbar-track {
  background: #f3f4f6;
  border-radius: 8px;
}

.menu-content::-webkit-scrollbar-thumb {
  background: #d1d5db;
  border-radius: 8px;
}

.menu-content::-webkit-scrollbar-thumb:hover {
  background: #9ca3af;
}

@media (prefers-color-scheme: dark) {
  .menu-content::-webkit-scrollbar-track {
    background: #374151;
  }
  
  .menu-content::-webkit-scrollbar-thumb {
    background: #4b5563;
  }
  
  .menu-content::-webkit-scrollbar-thumb:hover {
    background: #6b7280;
  }
}

/* Mobile Responsive */
@media (max-width: 640px) {
  .dev-menu-panel {
    width: calc(100vw - 48px);
    max-width: 380px;
  }
}
</style>
