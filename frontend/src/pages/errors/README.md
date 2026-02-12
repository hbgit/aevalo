# 🚨 Error & System Pages

Esta pasta contém todas as páginas de erro e status do sistema, conforme especificado em `/doc/engineering/interface_flow.md` (Seção 14).

## 📁 Estrutura

```
errors/
├── NotFound404.vue          # ✅ Implementado
├── ServerError500.vue        # ✅ Implementado
├── ServiceUnavailable503.vue # ✅ Implementado
├── Unauthorized403.vue       # ✅ Implementado
├── StatusPage.vue           # ⏳ TODO (subdomain externo)
└── index.ts                 # Export centralizador
```

## 🎨 Páginas Implementadas

### ✅ NotFound404.vue (404 - Página Não Encontrada)

**Rota:** Catch-all `/:pathMatch(.*)*`

**Características:**
- Design responsivo com gradiente roxo/laranja
- Ilustração animada (ícone bounce + pulse)
- Código de erro 404 em destaque
- Lista de possíveis motivos
- 4 botões de ação:
  - Voltar (histórico do navegador)
  - Ir para Dashboard
  - Buscar
  - Reportar problema
- Links para páginas populares
- Log de erro com timestamp e URL tentada
- Dark mode suportado
- Animações suaves (200-300ms)

**UX Features:**
- Preserva header/sidebar se usuário autenticado
- Log automático para analytics
- Sugestões contextuais
- Feedback visual em todas as interações

### ✅ ServerError500.vue (500 - Erro Interno do Servidor)

**Rota:** Programaticamente ou via interceptor de erros

**Características:**
- Design responsivo com gradiente vermelho/laranja
- Ilustração animada (engrenagem girando + explosão pulsando)
- Código de erro 500 em destaque
- **ID de erro único** gerado automaticamente
- 4 botões de ação:
  - Tentar Novamente (primário com loading state)
  - Voltar (histórico do navegador)
  - Status (abre página de status do sistema)
  - Reportar (envia detalhes do erro)
- **Card de Status do Sistema:**
  - Status operacional (verde/vermelho)
  - Número de servidores online
  - Timestamp da última verificação
  - Link para status detalhado
- **Auto-retry configurável:**
  - Countdown de 5 segundos
  - Progress bar visual
  - Máximo de 3 tentativas
  - Desabilita após máx tentativas
- Dark mode completo
- Animações suaves

**UX Features:**
- Log automático para Sentry/tracking
- ID único para referência em suporte
- Auto-retry inteligente (5s delay)
- Status do sistema em tempo real
- Feedback visual durante retry
- Email/form pré-preenchido para reportar

**Comportamento Técnico:**
```typescript
// Geração de ID único
#ERR-YYYY-MM-DD-XXXX

// Auto-retry
- Delay: 5 segundos
- Max tentativas: 3
- Ação após falha: Desabilita auto-retry

// Logging
{
  errorId: string,
  timestamp: ISO string,
  path: string,
  referrer: string,
  userAgent: string
}
```

## ⏳ Próximas Implementações

### ✅ ServiceUnavailable503.vue (503 - Serviço Indisponível)

**Rota:** Programaticamente ou durante janela de manutenção

**Características:**
- Design responsivo com gradiente azul/turquesa
- Ilustração animada (chave inglesa girando + relógio pulsando)
- Título "Manutenção em Andamento"
- Descrição clara do motivo
- **Cronômetro em tempo real:**
  - Início da manutenção
  - Previsão de retorno
  - Tempo restante atualizado a cada segundo
- **Progress bar visual:**
  - Mostra tempo decorrido
  - Anima suavemente (transition 1s linear)
  - Gradiente laranja → roxo
- **Lista de atividades durante manutenção:**
  - Upgrade de infraestrutura
  - Melhorias de performance
  - Novos recursos sendo implantados
- **2 botões de ação:**
  - Twitter (link externo)
  - Status Page (status.aevalo.app)
- **Botão de ação primária:**
  - Recarregar Página (permite reload manual)
- **Auto-reload automático:**
  - A cada 60 segundos
  - Com countdown visível
  - Auto-refresh quando manutenção termina
- Dark mode completo
- Animações suaves

**UX Features:**
- Timestamp formatado para timezone local
- Countdown preciso (atualizado a cada segundo)
- Link clicável para Twitter e Status Page
- Botão de manual reload para usuários impaciêntes
- Auto-reload sem perder dados (páginas estáticas durante manutenção)
- Suporte para email direto de contato

**Comportamento Técnico:**
```typescript
// Auto-reload
- Intervalo: 60 segundos
- Verifica se manutenção terminou
- Se terminou: location.reload()

// Countdown
- Atualizado: 1000ms
- Formato: Xh Ym ou Xm Ys

// Progress bar
- Calcula: (endTime - now) / (endTime - startTime)
- Transition: 1s ease-linear
```

### ✅ Unauthorized403.vue (403 - Acesso Negado)

**Rota:** Programaticamente ou via erro de autorização

**Características:**
- Design responsivo com gradiente rosa/vermelho
- Ilustração animada (cadeado + símbolo proibido pulsando)
- Código de erro 403 em destaque (vermelho)
- Título "Acesso Negado"
- **Card de motivos possíveis:**
  - Recurso pertence a outro usuário
  - Sua função não permite esta ação
  - Avaliação foi arquivada ou deletada
- **4 botões de ação:**
  - Voltar para área segura (histórico)
  - Dashboard inicial
  - Ver permissões da sua conta (modal)
  - Falar com administrador (modal)
- **Modais interativas:**
  - Modal de contato com admin (email direto)
  - Modal de permissões (links para settings)
- **Seção de contato:**
  - Link direto para suporte por email
  - Estilo destacado em azul
- **Informações de erro:**
  - Exibição de path em modo dev
  - Timestamp da tentativa
- Dark mode completo
- Animações suaves (fade in/out)

### ✅ StatusPage.vue (Sistema - Status Monitoramento)

**Rota:** `/status` · Acessível via DevMenu

**Características:**
- Status geral em tempo real (🟢 All Systems Operational)
- **6 componentes do sistema monitorados:**
  - API Principal (99.98%)
  - Dashboard Frontend (100%)
  - Database Supabase (99.99%)
  - Gemini AI Integration (98.5%)
  - Email Service (99.95%)
  - Analytics Engine (100%)
- **Performance Metrics (24h):**
  - Response Time (avg): 89ms
  - Requests Processed: 1.2M
  - Error Rate: 0.02%
- **Histórico de Incidentes:**
  - Eventos com timestamps
  - Status de resolução (🟢 Resolved)
  - Duração e impacto
  - Toggle para ver incidentes antigos
- **Manutenções Programadas:**
  - Data e hora em BRT
  - Descrição detalhada
  - Botão "Add to Calendar"
- **Subscribe & Resources:**
  - Modal de inscrição (Email, Slack, Push)
  - Estatísticas de uptime
  - Links para Help Center
- **Dark mode completo**
- **Uptime indicators com animação de pulso**
- **Progress bars para cada componente**
- Layout responsivo (mobile/tablet/desktop)
- Last updated timestamp

### StatusPage.vue (Sistema - Subdomain externo)
- Monitoramento de componentes
- Uptime metrics
- Performance metrics (24h)
- Histórico de incidentes
- Manutenções programadas

## 🎯 Design System

Todas as páginas seguem o design system do Aevalo:

**Cores:**
- Primary: `#4B0082` (Roxo Profundo)
- Secondary: `#FF8C00` (Laranja Vibrante)
- Accent: `#9333EA` (Roxo Médio)
- Success: `#10B981`
- Warning: `#F59E0B`
- Error: `#EF4444`

**Tipografia:**
- Font: Inter (sistema) / Geist Sans
- Escala: 12px → 16px → 20px/24px/32px

**Componentes:**
- Border radius: 12px-16px
- Transitions: 200ms ease (hover), 300ms ease-out (modals)
- Spacing: Sistema de 4px

## 🔧 Como Usar

### Navegação Programática

```typescript
import { useRouter } from 'vue-router'

const router = useRouter()

// Redirecionar para 404
router.push({ name: 'NotFound' })

// Ou usando path
router.push('/any-invalid-path')
```

### Guard de Rotas

```typescript
// Em router/index.ts
router.beforeEach((to, from, next) => {
  // Validação customizada
  if (someCondition) {
    next({ name: 'NotFound' })
  } else {
    next()
  }
})
```

## 📊 Analytics & Tracking

Todas as páginas de erro devem logar informações para análise:

```typescript
{
  errorType: '404' | '500' | '503' | '403',
  path: string,
  fullPath: string,
  timestamp: Date,
  referrer: string,
  userAgent: string,
  userId?: string
}
```

## 🧪 Testes

```bash
# Testar página 404
npm run dev
# Acessar http://localhost:5173/pagina-inexistente

# Testar rotas específicas
http://localhost:5173/abc123
http://localhost:5173/test/nested/404
```

## 📚 Referências

- [Interface Flow Docs](/doc/engineering/interface_flow.md) - Seção 14
- [Sitemap](/doc/sitemap.md) - Páginas de Sistema
- [Design System](/doc/engineering/interface_flow.md) - Seção 1

## ✅ Checklist de Implementação

- [x] NotFound404.vue
  - [x] Wireframe implementado
  - [x] Animações
  - [x] Navegação funcional
  - [x] Dark mode
  - [x] Responsivo
  - [x] Analytics logging
- [x] ServerError500.vue
  - [x] Wireframe implementado
  - [x] ID de erro único
  - [x] Auto-retry com countdown
  - [x] Status do sistema
  - [x] Animações (spin + pulse)
  - [x] Dark mode
  - [x] Responsivo
  - [x] Error tracking logging
- [x] ServiceUnavailable503.vue
  - [x] Wireframe implementado
  - [x] Countdown timer em tempo real
  - [x] Auto-reload a cada 60s
  - [x] Progress bar visual
  - [x] Links para Twitter e Status Page
  - [x] Animações (spin + pulse)
  - [x] Dark mode
  - [x] Responsivo
- [ ] Unauthorized403.vue
- [ ] StatusPage.vue

---

*Última atualização: 11 de fevereiro de 2026*
