<template>
  <teleport to="body">
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50 backdrop-blur-sm">
      <div class="bg-gradient-to-br from-slate-800 to-slate-900 rounded-2xl border border-slate-700 max-w-md w-full mx-4 p-8 shadow-2xl">
        <!-- Success Icon -->
        <div class="flex justify-center mb-6">
          <div class="w-16 h-16 bg-gradient-to-br from-green-500 to-emerald-600 rounded-full flex items-center justify-center animate-pulse">
            <svg class="w-8 h-8 text-white" fill="currentColor" viewBox="0 0 20 20">
              <path fill-rule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm3.707-9.293a1 1 0 00-1.414-1.414L9 10.586 7.707 9.293a1 1 0 00-1.414 1.414l2 2a1 1 0 001.414 0l4-4z" clip-rule="evenodd" />
            </svg>
          </div>
        </div>

        <!-- Title -->
        <h2 class="text-2xl font-bold text-white text-center mb-2">✅ Resposta Enviada!</h2>

        <!-- Message -->
        <p class="text-slate-400 text-center mb-8">
          Obrigado por participar da avaliação. Suas respostas foram salvas com sucesso.
        </p>

        <!-- Stats -->
        <div class="bg-slate-700/30 rounded-lg p-4 mb-8 border border-slate-600">
          <p class="text-sm text-slate-400 text-center">
            Você respondeu todas as perguntas corretamente e suas respostas foram registradas.
          </p>
        </div>

        <!-- Actions -->
        <div class="space-y-3">
          <button
            @click="$emit('close')"
            class="w-full px-6 py-3 rounded-lg bg-gradient-to-r from-purple-600 to-purple-500 text-white font-medium hover:shadow-lg hover:shadow-purple-600/50 transition"
          >
            Fechar
          </button>
          <button
            @click="$emit('close')"
            class="w-full px-6 py-3 rounded-lg text-slate-300 bg-slate-700/50 border border-slate-600 hover:bg-slate-700 transition"
          >
            Responder Outra Avaliação
          </button>
        </div>

        <!-- Footer Info -->
        <p class="text-xs text-slate-500 text-center mt-6">
          Avaliação encerrada. Obrigado pela sua participação!
        </p>
      </div>
    </div>
  </teleport>
</template>

<script setup lang="ts">
defineEmits<{
  close: []
}>()
</script>
