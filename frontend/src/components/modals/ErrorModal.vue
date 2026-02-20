<template>
  <teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div class="bg-gradient-to-br from-slate-800 to-slate-900 rounded-2xl border border-slate-700 max-w-md w-full mx-4 p-8 shadow-2xl">
        <!-- Error Icon -->
        <div class="flex justify-center mb-6">
          <div class="w-16 h-16 bg-gradient-to-br from-red-500 to-red-600 rounded-full flex items-center justify-center">
            <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4v.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
            </svg>
          </div>
        </div>

        <!-- Title -->
        <h2 class="text-2xl font-bold text-white text-center mb-2">Erro ao Enviar</h2>

        <!-- Error Message -->
        <div class="bg-red-500/10 rounded-lg p-4 mb-6 border border-red-500/30">
          <p class="text-red-400 text-center text-sm">
            {{ error }}
          </p>
        </div>

        <!-- Status Info -->
        <div class="bg-slate-700/30 rounded-lg p-4 mb-8 border border-slate-600">
          <p class="text-xs text-slate-400 text-center">
            <span class="inline-block">⚠️ Suas respostas foram salvas localmente.</span>
          </p>
        </div>

        <!-- Actions -->
        <div class="space-y-3">
          <button
            @click="$emit('retry')"
            class="w-full px-6 py-3 rounded-lg bg-gradient-to-r from-purple-600 to-purple-500 text-white font-medium hover:shadow-lg hover:shadow-purple-600/50 transition"
          >
            🔄 Tentar Novamente
          </button>
          <button
            @click="$emit('close')"
            class="w-full px-6 py-3 rounded-lg text-slate-300 bg-slate-700/50 border border-slate-600 hover:bg-slate-700 transition"
          >
            Fechar
          </button>
        </div>

        <!-- Footer Info -->
        <p class="text-xs text-slate-500 text-center mt-6">
          Se o problema persistir, tente recarregar a página ou entre em contato com o suporte.
        </p>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
interface Props {
  error: string
}

defineProps<Props>()
defineEmits<{
  retry: []
  close: []
}>()
</script>
