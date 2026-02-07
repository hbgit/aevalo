<template>
  <div class="relative" ref="dropdownRef">
    <!-- Trigger Button -->
    <button 
      @click="toggleDropdown"
      class="h-9 w-9 rounded-full bg-gradient-to-br from-secondary to-orange-600 text-white font-semibold flex items-center justify-center hover:shadow-lg transition-shadow focus:outline-none focus:ring-2 focus:ring-primary focus:ring-offset-2"
      :aria-expanded="isOpen"
      aria-haspopup="true"
    >
      {{ userInitials }}
    </button>

    <!-- Dropdown Menu -->
    <Transition
      enter-active-class="transition ease-out duration-100"
      enter-from-class="transform opacity-0 scale-95"
      enter-to-class="transform opacity-100 scale-100"
      leave-active-class="transition ease-in duration-75"
      leave-from-class="transform opacity-100 scale-100"
      leave-to-class="transform opacity-0 scale-95"
    >
      <div
        v-show="isOpen"
        class="absolute right-0 mt-2 w-56 rounded-lg shadow-lg bg-purple-700 text-white ring-1 ring-black ring-opacity-5 z-50"
        role="menu"
        aria-orientation="vertical"
      >
        <div class="py-1">
          <!-- Seção 1: Conta -->
          <a
            href="#"
            @click.prevent="handleSwitchAccount"
            class="flex items-center gap-3 px-4 py-2.5 text-sm hover:bg-purple-600 transition-colors"
            role="menuitem"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M8 7h12m0 0l-4-4m4 4l-4 4m0 6H4m0 0l4 4m-4-4l4-4" />
            </svg>
            <span>Trocar conta</span>
          </a>
          
          <a
            href="#"
            @click.prevent="handleManageAccount"
            class="flex items-center gap-3 px-4 py-2.5 text-sm hover:bg-purple-600 transition-colors"
            role="menuitem"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z" />
            </svg>
            <span>Gerenciar conta</span>
          </a>

          <!-- Divisor -->
          <hr class="border-white/20 my-1" />

          <!-- Seção 2: Preferências -->
          <a
            href="#"
            @click.prevent="handleSettings"
            class="flex items-center gap-3 px-4 py-2.5 text-sm hover:bg-purple-600 transition-colors"
            role="menuitem"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
            </svg>
            <span>Configurações</span>
          </a>

          <button
            @click="handleToggleDarkMode"
            class="w-full flex items-center gap-3 px-4 py-2.5 text-sm hover:bg-purple-600 transition-colors text-left"
            role="menuitem"
          >
            <svg v-if="!isDarkMode" class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M20.354 15.354A9 9 0 018.646 3.646 9.003 9.003 0 0012 21a9.003 9.003 0 008.354-5.646z" />
            </svg>
            <svg v-else class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 3v1m0 16v1m9-9h-1M4 12H3m15.364 6.364l-.707-.707M6.343 6.343l-.707-.707m12.728 0l-.707.707M6.343 17.657l-.707.707M16 12a4 4 0 11-8 0 4 4 0 018 0z" />
            </svg>
            <span>{{ isDarkMode ? 'Modo Claro' : 'Modo Dark' }}</span>
          </button>

          <!-- Divisor -->
          <hr class="border-white/20 my-1" />

          <!-- Seção 3: Saída -->
          <button
            @click="handleLogout"
            class="w-full flex items-center gap-3 px-4 py-2.5 text-sm hover:bg-red-600 transition-colors text-left"
            role="menuitem"
          >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1" />
            </svg>
            <span>Sair</span>
          </button>
        </div>
      </div>
    </Transition>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted, onUnmounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore } from '../stores/auth'

const router = useRouter()
const authStore = useAuthStore()

// State
const isOpen = ref(false)
const isDarkMode = ref(false)
const dropdownRef = ref<HTMLElement | null>(null)

// Computed
const userInitials = computed(() => {
  const user = authStore.user
  if (user?.name) {
    return user.name
      .split(' ')
      .map(word => word[0])
      .slice(0, 2)
      .join('')
      .toUpperCase()
  }
  return 'JD'
})

// Methods
const toggleDropdown = () => {
  isOpen.value = !isOpen.value
}

const closeDropdown = () => {
  isOpen.value = false
}

const handleSwitchAccount = () => {
  console.log('Trocar conta')
  closeDropdown()
  // TODO: Implementar lógica de troca de conta
}

const handleManageAccount = () => {
  console.log('Gerenciar conta')
  closeDropdown()
  router.push('/account')
}

const handleSettings = () => {
  console.log('Configurações')
  closeDropdown()
  router.push('/settings')
}

const handleToggleDarkMode = () => {
  isDarkMode.value = !isDarkMode.value
  console.log('Modo Dark:', isDarkMode.value)
  // TODO: Implementar lógica de dark mode
  // document.documentElement.classList.toggle('dark', isDarkMode.value)
}

const handleLogout = async () => {
  console.log('Sair')
  closeDropdown()
  await authStore.logout()
  router.push('/login')
}

// Click outside to close
const handleClickOutside = (event: MouseEvent) => {
  if (dropdownRef.value && !dropdownRef.value.contains(event.target as Node)) {
    closeDropdown()
  }
}

// Lifecycle
onMounted(() => {
  document.addEventListener('click', handleClickOutside)
})

onUnmounted(() => {
  document.removeEventListener('click', handleClickOutside)
})
</script>

<style scoped>
/* Estilos adicionais se necessário */
</style>
