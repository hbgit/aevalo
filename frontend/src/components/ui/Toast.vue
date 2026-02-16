<template>
  <Teleport to="body">
    <Transition name="toast-slide">
      <div
        v-if="show"
        :class="[
          'fixed bottom-6 right-6 max-w-sm w-full rounded-lg shadow-lg p-4 flex items-start gap-3 z-50',
          toastClasses
        ]"
      >
        <span class="text-xl flex-shrink-0">{{ icon }}</span>
        <div class="flex-1">
          <p class="font-semibold text-sm">{{ title }}</p>
          <p v-if="message" class="text-xs mt-1 opacity-90">{{ message }}</p>
        </div>
        <button
          @click="closeToast"
          class="text-xs opacity-70 hover:opacity-100 transition"
        >
          ✕
        </button>
      </div>
    </Transition>
  </Teleport>
</template>

<script setup lang="ts">
import { ref, computed, watch, onMounted } from 'vue'

type ToastType = 'success' | 'error' | 'warning' | 'info' | 'loading'

interface Props {
  show?: boolean
  type?: ToastType
  title?: string
  message?: string
  duration?: number
  icon?: string
}

const props = withDefaults(defineProps<Props>(), {
  show: false,
  type: 'info',
  title: 'Notificação',
  duration: 4000,
  icon: ''
})

defineEmits<{
  close: []
}>()

const localShow = ref(props.show)

watch(() => props.show, (newVal) => {
  localShow.value = newVal
})

const toastClasses = computed(() => {
  const types = {
    success: 'bg-green-100 dark:bg-green-900/30 text-green-800 dark:text-green-200',
    error: 'bg-red-100 dark:bg-red-900/30 text-red-800 dark:text-red-200',
    warning: 'bg-yellow-100 dark:bg-yellow-900/30 text-yellow-800 dark:text-yellow-200',
    info: 'bg-blue-100 dark:bg-blue-900/30 text-blue-800 dark:text-blue-200',
    loading: 'bg-slate-100 dark:bg-slate-800 text-slate-800 dark:text-slate-200'
  }
  return types[props.type]
})

const defaultIcons = {
  success: '✅',
  error: '❌',
  warning: '⚠️',
  info: 'ℹ️',
  loading: '⏳'
}

const displayIcon = computed(() => {
  return props.icon || defaultIcons[props.type]
})

const closeToast = () => {
  localShow.value = false
}

onMounted(() => {
  if (localShow.value && props.duration > 0 && props.type !== 'loading') {
    setTimeout(() => {
      closeToast()
    }, props.duration)
  }
})
</script>

<style scoped>
.toast-slide-enter-active,
.toast-slide-leave-active {
  transition: transform 0.3s ease, opacity 0.3s ease;
}

.toast-slide-enter-from,
.toast-slide-leave-to {
  transform: translateX(400px);
  opacity: 0;
}
</style>
