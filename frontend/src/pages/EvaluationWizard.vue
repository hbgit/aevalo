<!-- Evaluation Creation Wizard - Multi-Step -->
<template>
  <div class="min-h-screen bg-gradient-to-br from-slate-900 via-purple-900 to-slate-900 py-12 px-4">
    <div class="max-w-4xl mx-auto">
      <!-- Progress Bar -->
      <div class="mb-8">
        <div class="flex items-center justify-center gap-4 mb-4">
          <div
            v-for="(step, index) in steps"
            :key="index"
            :class="[
              'h-3 w-3 rounded-full transition-all duration-300',
              index + 1 <= currentStep
                ? 'bg-primary w-8'
                : 'bg-slate-600'
            ]"
          />
        </div>
        <p class="text-center text-sm text-slate-400">
          Passo {{ currentStep }} de {{ steps.length }}: {{ steps[currentStep - 1] }}
        </p>
      </div>

      <!-- Step 1: Choose Method -->
      <div v-if="currentStep === 1" class="animate-fadeIn space-y-6">
        <div class="text-center mb-8">
          <h1 class="text-4xl font-bold text-white mb-2">Crie sua Avaliação</h1>
          <p class="text-slate-400">Escolha como você prefere começar</p>
        </div>

        <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
          <!-- Template Card -->
          <button
            @click="selectMethod('template')"
            class="group relative p-8 rounded-2xl border-2 border-slate-700 bg-slate-800/50 hover:border-primary hover:bg-slate-800 transition-all duration-300 text-left"
          >
            <div class="absolute top-0 right-0 w-1 h-full bg-gradient-to-b from-primary to-transparent rounded-r-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
            <span class="text-5xl mb-4 block">📋</span>
            <h2 class="text-2xl font-bold text-white mb-2">Começar com Template</h2>
            <p class="text-slate-400 mb-4">Escolha entre modelos prontos e customize</p>
            <div class="space-y-2 text-sm text-slate-300">
              <div class="flex items-center gap-2">
                <span class="text-primary">✓</span>
                <span>Rápido</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-primary">✓</span>
                <span>Testado</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-primary">✓</span>
                <span>Categorizado</span>
              </div>
            </div>
            <div class="mt-6 pt-6 border-t border-slate-700">
              <span class="inline-block px-3 py-1 rounded-full text-sm font-semibold text-primary bg-primary/10">
                Escolher Template
              </span>
            </div>
          </button>

          <!-- AI Card -->
          <button
            @click="selectMethod('ai')"
            class="group relative p-8 rounded-2xl border-2 border-slate-700 bg-slate-800/50 hover:border-secondary hover:bg-slate-800 transition-all duration-300 text-left"
          >
            <div class="absolute top-0 right-0 w-1 h-full bg-gradient-to-b from-secondary to-transparent rounded-r-2xl opacity-0 group-hover:opacity-100 transition-opacity" />
            <span class="text-5xl mb-4 block">✨</span>
            <h2 class="text-2xl font-bold text-white mb-2">Criar com IA</h2>
            <p class="text-slate-400 mb-4">Descreva o que precisa e deixe a IA criar</p>
            <div class="space-y-2 text-sm text-slate-300">
              <div class="flex items-center gap-2">
                <span class="text-secondary">✓</span>
                <span>Personalizado</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-secondary">✓</span>
                <span>Inteligente</span>
              </div>
              <div class="flex items-center gap-2">
                <span class="text-secondary">✓</span>
                <span>Criativo</span>
              </div>
            </div>
            <div class="mt-6 pt-6 border-t border-slate-700">
              <span class="inline-block px-3 py-1 rounded-full text-sm font-semibold text-secondary bg-secondary/10">
                Gerar com Gemini
              </span>
            </div>
          </button>
        </div>
      </div>

      <!-- Step 2A: Select Template -->
      <div v-if="currentStep === 2 && selectedMethod === 'template'" class="animate-fadeIn space-y-6">
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-white mb-2">Escolha um Template</h1>
          <p class="text-slate-400">Selecione o que melhor se adequa às suas necessidades</p>
        </div>

        <!-- Search & Filter -->
        <div class="flex flex-col sm:flex-row gap-4">
          <input
            v-model="templateSearch"
            type="text"
            placeholder="🔍 Buscar templates..."
            class="flex-1 px-4 py-2 rounded-lg bg-slate-800 border border-slate-700 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition"
          />
          <select
            v-model="templateCategory"
            class="px-4 py-2 rounded-lg bg-slate-800 border border-slate-700 text-white focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition"
          >
            <option value="">Todas as Categorias</option>
            <option value="hr">RH</option>
            <option value="ux">UX/Product</option>
            <option value="customer">Customer Success</option>
            <option value="education">Educação</option>
            <option value="marketing">Marketing</option>
          </select>
        </div>

        <!-- Templates Grid -->
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
          <div
            v-for="template in filteredTemplates"
            :key="template.id"
            @click="selectTemplate(template)"
            :class="[
              'p-6 rounded-xl border-2 cursor-pointer transition-all duration-300',
              selectedTemplate?.id === template.id
                ? 'border-primary bg-primary/10'
                : 'border-slate-700 bg-slate-800/50 hover:border-slate-600 hover:bg-slate-800'
            ]"
          >
            <div class="flex items-start justify-between mb-3">
              <span class="text-3xl">{{ template.icon }}</span>
              <span v-if="template.popular" class="text-xs font-semibold px-2 py-1 rounded-full bg-secondary/20 text-secondary">
                Mais usado
              </span>
            </div>
            <h3 class="font-bold text-white mb-1">{{ template.name }}</h3>
            <p class="text-sm text-slate-400 mb-3">{{ template.description }}</p>
            <div class="space-y-2 mb-4">
              <p v-for="(question, idx) in template.previewQuestions.slice(0, 2)" :key="idx" class="text-xs text-slate-500 line-clamp-1">
                • {{ question }}
              </p>
            </div>
            <p class="text-xs text-slate-600">{{ template.questionCount }} perguntas</p>
          </div>
        </div>

        <!-- Navigation -->
        <div class="flex gap-4 mt-8">
          <button
            @click="previousStep()"
            class="px-6 py-2 rounded-lg border border-slate-700 text-white hover:bg-slate-800 transition"
          >
            ← Voltar
          </button>
          <button
            @click="nextStep()"
            :disabled="!selectedTemplate"
            :class="[
              'flex-1 px-6 py-2 rounded-lg font-semibold transition',
              selectedTemplate
                ? 'bg-primary text-white hover:bg-accent'
                : 'bg-slate-700 text-slate-500 cursor-not-allowed'
            ]"
          >
            Continuar →
          </button>
        </div>
      </div>

      <!-- Step 2B: AI Generation -->
      <div v-if="currentStep === 2 && selectedMethod === 'ai'" class="animate-fadeIn space-y-6">
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-white mb-2">Descreva sua Avaliação</h1>
          <p class="text-slate-400">Deixe a IA criar perguntas personalizadas</p>
        </div>

        <!-- AI Loading States -->
        <div v-if="aiGenerationState !== 'idle'" class="p-8 rounded-xl border border-secondary bg-secondary/5 text-center">
          <div v-if="aiGenerationState === 'processing'" class="space-y-4">
            <div class="flex justify-center">
              <div class="h-12 w-12 rounded-full border-4 border-secondary/30 border-t-secondary animate-spin" />
            </div>
            <p class="text-white font-semibold">Analisando sua solicitação...</p>
            <p class="text-slate-400">Gemini está pensando... 🤔</p>
          </div>

          <div v-if="aiGenerationState === 'generating'" class="space-y-4">
            <div class="w-full h-1 bg-slate-700 rounded-full overflow-hidden">
              <div class="h-full w-2/3 bg-gradient-to-r from-secondary to-primary animate-pulse" />
            </div>
            <p class="text-white font-semibold">Criando suas perguntas...</p>
            <p class="text-slate-400">Quase pronto! ✨</p>
          </div>

          <div v-if="aiGenerationState === 'success'" class="space-y-4">
            <div class="text-4xl mb-2">✅</div>
            <p class="text-white font-semibold">Avaliação gerada com sucesso!</p>
            <p class="text-slate-400">{{ generatedQuestions.length }} perguntas criadas</p>
            <button
              @click="proceedWithAI()"
              class="mt-4 px-6 py-2 rounded-lg bg-secondary text-white hover:bg-secondary/90 transition"
            >
              Continuar para Edição →
            </button>
          </div>

          <div v-if="aiGenerationState === 'error'" class="space-y-4">
            <div class="text-4xl mb-2">⚠️</div>
            <p class="text-white font-semibold">{{ aiErrorMessage }}</p>
            <div class="flex gap-3 justify-center mt-4">
              <button
                @click="aiGenerationState = 'idle'"
                class="px-4 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition"
              >
                Tentar Novamente
              </button>
              <button
                @click="selectedMethod = 'template'; currentStep = 2"
                class="px-4 py-2 rounded-lg bg-primary text-white hover:bg-accent transition"
              >
                Usar Template
              </button>
            </div>
          </div>
        </div>

        <!-- AI Input -->
        <div v-if="aiGenerationState === 'idle'" class="space-y-6">
          <textarea
            v-model="aiPrompt"
            placeholder="Ex: 'Avaliação de clima organizacional focada em trabalho remoto' ou 'Pesquisa de satisfação pós-compra para e-commerce'"
            rows="6"
            class="w-full px-4 py-3 rounded-lg bg-slate-800 border border-slate-700 text-white placeholder-slate-500 focus:outline-none focus:ring-2 focus:ring-primary/40 focus:border-primary transition resize-none"
          />

          <!-- Examples -->
          <div class="p-4 rounded-lg bg-slate-800/50 border border-slate-700">
            <p class="text-sm font-semibold text-slate-300 mb-3">💡 Exemplos que funcionam bem:</p>
            <ul class="space-y-2">
              <li class="text-sm text-slate-400">• Avaliação de clima organizacional focada em trabalho remoto</li>
              <li class="text-sm text-slate-400">• Pesquisa de satisfação pós-compra para e-commerce</li>
              <li class="text-sm text-slate-400">• Feedback 360° para líderes em transição</li>
            </ul>
          </div>

          <!-- Advanced Options -->
          <div class="p-4 rounded-lg bg-slate-800/50 border border-slate-700 space-y-4">
            <details class="cursor-pointer">
              <summary class="font-semibold text-white hover:text-primary transition">
                ⚙️ Opções Avançadas
              </summary>
              <div class="mt-4 space-y-4 pt-4 border-t border-slate-700">
                <div>
                  <label class="block text-sm text-slate-400 mb-2">Número de Perguntas</label>
                  <input
                    v-model.number="aiOptions.questionCount"
                    type="range"
                    min="3"
                    max="20"
                    class="w-full"
                  />
                  <p class="text-xs text-slate-500 mt-1">{{ aiOptions.questionCount }} perguntas</p>
                </div>
                <div>
                  <label class="block text-sm text-slate-400 mb-2">Tom</label>
                  <select v-model="aiOptions.tone" class="w-full px-3 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white text-sm">
                    <option value="formal">Formal</option>
                    <option value="casual">Casual</option>
                    <option value="technical">Técnico</option>
                  </select>
                </div>
              </div>
            </details>
          </div>

          <!-- Generate Button -->
          <div class="flex gap-4">
            <button
              @click="previousStep()"
              class="px-6 py-2 rounded-lg border border-slate-700 text-white hover:bg-slate-800 transition"
            >
              ← Voltar
            </button>
            <button
              @click="generateWithAI()"
              :disabled="aiPrompt.trim().length < 20"
              :class="[
                'flex-1 px-6 py-2 rounded-lg font-semibold transition flex items-center justify-center gap-2',
                aiPrompt.trim().length >= 20
                  ? 'bg-secondary text-white hover:bg-secondary/90'
                  : 'bg-slate-700 text-slate-500 cursor-not-allowed'
              ]"
            >
              <span>✨</span>
              <span>Gerar com IA</span>
            </button>
          </div>

          <!-- Manual Bailout -->
          <button
            @click="bailoutToManualCreation()"
            class="w-full px-4 py-2 text-sm text-slate-400 hover:text-white transition"
          >
            Prefiro criar manualmente →
          </button>
        </div>
      </div>

      <!-- Step 3: Edit & Customize -->
      <div v-if="currentStep === 3" class="animate-fadeIn space-y-6">
        <div class="mb-8">
          <h1 class="text-3xl font-bold text-white mb-2">Configure sua Avaliação</h1>
          <p class="text-slate-400">Personalize as perguntas e escalas científicas</p>
        </div>

        <div class="grid grid-cols-1 lg:grid-cols-3 gap-6">
          <!-- Left: Questions List -->
          <div class="lg:col-span-2 space-y-4">
            <!-- Title and Description -->
            <div class="space-y-4 p-6 rounded-xl bg-slate-800 border border-slate-700">
              <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Título da Avaliação</label>
                <input
                  v-model="evaluationForm.title"
                  type="text"
                  placeholder="Ex: Feedback de Trabalho Remoto"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-slate-400 focus:border-primary focus:outline-none transition"
                />
                <p class="text-xs text-slate-500 mt-1">{{ evaluationForm.title.length }}/100</p>
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Descrição</label>
                <textarea
                  v-model="evaluationForm.description"
                  placeholder="Descreva o objetivo desta avaliação..."
                  rows="3"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white placeholder-slate-400 focus:border-primary focus:outline-none resize-none transition"
                />
                <p class="text-xs text-slate-500 mt-1">{{ evaluationForm.description.length }}/500</p>
              </div>
              <div>
                <label class="block text-sm font-semibold text-slate-300 mb-2">Categoria</label>
                <select
                  v-model="evaluationForm.category"
                  class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white focus:border-primary focus:outline-none transition"
                >
                  <option value="">Selecione uma categoria</option>
                  <option value="hr">Recursos Humanos</option>
                  <option value="product">Produto</option>
                  <option value="ux">UX/Design</option>
                  <option value="customer">Customer Success</option>
                  <option value="education">Educação</option>
                </select>
              </div>
            </div>

            <!-- Questions Editing -->
            <div class="space-y-4">
              <div class="flex items-center justify-between">
                <h2 class="text-lg font-bold text-white">Perguntas ({{ evaluationForm.questions.length }})</h2>
                <button
                  @click="addQuestion()"
                  class="px-3 py-1 rounded-lg bg-primary/20 text-primary hover:bg-primary/30 transition text-sm font-medium"
                >
                  + Adicionar
                </button>
              </div>

              <!-- Questions List with Drag & Drop -->
              <draggable
                v-model="evaluationForm.questions"
                tag="div"
                :animation="200"
                handle=".drag-handle"
                class="space-y-3"
              >
                <template #item="{ element: question, index }">
                  <div
                    class="rounded-lg border-2 transition-all duration-200"
                    :class="[
                      editingQuestionIndex === index
                        ? 'border-primary bg-slate-800'
                        : 'border-slate-600 bg-slate-800/50 hover:border-slate-500 cursor-pointer'
                    ]"
                    @click="editingQuestionIndex = editingQuestionIndex === index ? null : index"
                  >
                    <!-- Summary View (when not editing) -->
                    <div v-if="editingQuestionIndex !== index" class="p-4 flex items-start gap-3">
                      <div class="drag-handle text-slate-500 hover:text-slate-300 mt-1 cursor-move">⋮⋮</div>
                      <div class="flex-1 min-w-0">
                        <p class="font-medium text-white truncate">{{ index + 1 }}. {{ question.text || '[Pergunta sem título]' }}</p>
                        <p class="text-xs text-slate-400 mt-1">{{ getScaleLabel(question.type) }}</p>
                      </div>
                      <div class="flex items-center gap-2">
                        <span v-if="question.required" class="text-xs px-2 py-1 rounded bg-red-500/20 text-red-400 flex-shrink-0">Obrigatória</span>
                        <button
                          @click.stop="removeQuestion(index)"
                          class="text-slate-500 hover:text-red-400 transition flex-shrink-0"
                        >
                          🗑️
                        </button>
                      </div>
                    </div>

                    <!-- Detailed Editor (when editing) -->
                    <div v-else class="p-4">
                      <QuestionEditor
                        :question="question"
                        @update:question="(updated) => evaluationForm.questions[index] = updated"
                      />
                      <button
                        @click="editingQuestionIndex = null"
                        class="mt-4 w-full px-4 py-2 rounded-lg bg-primary/20 text-primary hover:bg-primary/30 transition text-sm font-medium"
                      >
                        ✓ Pronto
                      </button>
                    </div>
                  </div>
                </template>
              </draggable>

              <!-- Empty State -->
              <div v-if="evaluationForm.questions.length === 0" class="flex flex-col items-center justify-center p-12 rounded-lg border-2 border-dashed border-slate-600">
                <span class="text-4xl mb-3">📋</span>
                <p class="text-slate-400 text-center">Nenhuma pergunta adicionada ainda</p>
                <button
                  @click="addQuestion()"
                  class="mt-4 px-4 py-2 rounded-lg bg-primary text-white hover:bg-accent transition"
                >
                  + Adicionar Primeira Pergunta
                </button>
              </div>
            </div>

            <!-- Validations -->
            <div v-if="validationErrors.length" class="p-4 rounded-lg bg-red-500/10 border border-red-500/30 space-y-2">
              <p class="font-semibold text-red-400">Erros encontrados:</p>
              <p v-for="error in validationErrors" :key="error" class="text-sm text-red-400">
                • {{ error }}
              </p>
            </div>
          </div>

          <!-- Right: Preview -->
          <div class="p-6 rounded-xl bg-slate-800 border border-slate-700 sticky top-4 h-fit">
            <h3 class="font-bold text-white mb-4">📊 Preview</h3>
            <div class="space-y-4 text-sm">
              <div>
                <p class="text-xs text-slate-500 uppercase font-semibold">Título</p>
                <p class="font-bold text-white mt-1">{{ evaluationForm.title || '[Sem título]' }}</p>
              </div>

              <div class="pt-4 border-t border-slate-700">
                <p class="text-xs text-slate-500 uppercase font-semibold">Descrição</p>
                <p class="text-slate-300 mt-1 text-xs line-clamp-3">{{ evaluationForm.description || '[Sem descrição]' }}</p>
              </div>

              <div class="pt-4 border-t border-slate-700">
                <p class="text-xs text-slate-500 uppercase font-semibold">Categoria</p>
                <p class="text-slate-300 mt-1 text-sm font-medium">{{ getCategoryLabel(evaluationForm.category) }}</p>
              </div>

              <div class="pt-4 border-t border-slate-700">
                <p class="text-xs text-slate-500 uppercase font-semibold mb-2">Perguntas ({{ evaluationForm.questions.length }})</p>
                <div v-if="evaluationForm.questions.length > 0" class="space-y-2">
                  <div v-for="(q, idx) in evaluationForm.questions.slice(0, 3)" :key="idx" class="rounded p-2 bg-slate-700/30 space-y-1">
                    <p class="text-xs font-medium text-white">P{{ idx + 1 }}: {{ q.text?.substring(0, 35) || '[...]' }}</p>
                    <p class="text-xs text-slate-400">{{ getScaleLabel(q.type) }}</p>
                  </div>
                  <p v-if="evaluationForm.questions.length > 3" class="text-xs text-slate-500 italic">
                    +{{ evaluationForm.questions.length - 3 }} mais...
                  </p>
                </div>
                <p v-else class="text-xs text-slate-500 italic">Nenhuma pergunta</p>
              </div>
            </div>
          </div>
        </div>

        <!-- Navigation -->
        <div class="flex gap-4 pt-4">
          <button
            @click="previousStep()"
            class="px-6 py-2 rounded-lg border border-slate-700 text-white hover:bg-slate-800 transition"
          >
            ← Voltar
          </button>
          <button
            @click="saveDraft()"
            class="px-6 py-2 rounded-lg bg-slate-700 text-slate-300 hover:bg-slate-600 transition"
          >
            💾 Salvar Rascunho
          </button>
          <button
            @click="nextStep()"
            :disabled="validationErrors.length > 0"
            :class="[
              'flex-1 px-6 py-2 rounded-lg font-semibold transition',
              validationErrors.length === 0
                ? 'bg-primary text-white hover:bg-accent'
                : 'bg-slate-700 text-slate-500 cursor-not-allowed'
            ]"
          >
            Continuar → 
          </button>
        </div>
      </div>

      <!-- Step 4: Publish & Share -->
      <div v-if="currentStep === 4" class="animate-fadeIn max-w-2xl mx-auto space-y-6">
        <div class="text-center mb-8">
          <h1 class="text-3xl font-bold text-white mb-2">Publicar e Compartilhar</h1>
          <p class="text-slate-400">Configure as opções finais e compartilhe sua avaliação</p>
        </div>

        <!-- Configuration Card -->
        <div class="p-6 rounded-xl border border-slate-700 bg-slate-800/50 space-y-6">
          <div>
            <label class="block text-sm font-semibold text-slate-300 mb-3">Status</label>
            <div class="space-y-2">
              <label class="flex items-center gap-3 cursor-pointer">
                <input v-model="publishConfig.status" type="radio" value="draft" class="w-4 h-4" />
                <span class="text-white">📝 Rascunho (visível só para você)</span>
              </label>
              <label class="flex items-center gap-3 cursor-pointer">
                <input v-model="publishConfig.status" type="radio" value="open" class="w-4 h-4" />
                <span class="text-white">🔓 Aberta (aceita respostas)</span>
              </label>
              <label class="flex items-center gap-3 cursor-pointer">
                <input v-model="publishConfig.status" type="radio" value="closed" class="w-4 h-4" />
                <span class="text-white">🔒 Fechada (não aceita mais respostas)</span>
              </label>
            </div>
          </div>

          <div class="border-t border-slate-700 pt-6 space-y-4">
            <div>
              <label class="flex items-center gap-3 cursor-pointer mb-3">
                <input v-model="publishConfig.hasDeadline" type="checkbox" class="w-4 h-4" />
                <span class="text-white">⏰ Definir prazo</span>
              </label>
              <input
                v-if="publishConfig.hasDeadline"
                v-model="publishConfig.deadline"
                type="datetime-local"
                class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white focus:outline-none focus:ring-2 focus:ring-primary/40 transition"
              />
            </div>

            <div>
              <label class="flex items-center gap-3 cursor-pointer mb-3">
                <input v-model="publishConfig.hasResponseLimit" type="checkbox" class="w-4 h-4" />
                <span class="text-white">📊 Limite de respostas</span>
              </label>
              <input
                v-if="publishConfig.hasResponseLimit"
                v-model.number="publishConfig.responseLimit"
                type="number"
                min="1"
                placeholder="0 = ilimitado"
                class="w-full px-4 py-2 rounded-lg bg-slate-700 border border-slate-600 text-white focus:outline-none focus:ring-2 focus:ring-primary/40 transition"
              />
            </div>

            <div>
              <label class="flex items-center gap-3 cursor-pointer">
                <input v-model="publishConfig.requireAuth" type="checkbox" class="w-4 h-4" />
                <span class="text-white">🔐 Requer autenticação</span>
              </label>
            </div>

            <div>
              <label class="flex items-center gap-3 cursor-pointer">
                <input v-model="publishConfig.allowAnonymous" type="checkbox" class="w-4 h-4" />
                <span class="text-white">👤 Permitir respostas anônimas</span>
              </label>
            </div>
          </div>
        </div>

        <!-- Success State -->
        <div v-if="publishState === 'published'" class="p-8 rounded-xl border-2 border-green-500 bg-green-500/10 text-center space-y-4">
          <div class="text-5xl">✅</div>
          <h2 class="text-2xl font-bold text-white">Avaliação publicada com sucesso!</h2>

          <!-- Generated Links -->
          <div class="space-y-4 mt-6 pt-6 border-t border-green-500/30">
            <div>
              <p class="text-sm text-slate-400 mb-2">Seu link público:</p>
              <div class="flex gap-2">
                <input
                  type="text"
                  value="aevalo.app/e/abc123"
                  readonly
                  class="flex-1 px-4 py-2 rounded-lg bg-slate-800 border border-slate-700 text-white text-center"
                />
                <button
                  @click="copyToClipboard('aevalo.app/e/abc123')"
                  class="px-4 py-2 rounded-lg bg-primary text-white hover:bg-accent transition font-semibold"
                >
                  📋 Copiar
                </button>
              </div>
            </div>

            <div class="flex gap-3 flex-wrap justify-center">
              <button class="px-4 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition text-sm">
                📥 Baixar QR Code
              </button>
              <button class="px-4 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition text-sm">
                📧 Enviar por Email
              </button>
              <button class="px-4 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition text-sm">
                🔗 Compartilhar Redes
              </button>
            </div>
          </div>

          <!-- Final Actions -->
          <div class="flex gap-3 mt-6 pt-6 border-t border-green-500/30">
            <button
              @click="goToDashboard()"
              class="flex-1 px-6 py-2 rounded-lg bg-slate-700 text-white hover:bg-slate-600 transition font-semibold"
            >
              Ir para Dashboard
            </button>
            <button
              @click="viewEvaluation()"
              class="flex-1 px-6 py-2 rounded-lg bg-primary text-white hover:bg-accent transition font-semibold"
            >
              Ver Avaliação
            </button>
          </div>
        </div>

        <!-- Navigation (not published yet) -->
        <div v-if="publishState === 'pending'" class="flex gap-4">
          <button
            @click="previousStep()"
            class="px-6 py-2 rounded-lg border border-slate-700 text-white hover:bg-slate-800 transition"
          >
            ← Voltar
          </button>
          <button
            @click="publish()"
            class="flex-1 px-6 py-2 rounded-lg bg-primary text-white hover:bg-accent transition font-semibold"
          >
            🚀 Publicar Avaliação
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import draggable from 'vuedraggable'
import QuestionEditor from '@/components/QuestionEditor.vue'

interface Template {
  id: string
  name: string
  description: string
  icon: string
  category: string
  popular: boolean
  questionCount: number
  previewQuestions: string[]
}

interface Question {
  id?: string
  text: string
  type: string
  required: boolean
  options?: string[]
  config?: Record<string, any>
}

interface EvaluationForm {
  title: string
  description: string
  category: string
  questions: Question[]
}

// State
const currentStep = ref(1)
const steps = ['Método', 'Template/IA', 'Editar', 'Publicar']
const selectedMethod = ref<'template' | 'ai' | null>(null)
const templateSearch = ref('')
const templateCategory = ref('')
const selectedTemplate = ref<Template | null>(null)

// AI State
const aiPrompt = ref('')
const aiGenerationState = ref<'idle' | 'processing' | 'generating' | 'success' | 'error'>('idle')
const aiErrorMessage = ref('')
const aiOptions = ref({
  questionCount: 8,
  tone: 'formal'
})
const generatedQuestions = ref<Question[]>([])

// Form State
const evaluationForm = ref<EvaluationForm>({
  title: '',
  description: '',
  category: '',
  questions: []
})
const newQuestionType = ref('likert')
const editingQuestionIndex = ref<number | null>(null)

// Publish State
const publishConfig = ref({
  status: 'open',
  hasDeadline: false,
  deadline: '',
  hasResponseLimit: false,
  responseLimit: 0,
  requireAuth: false,
  allowAnonymous: true
})
const publishState = ref<'pending' | 'published'>('pending')

// Mock Templates
const templates: Template[] = [
  {
    id: '1',
    name: 'NPS (Net Promoter Score)',
    description: 'Avalie a probabilidade de recomendação',
    icon: '⭐',
    category: 'customer',
    popular: true,
    questionCount: 3,
    previewQuestions: ['Qual a probabilidade de recomendar?', 'Motivo principal?']
  },
  {
    id: '2',
    name: 'Clima Organizacional',
    description: 'Avalie a satisfação dos colaboradores',
    icon: '👥',
    category: 'hr',
    popular: true,
    questionCount: 12,
    previewQuestions: ['Sente-se engajado?', 'Relacionamento com time?']
  },
  {
    id: '3',
    name: 'UX Research',
    description: 'Pesquise experiência do usuário',
    icon: '🎯',
    category: 'ux',
    popular: false,
    questionCount: 8,
    previewQuestions: ['Facilidade de uso?', 'Que features faltam?']
  },
  {
    id: '4',
    name: 'Satisfação com Produto',
    description: 'Avalie satisfação geral',
    icon: '📦',
    category: 'customer',
    popular: false,
    questionCount: 6,
    previewQuestions: ['Recomendaria o produto?', 'Qualidade atende?']
  },
  {
    id: '5',
    name: 'Feedback 360°',
    description: 'Colha feedback de liderança',
    icon: '🎓',
    category: 'hr',
    popular: false,
    questionCount: 20,
    previewQuestions: ['Comunicação eficaz?', 'Desenvolvimento de pessoas?']
  },
  {
    id: '6',
    name: 'Pesquisa Educacional',
    description: 'Avalie qualidade da educação',
    icon: '📚',
    category: 'education',
    popular: false,
    questionCount: 15,
    previewQuestions: ['Qualidade do ensino?', 'Recursos disponíveis?']
  }
]

// Computed
const filteredTemplates = computed(() => {
  return templates.filter(t => {
    const matchesSearch = t.name.toLowerCase().includes(templateSearch.value.toLowerCase()) ||
      t.description.toLowerCase().includes(templateSearch.value.toLowerCase())
    const matchesCategory = !templateCategory.value || t.category === templateCategory.value
    return matchesSearch && matchesCategory
  })
})

const validationErrors = computed(() => {
  const errors = []
  if (evaluationForm.value.title.length < 5) errors.push('Título deve ter ao menos 5 caracteres')
  if (evaluationForm.value.questions.length === 0) errors.push('Precisa de ao menos 1 pergunta')
  if (evaluationForm.value.questions.some(q => !q.text)) errors.push('Todas as perguntas devem ter texto')
  return errors
})

// Methods
const selectMethod = (method: 'template' | 'ai') => {
  selectedMethod.value = method
  nextStep()
}

const selectTemplate = (template: Template) => {
  selectedTemplate.value = template
  evaluationForm.value.questions = template.previewQuestions.map((text, idx) => ({
    id: `q_${idx}`,
    text,
    type: 'likert',
    required: false,
    options: [],
    config: { scale: 5 }
  }))
}

const generateWithAI = async () => {
  aiGenerationState.value = 'processing'
  try {
    // Simulated AI generation
    await new Promise(resolve => setTimeout(resolve, 2000))
    aiGenerationState.value = 'generating'
    await new Promise(resolve => setTimeout(resolve, 2000))

    // Mock generated questions
    generatedQuestions.value = [
      { 
        id: 'q_1',
        text: 'Qual seu nível de interesse em trabalho remoto?', 
        type: 'likert', 
        required: true,
        options: [],
        config: { scale: 5, minLabel: 'Sem interesse', maxLabel: 'Muito interessado' }
      },
      { 
        id: 'q_2',
        text: 'Como você avalia a comunicação da equipe?', 
        type: 'likert', 
        required: true,
        options: [],
        config: { scale: 5, minLabel: 'Muito ruim', maxLabel: 'Excelente' }
      },
      { 
        id: 'q_3',
        text: 'Quais benefícios você mais valoriza?', 
        type: 'multiple_choice', 
        required: false,
        options: ['Flexibilidade', 'Economia de tempo', 'Melhor foco', 'Autonomia'],
        config: {}
      },
      { 
        id: 'q_4',
        text: 'Qual é seu maior desafio no trabalho remoto?', 
        type: 'text', 
        required: false,
        options: [],
        config: { maxLength: 500 }
      }
    ]

    evaluationForm.value.questions = generatedQuestions.value
    aiGenerationState.value = 'success'
  } catch (error) {
    aiErrorMessage.value = 'Erro ao gerar com IA. Tente novamente.'
    aiGenerationState.value = 'error'
  }
}

const proceedWithAI = () => {
  aiGenerationState.value = 'idle'
  nextStep()
}

const bailoutToManualCreation = () => {
  selectedMethod.value = null
  currentStep.value = 3
  evaluationForm.value.questions = [
    { 
      id: 'q_1',
      text: '', 
      type: 'likert', 
      required: false,
      options: [],
      config: { scale: 5 }
    }
  ]
}

const addQuestion = () => {
  evaluationForm.value.questions.push({
    id: `q_${Date.now()}`,
    text: '',
    type: newQuestionType.value,
    required: false,
    options: [],
    config: {}
  })
}

const removeQuestion = (index: number) => {
  evaluationForm.value.questions.splice(index, 1)
  editingQuestionIndex.value = null
}

const getScaleLabel = (type: string): string => {
  const labels: Record<string, string> = {
    likert: '📊 Likert (Escala de concordância)',
    frequency: '📈 Frequência (Nunca → Sempre)',
    paired: '⚖️ Comparação Pareada',
    fixed_sum: '🎯 Distribuir 100 Pontos',
    text: '📝 Texto Aberto',
    multiple_choice: '☑️ Múltipla Escolha',
    single_choice: '⭕ Escolha Única'
  }
  return labels[type] || 'Tipo desconhecido'
}

const getCategoryLabel = (value: string): string => {
  const labels: Record<string, string> = {
    hr: 'Recursos Humanos',
    product: 'Produto',
    ux: 'UX/Design',
    customer: 'Customer Success',
    education: 'Educação'
  }
  return labels[value] || 'Não selecionada'
}

const publish = async () => {
  publishState.value = 'published'
}

const saveDraft = () => {
  console.log('Saving draft:', evaluationForm.value)
}

const copyToClipboard = (text: string) => {
  navigator.clipboard.writeText(text)
}

const nextStep = () => {
  if (currentStep.value < steps.length) {
    currentStep.value++
  }
}

const previousStep = () => {
  if (currentStep.value > 1) {
    currentStep.value--
  }
}

const goToDashboard = () => {
  // Navigate to dashboard
}

const viewEvaluation = () => {
  // View evaluation
}
</script>

<style scoped>
@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.animate-fadeIn {
  animation: fadeIn 0.3s ease-out;
}
</style>
