# 🎨 Interface Flow & UX Design Guide - Aevalo

> **Documento Completo de Especificação de Interface e Experiência do Usuário**  
> Última atualização: 08 de fevereiro de 2026

---

## 📋 Índice

1. [Design System & Identidade Visual](#-design-system--identidade-visual)
2. [Fluxos Principais e Telas](#-fluxos-principais-e-telas)
   - Autenticação e Onboarding
   - Dashboard Centralizador
   - Wizard de Criação Multi-Step
   - Gestão de Escalas Científicas
   - Visualização Pública
   - Cooperação e Compartilhamento
   - Resultados e Analytics
3. [Fluxos de Navegação Detalhados](#-fluxos-de-navegação-detalhados)
4. [Matriz Completa de Tratamento de Erros](#-matriz-completa-de-tratamento-de-erros)
5. [Estados Especiais de UI](#-estados-especiais-de-ui)
6. [Práticas de UX Inspiradas em SaaS Líderes](#-práticas-de-ux-inspiradas-em-saas-líderes)
7. [Responsividade e Acessibilidade](#-responsividade-e-acessibilidade)
8. [Sistema de Notificações](#-sistema-de-notificações)
9. [Persistência e Auto-Save](#-persistência-e-auto-save)
10. [Animações e Micro-interações](#-animações-e-micro-interações)
11. [Performance e Otimizações](#-performance-e-otimizações)
12. [Testes de Usabilidade](#-testes-de-usabilidade)
13. [Documentação para Desenvolvedores](#-documentação-para-desenvolvedores)
14. [Checklist de Implementação](#-checklist-de-implementação)

---

## 🎯 Resumo Executivo

Este documento especifica **todos os aspectos de UI/UX** do Aevalo - desde o design system base até fluxos complexos de erro e recuperação. Foi construído com base em **best practices de SaaS líderes** como Linear, Vercel, Notion, Stripe, Typeform e Figma.

### Destaques do Design:

- **Estética Modern Enterprise** com paleta roxo/laranja
- **7 tipos de escalas científicas** (Likert, Fixed Sum, Paired Comparison, etc.)
- **Geração inteligente via IA** (Gemini) com fallbacks robustos
- **Tratamento abrangente de erros** em todas as camadas
- **Monitoramento em tempo real** via Supabase Realtime
- **Acessibilidade WCAG 2.1 AA** em todos os componentes
- **Performance otimizada** (LCP < 2.5s, FID < 100ms)

### Princípios Fundamentais:

1. **Feedback Imediato:** Toda ação tem resposta visual instantânea
2. **Recuperação Graciosa:** Erros tratados com opções claras de correção
3. **Progressão Natural:** Fluxos guiam o usuário sem fricção
4. **Transparência:** Estado do sistema sempre visível
5. **Acessibilidade First:** Navegação por teclado e screen readers

---

## 🎨 Design System & Identidade Visual

Para este MicroSaaS, utilizaremos uma estética **"Modern Enterprise"** inspirada em SaaS como *Linear*, *Vercel*, *Notion* e *Stripe*:

### Paleta de Cores
* **Primary (Roxo Profundo):** `#4B0082` - Estrutura, branding, sidebar
* **Secondary (Laranja Vibrante):** `#FF8C00` - CTAs primários, alertas críticos
* **Accent (Roxo Médio):** `#9333EA` - Estados hover, badges, links
* **Success:** `#10B981` - Feedback positivo, confirmações
* **Warning:** `#F59E0B` - Avisos, validações pendentes
* **Error:** `#EF4444` - Erros críticos, validações falhas
* **Neutral:** `#64748B` - Textos secundários, borders

### Tipografia
* **Font Family:** Inter (sistema) / Geist Sans (alternativa)
* **Escala:** 12px/14px (caption) → 16px (body) → 20px/24px/32px (headings)
* **Line Height:** 1.5 para body, 1.2 para headings
* **Font Weight:** 400 (regular), 500 (medium), 600 (semibold), 700 (bold)

### Componentes Base
* **Cards:** `border-radius: 12px-16px`, sombra sutil `shadow-sm` a `shadow-lg`
* **Buttons:** 
  - Primary: bg-gradient com hover elevation
  - Secondary: outline com hover fill
  - Ghost: transparent com hover background
* **Inputs:** Borders leves, focus ring roxo, ícones à esquerda
* **Spacing:** Sistema de 4px (4, 8, 12, 16, 24, 32, 48, 64px)
* **Transitions:** `200ms ease` para hover, `300ms ease-out` para modals

### Micro-interações
* **Loading States:** Skeleton screens em gradiente pulsante
* **Empty States:** Ilustrações minimalistas + CTA destacado
* **Success Feedback:** Checkmark animado + toast verde com auto-dismiss
* **Error Feedback:** Toast vermelho persistente + opção de retry

---

## 🛣️ Fluxos Principais e Telas

### 0. Landing Page Pública (Marketing)

**URL:** `/` (Página inicial não autenticada)

**Inspirado em:** Vercel, Linear, Notion landing pages

#### 0.1 Estrutura Visual

**Hero Section (Above the Fold):**
```
┌──────────────────────────────────────────────────────┐
│  [Logo Aevalo]              [Recursos] [Preços]     │
│                            [Docs] [Entrar] [Começar]│
│                                                      │
│           ✨ Avaliações Científicas                  │
│              Alimentadas por IA                      │
│                                                      │
│   Crie pesquisas profissionais em minutos com       │
│   escalas validadas cientificamente                 │
│                                                      │
│   [email@exemplo.com] [ Começar Grátis →]          │
│                                                      │
│   ✓ Gratuito por 14 dias  ✓ Sem cartão  ✓ IA inclusa│
│                                                      │
│              [Preview animado do produto]            │
└──────────────────────────────────────────────────────┘
```

**Features Grid:**
```
┌─────────────────────────────────────────────┐
│  Por que escolher Aevalo?                   │
│  ──────────────────────────────────────     │
│                                             │
│  ┌──────────┬──────────┬──────────┐        │
│  │ 🤖 IA    │ 📊 7 Tipos│ ⚡ Rápido│        │
│  │          │          │          │        │
│  │ Gere     │ Likert,  │ Publique │        │
│  │ perguntas│ Fixed Sum│ em < 5min│        │
│  │ em 30s   │ e mais   │          │        │
│  └──────────┴──────────┴──────────┘        │
│                                             │
│  ┌──────────┬──────────┬──────────┐        │
│  │ 📈 Real  │ 🔒 Seguro│ 🎨 Design│        │
│  │ -Time    │          │          │        │
│  │          │ SOC2     │ Moderno  │        │
│  │ Veja     │ compliant│ e limpo  │        │
│  │ respostas│          │          │        │
│  └──────────┴──────────┴──────────┘        │
└─────────────────────────────────────────────┘
```

**How It Works (3 Steps):**
```
┌───────────────────────────────────────────────┐
│  Como funciona                                │
│  ───────────────────────────────────────      │
│                                               │
│  1️⃣ Descreva          2️⃣ Customize          3️⃣ Compartilhe │
│  ┌──────────┐      ┌──────────┐      ┌──────────┐ │
│  │ "Pesquisa│      │ Edite    │      │ Link ou  │ │
│  │  de NPS  │  →   │ perguntas│  →   │ QR Code  │ │
│  │  B2B"    │      │ e escalas│      │          │ │
│  └──────────┘      └──────────┘      └──────────┘ │
│                                               │
│         [ Ver Demo Interativo ]               │
└───────────────────────────────────────────────┘
```

**Use Cases Section:**
```
┌────────────────────────────────────────────┐
│  Casos de uso                              │
│  ──────────────────────────────────────    │
│                                            │
│  👔 RH & People Ops                        │
│  Pulse surveys, eNPS, clima organizacional │
│                                            │
│  🛍️ Produto & UX                          │
│  Satisfação, usabilidade, feature requests │
│                                            │
│  📚 Educação & Pesquisa                    │
│  Coleta de dados acadêmicos com rigor      │
│                                            │
│  🎯 Marketing                              │
│  Brand awareness, NPS, customer journey    │
└────────────────────────────────────────────┘
```

**Social Proof:**
```
┌────────────────────────────────────────────┐
│  Confiado por equipes de                   │
│  ────────────────────────────────────       │
│                                            │
│  [Logo] [Logo] [Logo] [Logo] [Logo]       │
│                                            │
│  "Reduzimos o tempo de criação de pesquisas│
│   de 2 horas para 5 minutos com Aevalo."  │
│   — Ana Silva, Head of UX @ TechCorp       │
│                                            │
│  ⭐⭐⭐⭐⭐ 4.9/5 de 200+ reviews           │
└────────────────────────────────────────────┘
```

**Pricing Table:**
```
┌─────────────────────────────────────────────────────┐
│  Planos simples e transparentes                     │
│  ─────────────────────────────────────────────      │
│                                                     │
│  ┌──────────┬──────────┬──────────┐               │
│  │ GRATUITO │ PRO      │ TEAM     │               │
│  │          │          │          │               │
│  │ R$ 0/mês │ R$ 49/mês│ R$ 149/mês│              │
│  │          │          │          │               │
│  │ 3 avaliações│ Ilimitado│ Ilimitado│            │
│  │ 50 resp. │ 1000 resp│ 5000 resp│               │
│  │ ✓ IA básica│ ✓ IA avançada│ ✓ Tudo PRO│        │
│  │          │ ✓ Export │ ✓ White label│           │
│  │          │ ✓ Analytics│ ✓ SSO      │           │
│  │          │          │ ✓ Suporte  │            │
│  │          │          │   prioritário│           │
│  │          │          │          │               │
│  │[Começar] │[Começar] │[Falar Vendas]│          │
│  └──────────┴──────────┴──────────┘               │
└─────────────────────────────────────────────────────┘
```

**FAQ Section:**
```
┌────────────────────────────────────────────┐
│  Perguntas Frequentes                      │
│  ──────────────────────────────────────    │
│                                            │
│  ▼ Preciso de cartão para testar?         │
│  ▶ Como funciona a geração por IA?        │
│  ▶ Posso exportar os dados?               │
│  ▶ É compatível com LGPD/GDPR?            │
│  ▶ Quanto tempo leva para criar?          │
│  ▶ Vocês têm API?                         │
└────────────────────────────────────────────┘
```

**Footer:**
```
┌────────────────────────────────────────────────────┐
│  ┌──────────┬──────────┬──────────┬──────────┐   │
│  │ Produto  │ Recursos │ Suporte  │ Empresa  │   │
│  │          │          │          │          │   │
│  │ Recursos │ Templates│ Docs     │ Sobre    │   │
│  │ Preços   │ IA       │ Status   │ Blog     │   │
│  │ Roadmap  │ Analytics│ Contato  │ Careers  │   │
│  │ Changelog│ API      │ Comunidade│ Legal    │   │
│  └──────────┴──────────┴──────────┴──────────┘   │
│                                                    │
│  © 2026 Aevalo. Todos os direitos reservados.    │
│  [Twitter] [LinkedIn] [GitHub] [YouTube]          │
└────────────────────────────────────────────────────┘
```

**Micro-interações:**
* Scroll suave entre seções
* Animação de fade-in conforme scroll
* Hover nos cards de features (elevation + scale)
* CTA buttons com gradient animado
* Preview do produto com slideshow automático

**Otimizações SEO:**
* Meta tags completos (título, descrição, OG)
* Schema.org markup (Product, Organization)
* Alt text em todas as imagens
* Sitemap.xml e robots.txt
* Core Web Vitals otimizados (LCP < 2.5s)

---

### 1. Autenticação e Onboarding

#### 1.1 Tela de Login/Cadastro (Modo Toggle)

**Layout Inspirado em:** Vercel, Linear
* **Componente Toggle:** Pill-style toggle entre "Entrar" e "Cadastrar" no topo
* **Gradiente Header:** Laranja → Rosa → Roxo com logo centralizado
* **Campos:**
  - Email (ícone de envelope à esquerda)
  - Senha (ícone de cadeado + toggle de visibilidade)
  - "Lembrar-me" checkbox (login) / "Aceito os termos" (cadastro)

**Validações em Tempo Real:**
* **Email:** Regex validation com feedback instantâneo
  - ✅ Verde: `seu@email.com` válido
  - ❌ Vermelho: formato inválido
* **Senha:** Medidor de força com 4 níveis
  - 🔴 Fraca: < 8 caracteres
  - 🟡 Média: 8+ caracteres
  - 🟢 Forte: 8+ chars + números + especiais
  - 🟢🟢 Muito Forte: 12+ chars completo

**Estados de Loading:**
```
Idle → [Usuário clica "Entrar"] 
     → Loading (botão com spinner, desabilitado)
     → Success (checkmark animado + redirect em 500ms)
     → Error (toast vermelho + botão volta ao estado normal)
```

**Tratamento de Erros de API:**

| Código HTTP | Mensagem UI | Ação do Sistema |
|------------|-------------|-----------------|
| `401` | "Email ou senha incorretos. Tente novamente." | Limpa campo senha, mantém email, foca no campo senha |
| `429` | "Muitas tentativas. Aguarde 60 segundos." | Desabilita botão por 60s com contador regressivo |
| `500` | "Nossos servidores estão temporariamente indisponíveis. Já estamos trabalhando nisso." | Botão "Tentar Novamente" + link para status page |
| `503` | "Sistema em manutenção. Retorne em instantes." | Banner informativo + estimativa de retorno |
| **Network Error** | "Sem conexão com a internet. Verifique sua rede." | Ícone offline + tentativa automática de reconexão |

**Fluxo de Onboarding (First-Time User):**
1. Após primeiro login bem-sucedido → Redirect para `/welcome`
2. **Step 1/3:** "Bem-vindo ao Aevalo! Vamos configurar sua conta"
3. **Step 2/3:** "Escolha suas categorias de interesse" (multi-select com chips)
4. **Step 3/3:** "Crie sua primeira avaliação" (wizard simplificado)
5. Ao completar → Confetti animation + redirect para Dashboard

---

### 2. Dashboard Centralizador (The Hub)

**Layout Inspirado em:** Linear, Notion, Stripe Dashboard

#### 2.1 Estrutura Visual

**Sidebar Esquerda (Persistente):**
* Logo + nome do workspace no topo
* Navegação principal:
  - 🏠 Dashboard (destaque)
  - 📊 Avaliações (com counter badge)
  - 📈 Analytics
  - ⚙️ Configurações
  - 👤 Perfil
* Footer: Avatar do usuário + dropdown com "Sair"

**Header Superior:**
* **Search Global:** Cmd+K (Mac) / Ctrl+K (Windows) para quick search
  - Busca por título de avaliação, perguntas, categorias
  - Resultados agrupados por tipo
  - Navegação por teclado (↑↓ + Enter)
* **Notifications Bell:** Badge com contador de alertas não lidos
* **User Dropdown:** Avatar + nome + email truncado

**Hero Section (Gradiente):**
* Título dinâmico baseado em hora do dia:
  - "Bom dia, [Nome]" (6h-12h)
  - "Boa tarde, [Nome]" (12h-18h)
  - "Boa noite, [Nome]" (18h-6h)
* 4 Cards de métricas em grid:
  1. **Prazo mais próximo** (countdown em dias, badge "Urgente" se < 5 dias)
  2. **Avaliadores ativos** (número + gráfico sparkline)
  3. **Taxa de conclusão** (porcentagem + progress ring visual)
  4. **Avaliações abertas** (número grande destacado)

#### 2.2 Área de Conteúdo Principal

**Search Bar + Filtros:**
* Input com debounce de 300ms
* Filtros dropdown:
  - Status: Todas / Abertas / Fechadas / Rascunho
  - Categoria: Multi-select com chips
  - Data: Última semana / Mês / Trimestre / Custom range
* Badge de "X filtros ativos" com opção "Limpar todos"

**Lista de Avaliações:**

**Vista: Cards Grid (padrão)** ou **Lista Compacta** (toggle no canto)

Cada card contém:
* **Header:** Emoji da categoria + título truncado (max 60 chars)
* **Meta:** Status badge + data de criação + última atividade
* **Stats Preview:** Mini progress bar + "X/Y respostas"
* **Actions:** Dropdown com:
  - 👁️ Visualizar
  - ✏️ Editar
  - 🔗 Copiar link
  - 📊 Ver resultados
  - 🗑️ Arquivar

**Pagination:** 
* Infinite scroll com "Load More" no final
* Skeleton cards durante loading (3-6 cards pulsando)

#### 2.3 Empty States (Muito Importante!)

**Cenário 1: Nenhuma avaliação criada (`count == 0`)**
```
┌─────────────────────────────────┐
│   [Ilustração minimalista]      │
│                                  │
│   Crie sua primeira avaliação   │
│   para começar                  │
│                                  │
│   [ + Nova Avaliação ]          │  ← Botão laranja destaque
│                                  │
│   💡 Dica: Use IA para gerar    │
│   perguntas em segundos         │
└─────────────────────────────────┘
```

**Cenário 2: Busca sem resultados**
```
┌─────────────────────────────────┐
│   [Ícone de lupa]               │
│                                  │
│   Nenhum resultado para         │
│   "[termo de busca]"            │
│                                  │
│   Sugestões:                    │
│   • Tente termos diferentes     │
│   • Remova filtros aplicados    │
│   • Busque por categorias       │
│                                  │
│   [ Limpar Busca ]              │
└─────────────────────────────────┘
```

**Cenário 3: Erro ao carregar dados**
```
┌─────────────────────────────────┐
│   [Ícone de alerta]             │
│                                  │
│   Não conseguimos carregar      │
│   suas avaliações               │
│                                  │
│   Erro: [mensagem técnica]      │
│                                  │
│   [ 🔄 Tentar Novamente ]       │
│   [ 📞 Reportar Problema ]      │
└─────────────────────────────────┘
```

#### 2.4 Analytics Widget (Dashboard)

**Gráfico de Barras Interativo:**
* Biblioteca: Chart.js ou Recharts
* Dados: Número de avaliações por categoria
* Interação: Click em barra → filtra lista abaixo automaticamente
* Loading state: Skeleton do gráfico com shimmer effect
* Empty state: "Crie avaliações para ver estatísticas"

**Estados de Erro do Analytics:**

| Erro | Feedback Visual | Ação |
|------|----------------|-------|
| Timeout da API | "Dados demorando para carregar..." + botão retry | Retry automático após 5s |
| Dados incompletos | Gráfico parcial + warning toast | Mostra dados disponíveis + aviso |
| Falha total | Empty state com ícone erro | Botão "Recarregar Dashboard" |

---

### 3. Wizard de Criação de Avaliação (Multi-Step)

**Inspirado em:** Typeform, Notion AI, ChatGPT UI

#### 3.1 Step 1 - Escolha do Método

**Layout:** Dois cards grandes lado a lado (mobile: stack vertical)

**Card 1: Usar Template**
```
┌────────────────────────┐
│  📋                    │
│  Começar com Template  │
│                        │
│  Escolha entre modelos │
│  prontos e customize   │
│                        │
│  ✓ Rápido             │
│  ✓ Testado            │
│  ✓ Categorizado       │
│                        │
│  [ Escolher Template ] │
└────────────────────────┘
```

**Card 2: Gerar com IA**
```
┌────────────────────────┐
│  ✨                    │
│  Criar com IA          │
│                        │
│  Descreva o que precisa│
│  e deixe a IA criar    │
│                        │
│  ✓ Personalizado      │
│  ✓ Inteligente        │
│  ✓ Criativo           │
│                        │
│  [ Gerar com Gemini ]  │
└────────────────────────┘
```

**Navegação:** Progress bar no topo: ● ○ ○ ○ (4 steps total)

#### 3.2 Step 2A - Seleção de Template (se escolheu template)

**Layout:** Grid de cards 3x2 com preview

Cada template mostra:
* Nome do template
* Categoria (badge colorido)
* Preview de 2-3 perguntas
* Número de perguntas total
* Badge "Mais usado" nos populares

**Busca:** Input no topo para filtrar templates
**Filtro:** Dropdown por categoria

**Loading State:** 6 skeleton cards durante fetch

#### 3.2 Step 2B - Input de IA (se escolheu IA)

**Layout:** Textarea grande com prompts de exemplo

**Interface:**
```
┌────────────────────────────────────────┐
│  Descreva a avaliação que você precisa│
│  ────────────────────────────────────  │
│  [Textarea grande]                     │
│                                        │
│  💡 Exemplos que funcionam bem:       │
│  • "Avaliação de clima organizacional │
│     focada em trabalho remoto"        │
│  • "Pesquisa de satisfação pós-compra │
│     para e-commerce"                  │
│  • "Feedback 360° para líderes"      │
│                                        │
│  [ ✨ Gerar com IA ]                  │
│                                        │
│  ⚙️ Avançado: [Número de perguntas]  │
│              [Tom: Formal/Casual]     │
└────────────────────────────────────────┘
```

**Durante Geração da IA:**

**Estado 1: Processando (0-3s)**
```
┌────────────────────────────────┐
│  [Spinner animado]             │
│  Analisando sua solicitação... │
│  Gemini está pensando...       │
└────────────────────────────────┘
```

**Estado 2: Gerando (3-10s)**
```
┌────────────────────────────────┐
│  [Progress bar animado]        │
│  Criando suas perguntas...     │
│  Quase pronto! ✨              │
└────────────────────────────────┘
```

**Estado 3: Sucesso**
```
┌────────────────────────────────┐
│  ✅ Avaliação gerada!          │
│  8 perguntas criadas           │
│  [ Continuar para Edição ]     │
└────────────────────────────────┘
```

**Tratamento de Erros da IA:**

| Erro | Mensagem | Ações Oferecidas |
|------|----------|------------------|
| **Timeout (>30s)** | "A IA está demorando mais que o esperado..." | • Continuar aguardando<br>• Usar template similar<br>• Criar manualmente |
| **Erro 429 (Rate Limit)** | "Muitas requisições. Tente em 1 minuto." | • Countdown timer<br>• Usar template enquanto espera |
| **Erro 500 (Gemini API)** | "Serviço de IA temporariamente indisponível." | • Tentar novamente<br>• Usar template<br>• Criar manualmente |
| **Input inválido** | "Descrição muito curta. Adicione mais detalhes." | • Tooltip com exemplos<br>• Mínimo 20 caracteres |
| **Conteúdo inapropriado** | "Não foi possível gerar. Revise o conteúdo." | • Sugestão de reformulação |

**Opção de Bailout:** Em qualquer momento, botão "Prefiro criar manualmente" no canto

#### 3.3 Step 3 - Configuração e Customização

**Layout:** Split screen (preview à direita, editor à esquerda)

**Lado Esquerdo - Editor:**
* **Título da avaliação:** Input editável inline
* **Descrição:** Textarea com contador de caracteres (max 500)
* **Categoria:** Dropdown + opção "Criar nova categoria"
* **Lista de Perguntas (Drag & Drop):**
  - Cada pergunta tem:
    - Handle para arrastar (⋮⋮)
    - Texto da pergunta (editável inline)
    - Dropdown de tipo de escala
    - Botão deletar (ícone lixeira)
    - Toggle "Obrigatória"
  - Botão "+ Adicionar Pergunta" no final

**Tipos de Escala (Dropdown):**
1. **Likert** (1-5, 1-7, 1-10)
2. **Frequency** (Nunca → Sempre)
3. **Paired Comparison** (A vs B)
4. **Fixed Sum** (Distribuir 100 pontos)
5. **Texto Aberto**
6. **Múltipla Escolha**
7. **Escolha Única**

**Lado Direito - Preview:**
* Mockup da avaliação como o avaliador verá
* Atualização em tempo real (debounce 300ms)
* Botão "Ver em dispositivo móvel" (toggle)

**Validações:**
* Título obrigatório (mín. 5 caracteres)
* Ao menos 1 pergunta
* Cada pergunta com texto e tipo definido

**Navegação:**
* "← Voltar" (salva como rascunho)
* "Salvar Rascunho" (botão secundário)
* "Continuar →" (valida antes de avançar)

#### 3.4 Step 4 - Publicação e Compartilhamento

**Layout:** Card centralizado com opções

**Configurações Finais:**
* **Status:** 
  - 📝 Rascunho (visível só para você)
  - 🔓 Aberta (aceita respostas)
  - 🔒 Fechada (não aceita mais respostas)
* **Prazo:** Date picker + time picker (opcional)
* **Limite de respostas:** Number input (opcional, 0 = ilimitado)
* **Requer autenticação:** Toggle (sim/não)
* **Permitir respostas anônimas:** Toggle

**Geração de Link:**
Ao publicar, o sistema gera:
1. **Short URL:** `aevalo.app/e/abc123` (copyable)
2. **QR Code:** Gerado em tempo real, download PNG/SVG
3. **Embed Code:** `<iframe>` para incorporar em sites

**Ações:**
* [ Copiar Link ] (click to copy + toast confirmation)
* [ Baixar QR Code ]
* [ Enviar por Email ] (abre modal)
* [ Compartilhar em Redes Sociais ] (Twitter, LinkedIn, WhatsApp)

**Finalização:**
```
✅ Avaliação publicada com sucesso!

Seu link público:
┌────────────────────────────┐
│ aevalo.app/e/abc123        │
│ [📋 Copiar]                │
└────────────────────────────┘

[ Ir para Dashboard ]  [ Ver Avaliação ]
```

---

## 🛠️ Tabela de Tratamento de Erros (Resiliência do Sistema)

| Cenário de Erro | Feedback UI/UX | Ação do Sistema |
| --- | --- | --- |
| **Falha na API Rust** | Toast flutuante: "Servidor indisponível" | Log de erro interno; mantém dados no estado do Vue. |
| **Timeout da Gemini API** | Alerta no editor: "A IA está demorando..." | Opção de "Continuar manualmente" ou "Re-gerar". |
| **Erro de Auth (Token Expulso)** | Redirecionamento suave para Login | Limpa `sessionStorage` e exibe mensagem de sessão expirada. |
| **Offline (Sem Internet)** | Banner discreto no topo: "Modo Offline" | Bloqueia ações de escrita; permite navegação em cache. |

---

## 🚀 Práticas de UI Inspiradas em SaaS Famosos

* **Notion-like:** Uso de ícones (emojis ou Lucide) para categorias para facilitar o reconhecimento visual.
* **Typeform-like:** Transições suaves entre as perguntas durante a criação para reduzir a carga cognitiva.
* **Stripe-like:** Dashboards limpos com muito "espaço em branco" para destacar as métricas de sucesso.

---
---

## 🏗️ Proposta de Wireframes (Low-Fidelity)

### 1. Dashboard Centralizador (The Hub)


### 2. Wizard de Criação Híbrida

Este fluxo foca em reduzir a fricção inicial através de IA ou modelos prontos.

* **Passo 1: Seleção de Método:** Dois cards grandes. "Usar Template" (Caminho Curado) ou "Gerar com IA" (Caminho Assistido).
* **Passo 2 (Se IA):** Campo de texto para descrição breve.
* *UX:* Enquanto a Gemini API processa, exibe um "Skeleton Screen" pulsante em roxo.


* **Passo 3: Editor de Itens:** Lista de perguntas geradas. Cada item tem um seletor para o tipo de escala (Likert, Frequency, Paired Comparison ou Fixed Sum).

### 3. Editor de Escalas Técnicas

A interface deve garantir o rigor científico das metodologias *MeasuringU*.

* **Componente Likert/Frequency:** Escala horizontal de 1 a 5 ou 1 a 7 com labels customizáveis.
* **Componente Fixed Sum:** Lista de itens com campos numéricos ao lado. No rodapé, um indicador de "Soma Total".
* *Lógica de Erro:* Se a soma , o indicador fica laranja e o botão "Publicar" é bloqueado.


---

### 4. Gestão de Escalas Científicas (MeasuringU Methods)

**Inspirado em:** Typeform, Google Forms, SurveyMonkey

#### 4.1 Likert Scale (1-5, 1-7, 1-10)

**Interface - Modo Desktop:**
* Escala horizontal com círculos clicáveis
* Labels nas extremidades: "Discordo Totalmente" ← → "Concordo Totalmente"
* Hover effect: círculo cresce + cor de preview
* Selected: círculo preenchido + gradiente roxo

**Interface - Modo Mobile:**
* Botões grandes verticais (acessibilidade touch)
* Número grande + label abaixo
* Scroll suave entre perguntas

**Validação:**
* Se marcado como "Obrigatória" e não respondido → border vermelha + mensagem
* Auto-save após seleção (debounce 500ms)

#### 4.2 Frequency Scale

**Interface:**
* Chips horizontais: `Nunca | Raramente | Às vezes | Frequentemente | Sempre`
* Seleção única, visual de radio buttons modernos
* Transição suave de cor ao selecionar

#### 4.3 Paired Comparison (A vs B)

**Interface:**
```
Qual opção você prefere?

┌─────────────┐         ┌─────────────┐
│   Opção A   │   VS    │   Opção B   │
│             │         │             │
│ [Imagem/    │         │ [Imagem/    │
│  Descrição] │         │  Descrição] │
│             │         │             │
│ [ Escolher] │         │ [ Escolher] │
└─────────────┘         └─────────────┘

       [ Sem preferência ]
```

**Lógica:**
* Matriz de preferências salva no backend
* Algoritmo calcula ranking final
* Permite empate com botão "Sem preferência"

#### 4.4 Fixed Sum (Distribuir 100 Pontos)

**Interface:**
```
Distribua 100 pontos entre as opções:

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Atributo 1    [Input: 40] ░░░░░░░░░░░░░░░░░░░░░░░░ 40%
Atributo 2    [Input: 35] ░░░░░░░░░░░░░░░░░░░░░ 35%
Atributo 3    [Input: 25] ░░░░░░░░░░░░░░░ 25%
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Total usado: 100/100 ✅
```

**Validações em Tempo Real:**

| Condição | Feedback Visual | Botão Enviar |
|----------|-----------------|--------------|
| Soma = 100 | Barra verde + ✅ "Perfeito!" | Habilitado |
| Soma < 100 | Barra laranja + "Faltam X pontos" | Desabilitado |
| Soma > 100 | Barra vermelha + "Retire X pontos" | Desabilitado |

**UX Adicional:**
* Inputs com step de 1
* Botões +/- ao lado de cada input
* Barra de progresso visual (progress bar)
* Cores mudam dinamicamente: Verde (=100), Laranja (<100), Vermelho (>100)
* Atalho: Botão "Distribuir Igualmente" (divide 100 por N)

#### 4.5 Texto Aberto

**Interface:**
* Textarea com contador de caracteres
* Limite configurável (ex: 500 caracteres)
* Auto-resize conforme usuário digita
* Salvar rascunho automático a cada 3 segundos

#### 4.6 Múltipla Escolha

**Interface:**
* Checkboxes com labels grandes
* Permite seleção de múltiplas opções
* Opcional: limite mínimo/máximo de seleções
* Contador: "2 de 3 selecionadas" (se limite definido)

#### 4.7 Escolha Única

**Interface:**
* Radio buttons estilizados como cards
* Visual de seleção clara (border + background)
* Obrigatoriamente uma única opção

---

### 5. Visualização Pública (Avaliador Externo)

**URL:** `aevalo.app/e/abc123` (link curto)

#### 5.1 Landing da Avaliação

**Header:**
* Logo Aevalo (discreto, canto superior esquerdo)
* Título da avaliação (grande, centralizado)
* Descrição breve
* Estimativa de tempo: "~5 minutos" (calculado automaticamente)

**Navegação:**
* Progress bar no topo: "Pergunta 3 de 10"
* Botões "← Anterior" e "Próxima →"
* Modo de uma pergunta por vez (Typeform-style) ou todas de uma vez (toggle)

**Footer:**
* "Powered by Aevalo" (link discreto)
* Política de privacidade
* Suporte/contato

#### 5.2 Estados do Link Público

| Status | Mensagem ao Acessar | Visual |
|--------|---------------------|--------|
| **Draft** | "Esta avaliação ainda não foi publicada." | Ícone 📝 + mensagem informativa |
| **Open** | [Exibe avaliação normalmente] | Interface completa |
| **Closed** | "Esta avaliação foi encerrada em [data]." | Ícone 🔒 + agradecimento |
| **Expired** | "O prazo para responder expirou." | Contador mostrando quando expirou |
| **Limit Reached** | "Limite de respostas atingido. Obrigado!" | Ícone ✅ + mensagem de sucesso |
| **Invalid UUID** | "Link inválido ou expirado." | Erro 404 amigável |

#### 5.3 Submissão de Resposta

**Validação Pré-Envio:**
1. Verifica perguntas obrigatórias não respondidas
2. Valida Fixed Sum = 100
3. Verifica limites de caracteres em texto aberto
4. Confirma todas as validações customizadas

**Se houver erros:**
* Scroll suave até primeira pergunta com erro
* Highlight vermelho + mensagem específica
* Lista de erros no topo: "3 perguntas precisam de atenção"

**Ao Submeter:**
```
Estado 1: Enviando...
┌────────────────────────┐
│ [Spinner]              │
│ Salvando respostas...  │
└────────────────────────┘

Estado 2: Sucesso! ✅
┌────────────────────────┐
│ ✅ Resposta enviada!   │
│                        │
│ Obrigado por participar│
│                        │
│ [Fechar] [Ver Outra]   │
└────────────────────────┘
```

**Erros na Submissão:**

| Erro | Mensagem | Ação |
|------|----------|------|
| **Network timeout** | "Conexão perdida. Suas respostas foram salvas localmente." | Botão "Tentar enviar novamente" |
| **Servidor 500** | "Erro ao salvar. Tente novamente em instantes." | Auto-retry em 5s + botão manual |
| **Avaliação fechada** | "Esta avaliação foi encerrada enquanto você respondia." | Mensagem de desculpas + link para reportar |
| **Resposta duplicada** | "Você já respondeu esta avaliação." | (Se anônima) / (Se autenticada) |

---

### 6. Cooperação e Compartilhamento

#### 6.1 Modal de Compartilhamento

**Trigger:** Botão "Compartilhar" no dashboard ou após publicar

**Layout:**
```
┌──────────────────────────────────────┐
│  🔗 Compartilhar Avaliação          │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                      │
│  Link Público:                       │
│  ┌────────────────────────────────┐ │
│  │ aevalo.app/e/abc123      [📋]  │ │
│  └────────────────────────────────┘ │
│                                      │
│  QR Code:                            │
│  ┌──────┐  [⬇️ Baixar PNG]         │
│  │██████│  [⬇️ Baixar SVG]         │
│  │██  ██│                           │
│  │██████│                           │
│  └──────┘                            │
│                                      │
│  Compartilhar via:                   │
│  [📧 Email] [🐦 Twitter]            │
│  [💼 LinkedIn] [💬 WhatsApp]        │
│                                      │
│  Código de Incorporação:             │
│  <iframe src="..."></iframe> [📋]   │
│                                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│  [ Fechar ]                          │
└──────────────────────────────────────┘
```

**Funcionalidades:**
* **Copy to Clipboard:** Click no ícone 📋 → Toast "Link copiado!"
* **QR Code:** Gerado dinamicamente via API
* **Social Share:** Usa Web Share API (mobile) ou abre pop-ups (desktop)
* **Email Share:** Modal secundário com:
  - Campo "Para:" (múltiplos emails separados por vírgula)
  - Assunto pré-preenchido: "Participe da avaliação: [Título]"
  - Corpo personalizável
  - Botão "Enviar Convites"

#### 6.2 Monitoramento em Tempo Real

**Widget no Dashboard:** "Avaliadores Ativos Agora"

```
┌──────────────────────────────────┐
│  👥 Avaliadores Ativos (3)      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                  │
│  🟢 Anônimo #1   Pergunta 5/10  │
│  🟢 João Silva   Pergunta 8/10  │
│  🟢 Anônimo #2   Pergunta 2/10  │
│                                  │
│  Última resposta: há 2 minutos  │
└──────────────────────────────────┘
```

**Tecnologia:** Supabase Realtime
* Atualização ao vivo quando nova resposta chega
* Animação de entrada para novo avaliador
* Contador de respostas totais incrementa em tempo real

#### 6.3 Encerramento da Avaliação

**Ação:** Botão "Encerrar Avaliação" no dashboard

**Confirmação:**
```
┌──────────────────────────────────┐
│  ⚠️ Encerrar Avaliação?         │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
│                                  │
│  Esta ação irá:                  │
│  ✓ Fechar o link público         │
│  ✓ Impedir novas respostas       │
│  ✓ Gerar relatório final         │
│                                  │
│  Você tem 47 respostas coletadas │
│                                  │
│  [ Cancelar ]  [ Sim, Encerrar ] │
└──────────────────────────────────┘
```

**Após Confirmar:**
1. Status muda para `Closed`
2. Link público exibe mensagem de encerrado
3. Processo de análise de dados inicia
4. Redirect para página de resultados

---

### 7. Resultados e Analytics

**Inspirado em:** Typeform Results, Google Analytics, Mixpanel

#### 7.1 Overview de Resultados

**Header:**
* Título da avaliação
* Datas: "Aberta em X - Encerrada em Y"
* Total de respostas
* Taxa de conclusão: "89% (47/53 iniciadas)"

**KPIs em Cards:**
```
┌────────────┬────────────┬────────────┬────────────┐
│ Respostas  │ Tempo Médio│ Taxa Drop  │ NPS Score  │
│    47      │  4min 32s  │    11%     │    +72     │
└────────────┴────────────┴────────────┴────────────┘
```

#### 7.2 Análise por Pergunta

**Likert/Frequency:**
* Gráfico de barras horizontais com distribuição
* Média calculada e destacada
* Desvio padrão
* Modo (resposta mais comum)

**Fixed Sum:**
* Gráfico de pizza ou donut chart
* Tabela com médias de cada atributo
* Ranking de preferência

**Paired Comparison:**
* Matriz de comparações
* Ranking final calculado
* Heatmap de preferências

**Texto Aberto:**
* Lista de respostas (paginada)
* Word cloud das palavras mais frequentes
* Filtro/busca por termo
* Exportar para CSV

**Múltipla Escolha:**
* Gráfico de barras
* Porcentagem de cada opção
* Combinações mais comuns (se aplicável)

#### 7.3 Filtros e Segmentação

**Filtros Disponíveis:**
* Por data de resposta
* Por tempo de conclusão (rápidas vs lentas)
* Por dispositivo (mobile vs desktop)
* Por origem (se tracking habilitado)

**Comparações:**
* Antes vs Depois (se range de datas)
* Grupo A vs Grupo B (se segmentação definida)

#### 7.4 Exportação de Dados

**Opções:**
* 📊 Exportar para Excel (.xlsx)
* 📄 Exportar para CSV
* 📈 Exportar gráficos (PNG/SVG)
* 📑 Gerar PDF do relatório completo

**Botão:** "Exportar Dados" no canto superior direito

---

## 🔄 Fluxos de Navegação Detalhados

### Fluxo A: Jornada Completa - Criar e Publicar Avaliação

**Etapas:**

1. **Dashboard** → Click "Nova Avaliação"
2. **Step 1/4:** Escolher método (Template vs IA)
3. **Step 2/4:** 
   - Se Template: Selecionar da biblioteca
   - Se IA: Descrever necessidade + aguardar geração
4. **Step 3/4:** Customizar perguntas (editor drag-and-drop)
5. **Step 4/4:** Configurar publicação + gerar link
6. **Sucesso:** Modal de compartilhamento + redirect para Dashboard

**Pontos de Saída:**
* A qualquer momento: "Salvar como Rascunho" (preserva progresso)
* Botão "← Voltar" em cada step (navegação preservada)
* Fechar aba: Auto-save local (recuperação ao retornar)

**Tempo Estimado:** 3-7 minutos

---

### Fluxo B: Recuperação de Erro na Geração IA

**Cenário:** Timeout da Gemini API

```
1. Usuário descreve: "Avaliação de UX para app mobile"
2. Click "Gerar com IA"
3. Loading 0-15s → Normal
4. Loading 15-30s → Warning: "Está demorando mais que esperado..."
5. Timeout 30s+ → Erro exibido com 3 opções:
   
   ┌─────────────────────────────────────┐
   │ ⚠️ A IA está demorando demais      │
   │                                     │
   │ Escolha uma opção:                  │
   │                                     │
   │ [ 🔄 Tentar Novamente ]            │
   │ [ 📋 Usar Template Similar ]       │
   │ [ ✍️ Criar Manualmente ]           │
   │                                     │
   │ 💡 Sua descrição foi salva         │
   └─────────────────────────────────────┘

6a. Se "Tentar Novamente": Reenvio da requisição
6b. Se "Template Similar": Busca templates com palavras-chave da descrição
6c. Se "Criar Manualmente": Vai direto para editor vazio
```

**Fallbacks Inteligentes:**
* Sistema detecta keywords na descrição (ex: "UX", "mobile")
* Sugere templates mais relevantes automaticamente
* Preserva descrição para uso futuro ou retry

---

### Fluxo C: Avaliador Respondendo (Via Link Público)

**Jornada do Respondente:**

```
1. Recebe link: aevalo.app/e/abc123
2. Acessa no navegador
3. Landing page:
   ┌─────────────────────────────────┐
   │  [Logo Aevalo]                  │
   │                                 │
   │  Avaliação de Satisfação        │
   │  E-commerce XYZ                 │
   │                                 │
   │  ⏱️ ~5 minutos                  │
   │  📊 10 perguntas                │
   │  🔒 Anônimo                     │
   │                                 │
   │  [ Começar Avaliação ]          │
   └─────────────────────────────────┘

4. Navegação entre perguntas:
   - Modo Typeform: Uma pergunta por tela
   - Progress bar: "Pergunta 3 de 10"
   - Botões: "← Anterior" | "Próxima →"

5. Validações em tempo real:
   - Fixed Sum: Contador atualiza a cada input
   - Obrigatórias: Botão "Próxima" desabilitado até responder

6. Última pergunta → Botão muda para "Enviar Respostas"

7. Confirmação:
   ┌─────────────────────────────────┐
   │  Revisar antes de enviar?       │
   │                                 │
   │  ✅ 10 perguntas respondidas    │
   │                                 │
   │  [ Revisar ]  [ Enviar ]        │
   └─────────────────────────────────┘

8. Ao enviar:
   - Loading spinner: "Salvando..."
   - Sucesso: ✅ + Confetti animation
   - Mensagem: "Obrigado! Suas respostas foram salvas."

9. Tela final:
   - Agradecimento personalizado
   - Opção: "Responder outra avaliação"
   - Powered by Aevalo (discreto)
```

**Interrupções Tratadas:**

| Situação | Comportamento |
|----------|---------------|
| Usuário fecha aba no meio | Respostas salvas localmente (localStorage) |
| Retorna ao link | "Você tem respostas não enviadas. Continuar?" |
| Link expira durante resposta | Aviso + opção de salvar respostas como PDF |
| Conexão perdida | "Modo offline - Respostas salvas localmente" |
| Bateria baixa (mobile) | Prompt para salvar e continuar depois |

---

### Fluxo D: Monitoramento em Tempo Real (Owner)

**Dashboard do Proprietário:**

```
1. Avaliação publicada e link compartilhado
2. Dashboard atualiza ao vivo via Supabase Realtime:

   ┌──────────────────────────────────────┐
   │  📊 Avaliação: Satisfação Cliente   │
   │  ─────────────────────────────────── │
   │                                      │
   │  Status: 🟢 Aberta                  │
   │  Link: aevalo.app/e/abc123 [📋]     │
   │                                      │
   │  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━ │
   │                                      │
   │  📈 Respostas: 47 / ∞               │
   │  [████████████████░░░░░] 78%        │
   │                                      │
   │  👥 Ativos agora: 3                 │
   │  🟢 Anônimo #12  - Pergunta 7/10   │
   │  🟢 Maria Silva  - Pergunta 3/10   │
   │  🟢 Anônimo #13  - Pergunta 9/10   │
   │                                      │
   │  ⏱️ Tempo médio: 4min 32s           │
   │  📍 Última resposta: há 12s         │
   │                                      │
   │  [ 📊 Ver Resultados ]              │
   │  [ 🔗 Compartilhar ]                │
   │  [ 🔴 Encerrar Avaliação ]          │
   └──────────────────────────────────────┘

3. A cada nova resposta:
   - Contador incrementa com animação
   - Toast discreto: "Nova resposta recebida"
   - Gráficos de resultados parciais atualizam

4. Quando decide encerrar:
   - Click "Encerrar Avaliação"
   - Modal de confirmação (evita acidente)
   - Após confirmar: Status → Closed
   - Link público exibe: "Avaliação encerrada"
   - Redirect automático para página de Resultados
```

---

### Fluxo E: Análise de Resultados

**Página de Resultados:**

```
1. Acesso via Dashboard → "Ver Resultados"
2. Carregamento:
   - Skeleton screens dos gráficos
   - "Processando 47 respostas..."
   
3. Overview carregado:
   ┌──────────────────────────────────────┐
   │  📊 Resultados: Satisfação Cliente  │
   │  ─────────────────────────────────── │
   │                                      │
   │  🗓️ 12 jan - 18 jan (7 dias)       │
   │  📬 47 respostas                    │
   │  ⏱️ 4min 32s (média)                │
   │  ✅ Taxa conclusão: 89%             │
   │                                      │
   │  [ 📊 Exportar Dados ]              │
   └──────────────────────────────────────┘

4. Scroll down: Análise por pergunta
   - Gráficos interativos (hover mostra detalhes)
   - Likert: Média + distribuição
   - Fixed Sum: Ranking de preferências
   - Texto aberto: Word cloud + lista

5. Filtros laterais:
   - Por data
   - Por dispositivo
   - Por tempo de resposta

6. Exportação:
   ┌─────────────────────────────┐
   │  Exportar Resultados        │
   │  ──────────────────────────  │
   │                             │
   │  [ 📊 Excel (.xlsx) ]       │
   │  [ 📄 CSV ]                 │
   │  [ 📑 PDF (Relatório) ]     │
   │  [ 📈 Gráficos (PNG) ]      │
   │                             │
   │  [ Baixar Tudo (.zip) ]     │
   └─────────────────────────────┘
```

---

### 8. Biblioteca de Templates

**URL:** `/templates`

**Inspirado em:** Notion Templates, Typeform Templates, Figma Community

#### 8.1 Página Principal de Templates

**Header com Busca:**
```
┌──────────────────────────────────────────────────┐
│  📋 Biblioteca de Templates                      │
│  ──────────────────────────────────────────────  │
│                                                  │
│  [🔍 Buscar templates...]         [Filtros ▼]   │
│                                                  │
│  ✨ Destaque: Template da Semana                │
│  ┌────────────────────────────────────────┐     │
│  │  [Preview Image]                       │     │
│  │  NPS para SaaS B2B                     │     │
│  │  12 perguntas · Likert + Text          │     │
│  │  ⭐ 4.8 · 1.2k usos                    │     │
│  │  [ Usar Template ]                     │     │
│  └────────────────────────────────────────┘     │
└──────────────────────────────────────────────────┘
```

**Filtros Laterais:**
```
┌───────────────────────┐
│ Filtros               │
│ ─────────────────     │
│                       │
│ 📂 Categoria          │
│ □ RH & People Ops (23)│
│ □ UX & Produto (18)   │
│ □ Marketing (15)      │
│ □ Educação (12)       │
│ □ Atendimento (9)     │
│                       │
│ 📊 Tipo de Escala     │
│ □ Likert (45)         │
│ □ Fixed Sum (12)      │
│ □ Paired Comp. (8)    │
│ □ Frequency (34)      │
│                       │
│ ⏱️ Duração            │
│ □ Rápida (< 3 min)   │
│ □ Média (3-7 min)    │
│ □ Longa (> 7 min)    │
│                       │
│ 🏷️ Tags               │
│ □ NPS (23)            │
│ □ eNPS (18)           │
│ □ CSAT (15)           │
│ □ Onboarding (12)     │
│                       │
│ [ Limpar Filtros ]    │
└───────────────────────┘
```

**Grid de Templates:**
```
┌────────────┬────────────┬────────────┐
│ 📊 RH      │ 🎨 UX      │ 📈 Marketing│
│            │            │            │
│ eNPS Q1    │ Onboarding │ Brand      │
│ 2026       │ Survey     │ Awareness  │
│            │            │            │
│ 8 perguntas│ 12 perguntas│ 10 perguntas│
│ 3 min      │ 5 min      │ 4 min      │
│ ⭐ 4.7 · 890│ ⭐ 4.9 · 1.5k│ ⭐ 4.6 · 450│
│            │            │            │
│ [Ver] [Usar]│ [Ver] [Usar]│ [Ver] [Usar]│
└────────────┴────────────┴────────────┘
```

**Estados:**
* **Loading:** Skeleton cards (6-9 cards pulsando)
* **Empty Search:** "Nenhum template encontrado para '[termo]'"
* **No Filters Match:** "Nenhum resultado com esses filtros. Tente remover alguns."

#### 8.2 Preview de Template

**URL:** `/templates/:id/preview`

**Layout Split:**
```
┌────────────────────────────────────────────────────┐
│  ← Voltar aos Templates                            │
│  ──────────────────────────────────────────────────│
│                                                    │
│  ┌─────────────┐  NPS para SaaS B2B              │
│  │  Preview    │                                   │
│  │  Image      │  Por Aevalo Official              │
│  └─────────────┘  ⭐ 4.8 (1.2k avaliações)        │
│                                                    │
│  📝 Descrição:                                     │
│  Template completo para medir NPS em empresas      │
│  SaaS B2B, incluindo perguntas de follow-up e      │
│  segmentação por persona.                          │
│                                                    │
│  ✨ Destaque:                                      │
│  • 12 perguntas otimizadas                         │
│  • Mix de escalas (Likert + Text)                 │
│  • Testado com 500+ clientes                       │
│  • Tempo médio: 4 minutos                          │
│                                                    │
│  🏷️ Tags: NPS, B2B, SaaS, Customer Success        │
│                                                    │
│  ──────────────────────────────────────────────────│
│                                                    │
│  📋 Perguntas incluídas (12):                      │
│                                                    │
│  1. Em uma escala de 0-10, qual a probabilidade   │
│     de recomendar nosso produto?                   │
│     Tipo: NPS Scale (0-10)                         │
│                                                    │
│  2. Qual o principal motivo da sua nota?          │
│     Tipo: Texto Aberto                             │
│                                                    │
│  3. Com que frequência você usa nosso produto?    │
│     Tipo: Frequency Scale                          │
│     (Diariamente → Raramente)                      │
│                                                    │
│  [... ver todas as 12 perguntas]                  │
│                                                    │
│  ──────────────────────────────────────────────────│
│                                                    │
│  💬 Reviews (87):                                  │
│                                                    │
│  ⭐⭐⭐⭐⭐ João Silva                              │
│  "Excelente template, economizou horas de trabalho!"│
│  há 2 dias                                         │
│                                                    │
│  ⭐⭐⭐⭐ Maria Santos                              │
│  "Ótimo ponto de partida, personalizei para meu    │
│   contexto facilmente."                            │
│  há 1 semana                                       │
│                                                    │
│  [ Ver todas reviews ]                             │
│                                                    │
│  ──────────────────────────────────────────────────│
│                                                    │
│  [ ← Voltar ]  [ ✨ Usar Este Template ]          │
└────────────────────────────────────────────────────┘
```

**Ação "Usar Template":**
1. Redirect para `/evaluations/create`
2. Wizard abre no Step 3 (edição)
3. Perguntas pré-preenchidas do template
4. Título sugere "[Template Name] - [Data]" (editável)
5. Toast: "Template carregado! Customize e publique."

---

### 9. Assistente de IA

**URL:** `/ai-assistant`

**Inspirado em:** ChatGPT UI, Notion AI, GitHub Copilot Chat

#### 9.1 Interface de Chat

**Layout:**
```
┌────────────────────────────────────────────────────┐
│  ✨ Assistente de IA                               │
│  ──────────────────────────────────────────────────│
│                                                    │
│  Histórico                         [Nova Conversa]│
│  ┌──────────────────┐                             │
│  │ 📝 NPS para SaaS │             [Chat Area]     │
│  │ há 2 horas       │                             │
│  ├──────────────────┤  ┌────────────────────────┐│
│  │ 🎯 Pesquisa UX   │  │ 🤖 Olá! Sou seu        ││
│  │ ontem            │  │ assistente IA.         ││
│  ├──────────────────┤  │                        ││
│  │ 📊 eNPS Q1       │  │ Como posso ajudar a    ││
│  │ há 3 dias        │  │ criar sua avaliação?   ││
│  └──────────────────┘  └────────────────────────┘│
│                                                    │
│                         ┌────────────────────────┐│
│                         │ 👤 Quero criar uma    ││
│                         │ pesquisa de satisfação││
│                         │ para clientes B2B     ││
│                         └────────────────────────┘│
│                                                    │
│                         ┌────────────────────────┐│
│                         │ 🤖 Ótimo! Vou criar   ││
│                         │ perguntas focadas em: ││
│                         │                        ││
│                         │ 1. NPS Score           ││
│                         │ 2. Satisfação com      ││
│                         │    suporte             ││
│                         │ 3. Qualidade do        ││
│                         │    produto             ││
│                         │                        ││
│                         │ Quantas perguntas      ││
│                         │ quer? (sugestão: 8-12)││
│                         │                        ││
│                         │ [Gerar 8] [Gerar 12]  ││
│                         │ [Personalizar]         ││
│                         └────────────────────────┘│
│                                                    │
│  ──────────────────────────────────────────────────│
│  [Digite sua mensagem...]              [Enviar →] │
│                                                    │
│  💡 Sugestões:                                     │
│  • "Crie uma avaliação de clima organizacional"   │
│  • "Adicione perguntas sobre trabalho remoto"     │
│  • "Gere um NPS para e-commerce"                  │
└────────────────────────────────────────────────────┘
```

**Funcionalidades:**
* **Histórico persistente:** Salva conversas anteriores
* **Edição de mensagens:** Click para editar e reenviar
* **Regenerar resposta:** Botão "🔄 Tentar novamente"
* **Copiar código:** Botão para copiar perguntas geradas
* **Usar resultado:** Botão "Criar Avaliação" insere no wizard
* **Markdown support:** Formatação rich text nas respostas
* **Code highlighting:** Se IA retornar JSON/code

**Estados:**
* **Typing indicator:** "IA está pensando..." com dots animados
* **Error state:** "Erro ao processar. Tente reformular."
* **Rate limit:** "Limite atingido. Aguarde 5 minutos."

#### 9.2 Configurações da IA

**URL:** `/ai-assistant/settings`

```
┌────────────────────────────────────────┐
│  ⚙️ Configurações da IA                │
│  ────────────────────────────────────  │
│                                        │
│  🎨 Tom das Perguntas                  │
│  ○ Formal                              │
│  ● Neutro (recomendado)                │
│  ○ Casual                              │
│                                        │
│  📊 Número Padrão de Perguntas         │
│  [8] (min: 3, max: 20)                │
│                                        │
│  🌍 Idioma de Geração                  │
│  [Português (BR) ▼]                    │
│                                        │
│  ✨ Criatividade                       │
│  Conservador ●─────○ Criativo          │
│  (Temperature: 0.5)                    │
│                                        │
│  🔒 Histórico                          │
│  ☑ Salvar conversas                    │
│  ☑ Sincronizar entre dispositivos      │
│                                        │
│  [ Limpar Histórico ]                  │
│  [ Restaurar Padrões ]                 │
│                                        │
│  [ Cancelar ]  [ Salvar Configurações ]│
└────────────────────────────────────────┘
```

---

### 10. Central de Notificações

**URL:** `/notifications`

**Inspirado em:** GitHub Notifications, Linear Inbox

#### 10.1 Página Principal

**Layout:**
```
┌──────────────────────────────────────────────────┐
│  🔔 Notificações                    [⚙️ Config]  │
│  ──────────────────────────────────────────────  │
│                                                  │
│  [ Todas ]  [ Não Lidas (5) ]  [ Arquivadas ]   │
│                                                  │
│  [ Marcar todas como lidas ]  [ Arquivar tudo ]  │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  🟢 Nova resposta recebida            há 5 min  │
│  "Avaliação NPS Q1 2026" recebeu nova resposta  │
│  Total: 48 respostas                             │
│  [ Ver Resultados ]  [ Arquivar ]                │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  📊 Meta atingida!                   há 1 hora  │
│  Parabéns! "Pesquisa de Onboarding" atingiu     │
│  50 respostas completas                          │
│  [ Ver Analytics ]  [ Compartilhar ]             │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  ⚠️ Prazo próximo                   há 2 horas  │
│  "Employee Satisfaction Q4" encerra em 3 dias    │
│  Apenas 23 de 50 respostas recebidas            │
│  [ Reenviar Convites ]  [ Estender Prazo ]       │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  ✅ Avaliação encerrada              ontem      │
│  "Customer Feedback Mar" foi encerrada           │
│  automaticamente. Relatório disponível.          │
│  [ Ver Relatório ]  [ Arquivar ]                 │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  🔧 Manutenção programada            há 2 dias  │
│  Sistema estará em manutenção dia 15/02 das     │
│  02:00 às 04:00. Prepare-se.                     │
│  [ Adicionar ao Calendário ]                     │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  [ Carregar mais... ]                            │
└──────────────────────────────────────────────────┘
```

**Tipos de Notificação:**

| Ícone | Tipo | Trigger | Ações Disponíveis |
|-------|------|---------|-------------------|
| 🟢 | Nova resposta | A cada 10 respostas | Ver Resultados, Arquivar |
| 📊 | Meta atingida | 25, 50, 100, 500 respostas | Ver Analytics, Compartilhar |
| ⚠️ | Prazo próximo | 7, 3, 1 dia antes | Reenviar, Estender, Snooze |
| ✅ | Avaliação encerrada | Automático ou manual | Ver Relatório, Arquivar |
| 🔧 | Sistema | Manutenção, updates | Add ao calendário |
| 🎉 | Conquista | Marcos especiais | Ver badge, Compartilhar |

#### 10.2 Configurações de Notificações

**URL:** `/notifications/settings`

```
┌────────────────────────────────────────────┐
│  ⚙️ Configurações de Notificações          │
│  ────────────────────────────────────────  │
│                                            │
│  📱 In-App (na plataforma)                 │
│  ☑ Nova resposta recebida                  │
│  ☑ Meta de respostas atingida              │
│  ☑ Prazo se aproximando                    │
│  ☑ Avaliação encerrada                     │
│  ☑ Atualizações do sistema                 │
│                                            │
│  📧 Email                                   │
│  ☑ Resumo diário (18:00)                   │
│  ☐ Cada nova resposta (não recomendado)   │
│  ☑ Alertas críticos apenas                 │
│                                            │
│  🔕 Não Perturbe                           │
│  ☑ Ativar em horários específicos          │
│  De [22:00] até [08:00]                    │
│  ☑ Fins de semana                          │
│                                            │
│  🔔 Preferências de Push (futuro)          │
│  ☐ Notificações push no navegador          │
│                                            │
│  [ Testar Notificação ]                    │
│                                            │
│  [ Cancelar ]  [ Salvar ]                  │
└────────────────────────────────────────────┘
```

---

### 11. Settings (Configurações)

**URL Base:** `/settings`

**Inspirado em:** Vercel Settings, Linear Settings, Notion Settings

#### 11.1 Navegação Lateral

```
┌─────────────────┐
│ ⚙️ Configurações│
│ ─────────────── │
│                 │
│ 👤 Perfil       │  ← Ativa
│ 🎨 Preferências │
│ 🔐 Conta        │
│ 👥 Equipe       │
│ 🔌 Integrações  │
│ 🛡️ Segurança    │
└─────────────────┘
```

#### 11.2 Settings > Perfil

**URL:** `/settings/profile`

```
┌──────────────────────────────────────────────┐
│  👤 Perfil                                   │
│  ──────────────────────────────────────────  │
│                                              │
│  Foto de Perfil                              │
│  ┌──────┐                                    │
│  │ [A] │  João Alves                         │
│  └──────┘  [ Alterar Foto ]  [ Remover ]     │
│                                              │
│  Informações Pessoais                        │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Nome Completo *                             │
│  [João Alves Silva                        ]  │
│                                              │
│  Email *                                     │
│  [joao.alves@empresa.com.br               ]  │
│  ✅ Verificado                              │
│                                              │
│  Cargo / Função                              │
│  [Product Manager                         ]  │
│                                              │
│  Empresa                                     │
│  [TechCorp Brasil                         ]  │
│                                              │
│  Bio (Opcional)                              │
│  [Textarea: Especialista em UX Research...] │
│  250/500 caracteres                          │
│                                              │
│  Links Sociais (Opcional)                    │
│  🐦 Twitter  [                            ]  │
│  💼 LinkedIn [                            ]  │
│  🌐 Website  [                            ]  │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Alterar Senha                               │
│  [ Clique para alterar sua senha ]           │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  [ Cancelar ]  [ Salvar Alterações ]         │
└──────────────────────────────────────────────┘
```

**Validações:**
* Nome: mínimo 3 caracteres
* Email: formato válido + verificação via link
* Foto: máx 2MB, formatos JPG/PNG/WebP

#### 11.3 Settings > Preferências

**URL:** `/settings/preferences`

```
┌──────────────────────────────────────────────┐
│  🎨 Preferências                             │
│  ──────────────────────────────────────────  │
│                                              │
│  Aparência                                   │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Tema                                        │
│  ┌────────┬────────┬────────┐              │
│  │ ☀️ Claro│ 🌙 Escuro│ 🔄 Auto│              │
│  │        │ [Ativo]│        │              │
│  └────────┴────────┴────────┘              │
│                                              │
│  Densidade da Interface                      │
│  ○ Compacta                                  │
│  ● Confortável (padrão)                      │
│  ○ Espaçosa                                  │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Idioma e Região                             │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Idioma da Interface                         │
│  [Português (Brasil) ▼]                      │
│                                              │
│  Fuso Horário                                │
│  [America/Sao_Paulo (GMT-3) ▼]               │
│                                              │
│  Formato de Data                             │
│  [DD/MM/YYYY ▼]                              │
│                                              │
│  Formato de Hora                             │
│  ○ 24 horas  ● 12 horas (AM/PM)              │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Dashboard                                   │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Vista Padrão de Avaliações                  │
│  ○ Cards (Grid)  ● Lista                     │
│                                              │
│  Items por Página                            │
│  [25 ▼]                                      │
│                                              │
│  Ordenação Padrão                            │
│  [Mais Recentes ▼]                           │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Acessibilidade                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ☑ Reduzir animações                         │
│  ☑ Alto contraste                            │
│  ☑ Aumentar tamanho de fonte (1.2x)          │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  [ Restaurar Padrões ]  [ Salvar ]           │
└──────────────────────────────────────────────┘
```

#### 11.4 Settings > Conta

**URL:** `/settings/account`

```
┌──────────────────────────────────────────────┐
│  🔐 Conta                                    │
│  ──────────────────────────────────────────  │
│                                              │
│  Plano Atual                                 │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │ 💼 PRO                             │     │
│  │ R$ 49/mês · Faturamento mensal     │     │
│  │                                    │     │
│  │ Próximo pagamento: 08 Mar 2026     │     │
│  │ Método: •••• 4532 (Visa)           │     │
│  │                                    │     │
│  │ [ Mudar Plano ] [ Alterar Pagamento]│    │
│  └────────────────────────────────────┘     │
│                                              │
│  Uso e Limites                               │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Avaliações criadas                          │
│  ████████████████░░░░  47 de ∞              │
│                                              │
│  Respostas coletadas (mês atual)             │
│  ██████████░░░░░░░░░░ 523 de 1000          │
│                                              │
│  Geração por IA (mês atual)                  │
│  ████████░░░░░░░░░░░░  18 de 50            │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Faturamento                                 │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Histórico de Pagamentos                     │
│  ┌────────────────────────────────────┐     │
│  │ 08 Fev 2026  R$ 49,00  ✅ Pago    │     │
│  │ 08 Jan 2026  R$ 49,00  ✅ Pago    │     │
│  │ 08 Dez 2025  R$ 49,00  ✅ Pago    │     │
│  └────────────────────────────────────┘     │
│  [ Ver todos ] [ Baixar Nota Fiscal ]        │
│                                              │
│  Método de Pagamento                         │
│  💳 •••• 4532 (Visa)  Exp: 03/27            │
│  [ Alterar Cartão ]  [ Adicionar Método ]    │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Zona de Perigo                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Exportar Todos os Dados                     │
│  [ Solicitar Exportação ]                    │
│  (Conforme LGPD/GDPR)                        │
│                                              │
│  Pausar Conta                                │
│  [ Pausar Temporariamente ]                  │
│  (Mantém dados, suspende cobranças)          │
│                                              │
│  Excluir Conta Permanentemente               │
│  [ Excluir Conta... ]                        │
│  ⚠️ Esta ação é irreversível!               │
└──────────────────────────────────────────────┘
```

**Modal de Exclusão de Conta:**
```
┌────────────────────────────────────────┐
│  ⚠️ Excluir Conta Permanentemente?     │
│  ────────────────────────────────────  │
│                                        │
│  Esta ação irá:                        │
│  • Deletar todas as suas avaliações    │
│  • Remover todas as respostas coletadas│
│  • Cancelar sua assinatura             │
│  • Apagar seus dados permanentemente   │
│                                        │
│  Esta ação NÃO pode ser desfeita!      │
│                                        │
│  Digite "EXCLUIR" para confirmar:      │
│  [                                  ]  │
│                                        │
│  [ Cancelar ]  [ Excluir Definitivamente]│
└────────────────────────────────────────┘
```

#### 11.5 Settings > Equipe

**URL:** `/settings/team`

```
┌──────────────────────────────────────────────┐
│  👥 Equipe                                   │
│  ──────────────────────────────────────────  │
│                                              │
│  [ + Convidar Membro ]          [🔍 Buscar]  │
│                                              │
│  Membros Ativos (3)                          │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ┌─────────────────────────────────────┐    │
│  │ [JA] João Alves (Você)              │    │
│  │ joao@empresa.com                    │    │
│  │ 👑 Admin · Membro desde Jan 2026   │    │
│  └─────────────────────────────────────┘    │
│                                              │
│  ┌─────────────────────────────────────┐    │
│  │ [MS] Maria Silva                    │    │
│  │ maria@empresa.com                   │    │
│  │ ✏️ Editor · Membro desde Jan 2026   │    │
│  │ [Alterar Permissão ▼] [Remover]     │    │
│  └─────────────────────────────────────┘    │
│                                              │
│  ┌─────────────────────────────────────┐    │
│  │ [PC] Pedro Costa                    │    │
│  │ pedro@empresa.com                   │    │
│  │ 👁️ Visualizador · Membro desde Fev  │    │
│  │ [Alterar Permissão ▼] [Remover]     │    │
│  └─────────────────────────────────────┘    │
│                                              │
│  Convites Pendentes (1)                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  📧 ana@empresa.com                          │
│  Enviado há 2 dias · Editor                  │
│  [ Reenviar Convite ]  [ Cancelar ]          │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Permissões                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  | Ação                  | Admin | Editor | Viewer |
│  |----------------------|-------|--------|--------|
│  | Criar avaliações     |   ✅  |   ✅   |   ❌   |
│  | Editar avaliações    |   ✅  |   ✅   |   ❌   |
│  | Excluir avaliações   |   ✅  |   ❌   |   ❌   |
│  | Ver respostas        |   ✅  |   ✅   |   ✅   |
│  | Exportar dados       |   ✅  |   ✅   |   ❌   |
│  | Gerenciar equipe     |   ✅  |   ❌   |   ❌   |
│  | Alterar faturamento  |   ✅  |   ❌   |   ❌   |
│                                              │
└──────────────────────────────────────────────┘
```

**Modal de Convite:**
```
┌────────────────────────────────────────┐
│  + Convidar Membro da Equipe           │
│  ────────────────────────────────────  │
│                                        │
│  Email *                               │
│  [ana@empresa.com                   ]  │
│                                        │
│  Permissão *                           │
│  [Editor ▼]                            │
│                                        │
│  Mensagem Personalizada (Opcional)     │
│  [Olá Ana, junte-se à nossa equipe!]  │
│                                        │
│  [ Cancelar ]  [ Enviar Convite ]      │
└────────────────────────────────────────┘
```

#### 11.6 Settings > Integrações

**URL:** `/settings/integrations`

```
┌──────────────────────────────────────────────┐
│  🔌 Integrações                              │
│  ──────────────────────────────────────────  │
│                                              │
│  Conecte Aevalo com suas ferramentas         │
│  favoritas para automatizar workflows.       │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Webhooks                                    │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Receba eventos em tempo real no seu servidor│
│                                              │
│  [ + Adicionar Webhook ]                     │
│                                              │
│  Webhooks ativos (1):                        │
│  ┌────────────────────────────────────┐     │
│  │ https://api.empresa.com/aevalo     │     │
│  │ Eventos: new_response, evaluation_closed│
│  │ Status: 🟢 Ativo · Última chamada: há 5min│
│  │ [ Testar ] [ Editar ] [ Deletar ]  │     │
│  └────────────────────────────────────┘     │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  API Keys                                    │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Use nossa API REST para integrar Aevalo    │
│  com sistemas customizados.                  │
│                                              │
│  [ + Gerar Nova API Key ]                    │
│                                              │
│  Keys ativas (2):                            │
│  ┌────────────────────────────────────┐     │
│  │ Produção                           │     │
│  │ aev_live_k8s9d7...  [Copiar]       │     │
│  │ Criada: 15 Jan 2026 · Último uso: hoje│  │
│  │ Permissões: read, write            │     │
│  │ [ Ver Logs ] [ Revogar ]           │     │
│  └────────────────────────────────────┘     │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │ Desenvolvimento                    │     │
│  │ aev_test_m3n2k1...  [Copiar]       │     │
│  │ Criada: 10 Jan 2026 · Último uso: ontem│ │
│  │ Permissões: read                   │     │
│  │ [ Ver Logs ] [ Revogar ]           │     │
│  └────────────────────────────────────┘     │
│                                              │
│  [ 📖 Ver Documentação da API ]              │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Integrações de Terceiros (Em Breve)        │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ┌────┬────┬────┬────┬────┐                │
│  │Slack│Teams│Zapier│Make│Notion│            │
│  │ Em │ Em │ Em │ Em │ Em │                │
│  │Breve│Breve│Breve│Breve│Breve│            │
│  └────┴────┴────┴────┴────┘                │
└──────────────────────────────────────────────┘
```

#### 11.7 Settings > Segurança

**URL:** `/settings/security`

```
┌──────────────────────────────────────────────┐
│  🛡️ Segurança                                │
│  ──────────────────────────────────────────  │
│                                              │
│  Autenticação de Dois Fatores (2FA)          │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Status: ❌ Desativado                       │
│                                              │
│  Adicione uma camada extra de segurança      │
│  usando um aplicativo autenticador.          │
│                                              │
│  [ Ativar 2FA ]                              │
│                                              │
│  Apps recomendados:                          │
│  • Google Authenticator                      │
│  • Authy                                     │
│  • 1Password                                 │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Sessões Ativas                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │ 💻 MacBook Pro                     │     │
│  │ São Paulo, BR                      │     │
│  │ Chrome 120 · Esta sessão          │     │
│  │ Ativa agora                        │     │
│  └────────────────────────────────────┘     │
│                                              │
│  ┌────────────────────────────────────┐     │
│  │ 📱 iPhone 15                       │     │
│  │ São Paulo, BR                      │     │
│  │ Safari · iOS                       │     │
│  │ Última atividade: há 2 horas       │     │
│  │ [ Encerrar Sessão ]                │     │
│  └────────────────────────────────────┘     │
│                                              │
│  [ Encerrar Todas as Outras Sessões ]        │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Log de Atividades                           │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Últimas 10 atividades:                      │
│                                              │
│  🟢 Login bem-sucedido                       │
│  11/02/2026 09:15 · São Paulo, BR            │
│  Chrome 120 · IP: 192.168.1.10               │
│                                              │
│  🟢 Configurações alteradas                  │
│  10/02/2026 18:32 · São Paulo, BR            │
│  Chrome 120 · IP: 192.168.1.10               │
│                                              │
│  🟢 Avaliação criada                         │
│  10/02/2026 14:20 · São Paulo, BR            │
│  Chrome 120 · IP: 192.168.1.10               │
│                                              │
│  🔴 Tentativa de login falhou                │
│  09/02/2026 22:15 · Rio de Janeiro, BR       │
│  Firefox 115 · IP: 201.10.5.88               │
│  ⚠️ Se não foi você, recomendamos trocar a senha│
│                                              │
│  [ Ver Log Completo ]                        │
│                                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  Backups de Recuperação                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                              │
│  ☑ Backup automático diário                  │
│  Último backup: 11/02/2026 03:00             │
│  Retenção: 30 dias                           │
│                                              │
│  [ Baixar Backup Mais Recente ]              │
└──────────────────────────────────────────────┘
```

---

### 12. Help Center (Central de Ajuda)

**URL:** `/help`

**Inspirado em:** Stripe Docs, Vercel Docs, Linear Help

#### 12.1 Página Principal

```
┌──────────────────────────────────────────────────┐
│  ❓ Como podemos ajudar?                         │
│  ──────────────────────────────────────────────  │
│                                                  │
│  [🔍 Buscar na documentação...                ]  │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  Tópicos Populares                               │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  ┌──────────────┬──────────────┬──────────────┐│
│  │ 🚀 Começando │ 🎨 Criando   │ 📊 Analytics ││
│  │              │              │              ││
│  │ • Primeiro   │ • Escolher   │ • Interpretar││
│  │   login      │   escalas    │   resultados ││
│  │ • Dashboard  │ • Usar IA    │ • Exportar   ││
│  │ • Criar      │ • Templates  │   dados      ││
│  │   avaliação  │ • Publicar   │ • Filtros    ││
│  └──────────────┴──────────────┴──────────────┘│
│                                                  │
│  ┌──────────────┬──────────────┬──────────────┐│
│  │ 🔗 Compartilhar│ ⚙️ Configurar│ 🛡️ Segurança││
│  │              │              │              ││
│  │ • Gerar link │ • Perfil     │ • 2FA        ││
│  │ • QR Code    │ • Preferências│• Permissões ││
│  │ • Embed      │ • Integrações│ • LGPD       ││
│  └──────────────┴──────────────┴──────────────┘│
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  📚 Documentação Completa                        │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  📖 Guias                                        │
│  • Guia Completo de Escalas Científicas         │
│  • Best Practices para Pesquisas                 │
│  • Como Aumentar Taxa de Resposta               │
│  • Análise Estatística de Resultados            │
│                                                  │
│  🎥 Tutoriais em Vídeo                          │
│  • Criar sua primeira avaliação (3:45)           │
│  • Usar geração por IA (2:15)                    │
│  • Interpretar resultados (5:30)                 │
│  • Configurar integrações (4:00)                 │
│                                                  │
│  ❓ FAQ (Perguntas Frequentes)                  │
│  • Como funciona a geração por IA?               │
│  • Quantas respostas posso coletar?              │
│  • É compatível com LGPD/GDPR?                   │
│  • Posso exportar os dados?                      │
│  • Como cancelar minha assinatura?               │
│  [ Ver todas as 45 perguntas ]                   │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  💬 Ainda precisa de ajuda?                      │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  [ 💬 Chat ao Vivo ]  [ 📧 Email Suporte ]       │
│  [ 🎫 Abrir Ticket ]  [ 💼 Falar com Vendas ]    │
│                                                  │
│  Tempo médio de resposta: 2 horas                │
└──────────────────────────────────────────────────┘
```

#### 12.2 Artigo de Documentação (Exemplo)

**URL:** `/help/docs/escalas-cientificas`

```
┌──────────────────────────────────────────────────┐
│  📖 Documentação > Escalas Científicas           │
│  ──────────────────────────────────────────────  │
│                                                  │
│  [🔍 Buscar neste artigo...]                     │
│                                                  │
│  ┌─────────────────┐  ┌──────────────────────┐ │
│  │ Nesta Página    │  │ [Conteúdo do artigo] │ │
│  │ ─────────────── │  │                      │ │
│  │                 │  │ # Escalas Científicas│ │
│  │ • O que são     │  │                      │ │
│  │ • Likert Scale  │  │ Aevalo suporta 7     │ │
│  │ • Fixed Sum     │  │ tipos de escalas...  │ │
│  │ • Paired Comp.  │  │                      │ │
│  │ • Frequency     │  │ ## 1. Likert Scale   │ │
│  │ • Rating        │  │ A escala Likert...   │ │
│  │ • Semantic Diff.│  │                      │ │
│  │ • Binary        │  │ [Código exemplo]     │ │
│  │ • Quando usar   │  │ [Screenshot]         │ │
│  │ • Best Practices│  │                      │ │
│  └─────────────────┘  │ ## 2. Fixed Sum      │ │
│                       │ Permite distribuir...│ │
│                       │                      │ │
│                       │ [... continua]       │ │
│                       └──────────────────────┘ │
│                                                  │
│  ──────────────────────────────────────────────  │
│                                                  │
│  Este artigo foi útil?                           │
│  [ 👍 Sim ]  [ 👎 Não ]                         │
│                                                  │
│  Última atualização: 05 Fev 2026                 │
│  Tempo de leitura: 8 minutos                     │
└──────────────────────────────────────────────────┘
```

---

### 13. Área Administrativa

**URL:** `/admin` (Acesso restrito)

**Inspirado em:** Vercel Admin, Stripe Dashboard, Railway Admin

#### 13.1 Admin Dashboard

```
┌──────────────────────────────────────────────────┐
│  👑 Admin Dashboard                              │
│  ──────────────────────────────────────────────  │
│                                                  │
│  Sistema Health                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  ┌──────────┬──────────┬──────────┬──────────┐ │
│  │ Uptime   │ Requests │ Errors   │ Latência │ │
│  │ 99.98%   │ 1.2M/dia │ 0.02%    │ 89ms     │ │
│  │ 🟢       │ ↑ 12%    │ ↓ 0.5%   │ ↓ 5ms    │ │
│  └──────────┴──────────┴──────────┴──────────┘ │
│                                                  │
│  Métricas de Negócio (Últimos 30 dias)           │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  ┌──────────┬──────────┬──────────┬──────────┐ │
│  │ Usuários │ Avaliações│ Respostas│ MRR      │ │
│  │ 1,247    │ 3,891    │ 47,832   │ R$ 12.3k │ │
│  │ ↑ 15%    │ ↑ 23%    │ ↑ 31%    │ ↑ 18%    │ │
│  └──────────┴──────────┴──────────┴──────────┘ │
│                                                  │
│  [Gráfico de crescimento ao longo do tempo]      │
│                                                  │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  Últimas Atividades                              │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                  │
│  🟢 Novo usuário registrado                      │
│  ana@techcorp.com · PRO Plan · há 5 min          │
│                                                  │
│  🔴 Erro crítico detectado                       │
│  Gemini API timeout · Server 3 · há 12 min       │
│  [ Ver Detalhes ]                                │
│                                                  │
│  🟢 Pagamento recebido                           │
│  R$ 149,00 · TEAM Plan · TechCorp · há 1h        │
│                                                  │
│  [ Ver Log Completo ]                            │
└──────────────────────────────────────────────────┘
```

#### 13.2 Admin > Usuários

```
┌──────────────────────────────────────────────────┐
│  👥 Gestão de Usuários                           │
│  ──────────────────────────────────────────────  │
│                                                  │
│  [🔍 Buscar usuários...]         [Filtros ▼]     │
│                                                  │
│  1,247 usuários · Exibindo 25 por página         │
│                                                  │
│  ┌────────────────────────────────────────────┐ │
│  │ Nome           │ Email          │ Plano  │Status││
│  ├────────────────────────────────────────────┤ │
│  │ João Silva     │ joao@...       │ PRO    │🟢   ││
│  │ Membro desde: 15 Jan 2026 · ID: usr_123   │ │
│  │ Avaliações: 23 · Respostas: 1.2k          │ │
│  │ [ Ver Perfil ] [ Editar ] [ Suspender ]   │ │
│  ├────────────────────────────────────────────┤ │
│  │ Maria Santos   │ maria@...      │ TEAM   │🟢   ││
│  │ Membro desde: 03 Fev 2026 · ID: usr_456   │ │
│  │ Avaliações: 8 · Respostas: 320            │ │
│  │ [ Ver Perfil ] [ Editar ] [ Suspender ]   │ │
│  └────────────────────────────────────────────┘ │
│                                                  │
│  [ ← Anterior ]  [ 1 2 3 ... 50 ]  [ Próxima →] │
│                                                  │
│  Ações em Massa:                                 │
│  [ Exportar CSV ] [ Enviar Email ] [ Moderar ]   │
└──────────────────────────────────────────────────┘
```

---

### 14. Páginas de Erro e Sistema

#### 14.1 Erro 404 (Página Não Encontrada)

**URL:** Qualquer rota inválida

```
┌────────────────────────────────────────────┐
│                                            │
│         [Ilustração minimalista]           │
│              🔍 ❓                         │
│                                            │
│            404                             │
│       Página não encontrada                │
│                                            │
│   A página que você procura não existe     │
│   ou foi movida para outro endereço.       │
│                                            │
│   Possíveis motivos:                       │
│   • Link quebrado ou desatualizado         │
│   • URL digitada incorretamente            │
│   • Recurso foi removido                   │
│                                            │
│   Sugestões:                               │
│   • [ ← Voltar ] para a página anterior    │
│   • [ 🏠 Ir para Dashboard ]               │
│   • [ 🔍 Buscar ] o que você precisa       │
│   • [ 💬 Reportar ] este problema          │
│                                            │
│   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   │
│                                            │
│   Páginas populares:                       │
│   • Dashboard                              │
│   • Criar Nova Avaliação                   │
│   • Templates                              │
│   • Analytics                              │
└────────────────────────────────────────────┘
```

**Comportamento:**
* Preserva header/sidebar se usuário autenticado
* Remove header se público (landing simples)
* Log do erro para análise (URL tentada, referrer)
* Sugestões contextuais baseadas em histórico

#### 14.2 Erro 500 (Erro Interno do Servidor)

**URL:** Trigger quando backend retorna 500

```
┌────────────────────────────────────────────┐
│                                            │
│         [Ilustração minimalista]           │
│              ⚙️ 💥                         │
│                                            │
│            500                             │
│       Algo deu errado                      │
│                                            │
│   Nossos servidores encontraram um erro    │
│   inesperado. Já fomos notificados e       │
│   estamos trabalhando na solução.          │
│                                            │
│   ID do Erro: #ERR-2026-02-11-1234         │
│   (Guarde este código para referência)     │
│                                            │
│   O que você pode fazer:                   │
│   • [ 🔄 Tentar Novamente ]                │
│   • [ ← Voltar ] para página segura        │
│   • [ 📊 Status ] do sistema               │
│   • [ 💬 Reportar ] com detalhes           │
│                                            │
│   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   │
│                                            │
│   Status do Sistema: 🟢 Operacional        │
│   Servidores: 3/3 online                   │
│   Última verificação: há 30 segundos       │
│                                            │
│   [ Ver Status Detalhado → ]               │
└────────────────────────────────────────────┘
```

**Comportamento:**
* Envia erro para Sentry/logging automaticamente
* Gera ID único para tracking
* Mostra status page se disponível
* Auto-retry depois de 5s (opcional, configurável)

#### 14.3 Erro 503 (Serviço Indisponível / Manutenção)

**URL:** Trigger durante manutenção programada

```
┌────────────────────────────────────────────┐
│                                            │
│         [Ilustração minimalista]           │
│              🔧 ⏰                         │
│                                            │
│      Manutenção em Andamento               │
│                                            │
│   Estamos realizando melhorias no sistema  │
│   para oferecer uma experiência ainda      │
│   melhor.                                  │
│                                            │
│   📅 Início: 11 Fev 2026, 02:00 BRT        │
│   ⏱️ Previsão de retorno: 04:00 BRT        │
│   ⏳ Tempo restante: ~45 minutos           │
│                                            │
│   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   │
│                                            │
│   Durante a manutenção:                    │
│   ✓ Upgrade de infraestrutura              │
│   ✓ Melhorias de performance               │
│   ✓ Novos recursos sendo implantados       │
│                                            │
│   ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━   │
│                                            │
│   Fique por dentro:                        │
│   [ 🐦 Twitter ]  [ 💬 Status Page ]       │
│                                            │
│   [ 🔄 Recarregar Página ]                 │
│                                            │
│   Obrigado pela compreensão!               │
└────────────────────────────────────────────┘
```

**Comportamento:**
* Auto-refresh a cada 60 segundos
* Link para status page externa (status.aevalo.app)
* Countdown timer atualizado
* Notificações via email/push antes da manutenção

#### 14.4 Unauthorized (403 - Sem Permissão)

```
┌────────────────────────────────────────────┐
│                                            │
│         [Ilustração minimalista]           │
│              🔒 🚫                         │
│                                            │
│            403                             │
│       Acesso Negado                        │
│                                            │
│   Você não tem permissão para acessar      │
│   este recurso.                            │
│                                            │
│   Possíveis motivos:                       │
│   • Recurso pertence a outro usuário       │
│   • Sua função não permite esta ação       │
│   • Avaliação foi arquivada ou deletada    │
│                                            │
│   O que fazer:                             │
│   • [ ← Voltar ] para área segura          │
│   • [ 🏠 Dashboard ] inicial               │
│   • [ 💬 Falar ] com administrador         │
│   • [ 📖 Ver ] permissões da sua conta     │
│                                            │
│   Se acredita que isso é um erro,          │
│   [ 📧 Entre em contato ] conosco.         │
└────────────────────────────────────────────┘
```

#### 14.5 Status Page (Sistema)

**URL:** `status.aevalo.app` (Subdomain externo)

```
┌────────────────────────────────────────────────┐
│  Aevalo System Status                          │
│  ────────────────────────────────────────────  │
│                                                │
│  🟢 Todos os sistemas operacionais             │
│  Última verificação: há 30 segundos            │
│                                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                │
│  Componentes                                   │
│                                                │
│  🟢 API Principal            99.98% uptime     │
│  🟢 Dashboard Frontend       100% uptime       │
│  🟢 Database (Supabase)      99.99% uptime     │
│  🟢 Gemini AI Integration    98.5% uptime      │
│  🟢 Email Service            99.95% uptime     │
│  🟢 Analytics Engine         100% uptime       │
│                                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                │
│  Performance Metrics (24h)                     │
│                                                │
│  ⚡ Response Time (avg):     89ms              │
│  📊 Requests Processed:     1.2M               │
│  ❌ Error Rate:              0.02%             │
│                                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                │
│  Histórico de Incidentes                       │
│                                                │
│  08 Fev 2026 · 🟢 Resolvido                   │
│  Lentidão na geração por IA                    │
│  Duração: 25 minutos · Impacto: Menor          │
│  [ Ver Detalhes ]                              │
│                                                │
│  01 Fev 2026 · 🟢 Resolvido                   │
│  Manutenção programada                         │
│  Duração: 2 horas · Impacto: Total             │
│  [ Ver Detalhes ]                              │
│                                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                │
│  Manutenções Programadas                       │
│                                                │
│  15 Fev 2026, 02:00-04:00 BRT                  │
│  Upgrade de infraestrutura                     │
│  [ Adicionar ao Calendário ]                   │
│                                                │
│  ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━  │
│                                                │
│  [ 🔔 Assinar Updates ]  [ 📊 Ver Uptime ]     │
└────────────────────────────────────────────────┘
```

---

---

## 🛠️ Matriz Completa de Tratamento de Erros

### Camada de Autenticação

| Cenário | Código HTTP | Mensagem ao Usuário | Ação do Sistema | Recuperação |
|---------|-------------|---------------------|-----------------|-------------|
| **Credenciais inválidas** | `401` | "Email ou senha incorretos." | Limpa senha, mantém email | Foca no campo senha |
| **Conta bloqueada** | `403` | "Sua conta foi temporariamente bloqueada por segurança." | Log de tentativa | Link para recuperação |
| **Rate limit (muitas tentativas)** | `429` | "Muitas tentativas. Aguarde 60 segundos." | Desabilita botão por 60s | Countdown timer visível |
| **Token expirado** | `401` | "Sua sessão expirou. Faça login novamente." | Limpa sessionStorage | Redirect para /login |
| **Sessão duplicada** | `409` | "Login detectado em outro dispositivo." | Opção de invalidar outras sessões | Modal de confirmação |
| **Email não verificado** | `403` | "Verifique seu email antes de continuar." | Bloqueia acesso | Botão "Reenviar email" |
| **Servidor indisponível** | `503` | "Sistema em manutenção. Retorne em instantes." | Exibe ETA se disponível | Status page link |
| **Rede offline** | `Network Error` | "Sem conexão. Verifique sua internet." | Tenta reconectar a cada 5s | Indicador de reconexão |

---

### Camada de API (Operações CRUD)

| Operação | Erro | Código | Mensagem UI | Ação do Sistema | Retry? |
|----------|------|--------|-------------|-----------------|--------|
| **GET /evaluations** | Timeout | `-` | "Demorando para carregar..." | Loading spinner persiste | Auto após 10s |
| **GET /evaluations** | Server error | `500` | "Erro ao carregar suas avaliações." | Exibe botão "Tentar Novamente" | Manual |
| **GET /evaluations** | Empty result | `200` | [Empty state ilustrado] | Mostra CTA "Criar primeira" | N/A |
| **POST /evaluations** | Validation error | `400` | "Campos obrigatórios faltando." | Destaca campos em vermelho | Corrigir + reenviar |
| **POST /evaluations** | Duplicate name | `409` | "Já existe avaliação com este nome." | Sugere adicionar número/data | Manual |
| **POST /evaluations** | Server error | `500` | "Erro ao criar. Tente novamente." | Mantém dados no form | Manual |
| **POST /evaluations** | Network error | `-` | "Conexão perdida. Dados salvos localmente." | Salva em localStorage | Auto quando online |
| **PATCH /evaluations/{id}** | Not found | `404` | "Avaliação não encontrada." | Redireciona para dashboard | N/A |
| **PATCH /evaluations/{id}** | Unauthorized | `403` | "Você não tem permissão para editar." | Bloqueia edição | N/A |
| **DELETE /evaluations/{id}** | Confirmation | `-` | "Tem certeza? Esta ação é irreversível." | Modal de confirmação | N/A |
| **DELETE /evaluations/{id}** | Has responses | `409` | "Não é possível deletar avaliação com respostas." | Sugere arquivar ao invés | N/A |

---

### Camada de IA (Gemini API)

| Cenário | Tempo | Feedback Visual | Mensagem | Opções de Recuperação |
|---------|-------|-----------------|----------|------------------------|
| **Processing** | 0-5s | Spinner + "Analisando..." | "A IA está pensando..." | Aguardar |
| **Generating** | 5-15s | Progress bar + "Gerando..." | "Criando perguntas..." | Aguardar |
| **Delayed** | 15-30s | Warning icon | "Está demorando mais que esperado. Aguarde mais um pouco?" | Continuar ou cancelar |
| **Timeout** | 30s+ | Error icon | "A IA não respondeu a tempo." | • Tentar novamente<br>• Usar template<br>• Criar manualmente |
| **Rate limit** | - | Lock icon | "Limite de IA atingido. Tente em 5 minutos." | • Countdown timer<br>• Usar template enquanto espera |
| **Invalid input** | Imediato | Warning | "Descrição muito curta ou vaga." | • Tooltip com exemplos<br>• Mínimo 20 caracteres |
| **Content filter** | Imediato | Block icon | "Conteúdo inapropriado detectado." | • Sugestão de reformulação<br>• Guidelines visíveis |
| **API error 500** | - | Error icon | "Serviço de IA temporariamente indisponível." | • Status page link<br>• Usar template<br>• Criar manualmente |
| **API error 503** | - | Maintenance icon | "IA em manutenção. Retorne em breve." | • ETA se disponível<br>• Usar template |

---

### Camada de Submissão de Respostas (Público)

| Cenário | Código | Mensagem ao Avaliador | Comportamento | Dados Salvos? |
|---------|--------|----------------------|---------------|---------------|
| **Validação local falhou** | - | "3 perguntas precisam de atenção" | Scroll até primeira com erro | Sim (localStorage) |
| **Fixed Sum != 100** | - | "A soma deve ser exatamente 100 pontos" | Desabilita botão enviar | Sim |
| **Obrigatória não respondida** | - | "Esta pergunta é obrigatória" | Border vermelha + ícone | Sim |
| **Timeout na submissão** | - | "Conexão perdida. Respostas salvas localmente." | Botão "Tentar enviar" aparece | Sim (localStorage) |
| **Server error 500** | `500` | "Erro ao salvar. Tentando novamente..." | Auto-retry em 5s (máx 3x) | Sim |
| **Avaliação fechada** | `403` | "Esta avaliação foi encerrada." | Desabilita form | Não |
| **Link expirado** | `404` | "Link inválido ou expirado." | Página 404 amigável | Não |
| **Resposta duplicada** | `409` | "Você já respondeu esta avaliação." | Agradecimento + data anterior | Não |
| **Limite atingido** | `403` | "Limite de respostas atingido." | Mensagem de agradecimento | Não |
| **Network offline** | - | "Sem internet. Respostas salvas." | Ícone offline + tentativa auto | Sim (localStorage) |

---

### Camada de Realtime (Supabase)

| Evento | Causa | Feedback | Ação do Sistema |
|--------|-------|----------|-----------------|
| **Conexão perdida** | Network drop | Toast discreto: "Atualizações pausadas" | Tenta reconectar a cada 5s |
| **Reconectado** | Network restored | Toast verde: "Reconectado" | Sincroniza dados perdidos |
| **Erro de permissão** | RLS policy | Toast vermelho: "Erro de sincronização" | Log error + fallback polling |
| **Latência alta** | Slow connection | Badge "Atualizações atrasadas" | Continua funcionando |

---

### Camada de Dados (GraphQL)

| Erro GraphQL | Cenário | Mensagem | Ação |
|--------------|---------|----------|------|
| **UNAUTHENTICATED** | Token inválido | "Sessão expirada. Faça login." | Redirect para login |
| **FORBIDDEN** | Sem permissão | "Acesso negado." | Volta para dashboard |
| **NOT_FOUND** | Recurso inexistente | "Não encontrado." | Sugere buscar novamente |
| **BAD_USER_INPUT** | Validação falhou | "Dados inválidos: [detalhes]" | Destaca campos problemáticos |
| **INTERNAL_SERVER_ERROR** | Erro de servidor | "Erro interno. Já estamos investigando." | Botão retry + ID do erro |

---

## 🎯 Estados Especiais de UI

### Empty States (Estados Vazios)

| Tela | Condição | Visual | CTA Principal |
|------|----------|--------|---------------|
| **Dashboard** | `evaluations.length == 0` | Ilustração + "Crie sua primeira avaliação" | [+ Nova Avaliação] (laranja) |
| **Busca** | `searchResults.length == 0` | Lupa + "Nenhum resultado para '[termo]'" | [Limpar Busca] |
| **Analytics** | `responses.length == 0` | Gráfico vazio + "Aguardando respostas" | [Compartilhar Link] |
| **Notifications** | `notifications.length == 0` | Sino + "Nenhuma notificação" | N/A |
| **Templates** | `templates.length == 0` (erro) | "Erro ao carregar templates" | [Recarregar] |

### Loading States (Estados de Carregamento)

| Componente | Tipo | Duração Esperada | Visual |
|------------|------|------------------|--------|
| **Lista de avaliações** | Skeleton cards | 0.5-2s | 3-6 cards pulsantes |
| **Gráfico analytics** | Skeleton chart | 1-3s | Barras em shimmer effect |
| **Geração IA** | Progress bar | 5-15s | Barra animada + texto dinâmico |
| **Login** | Button spinner | 1-2s | Spinner no botão + texto "Entrando..." |
| **Upload arquivo** | Progress ring | Variável | Círculo com porcentagem |
| **Submissão resposta** | Overlay | 1-3s | Fullscreen semi-transparente + spinner |

### Success States (Estados de Sucesso)

| Ação | Feedback | Duração | Auto-dismiss? |
|------|----------|---------|---------------|
| **Login bem-sucedido** | ✅ + "Bem-vindo!" | 500ms | Sim → redirect |
| **Avaliação criada** | Confetti + modal de compartilhamento | Permanente | Não (manual) |
| **Resposta enviada** | ✅ Checkmark animado + "Obrigado!" | 2s | Sim → mensagem final |
| **Link copiado** | Toast verde: "Link copiado!" | 2s | Sim |
| **Dados exportados** | Download automático + toast | 2s | Sim |
| **Settings saved** | Toast discreto: "Salvo" | 1s | Sim |

---

## 🚀 Práticas de UX Inspiradas em SaaS Líderes

### Linear (Gestão de Projetos)

**O que adotamos:**
* **Atalhos de teclado:** `Cmd+K` para busca global, `C` para criar nova avaliação
* **Navegação fluida:** Transições suaves, sem page reloads
* **Design minimalista:** Foco no conteúdo, não na interface
* **Feedback instantâneo:** Toda ação tem resposta visual imediata

**Implementação:**
* Hotkeys documentados em tooltip (hover sobre botões)
* SPA com Vue Router (zero reloads)
* Paleta reduzida, muito white space
* Toasts posicionados no canto superior direito

---

### Vercel (Deploy Platform)

**O que adotamos:**
* **Status em tempo real:** Deploy status → Resposta status em nosso caso
* **Log streaming:** Build logs → Geração IA logs
* **URLs curtas e memoráveis:** `vercel.app/proj` → `aevalo.app/e/abc`
* **Dark mode toggle:** Preferência do usuário respeitada

**Implementação:**
* WebSocket para updates ao vivo de respostas
* Console.log visível durante geração IA (modo debug)
* Short URLs com nanoid (6 caracteres)
* Theme switcher em settings, persiste em localStorage

---

### Notion (Workspace)

**O que adotamos:**
* **Emojis como ícones:** Categorias identificadas por emojis
* **Drag and drop intuitivo:** Reordenar perguntas arrastando
* **Inline editing:** Click para editar título/descrição
* **Hierarquia visual clara:** Indentação e cards aninhados

**Implementação:**
* Emoji picker em seleção de categoria
* Vue Draggable para lista de perguntas
* Contenteditable em títulos com auto-save
* Tailwind spacing consistente (4/8/16/24px)

---

### Typeform (Forms)

**O que adotamos:**
* **Uma pergunta por vez:** Modo focus para avaliadores
* **Progress bar sempre visível:** "Pergunta 3 de 10"
* **Validação não intrusiva:** Mensagens inline, não pop-ups
* **Animações suaves:** Fade in/out entre perguntas

**Implementação:**
* Toggle "Modo Focus" vs "Ver todas"
* Barra de progresso sticky no topo
* Validação em tempo real com mensagens abaixo do input
* Transition CSS: `fade` com 300ms

---

### Stripe (Payments)

**O que adotamos:**
* **Dashboard limpo:** Métricas grandes, gráficos simples
* **Documentação inline:** Tooltips com "?" explicam tudo
* **Erros informativos:** Mensagens técnicas + sugestão de solução
* **API logs visíveis:** Histórico de chamadas para debug

**Implementação:**
* Cards com números grandes (48px) para KPIs
* Ícone "?" ao lado de termos técnicos com popover
* Erros com formato: "O que aconteceu" + "Como resolver"
* Tab "Logs" no dashboard para admin

---

### Figma (Design Tool)

**O que adotamos:**
* **Colaboração ao vivo:** Ver quem está respondendo agora
* **Versionamento:** Histórico de edições da avaliação
* **Comentários:** Anotar perguntas específicas (futuro)
* **Multiplayer cursors:** Ver avatar de outros editores

**Implementação:**
* Supabase Realtime para presença
* Tabela `evaluation_versions` com diffs
* Sistema de comentários com threading
* WebRTC para cursores ao vivo (fase 2)

---

## 📱 Responsividade e Acessibilidade

### Breakpoints (Tailwind)

| Dispositivo | Breakpoint | Layout |
|-------------|-----------|--------|
| **Mobile** | `< 640px` | Stack vertical, menu hamburguer |
| **Tablet** | `640px - 1024px` | Grid 2 colunas, sidebar colapsável |
| **Desktop** | `> 1024px` | Sidebar fixa, grid 3-4 colunas |
| **Large** | `> 1536px` | Max-width container, centered |

### Touch Targets (Mobile)

* **Mínimo:** 44x44px para todos os botões/links
* **Espaçamento:** 8px entre elementos clicáveis
* **Gestos:** Swipe left/right entre perguntas

### Acessibilidade (WCAG 2.1 AA)

**Checklist:**
* ✅ Contraste mínimo 4.5:1 para texto
* ✅ Todos os inputs com labels associados
* ✅ Focus visible em navegação por teclado
* ✅ Alt text em todas as imagens
* ✅ Roles ARIA em componentes customizados
* ✅ Testes com screen readers (NVDA, VoiceOver)
* ✅ Captions em vídeos (se houver)
* ✅ Formulários validados com mensagens descritivas

**Navegação por Teclado:**
* `Tab` / `Shift+Tab`: Navegar entre campos
* `Enter`: Submeter / Selecionar
* `Esc`: Fechar modals
* `Arrow keys`: Navegar em listas/escalas
* `Space`: Toggle checkboxes
* `Cmd+K` / `Ctrl+K`: Busca global
* `C`: Criar nova avaliação (quando no dashboard)
* `/`: Focar na busca

---

## 🔔 Sistema de Notificações

### Tipos de Toast

| Tipo | Cor | Ícone | Auto-dismiss | Posição | Duração |
|------|-----|-------|--------------|---------|---------|
| **Success** | Verde `#10B981` | ✅ | Sim | Top-right | 2s |
| **Error** | Vermelho `#EF4444` | ❌ | Persistente | Top-right | Manual |
| **Warning** | Laranja `#F59E0B` | ⚠️ | Sim | Top-right | 4s |
| **Info** | Azul `#3B82F6` | ℹ️ | Sim | Top-right | 3s |
| **Loading** | Roxo `#9333EA` | ⏳ | Manual | Top-right | Manual |

**Stack de Toasts:**
* Máximo 3 toasts simultâneos
* Novos toasts empurram os antigos para baixo
* Click no toast o fecha imediatamente
* Swipe right (mobile) para dispensar

### In-app Notifications (Bell Icon)

**Header com Badge:**
```
┌─────────────────────────────┐
│  🔔 Notificações (3)        │
│  ──────────────────────────  │
│                             │
│  🟢 Nova resposta recebida  │
│     "Avaliação NPS Q1"      │
│     há 5 minutos            │
│                             │
│  📊 Meta atingida!          │
│     "50 respostas coletadas"│
│     há 1 hora               │
│                             │
│  ⚠️ Prazo próximo           │
│     "Encerra em 3 dias"     │
│     ontem                   │
│                             │
│  [ Marcar todas como lidas ]│
│  [ Configurações ]          │
└─────────────────────────────┘
```

**Tipos de Notificações:**
1. **Nova resposta** - A cada 10 respostas ou resposta importante
2. **Meta atingida** - 25, 50, 100, 500 respostas
3. **Prazo próximo** - 7, 3, 1 dia antes do deadline
4. **Avaliação encerrada automaticamente** - Por prazo ou limite
5. **Sistema** - Manutenção programada, novos recursos

**Settings de Notificações:**
* Toggle por tipo de notificação
* Opção de email para eventos críticos
* Digest diário (resumo de atividades)
* Não perturbe (horário configurável)

---

## 💾 Persistência e Auto-Save

### Estratégias por Contexto

| Tela | Estratégia | Trigger | Storage | TTL (Time to Live) |
|------|-----------|---------|---------|-------------------|
| **Editor de avaliação** | Auto-save | Debounce 3s após edição | Supabase (draft) | Indefinido |
| **Resposta (público)** | Auto-save local | Cada resposta + 2s debounce | localStorage | 7 dias |
| **Settings** | Save manual | Click "Salvar" | Supabase (user_prefs) | Indefinido |
| **Busca/Filtros** | Query params | Tempo real | URL params | Sessão |
| **Theme preference** | Persist local | Change | localStorage | Indefinido |
| **Draft IA prompt** | Auto-save | Debounce 1s | localStorage | 24 horas |

### Recuperação de Sessão

**Cenário 1: Editor de Avaliação Interrompido**

```
1. Usuário fecha aba durante criação
2. Retorna ao site
3. Sistema detecta draft em Supabase ou localStorage
4. Modal aparece:

   ┌─────────────────────────────────────┐
   │  💾 Continuar de onde parou?        │
   │  ─────────────────────────────────  │
   │                                     │
   │  Encontramos uma avaliação não      │
   │  finalizada de 15 minutos atrás:    │
   │                                     │
   │  📝 "Avaliação de UX - App Mobile" │
   │  ⏰ Última edição: 14:32            │
   │  📊 5 perguntas criadas             │
   │                                     │
   │  [ Descartar ]  [ Continuar ]       │
   └─────────────────────────────────────┘

5a. Se "Continuar": Restaura estado exato do editor
5b. Se "Descartar": Limpa draft + redirect para dashboard
```

**Cenário 2: Resposta Pública Interrompida**

```
1. Avaliador fecha aba no meio da resposta
2. Retorna ao mesmo link
3. Sistema detecta respostas em localStorage
4. Banner aparece no topo:

   ┌─────────────────────────────────────┐
   │  ℹ️ Você tem respostas não enviadas │
   │                                     │
   │  Última resposta: Pergunta 6 de 10 │
   │  Salvo em: 08 fev, 14:45           │
   │                                     │
   │  [ Começar do Zero ]  [ Continuar ] │
   └─────────────────────────────────────┘

5a. Se "Continuar": Scroll até pergunta 7, respostas 1-6 preenchidas
5b. Se "Começar do Zero": Limpa localStorage + reload
```

### Sincronização Multi-Dispositivo

**Para usuários autenticados (Owner):**
* Drafts sincronizam automaticamente via Supabase
* Preferences (tema, filtros salvos) sincronizam
* Histórico de busca (opcional, setting)

**Para avaliadores anônimos:**
* Apenas localStorage (não sincroniza)
* Opção de "Enviar link para continuar em outro dispositivo"

---

## 🎬 Animações e Micro-interações

### Princípios de Animação

1. **Propósito:** Toda animação deve ter um propósito (feedback, guia, prazer)
2. **Duração:** 100-300ms para hover, 300-500ms para transições
3. **Easing:** `ease-out` para entrada, `ease-in` para saída
4. **Redução de movimento:** Respeitar `prefers-reduced-motion`

### Catálogo de Animações

| Componente | Animação | Trigger | Duração |
|------------|----------|---------|---------|
| **Button hover** | Elevation + scale(1.02) | Mouse over | 150ms |
| **Card hover** | Shadow intensifica | Mouse over | 200ms |
| **Toast entrada** | Slide in from right | Notification trigger | 300ms |
| **Modal open** | Fade in + scale(0.95 → 1) | Click CTA | 300ms |
| **Confetti** | Particle explosion | Success action | 2s |
| **Skeleton pulse** | Shimmer gradient | Loading state | Loop |
| **Checkmark** | Draw path + scale bounce | Success | 500ms |
| **Progress bar** | Width transition | Value change | 400ms |
| **Drag handle** | Vertical shake | Grab handle | 100ms |
| **Number counter** | Count up animation | Value increase | 800ms |

### Animações Especiais

**1. Confetti Success (Avaliação Publicada)**
```javascript
// Usando canvas-confetti library
confetti({
  particleCount: 100,
  spread: 70,
  origin: { y: 0.6 },
  colors: ['#4B0082', '#FF8C00', '#9333EA']
})
```

**2. Loading Skeleton com Shimmer**
```css
@keyframes shimmer {
  0% { background-position: -1000px 0; }
  100% { background-position: 1000px 0; }
}

.skeleton {
  background: linear-gradient(
    90deg,
    #f0f0f0 25%,
    #e0e0e0 50%,
    #f0f0f0 75%
  );
  background-size: 1000px 100%;
  animation: shimmer 2s infinite;
}
```

**3. Checkmark Success Animation**
```css
@keyframes checkmark {
  0% { stroke-dashoffset: 100; }
  100% { stroke-dashoffset: 0; }
}

.checkmark-path {
  stroke-dasharray: 100;
  animation: checkmark 0.5s ease-out forwards;
}
```

---

## 🔍 Performance e Otimizações

### Métricas Alvo (Core Web Vitals)

| Métrica | Target | Contexto |
|---------|--------|----------|
| **LCP** (Largest Contentful Paint) | < 2.5s | Dashboard deve carregar rápido |
| **FID** (First Input Delay) | < 100ms | Interações devem ser instantâneas |
| **CLS** (Cumulative Layout Shift) | < 0.1 | Sem jumps durante loading |
| **TTFB** (Time to First Byte) | < 600ms | Backend Rust otimizado |

### Estratégias de Otimização

**Frontend:**
* **Code splitting:** Lazy load de rotas (Vue Router)
* **Image optimization:** WebP format + lazy loading
* **Bundle size:** Máximo 200KB inicial (gzipped)
* **Caching:** Service Worker para assets estáticos
* **Debouncing:** Inputs de busca (300ms)
* **Virtual scrolling:** Listas com 100+ items
* **Memoization:** Componentes pesados com `memo()`

**Backend (Rust):**
* **Connection pooling:** PostgreSQL pool otimizado
* **Query optimization:** Indexes em colunas frequentes
* **Response compression:** Gzip/Brotli
* **GraphQL DataLoader:** Batch + cache de queries
* **Redis cache:** Resultados de analytics (TTL 5min)
* **Rate limiting:** Por IP e por usuário

**Database (Supabase):**
* **Indexes:** `(user_id, created_at)` para listings
* **Partitioning:** Tabela `responses` por mês
* **RLS policies:** Otimizadas com `SECURITY DEFINER`
* **Materialized views:** Analytics pré-computados

---

## 🧪 Testes de Usabilidade

### Checklist de Validação UX

**Testes Funcionais:**
- [ ] Criar avaliação do zero até publicação (< 5 minutos)
- [ ] Gerar avaliação com IA (testada com 10 prompts variados)
- [ ] Responder avaliação em mobile (touch targets adequados)
- [ ] Exportar resultados (formatos CSV, Excel, PDF)
- [ ] Compartilhar link via 3 canais (email, social, QR)

**Testes de Erro:**
- [ ] Interromper criação no meio e recuperar
- [ ] Perder conexão durante resposta (localStorage funciona)
- [ ] Timeout da IA (fallbacks acionados)
- [ ] Sessão expirada (redirect suave para login)
- [ ] Validação de Fixed Sum = 100 (bloqueio funciona)

**Testes de Acessibilidade:**
- [ ] Navegação completa por teclado
- [ ] Screen reader (NVDA) lê todos os elementos
- [ ] Contraste de cores WCAG AA
- [ ] Formulários com labels associados
- [ ] Focus visível em todos os interativos

**Testes de Performance:**
- [ ] Dashboard carrega em < 2s (3G rápido)
- [ ] Lista com 100 avaliações rola suavemente
- [ ] Gráficos de analytics renderizam em < 1s
- [ ] Skeleton screens aparecem em < 100ms
- [ ] Animações fluidas a 60fps

### Métricas de Sucesso (KPIs)

| Métrica | Target | Como Medir |
|---------|--------|-----------|
| **Time to First Evaluation** | < 7 minutos | Analytics: tempo entre signup e primeira publicação |
| **AI Generation Success Rate** | > 85% | Backend logs: successful / total attempts |
| **Response Completion Rate** | > 70% | (respostas completas / iniciadas) |
| **Mobile Usage** | > 40% | Device analytics |
| **User Retention (7 dias)** | > 50% | Usuários ativos após 7 dias do signup |
| **Error Rate** | < 1% | Sentry: erros / total requests |

---

## 📚 Documentação para Desenvolvedores

### Guia de Contribuição UI/UX

**Para adicionar nova tela:**

1. **Design:** Criar wireframe seguindo o design system
2. **Componentes:** Reusar componentes da biblioteca
3. **Estados:** Implementar loading, error, empty, success
4. **Animações:** Usar classes Tailwind + custom animations
5. **Responsive:** Testar em 3 breakpoints (mobile, tablet, desktop)
6. **A11y:** Validar com Lighthouse e screen reader
7. **Tests:** Escrever testes de interação (Vitest + Testing Library)

**Componentes Base Disponíveis:**

```
/frontend/src/components/
├── ui/
│   ├── Button.vue          # Primary, secondary, ghost variants
│   ├── Input.vue           # Com validação inline
│   ├── Modal.vue           # Fade in/out, ESC para fechar
│   ├── Toast.vue           # 5 tipos: success, error, warning, info, loading
│   ├── Card.vue            # Elevação hover, padding consistente
│   ├── Badge.vue           # Status indicators
│   ├── Skeleton.vue        # Loading placeholder
│   └── EmptyState.vue      # Ilustração + CTA
├── forms/
│   ├── LikertScale.vue
│   ├── FixedSumInput.vue
│   ├── PairedComparison.vue
│   └── FormField.vue       # Wrapper com label + error
└── dashboard/
    ├── MetricCard.vue
    ├── ChartBar.vue
    └── EvaluationCard.vue
```

### Design Tokens (Tailwind Config)

```javascript
// tailwind.config.ts
export default {
  theme: {
    extend: {
      colors: {
        primary: '#4B0082',
        secondary: '#FF8C00',
        accent: '#9333EA',
        success: '#10B981',
        warning: '#F59E0B',
        error: '#EF4444',
      },
      spacing: {
        '4.5': '1.125rem', // 18px
      },
      animation: {
        'shimmer': 'shimmer 2s infinite',
        'checkmark': 'checkmark 0.5s ease-out',
      },
    },
  },
}
```

---

## ✅ Checklist de Implementação

### Fase 1: MVP (4-6 semanas)

**Week 1-2: Fundação**
- [x] Design System implementado (cores, tipografia, componentes base)
- [ ] Sistema de autenticação completo (login, registro, JWT)
- [ ] Dashboard básico com empty state
- [ ] Routing configurado (Vue Router)

**Week 3-4: Core Features**
- [ ] Wizard de criação (template only, sem IA ainda)
- [ ] Editor de perguntas com drag-and-drop
- [ ] Tipos de escala: Likert, Frequency, Fixed Sum
- [ ] Visualização pública funcional
- [ ] Submissão e validação de respostas

**Week 5-6: Polish & Launch**
- [ ] Sistema de compartilhamento (link + QR code)
- [ ] Página de resultados básica (gráficos simples)
- [ ] Tratamento de erros completo
- [ ] Testes de usabilidade com 10 usuários
- [ ] Deploy para produção

### Fase 2: AI & Analytics (2-3 semanas)

- [ ] Integração Gemini API para geração
- [ ] Tratamento de erros da IA
- [ ] Analytics avançados (filtros, segmentação)
- [ ] Exportação de dados (CSV, Excel, PDF)
- [ ] Monitoramento em tempo real (Supabase Realtime)

### Fase 3: Refinamento (Ongoing)

- [ ] Paired Comparison method
- [ ] Sistema de notificações completo
- [ ] Templates marketplace
- [ ] Colaboração multi-usuário
- [ ] Histórico de versões
- [ ] API pública (webhooks)

---

## 📝 Notas Finais

Este documento foi criado para servir como **fonte única de verdade** para todos os aspectos de UI/UX do Aevalo. Deve ser atualizado conforme:

1. **Feedback de usuários** revela padrões de uso inesperados
2. **Novos SaaS de referência** trazem inovações relevantes
3. **Tecnologias emergentes** permitem melhorias significativas
4. **Métricas de performance** indicam gargalos ou oportunidades

**Responsabilidade:** Todos os desenvolvedores frontend devem consultar este doc antes de implementar novas telas ou fluxos.

**Versionamento:** Usar commits semânticos no Git para rastrear mudanças deste arquivo.

---

*Última atualização: 08 de fevereiro de 2026*  
*Mantido por: Equipe Aevalo*
* ✅ Captions em vídeos (se houver)
* ✅ Formulários validados com mensagens descritivas

**Navegação por Teclado:**
* `Tab` / `Shift+Tab`: Navegar entre campos
* `Enter`: Submeter / Selecionar
* `Esc`: Fechar modals
* `Arrow keys`: Navegar em listas/escalas
* `Space`: Toggle checkboxes

---

## 🔔 Sistema de Notificações

### Tipos de Toast

| Tipo | Cor | Ícone | Auto-dismiss | Posição |
|------|-----|-------|--------------|---------|
| **Success** | Verde | ✅ | 2s | Top-right |
| **Error** | Vermelho | ❌ | Persistente | Top-right |
| **Warning** | Laranja | ⚠️ | 4s | Top-right |
| **Info** | Azul | ℹ️ | 3s | Top-right |
| **Loading** | Roxo | ⏳ | Manual dismiss | Top-right |

### In-app Notifications

**Bell icon no header com badge:**
* Click abre dropdown com lista
* Tipos:
  - Nova resposta recebida
  - Avaliação atingiu X respostas
  - Prazo próximo (3 dias antes)
  - Sistema: manutenção programada
* Mark all as read
* Settings: Escolher quais notificações receber

---

## 💾 Persistência e Auto-Save

### Estratégias por Contexto

| Tela | Estratégia | Trigger | Storage |
|------|-----------|---------|---------|
| **Editor de avaliação** | Auto-save | Debounce 3s após edição | Supabase (draft) |
| **Resposta (público)** | Auto-save local | Cada resposta | localStorage |
| **Settings** | Save manual | Click "Salvar" | Supabase (user_prefs) |
| **Busca** | Query params | Tempo real | URL params |
| **Filtros** | Persist local | Change | localStorage |

### Recuperação de Sessão

**Cenário:** Usuário fecha aba durante criação

```
1. Retorna ao site
2. Sistema detecta draft não salvo em localStorage
3. Modal aparece:
   ┌─────────────────────────────────────┐
   │  Continuar de onde parou?           │
   │  ─────────────────────────────────  │
   │                                     │
   │  Encontramos uma avaliação não      │
   │  finalizada de 15 minutos atrás.    │
   │                                     │
   │  "Avaliação de UX..."               │
   │                                     │
   │  [ Descartar ]  [ Continuar ]       │
   └─────────────────────────────────────┘

4a. Se "Continuar": Restaura estado exato + redirect
4b. Se "Descartar": Limpa localStorage + dashboard
```

---