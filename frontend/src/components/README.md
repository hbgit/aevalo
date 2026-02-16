# 🎨 Componentes Base do Sistema Aevalo

Sistema completo de componentes reutilizáveis construído com Vue 3 + TypeScript + Tailwind CSS, seguindo o design system Aevalo.

## 📁 Estrutura

```
/frontend/src/components/
├── ui/                  # Componentes genéricos da interface
│   ├── Button.vue      # Botões com variantes (primary, secondary, ghost, danger)
│   ├── Input.vue       # Inputs com validação inline
│   ├── Modal.vue       # Modais com fade/scale animations
│   ├── Toast.vue       # Notificações (5 tipos: success, error, warning, info, loading)
│   ├── Card.vue        # Cards com elevação configurável
│   ├── Badge.vue       # Status indicators com 6 variantes
│   ├── Skeleton.vue    # Loading placeholders com shimmer effect
│   ├── EmptyState.vue  # Empty state ilustrado com CTA
│   └── index.ts        # Exports centralizados
├── forms/              # Componentes de formulários e escalas
│   ├── LikertScale.vue         # Escala Likert 1-5 (com descrições)
│   ├── FixedSumInput.vue       # Distribuição de pontos (Fixed Sum)
│   ├── PairedComparison.vue    # Comparação pareada (A vs B vs C)
│   ├── FormField.vue           # Wrapper para formulários com validação
│   └── index.ts                # Exports centralizados
├── dashboard/          # Componentes específicos do dashboard
│   ├── MetricCard.vue          # Cards com métricas, trend e progress
│   ├── ChartBar.vue            # Gráfico de barras simples
│   ├── EvaluationCard.vue      # Card de avaliação com stats
│   └── index.ts                # Exports centralizados
└── (outros componentes específicos...)
```

## 🚀 Uso Rápido

### Componentes UI

#### Button
```vue
<template>
  <Button variant="primary" size="md" @click="handleClick">
    Clique aqui
  </Button>

  <Button variant="secondary" :loading="isLoading">
    Enviando...
  </Button>

  <Button variant="ghost" disabled>Desabilizado</Button>
  <Button variant="danger">Deletar</Button>
</template>

<script setup>
import { Button } from '@/components/ui'
const isLoading = ref(false)
const handleClick = () => { /* ... */ }
</script>
```

**Props:**
- `variant`: `'primary' | 'secondary' | 'ghost' | 'danger'` (default: `'primary'`)
- `size`: `'sm' | 'md' | 'lg'` (default: `'md'`)
- `type`: `'button' | 'submit' | 'reset'` (default: `'button'`)
- `disabled`: `boolean` (default: `false`)
- `loading`: `boolean` (default: `false`)

---

#### Input
```vue
<template>
  <Input
    v-model="email"
    type="email"
    label="E-mail"
    placeholder="seu@email.com"
    prefixIcon="📧"
    hint="Usaremos para confirmar sua resposta"
  />

  <Input
    v-model="message"
    type="text"
    label="Mensagem"
    :error="formErrors.message"
    required
  />
</template>

<script setup>
import { Input } from '@/components/ui'
const email = ref('')
const message = ref('')
</script>
```

**Props:**
- `modelValue`: valor do input
- `type`: tipo HTML (default: `'text'`)
- `label`: rótulo do campo
- `placeholder`: placeholder
- `hint`: texto de ajuda
- `error`: mensagem de erro
- `required`: boolean
- `disabled`: boolean
- `prefixIcon`: emoji/ícone antes do input
- `suffixIcon`: emoji/ícone depois do input

---

#### Modal
```vue
<template>
  <button @click="showModal = true">Abrir Modal</button>

  <Modal v-model:open="showModal" title="Confirmação">
    <p>Tem certeza que deseja continuar?</p>

    <template #footer>
      <Button variant="ghost" @click="showModal = false">
        Cancelar
      </Button>
      <Button @click="confirm">Confirmar</Button>
    </template>
  </Modal>
</template>

<script setup>
import { Modal } from '@/components/ui'
const showModal = ref(false)
const confirm = () => { /* ... */ }
</script>
```

**Props:**
- `open`: boolean (v-model)
- `title`: título do modal

**Slots:**
- `title`: conteúdo customizado para o título
- `footer`: ações do rodapé

---

#### Toast
```vue
<template>
  <Button @click="showSuccess">Sucesso</Button>
  <Button @click="showError">Erro</Button>

  <Toast
    :show="showToast"
    :type="toastType"
    title="Operação realizada"
    message="Seu item foi criado com sucesso"
    @close="showToast = false"
  />
</template>

<script setup>
import { Toast } from '@/components/ui'
const showToast = ref(false)
const toastType = ref('success')

const showSuccess = () => {
  toastType.value = 'success'
  showToast.value = true
}
</script>
```

**Props:**
- `show`: boolean
- `type`: `'success' | 'error' | 'warning' | 'info' | 'loading'` (default: `'info'`)
- `title`: título
- `message`: mensagem
- `duration`: tempo em ms antes de fechar (default: `4000`, 0 = nunca fecha)
- `icon`: customizar ícone (default: automático)

---

#### Card, Badge, Skeleton, EmptyState
```vue
<!-- Card -->
<Card elevation="lg">
  <h3>Conteúdo do Card</h3>
  <p>Lorem ipsum...</p>
</Card>

<!-- Badge -->
<Badge variant="success">Ativo</Badge>
<Badge variant="danger" animated>Novo</Badge>

<!-- Skeleton (loading) -->
<Skeleton :lines="3" />

<!-- EmptyState -->
<EmptyState
  icon="🎯"
  title="Nenhuma avaliação"
  description="Comece criando uma nova avaliação"
  actionLabel="Criar"
  @action="goToCreate"
/>
```

---

### Componentes de Formulário (Escalas)

#### LikertScale
```vue
<template>
  <LikertScale
    v-model="rating"
    question="Como você avalia nosso serviço?"
    hint="Sua opinião é importante para nós"
    @update:modelValue="handleRatingChange"
  />
</template>

<script setup>
import { LikertScale } from '@/components/forms'
const rating = ref(3)
</script>
```

**Props:**
- `modelValue`: valor selecionado (1-5)
- `question`: pergunta
- `options`: customizar opções (default: 5 pontos com descrições)
- `hint`: texto de ajuda

---

#### FixedSumInput
```vue
<template>
  <FixedSumInput
    v-model="distribution"
    question="Distribua 100 pontos entre os fatores"
    :options="['Preço', 'Qualidade', 'Atendimento']"
    :total="100"
  />
</template>

<script setup>
import { FixedSumInput } from '@/components/forms'
const distribution = ref([0, 0, 0])
</script>
```

**Props:**
- `modelValue`: array de números
- `question`: pergunta
- `options`: array de labels
- `total`: soma esperada (default: 100)

---

#### PairedComparison
```vue
<template>
  <PairedComparison
    v-model="preferences"
    question="Qual você prefere?"
    :pairs="[
      { left: 'Opção A', right: 'Opção B' },
      { left: 'Opção A', right: 'Opção C' },
      { left: 'Opção B', right: 'Opção C' }
    ]"
  />
</template>

<script setup>
import { PairedComparison } from '@/components/forms'
const preferences = ref(['left', 'right', 'left'])
</script>
```

---

#### FormField (wrapper inteligente)
```vue
<template>
  <!-- Automaticamente escolhe o componente correto -->
  <FormField
    v-model="answer"
    type="email"
    label="E-mail"
    hint="Para notificações"
    required
  />

  <FormField
    v-model="rating"
    type="likert"
    label="Avaliação"
    :field-props="{ question: 'Como você nos avalia?' }"
  />

  <FormField
    v-model="distribution"
    type="fixed_sum"
    label="Importância"
    :field-props="{ 
      question: 'Distribua 100 pontos',
      options: ['Fator A', 'Fator B']
    }"
  />
</template>
```

---

### Componentes Dashboard

#### MetricCard
```vue
<template>
  <MetricCard
    label="Avaliações Ativas"
    value="24"
    icon="📊"
    :trend="12"
    trend-label="desde ontem"
    :progress="75"
    action-label="Ver Detalhes"
    @action="viewDetails"
  />
</template>
```

**Props:**
- `label`: rótulo
- `value`: valor principal
- `icon`: emoji
- `trend`: percentual de mudança (positivo = seta para cima)
- `trendLabel`: contexto (ex: "desde ontem")
- `progress`: percentual de progresso (0-100)
- `actionLabel`: texto do botão de ação

---

#### ChartBar
```vue
<template>
  <ChartBar
    title="Respostas por Dia"
    :bars="[
      { label: 'Seg', value: 45 },
      { label: 'Ter', value: 52 },
      { label: 'Qua', value: 38 }
    ]"
  />
</template>
```

---

#### EvaluationCard
```vue
<template>
  <EvaluationCard
    title="Pesquisa NPS Q1 2026"
    category="Satisfação"
    category-icon="🎯"
    status="open"
    :responses="15"
    :total-responses="50"
    created-date="15/02/2026"
    updated-at="há 2 dias"
    @view="viewResults"
    @edit="editEvaluation"
  />
</template>
```

---

## 🎨 Design System

### Cores
```typescript
// primary.ts ou tailwind.config.ts
const colors = {
  primary: '#4B0082',      // Roxo Profundo
  secondary: '#FF8C00',    // Laranja Vibrante
  accent: '#9333EA',       // Roxo Médio
}
```

### Variants em Components
- **Button**: primary, secondary, ghost, danger
- **Badge**: success, warning, danger, info, secondary, primary
- **Toast**: success, error, warning, info, loading

---

## ♿ Acessibilidade

Todos os componentes seguem WCAG 2.1 AA:
- ✅ Labels associados em inputs
- ✅ Focus visible em navegação por teclado
- ✅ Contraste mínimo 4.5:1
- ✅ Roles ARIA apropriados
- ✅ Suporte a screen readers

---

## 🧪 Exemplo Completo (Formulário)

```vue
<template>
  <div class="max-w-2xl mx-auto p-6">
    <Card>
      <h2 class="text-2xl font-bold mb-6">Sua Feedback</h2>

      <!-- Email -->
      <div class="mb-6">
        <Input
          v-model="form.email"
          type="email"
          label="E-mail"
          placeholder="seu@email.com"
          prefixIcon="📧"
          :error="errors.email"
          required
          @blur="validateEmail"
        />
      </div>

      <!-- Likert Scale -->
      <div class="mb-6">
        <h3 class="text-lg font-semibold mb-4">Como você nos avalia?</h3>
        <LikertScale v-model="form.rating" />
      </div>

      <!-- Fixed Sum -->
      <div class="mb-6">
        <FixedSumInput
          v-model="form.importance"
          question="Distribua 100 pontos entre os fatores:"
          :options="['Preço', 'Qualidade', 'Atendimento']"
        />
      </div>

      <!-- Actions -->
      <div class="flex gap-3">
        <Button variant="ghost" @click="reset">Limpar</Button>
        <Button @click="submit" :loading="isSubmitting">
          Enviar Resposta
        </Button>
      </div>
    </Card>

    <!-- Toast feedback -->
    <Toast
      :show="showToast"
      :type="toastType"
      :title="toastTitle"
      @close="showToast = false"
    />
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { Card, Button, Input, Toast } from '@/components/ui'
import { LikertScale, FixedSumInput } from '@/components/forms'

const form = ref({
  email: '',
  rating: 3,
  importance: [0, 0, 0]
})

const errors = ref<Record<string, string>>({})
const isSubmitting = ref(false)
const showToast = ref(false)
const toastType = ref<'success' | 'error'>('success')
const toastTitle = ref('')

const validateEmail = () => {
  const email = form.value.email
  if (!email) {
    errors.value.email = 'E-mail é obrigatório'
  } else if (!email.match(/^[^\s@]+@[^\s@]+\.[^\s@]+$/)) {
    errors.value.email = 'E-mail inválido'
  } else {
    errors.value.email = ''
  }
}

const submit = async () => {
  validateEmail()
  if (errors.value.email) return

  isSubmitting.value = true
  try {
    // API call
    await new Promise(r => setTimeout(r, 1000))
    
    showToast.value = true
    toastType.value = 'success'
    toastTitle.value = 'Feedback enviado com sucesso!'
    reset()
  } catch (error) {
    showToast.value = true
    toastType.value = 'error'
    toastTitle.value = 'Erro ao enviar feedback'
  } finally {
    isSubmitting.value = false
  }
}

const reset = () => {
  form.value = { email: '', rating: 3, importance: [0, 0, 0] }
  errors.value = {}
}
</script>
```

---

## 📦 Imports

```typescript
// Individual imports
import { Button, Input, Modal, Toast, Card, Badge, Skeleton, EmptyState } from '@/components/ui'
import { LikertScale, FixedSumInput, PairedComparison, FormField } from '@/components/forms'
import { MetricCard, ChartBar, EvaluationCard } from '@/components/dashboard'

// Or create an alias in `auto-imports` config for shorter usage
```

---

## ✨ Roadmap de Componentes

- ✅ UI Base (Button, Input, Modal, Toast, Card, Badge, Skeleton, EmptyState)
- ✅ Formulários (Likert, FixedSum, PairedComparison, FormField)
- ✅ Dashboard (MetricCard, ChartBar, EvaluationCard)
- ⏳ Tabelas (Table, Pagination)
- ⏳ Navegação (Tabs, Breadcrumb, Pagination)
- ⏳ Gráficos avançados (LineChart, PieChart, AreaChart)
- ⏳ Upload (FileUpload, ImageUpload)
- ⏳ Seleção (Select, MultiSelect, Autocomplete)
- ⏳ Date/Time (DatePicker, TimePicker, DateRangePicker)
- ⏳ Layouts (Header, Sidebar, Footer)

---

## 🤝 Contribuindo

Ao adicionar novos componentes:

1. **Estrutura:** Colocar em `ui/`, `forms/`, ou `dashboard/` conforme tipo
2. **Props:** Usar `interface Props` com defaults apropriados
3. **Emits:** Documentar eventos emitidos
4. **Slots:** Oferecer slots para customização
5. **Acessibilidade:** Validar com Lighthouse + screen reader
6. **Testes:** Escrever testes de interação (Vitest)
7. **Documentação:** Adicionar exemplos neste README

## 📞 Suporte

Para adicionar novos componentes:
1. Criar arquivo em categoria apropriada (ui/, forms/, dashboard/, shared/)
2. Adicionar export em `index.ts` da categoria
3. Componentes serão automaticamente exportados via `components/index.ts`

---

