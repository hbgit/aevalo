# 🧪 Guia de Teste - Seção 4: Escalas Científicas

## ✅ Build Status

**Data de Build:** 2026-02-18  
**Status:** ✅ **SUCESSO**

```
✓ 220 modules transformed
✓ EvaluationWizard bundle: 138.61 kB (gzip: 45.67 kB)
✓ Frontend total: 436.86 kB (gzip: 136.64 kB)
✓ Docker containers: All running (frontend, backend, db)
```

---

## 🚀 Como Acessar a Aplicação

### URLs Locais (Docker Compose)

| Serviço | URL | Status |
|---------|-----|--------|
| **Frontend** | http://localhost | ✅ Running |
| **Wizard** | http://localhost/create | ✅ Ready to test |
| **Backend API** | http://localhost:3000 | ✅ Running |
| **PostgreSQL** | localhost:5432 | ✅ Running |
| **Grafana** | http://localhost:3001 | ✅ Running |
| **Prometheus** | http://localhost:9090 | ✅ Running |

---

## 📋 Checklist de Testes Manuais

### Phase 1: Visualização & Navegação

- [ ] Abrir http://localhost/create no navegador
- [ ] **Step 1:** Aparecem 2 cards (Template vs IA)
- [ ] **Step 1:** Hover effect funciona nos cards
- [ ] Progress bar atualiza corretamente

### Phase 2: Step 1 - Seleção de Método

**Template:**
- [ ] Clicar em "Começar com Template" → vai para Step 2
- [ ] Voltar funciona corretamente

**IA:**
- [ ] Clicar em "Criar com IA" → vai para Step 2B
- [ ] Campo de texto está focável

### Phase 3: Step 2A - Seleção de Template

- [ ] Grid de templates aparece (6 cards)
- [ ] Filtro por categoria funciona
- [ ] Busca filtra templates
- [ ] Ao clicar em template → avança para Step 3
- [ ] Perguntas do template carregam corretamente

### Phase 4: Step 2B - Geração com IA

- [ ] Textarea está focável
- [ ] Botão "Gerar com IA" está disabled inicialmente
- [ ] Após 20+ caracteres → botão ativa
- [ ] Estados de loading aparecem (processing → generating → success)
- [ ] Mock questions aparecem após geração
- [ ] Botão "Continuar para Edição" avança para Step 3

### Phase 5: Step 3 - Configuração (★ **Teste Principal**)

#### 5.1 - Informações Básicas

- [ ] **Título:** Input funciona e mostra contador (X/100)
- [ ] **Descrição:** Textarea funciona e mostra contador (X/500)
- [ ] **Categoria:** Dropdown com 5 opções aparece
- [ ] **Preview:** Atualiza em tempo real

#### 5.2 - Lista de Perguntas

**Summary View:**
- [ ] Perguntas aparecem em lista
- [ ] Cada pergunta mostra: número, texto, tipo (icon + label)
- [ ] Badge "Obrigatória" aparece quando marcada
- [ ] Botão delete (🗑️) aparece no hover

**Editing View:**
- [ ] Clicar em pergunta → expande QuestionEditor
- [ ] QuestionEditor mostra todos os campos

#### 5.3 - QuestionEditor Component Tests

**Campo de Pergunta:**
- [ ] Input de texto editável
- [ ] Estado salvo ao focar outro campo

**Seletor de Tipo:**
```
Types testáveis:
- likert (Likert 1-5, 1-7, 1-10)
- frequency (Frequência: Nunca → Sempre)
- paired (Comparação A vs B)
- fixed_sum (Distribuir 100 pontos)
- text (Texto Aberto)
- multiple_choice (Múltipla Escolha)
- single_choice (Escolha Única)
```

**Teste cada tipo:**

**Likert Scale:**
- [ ] Dropdown: 1-5, 1-7, 1-10 selecionável
- [ ] Min/Max labels editáveis
- [ ] Preview mostra rótulos
- [ ] Escala visual aparece

**Frequency Scale:**
- [ ] Preview: "Frequência: Nunca | Raramente | Às vezes | ..."

**Paired Comparison:**
- [ ] Campos "Option A Label", "Option B Label"
- [ ] Campos de descrição (optional)

**Fixed Sum:**
- [ ] Campo "Opções (uma por linha)"
- [ ] Textarea para inserir atributos
- [ ] Preview: "Distribuir 100 pontos entre X opções"

**Open Text:**
- [ ] Campo "Limite de Caracteres"
- [ ] Number input com min/max

**Multiple Choice:**
- [ ] Campo "Opções (uma por linha)"
- [ ] Campos "Seleções Mínimas" e "Máximas"

**Single Choice:**
- [ ] Campo "Opções (uma por linha)"

**Toggle "Obrigatória":**
- [ ] Checkbox funciona
- [ ] Estado persiste

**Botão "✓ Pronto":**
- [ ] ColapsaEditor de volta à summary view
- [ ] Alterações são salvas

#### 5.4 - Drag & Drop

- [ ] Handle (⋮⋮) é visível
- [ ] Reordenar perguntas funciona
- [ ] Animação suave (200ms)
- [ ] Ordem persiste ao desselecionar

#### 5.5 - Adicionar/Remover

- [ ] Botão "+ Adicionar" cria nova pergunta
- [ ] Nova pergunta aparece ao final
- [ ] Botão delete remove pergunta
- [ ] Após remover, editingQuestionIndex reseta

#### 5.6 - Empty States

- [ ] Com 0 perguntas: mensagem "Nenhuma pergunta adicionada"
- [ ] Botão "+ Adicionar Primeira Pergunta" funciona

#### 5.7 - Validações

```
Erros esperados:
- ❌ Título < 5 caracteres
- ❌ 0 perguntas
- ❌ Pergunta sem texto
```

- [ ] Mensagem de erro lista todos os problemas
- [ ] Botão "Continuar" fica disabled com erros
- [ ] Após corrigir → botão ativa

#### 5.8 - Preview Lateral

- [ ] Seção "Título" mostra valor do input
- [ ] Seção "Descrição" mostra primeiras linhas
- [ ] Seção "Categoria" mostra label traduzido
- [ ] Seção "Perguntas" lista até 3 primeiras
- [ ] "+X mais..." aparece se > 3 perguntas

#### 5.9 - Botões de Ação

- [ ] **Voltar:** Vai para Step 2
- [ ] **Salvar Rascunho:** Não navega (feedback visual?)
- [ ] **Continuar →:** Valida e avança para Step 4

### Phase 6: Step 4 - Publicação

- [ ] Página carrega corretamente
- [ ] Configurações de publicação aparecem
- [ ] Botões de ação funcionam

### Phase 7: Responsividade

**Desktop (> 768px):**
- [ ] 3-coluna layout (leftside: 2 cols, rightside: 1 col)
- [ ] Preview fica sticky
- [ ] Tudo alinhado corretamente

**Tablet (768px):**
- [ ] Layout adapta para 2 colunas
- [ ] Preview move para baixo
- [ ] Legível e usável

**Mobile (< 480px):**
- [ ] Layout single column
- [ ] Perguntas ocupam 100% width
- [ ] Preview pode expandir/colapsar
- [ ] Botões de ação ficam stacked

### Phase 8: Acessibilidade

- [ ] **Keyboard Navigation:** Tab entre campos
- [ ] **Focus States:** Todas as inputs têm focus ring roxo
- [ ] **Labels:** Todos os inputs têm labels associados
- [ ] **Color Contrast:** Texto legível em todos os backgrounds

---

## 🐛 Testes de Edge Cases

### Eventos Extremos

```javascript
// Teste 1: Muitas perguntas
validationErrors.length === 0 // com 100+ perguntas

// Teste 2: Texto muito longo
Quando título > 100 caracteres // deve truncar ou avisar

// Teste 3: Drag & drop na última posição
Reordenar última pergunta para primeira // funciona?

// Teste 4: Deletar única pergunta
Remover única pergunta // validação dispara

// Teste 5: FixedSum soma > 100
attr_0: 60, attr_1: 50 // deve mostrar erro em tempo real
```

### Validações Specificas por Type

#### Likert: 
- [ ] Todas as 3 escalas (5, 7, 10) funcionam
- [ ] Labels min/max não vazios

#### Fixed Sum:
- [ ] Mínimo 2 atributos
- [ ] Labels únicos (teste duplicatas)

#### Multiple Choice:
- [ ] Min ≤ Max
- [ ] Min ≤ quantidade de opções

#### Open Text:
- [ ] maxLength > 10 e < 5000

---

## 📊 Performance Checks

```bash
# 1. Bundle size
dist/assets/EvaluationWizard-*.js    # < 150KB é bom

# 2. Module count
220 modules # aceitável para Single Page App

# 3. Build time
2.25s # performance aceita para Vite
```

### Runtime Performance:
- [ ] Sem lag ao adicionar 10+ perguntas
- [ ] Drag & drop suave com 50 perguntas
- [ ] Re-render rápido ao editar configurações
- [ ] Preview atualiza sem atraso (debounce 300ms)

---

## 🔍 Browser Console

**Esperado:**
```
❌ Sem console errors
❌ Sem console warnings (exceto:UNSAFE_* do Vue)
✅ Sem network errors
✅ Sem 404s
```

**Teste:**
```
1. Abrir DevTools (F12)
2. Aba Console
3. Executar ações no wizard
4. Verificar se há erros vermelhos
```

---

## 📝 Testes Específicos por Componente de Scale

### LikertScale.vue

```vue
<!-- Desktop test -->
<LikertScale v-model="value" scale="5" />
<!-- Resultado esperado: 5 círculos em linha horizontal -->

<!-- Mobile test -->
<!-- Resultado esperado: 5 botões empilhados verticalmente -->

<!-- Scale test -->
<LikertScale scale="7" />
<!-- Resultado esperado: 7 círculos -->

<LikertScale scale="10" />
<!-- Resultado esperado: 10 círculos -->
```

**Testes:**
- [ ] Click em cada número: 1, 2, 3, 4, 5
- [ ] Hover effect funciona
- [ ] Seleção anterior muda corretamente
- [ ] Labels min/max aparecem nas extremidades

---

### FrequencyScale.vue

```vue
<FrequencyScale v-model="value" />
```

**Testes:**
- [ ] 5 chips aparecem: Nunca | Raramente | Às vezes | Frequentemente | Sempre
- [ ] Click em cada opção funciona
- [ ] Seleção anterior muda
- [ ] Horizontal scroll em mobile

---

### PairedComparison.vue

```vue
<PairedComparison
  v-model="value"
  option-a-label="Opção A"
  option-b-label="Opção B"
/>
```

**Testes:**
- [ ] 2 cards aparecem lado a lado
- [ ] Click em A seleciona A com checkmark
- [ ] Click em B seleciona B com checkmark
- [ ] "Sem preferência" desseleciona tudo
- [ ] Click duplo em mesma opção → desseleciona

---

### FixedSum.vue

```vue
<FixedSum
  v-model="distribution"
  :labels="['Atributo 1', 'Atributo 2', 'Atributo 3']"
/>
```

**Testes:**
- [ ] 3 rows com inputs numéricos
- [ ] Incrementar com + button
- [ ] Decrementar com - button
- [ ] Input direto funciona
- [ ] Progress bars aparecem
- [ ] Cores dinâmicas: 🟢 100, 🟠 <100, 🔴 >100
- [ ] Contador total atualiza
- [ ] "Distribuir Igualmente" divide 100/3 = 33,33,34
- [ ] "Limpar" reseta para 0,0,0

---

### OpenText.vue

```vue
<OpenText v-model="text" :max-length="500" />
```

**Testes:**
- [ ] Textarea editável
- [ ] Contador: 0/500 inicialmente
- [ ] Auto-resize ao digitar
- [ ] Limite respeitado (não digita além de 500)
- [ ] Cores: 🟢 <375, 🟠 375-450, 🔴 >450

---

### MultipleChoice.vue

```vue
<MultipleChoice
  v-model="selected"
  :options="['A', 'B', 'C']"
  :min-selections="1"
  :max-selections="2"
/>
```

**Testes:**
- [ ] 3 checkboxes aparecem
- [ ] Click seleciona A, B, ou C
- [ ] Click duplo desseleciona
- [ ] Máximo 2 seleções: 3ª fica disabled
- [ ] Contador: "2 de 2 selecionadas"
- [ ] Mensagem de erro se < 1 seleção
- [ ] Mensagem de erro se > 2 seleções

---

### SingleChoice.vue

```vue
<SingleChoice
  v-model="selected"
  :options="['A', 'B', 'C']"
/>
```

**Testes:**
- [ ] 3 radio buttons aparecem
- [ ] Click em A: A fica selecionado
- [ ] Click em B: B fica selecionado, A desseleciona
- [ ] Click em C: C fica selecionado, B desseleciona
- [ ] Checkmark animado ao selecionar

---

## 🐳 Docker Compose Commands

```bash
# Verificar status
docker compose ps

# Ver logs do frontend
docker compose logs frontend -f

# Ver logs do backend
docker compose logs backend -f

# Parar tudo
docker compose down

# Reiniciar
docker compose restart frontend

# Limpar volumes
docker compose down -v

# Rebuild
docker compose build --no-cache
```

---

## 📸 Screenshots para Documentação

1. **Homepage do Wizard** - Step 1 com 2 cards
2. **Template Selection** - Grid de 6 templates
3. **Question Editor** - Expanded view com QuestionEditor
4. **Fixed Sum** - Com distribuição de pontos
5. **Preview Panel** - Mostra resumo à direita
6. **Mobile View** - Layout responsivo

---

## ✅ Critérios de Aceitação

### Criterium 1: Zero Build Errors
- [x] Docker build completa com sucesso
- [x] 220 modules transformados sem erro
- [x] Bundle size < 200KB

### Criterium 2: Funcionalidade Completa
- [ ] Todos os 7 tipos de escala funcionam
- [ ] Drag & drop reordena perguntas
- [ ] Validações funcionam
- [ ] Responsividade OK

### Criterium 3: UX & Polish
- [ ] Animações suaves (200ms)
- [ ] Feedback visual para cada ação
- [ ] Sem console errors
- [ ] Acessível com keyboard

### Criterium 4: Performance
- [ ] Page load < 3s
- [ ] Interactions < 200ms
- [ ] Smooth 60fps animations
- [ ] Memory stable

---

## 🎯 Próximos Passos

1. **Backend Integration:**
   - Conectar POST /evaluations ao backend
   - Testar persistência em DB

2. **QR Code Generation:**
   - Integrar library qrcode.js
   - Gerar QR para link público

3. **Email Sharing:**
   - Integrar nodemailer
   - Enviar convites por email

4. **Analytics:**
   - Track: pergunta criada, tipo selecionado
   - Dashboard de analytics

5. **Unit Tests:**
   - Jest + Vue Test Utils
   - Coverage > 80%

---

## 📞 Suporte

**Dúvidas sobre componentes?**
Consultar: [SCALE_COMPONENTS_GUIDE.md](./SCALE_COMPONENTS_GUIDE.md)

**Spec de interface?**
Consultar: [/doc/engineering/interface_flow.md#seção-4]

