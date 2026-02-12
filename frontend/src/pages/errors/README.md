# 🚨 Error & System Pages

Esta pasta contém todas as páginas de erro e status do sistema, conforme especificado em `/doc/engineering/interface_flow.md` (Seção 14).

## 📁 Estrutura

```
errors/
├── NotFound404.vue          # ✅ Implementado
├── ServerError500.vue        # ✅ Implementado
├── ServiceUnavailable503.vue # ⏳ TODO
├── Unauthorized403.vue       # ⏳ TODO
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

### ServiceUnavailable503.vue
- Countdown timer em tempo real
- Auto-refresh a cada 60s
- Link para status page externa
- Notificações pré-manutenção

### Unauthorized403.vue
- Explicação clara de permissões
- Link para contato com admin
- Visualização de permissões da conta

### StatusPage.vue
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
- [ ] ServiceUnavailable503.vue
- [ ] Unauthorized403.vue
- [ ] StatusPage.vue

---

*Última atualização: 11 de fevereiro de 2026*
