<template>
  <div class="space-y-4 p-4 rounded-lg bg-slate-700/50 border border-slate-600">
    <!-- Question Text Input -->
    <div class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Pergunta</label>
      <input
        :value="question.text"
        @input="updateQuestion('text', ($event.target as HTMLInputElement).value)"
        type="text"
        placeholder="Digite a pergunta..."
        class="w-full px-4 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white placeholder-slate-400 focus:border-purple-500 focus:outline-none transition-colors"
      />
    </div>

    <!-- Scale Type Selector -->
    <div class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Tipo de Escala</label>
      <select
        :value="question.type"
        @change="updateQuestion('type', ($event.target as HTMLSelectElement).value)"
        class="w-full px-4 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white focus:border-purple-500 focus:outline-none transition-colors cursor-pointer"
      >
        <option value="likert">Likert (1-5)</option>
        <option value="frequency">Frequência</option>
        <option value="paired">Comparação Pareada</option>
        <option value="fixed_sum">Distribuir 100 Pontos</option>
        <option value="text">Texto Aberto</option>
        <option value="multiple_choice">Múltipla Escolha</option>
        <option value="single_choice">Escolha Única</option>
      </select>
    </div>

    <!-- Likert Options -->
    <div v-if="question.type === 'likert'" class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Escala Likert</label>
      <select
        :value="question.config?.scale || 5"
        @change="updateQuestion('config', { ...question.config, scale: parseInt(($event.target as HTMLSelectElement).value) })"
        class="w-full px-4 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white focus:border-purple-500 focus:outline-none"
      >
        <option value="5">1 a 5</option>
        <option value="7">1 a 7</option>
        <option value="10">1 a 10</option>
      </select>

      <div class="grid grid-cols-2 gap-2">
        <div>
          <label class="text-xs text-slate-400">Label Mínimo</label>
          <input
            :value="question.config?.minLabel || 'Discordo Totalmente'"
            @input="updateQuestion('config', { ...question.config, minLabel: ($event.target as HTMLInputElement).value })"
            type="text"
            class="w-full px-2 py-1 rounded bg-slate-600 border border-slate-500 text-white text-xs focus:border-purple-500 focus:outline-none"
          />
        </div>
        <div>
          <label class="text-xs text-slate-400">Label Máximo</label>
          <input
            :value="question.config?.maxLabel || 'Concordo Totalmente'"
            @input="updateQuestion('config', { ...question.config, maxLabel: ($event.target as HTMLInputElement).value })"
            type="text"
            class="w-full px-2 py-1 rounded bg-slate-600 border border-slate-500 text-white text-xs focus:border-purple-500 focus:outline-none"
          />
        </div>
      </div>
    </div>

    <!-- Fixed Sum Options -->
    <div v-if="question.type === 'fixed_sum'" class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Opções (uma por linha)</label>
      <textarea
        :value="(question.config?.options || []).join('\n')"
        @input="updateQuestion('config', { ...question.config, options: ($event.target as HTMLTextAreaElement).value.split('\n').filter(o => o.trim()) })"
        rows="4"
        placeholder="Atributo 1&#10;Atributo 2&#10;Atributo 3"
        class="w-full px-3 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white placeholder-slate-400 focus:border-purple-500 focus:outline-none resize-none text-sm"
      />
    </div>

    <!-- Multiple Choice / Single Choice Options -->
    <div v-if="['multiple_choice', 'single_choice'].includes(question.type)" class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Opções (uma por linha)</label>
      <textarea
        :value="(question.options || []).join('\n')"
        @input="updateQuestion('options', ($event.target as HTMLTextAreaElement).value.split('\n').filter(o => o.trim()))"
        rows="4"
        placeholder="Opção 1&#10;Opção 2&#10;Opção 3"
        class="w-full px-3 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white placeholder-slate-400 focus:border-purple-500 focus:outline-none resize-none text-sm"
      />

      <!-- Min/Max selections for multiple choice -->
      <div v-if="question.type === 'multiple_choice'" class="grid grid-cols-2 gap-2">
        <div>
          <label class="text-xs text-slate-400">Seleções Mínimas</label>
          <input
            :value="question.config?.minSelections || 0"
            @input="updateQuestion('config', { ...question.config, minSelections: parseInt(($event.target as HTMLInputElement).value) })"
            type="number"
            min="0"
            class="w-full px-2 py-1 rounded bg-slate-600 border border-slate-500 text-white text-xs focus:border-purple-500 focus:outline-none"
          />
        </div>
        <div>
          <label class="text-xs text-slate-400">Seleções Máximas</label>
          <input
            :value="question.config?.maxSelections || 0"
            @input="updateQuestion('config', { ...question.config, maxSelections: parseInt(($event.target as HTMLInputElement).value) })"
            type="number"
            min="0"
            class="w-full px-2 py-1 rounded bg-slate-600 border border-slate-500 text-white text-xs focus:border-purple-500 focus:outline-none"
          />
        </div>
      </div>
    </div>

    <!-- Open Text Max Length -->
    <div v-if="question.type === 'text'" class="space-y-2">
      <label class="block text-sm font-medium text-slate-200">Limite de Caracteres</label>
      <input
        :value="question.config?.maxLength || 500"
        @input="updateQuestion('config', { ...question.config, maxLength: parseInt(($event.target as HTMLInputElement).value) })"
        type="number"
        min="10"
        max="5000"
        class="w-full px-4 py-2 rounded-lg bg-slate-600 border border-slate-500 text-white focus:border-purple-500 focus:outline-none"
      />
    </div>

    <!-- Required Toggle -->
    <div class="flex items-center gap-3 p-3 rounded-lg bg-slate-600/50 border border-slate-600">
      <input
        type="checkbox"
        :checked="question.required"
        @change="updateQuestion('required', ($event.target as HTMLInputElement).checked)"
        class="w-4 h-4 rounded accent-purple-600 cursor-pointer"
      />
      <label class="text-sm text-slate-200 cursor-pointer flex-1">Pergunta Obrigatória</label>
    </div>

    <!-- Preview -->
    <div class="space-y-3 p-3 rounded-lg bg-slate-600/30 border border-slate-600/50">
      <p class="text-xs font-semibold text-slate-400 uppercase">Prévia</p>
      <div class="text-sm text-slate-200 mb-3">{{ question.text || '[Pergunta aqui]' }}</div>

      <!-- Render preview based on type -->
      <div class="text-xs">
        <p v-if="question.type === 'likert'" class="text-slate-400">
          Likert 1-{{ question.config?.scale || 5 }}: {{ question.config?.minLabel || 'Min' }} até {{ question.config?.maxLabel || 'Max' }}
        </p>
        <p v-else-if="question.type === 'frequency'" class="text-slate-400">
          Frequência: Nunca | Raramente | Às vezes | Frequentemente | Sempre
        </p>
        <p v-else-if="question.type === 'paired'" class="text-slate-400">
          Comparação entre 2 opções
        </p>
        <p v-else-if="question.type === 'fixed_sum'" class="text-slate-400">
          Distribuir 100 pontos entre {{ (question.config?.options || []).length }} opções
        </p>
        <p v-else-if="question.type === 'text'" class="text-slate-400">
          Texto aberto (máx {{ question.config?.maxLength || 500 }} caracteres)
        </p>
        <p v-else-if="question.type === 'multiple_choice'" class="text-slate-400">
          Múltipla Escolha: {{ (question.options || []).length }} opções
        </p>
        <p v-else-if="question.type === 'single_choice'" class="text-slate-400">
          Escolha Única: {{ (question.options || []).length }} opções
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
interface Question {
  id?: string
  text: string
  type: string
  required: boolean
  options?: string[]
  config?: Record<string, any>
}

interface Props {
  question: Question
}

interface Emits {
  (e: 'update:question', value: Question): void
}

const props = defineProps<Props>()

const emit = defineEmits<Emits>()

const updateQuestion = (field: string, value: any) => {
  const updated = {
    ...props.question,
    [field]: value
  }
  emit('update:question', updated)
}
</script>

<style scoped>
textarea {
  resize: none;
}
</style>
