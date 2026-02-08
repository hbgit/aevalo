# 🗺️ Sitemap - Aevalo SaaS

## 📍 Estrutura de Navegação

### **1. Área Pública (Não Autenticada)**

```
/
├── / (Landing Page)
│   ├── Seções:
│   │   ├── Hero (CTA principal)
│   │   ├── Recursos principais
│   │   ├── Como funciona
│   │   ├── Casos de uso
│   │   ├── Planos e preços
│   │   ├── Testemunhos
│   │   └── Footer
│   │
├── /login
│   └── Toggle: Login / Cadastro
│
├── /register
│   └── Formulário de cadastro
│
├── /forgot-password
│   └── Recuperação de senha
│
├── /reset-password/:token
│   └── Definir nova senha
│
├── /public/:share_token (Avaliação Pública)
│   ├── Formulário de resposta
│   ├── Visualização de perguntas
│   └── Confirmação de envio
│
└── /public/:share_token/stats (Estatísticas Públicas)
    ├── Resultados agregados
    └── Gráficos públicos
```

---

### **2. Área Autenticada (Dashboard)**

```
/app
├── /dashboard (Hub Central)
│   ├── Hero com saudação dinâmica
│   ├── Cards de métricas:
│   │   ├── Prazo mais próximo
│   │   ├── Avaliadores ativos
│   │   ├── Taxa de conclusão
│   │   └── Avaliações abertas
│   ├── Barra de busca global (⌘K)
│   ├── Filtros (status, categoria, data)
│   ├── Lista/Grid de avaliações
│   └── Widget de analytics
│
├── /evaluations (Gerenciamento de Avaliações)
│   ├── /evaluations (Lista completa)
│   ├── /evaluations/create (Wizard de criação)
│   │   ├── Step 1: Escolher método (Template vs IA)
│   │   ├── Step 2: Configuração básica
│   │   ├── Step 3: Definir perguntas/escalas
│   │   ├── Step 4: Personalização visual
│   │   └── Step 5: Revisão e publicação
│   │
│   ├── /evaluations/:id (Detalhes da avaliação)
│   │   ├── Visão geral
│   │   ├── Perguntas e escalas
│   │   ├── Configurações
│   │   └── Ações (editar, arquivar, compartilhar)
│   │
│   ├── /evaluations/:id/edit (Edição)
│   │   └── Formulário completo de edição
│   │
│   ├── /evaluations/:id/responses (Respostas recebidas)
│   │   ├── Lista de respostas
│   │   ├── Filtros por data/usuário
│   │   └── Exportação (CSV/PDF)
│   │
│   ├── /evaluations/:id/analytics (Análise detalhada)
│   │   ├── Dashboard de resultados
│   │   ├── Gráficos por pergunta
│   │   ├── Comparações estatísticas
│   │   ├── Insights da IA
│   │   └── Exportação de relatórios
│   │
│   ├── /evaluations/:id/share (Compartilhamento)
│   │   ├── Link público
│   │   ├── QR Code
│   │   ├── Embed code
│   │   └── Configurações de privacidade
│   │
│   └── /evaluations/:id/collaborate (Colaboração)
│       ├── Convidar co-autores
│       ├── Gerenciar permissões
│       └── Log de atividades
│
├── /analytics (Analytics Global)
│   ├── Visão geral de todas avaliações
│   ├── Tendências por categoria
│   ├── Performance ao longo do tempo
│   ├── Comparações entre avaliações
│   └── Exportação de dados agregados
│
├── /templates (Biblioteca de Templates)
│   ├── /templates (Galeria de templates)
│   │   ├── Filtros por categoria
│   │   ├── Busca
│   │   └── Preview de templates
│   │
│   ├── /templates/:id/preview (Visualização)
│   │   └── Prévia completa do template
│   │
│   └── /templates/:id/use (Usar template)
│       └── Wizard de criação pré-preenchido
│
├── /scales (Escalas Científicas)
│   ├── Documentação de escalas suportadas:
│   │   ├── Likert Scale
│   │   ├── Frequency Scale
│   │   ├── Paired Comparison
│   │   ├── Fixed Sum
│   │   ├── Rating Scale
│   │   ├── Semantic Differential
│   │   └── Binary Scale
│   │
│   └── Exemplos práticos de uso
│
├── /ai-assistant (Assistente IA)
│   ├── Chat interface para geração
│   ├── Histórico de gerações
│   ├── Configurações da IA
│   └── Modelos salvos
│
├── /notifications (Central de Notificações)
│   ├── Todas as notificações
│   ├── Filtros (lidas/não lidas, tipo)
│   └── Configurações de alertas
│
├── /settings (Configurações)
│   ├── /settings/profile (Perfil)
│   │   ├── Informações pessoais
│   │   ├── Avatar
│   │   └── Senha
│   │
│   ├── /settings/preferences (Preferências)
│   │   ├── Tema (light/dark/auto)
│   │   ├── Idioma
│   │   ├── Notificações
│   │   └── Timezone
│   │
│   ├── /settings/account (Conta)
│   │   ├── Plano atual
│   │   ├── Uso e limites
│   │   ├── Faturamento
│   │   └── Excluir conta
│   │
│   ├── /settings/team (Equipe - se aplicável)
│   │   ├── Membros
│   │   ├── Convites pendentes
│   │   └── Permissões
│   │
│   ├── /settings/integrations (Integrações)
│   │   ├── Webhooks
│   │   ├── API keys
│   │   └── Conexões externas
│   │
│   └── /settings/security (Segurança)
│       ├── Autenticação de dois fatores
│       ├── Sessões ativas
│       └── Log de atividades
│
└── /help (Central de Ajuda)
    ├── /help/docs (Documentação)
    ├── /help/tutorials (Tutoriais)
    ├── /help/faq (Perguntas frequentes)
    ├── /help/contact (Suporte)
    └── /help/changelog (Notas de versão)
```

---

### **3. Área Administrativa (Admin)**

```
/admin
├── /admin/dashboard (Métricas globais)
├── /admin/users (Gestão de usuários)
├── /admin/evaluations (Todas avaliações)
├── /admin/analytics (Analytics do sistema)
├── /admin/moderation (Moderação de conteúdo)
└── /admin/settings (Configurações globais)
```

---

### **4. Páginas de Sistema**

```
/system
├── /404 (Página não encontrada)
├── /500 (Erro do servidor)
├── /503 (Manutenção)
├── /unauthorized (Sem permissão)
├── /terms (Termos de uso)
├── /privacy (Política de privacidade)
└── /status (Status do sistema)
```

---

## 🔑 Hierarquia de Informação

### **Nível 1 - Navegação Principal (Sidebar)**
- 🏠 Dashboard
- 📊 Avaliações
- 📈 Analytics
- 📋 Templates
- ⚙️ Configurações
- 👤 Perfil

### **Nível 2 - Ações Contextuais**
- Busca global (⌘K)
- Notificações
- Criar nova avaliação (+ flutuante)
- Ajuda (?)

### **Nível 3 - Conteúdo Dinâmico**
- Detalhes de cada avaliação
- Resultados e analytics
- Formulários de edição

---

## 📱 Considerações de UX

### **Navegação por Teclado**
- `⌘K` / `Ctrl+K` → Busca global
- `⌘N` / `Ctrl+N` → Nova avaliação
- `⌘/` / `Ctrl+/` → Central de ajuda
- `Esc` → Fechar modais/overlays
- `Tab` → Navegação entre campos
- `↑↓` → Navegação em listas

### **Breadcrumbs** (em páginas profundas)
```
Dashboard > Avaliações > NPS Q1 2026 > Analytics
```

### **Estado de Carregamento**
- Skeleton screens em todas as páginas
- Spinners em ações assíncronas
- Progress bars em uploads/processos longos

### **Feedback Visual**
- Toasts para confirmações
- Modals para ações destrutivas
- Inline validation em formulários
- Badge counters em notificações

---

## 🎯 URLs Amigáveis (SEO & UX)

```
✅ Bom: /evaluations/nps-q1-2026
❌ Ruim: /evaluations/550e8400-e29b-41d4-a716-446655440000

✅ Bom: /public/customer-satisfaction
❌ Ruim: /p/x9kL2mP

✅ Bom: /templates/employee-feedback
❌ Ruim: /t/123
```

---

## 🔄 Fluxos Principais

### **1. Onboarding (First-Time User)**
```
/register → /welcome/step-1 → /welcome/step-2 → /welcome/step-3 → /dashboard
```

### **2. Criação de Avaliação**
```
/dashboard → /evaluations/create → [wizard steps] → /evaluations/:id
```

### **3. Responder Avaliação**
```
/public/:token → [formulário] → /public/:token/thanks
```

### **4. Análise de Resultados**
```
/evaluations/:id → /evaluations/:id/analytics → [exportar relatório]
```

---

## 📊 Métricas de Navegação (para monitorar)

- Taxa de conclusão do wizard de criação
- Tempo médio no dashboard
- Páginas mais acessadas
- Taxa de abandono em formulários
- Uso de busca global vs navegação manual
- Cliques em CTAs principais