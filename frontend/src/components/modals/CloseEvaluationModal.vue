<template>
  <div
    class="fixed inset-0 z-50 flex items-center justify-center p-4"
    @click.self="$emit('close')"
  >
    <!-- Backdrop -->
    <div class="absolute inset-0 bg-black/60 backdrop-blur-sm"></div>

    <!-- Dialog -->
    <div class="relative bg-slate-800 border border-red-600/30 rounded-2xl p-6 w-full max-w-md shadow-2xl">
      <!-- Icon -->
      <div class="flex items-center justify-center w-14 h-14 rounded-full bg-red-500/15 border border-red-500/30 mb-5 mx-auto">
        <span class="text-3xl">⚠️</span>
      </div>

      <h2 class="text-xl font-bold text-white text-center mb-1">Encerrar Avaliação?</h2>
      <p class="text-sm text-slate-400 text-center mb-6">
        Esta ação não pode ser desfeita.
      </p>

      <!-- Stats block -->
      <div class="bg-slate-700/50 border border-slate-600 rounded-xl p-4 mb-5">
        <div class="flex items-center justify-between text-sm">
          <span class="text-slate-400">Respostas coletadas</span>
          <span class="font-bold text-white text-lg">{{ responseCount }}</span>
        </div>
      </div>

      <!-- Consequences -->
      <ul class="space-y-2 mb-6">
        <li
          v-for="item in consequences"
          :key="item"
          class="flex items-start gap-2 text-sm text-slate-300"
        >
          <span class="text-red-400 mt-0.5">✕</span>
          {{ item }}
        </li>
      </ul>

      <!-- Buttons -->
      <div class="flex gap-3">
        <button
          @click="$emit('close')"
          :disabled="closing"
          class="flex-1 px-4 py-2.5 rounded-xl bg-slate-700 text-slate-200 hover:bg-slate-600 transition font-medium text-sm"
        >
          Cancelar
        </button>
        <button
          @click="handleConfirm"
          :disabled="closing"
          class="flex-1 px-4 py-2.5 rounded-xl bg-red-600 hover:bg-red-700 disabled:opacity-60 transition font-medium text-sm text-white flex items-center justify-center gap-2"
        >
          <span v-if="closing" class="w-4 h-4 border-2 border-white/30 border-t-white rounded-full animate-spin"></span>
          {{ closing ? 'Encerrando...' : '🔒 Sim, Encerrar' }}
        </button>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const props = defineProps<{
  evaluationId: string
  evaluationTitle: string
  responseCount: number
}>()

const emit = defineEmits<{
  close: []
  confirmed: []
}>()

const closing = ref(false)

const consequences = [
  'O link público será desativado imediatamente',
  'Novas respostas não serão aceitas',
  'Os resultados permanecerão acessíveis',
  'Um relatório final será gerado automaticamente',
]

const handleConfirm = async () => {
  try {
    closing.value = true
    const res = await fetch(`/api/evaluations/${props.evaluationId}/close`, {
      method: 'PATCH',
      headers: { 'Content-Type': 'application/json' },
    })
    if (!res.ok) throw new Error('Falha ao encerrar')
    emit('confirmed')
  } catch (err) {
    console.error(err)
    // Emit anyway so UI can handle optimistically if needed
    emit('confirmed')
  } finally {
    closing.value = false
  }
}
</script>
