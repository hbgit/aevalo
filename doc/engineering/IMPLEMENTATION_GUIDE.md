/**
 * @file IMPLEMENTATION_GUIDE.md
 * @description Complete implementation guide for Hybrid Session Architecture
 */

# Guia de Implementação - Hybrid Session Architecture (Aevalo)

## 📋 Visão Geral

Este documento fornece um guia passo-a-passo para implementar o sistema de gerenciamento de sessões híbrido do Aevalo.

## 🎯 Arquivos Criados

### Frontend (TypeScript/Vue)

1. **`frontend/src/types/auth.ts`**
   - Interface `User` - Perfil do usuário
   - Interface `Session` - Dados da sessão
   - Interface `LoginRequest/Response` - Requisições de autenticação
   - Interface `UserContext` - Contexto extraído do JWT
   - Interface `SecurityEvent/Alert` - Eventos de segurança

2. **`frontend/src/types/utils.ts`**
   - `decodeJWT()` - Decodifica token JWT
   - `isTokenExpired()` - Verifica expiração
   - `generateDeviceFingerprint()` - Gera fingerprint do device
   - `validatePasswordStrength()` - Valida força da senha

3. **`frontend/src/stores/auth.ts`** (Pinia Store)
   - `useAuthStore()` - Estado global de autenticação
   - Métodos: `login()`, `logout()`, `getValidToken()`, `restoreSession()`
   - Storage automático em sessionStorage e localStorage

4. **`frontend/src/composables/useAuthInterceptor.ts`**
   - Interceptor de requisições HTTP
   - Adiciona headers de autenticação automaticamente
   - Trata erros 401/403

5. **`frontend/src/composables/useMultiTabSync.ts`**
   - Sincronização de logout entre abas
   - Broadcast de atualizações de perfil
   - Event listeners para localStorage

6. **`frontend/src/utils/auth-client.ts`**
   - Client Apollo GraphQL com autenticação
   - REST fetcher autenticado
   - Error handling para auth errors

7. **`frontend/src/lib/supabase.ts`**
   - Inicialização do cliente Supabase
   - Configuração de persistência de sessão

### Backend (Rust)

1. **`backend/src/middleware/security_headers.rs`**
   - CSP headers
   - X-Content-Type-Options
   - HSTS, Referrer-Policy
   - Headers de proteção contra XSS

2. **`backend/src/modules/auth.rs`** (atualizado)
   - `struct Claims` - Estrutura JWT
   - `struct AuthUser` - Usuário autenticado
   - `decode_jwt()` - Valida JWT
   - `extract_user_from_headers()` - Extrai usuário da requisição
   - `extract_session_id()` - Extrai ID da sessão

3. **`backend/src/modules/security.rs`**
   - `AnomalyDetector` - Detecção de anomalias
   - `detect_concurrent_sessions()` - Detecta múltiplas sessões
   - `detect_impossible_travel()` - Detecta viagens impossíveis
   - `flag_suspicious_session()` - Marca sessão como suspeita
   - `log_security_event()` - Registra eventos de segurança

4. **`backend/src/middleware/auth.rs`** (atualizado)
   - `require_auth()` - Middleware de autenticação obrigatória
   - `optional_auth()` - Middleware de autenticação opcional
   - Validação de JWT
   - Validação de sessão no banco

5. **`backend/src/handlers/auth.rs`** (atualizado)
   - `login()` - Handler de login
   - `logout()` - Handler de logout
   - `refresh_token()` - Handler de refresh de token
   - Estruturas de resposta (LoginResponse, UserData)

### Database (SQL)

1. **`backend/migrations/003_sessions_and_audit.sql`**
   - Tabela `sessions` - Gerenciamento de sessões
   - Tabela `user_preferences` - Preferências do usuário
   - Row-Level Security (RLS) policies
   - Índices para performance

2. **`backend/migrations/004_security_audit.sql`**
   - Tabela `security_events` - Log de eventos
   - Tabela `security_alerts` - Alertas de segurança
   - Tabela `audit_log` - Auditoria de banco
   - Funções PL/pgSQL para cleanup e detecção

## 🚀 Passos de Implementação

### 1. Frontend Setup

```bash
# Instale dependências
npm install @supabase/supabase-js
npm install pinia @pinia/nuxt
npm install axios
npm install @apollo/client graphql

# Crie arquivo .env.local
cp frontend/.env.example frontend/.env.local

# Configure VITE_SUPABASE_URL e VITE_SUPABASE_ANON_KEY
```

### 2. Backend Setup

```bash
# Configure arquivo .env
cp backend/.env.example backend/.env

# Atualize DATABASE_URL, JWT_SECRET, etc.
nano backend/.env

# Execute migrations
sqlx migrate run

# Compile backend
cargo build --release
```

### 3. Integração Principal

**frontend/src/main.ts:**
```typescript
import { createApp } from 'vue'
import { createPinia } from 'pinia'
import App from './App.vue'
import { createApolloClient } from './utils/auth-client'
import { useAuthStore } from './stores/auth'

const app = createApp(App)
const pinia = createPinia()

app.use(pinia)

// Restaurar sessão
const authStore = useAuthStore()
authStore.restoreSession()

// Setup Apollo
const apolloClient = createApolloClient()
app.use(ApolloClient, { defaultClient: apolloClient })

app.mount('#app')
```

**backend/src/main.rs:**
```rust
use tower_http::cors::CorsLayer;
use crate::middleware::security_headers::*;
use crate::middleware::auth::require_auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ... setup logging e database ...

    let app = Router::new()
        // Public endpoints
        .route("/auth/login", post(handlers::auth::login))
        
        // Protected endpoints
        .route("/api/user", get(handlers::user::get_user)
            .layer(axum_middleware::from_fn_with_state(
                pool.clone(),
                require_auth,
            ))
        )
        
        // Security headers
        .layer(csp_header_layer())
        .layer(x_content_type_options_layer())
        .layer(x_frame_options_layer())
        .layer(x_xss_protection_layer())
        .layer(hsts_header_layer())
        .layer(referrer_policy_layer())
        
        // CORS
        .layer(
            CorsLayer::permissive()
                .allow_origin("http://localhost:5173".parse()?)
                .allow_credentials(true)
        )
        
        .with_state(pool);

    // ... start server ...
}
```

### 4. Login Page (Vue Component)

```vue
<template>
  <form @submit.prevent="handleLogin">
    <input v-model="email" type="email" placeholder="Email" />
    <input v-model="password" type="password" placeholder="Password" />
    <button :disabled="isLoading">Login</button>
  </form>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import { useAuthStore } from '@/stores/auth'
import { useRouter } from 'vue-router'

const authStore = useAuthStore()
const router = useRouter()
const email = ref('')
const password = ref('')
const isLoading = ref(false)

const handleLogin = async () => {
  isLoading.value = true
  try {
    await authStore.login({ email: email.value, password: password.value })
    router.push('/dashboard')
  } catch (err) {
    console.error('Login failed:', err)
  } finally {
    isLoading.value = false
  }
}
</script>
```

## 🔒 Checklist de Segurança

- [ ] JWT_SECRET configurado com valor forte em produção
- [ ] HTTPS habilitado em produção
- [ ] CORS configurado apenas para domínios permitidos
- [ ] CSP headers configurados apropriadamente
- [ ] Refresh Token Rotation habilitado no Supabase
- [ ] Rate limiting implementado na rota `/auth/login`
- [ ] Anomaly detection testado
- [ ] Migrations executadas e validadas
- [ ] Testes de segurança (OWASP Top 10) realizados
- [ ] Monitoring/alerting de eventos suspeitos configurado

## 📊 Fluxo de Autenticação

```
1. User → Login Page
2. LoginRequest → /api/auth/login
3. Backend → Valida com Supabase + Cria Session
4. Response → { access_token, session_id, user, expires_in }
5. Frontend → sessionStorage.ae_at = access_token
6. Frontend → localStorage.ae_user = user
7. Frontend → localStorage (Refresh Token via Supabase SDK)
8. Request → /api/protected (header: Authorization: Bearer <token>)
9. Middleware → Valida JWT + Verifica Session
10. Handler → Executa com UserContext confiável
```

## 🚨 Troubleshooting

### Token expirado recorrentemente
- Aumente `JWT_EXPIRY_SECONDS` em backend/.env
- Verifique se sistema de refresh está funcionando
- Valide clock sync entre frontend/backend

### Sessão não sincroniza entre abas
- Verifique se composable `useMultiTabSync` está montado em App.vue
- Confirme que localStorage não está sendo bloqueado
- Teste em modo incógnito (evita extensões do browser)

### CORS error em requisições
- Atualize CORS_ALLOWED_ORIGINS em backend/.env
- Confirme que credenciais estão incluídas nas requisições
- Verifique Origin header nas requisições

### Detecção de anomalias muito agressiva
- Reduza `max_concurrent_sessions` em `AnomalyDetector`
- Ajuste threshold de `impossible_travel`
- Implemente whitelist de IPs confiáveis

## 📚 Referências

- [Supabase Auth](https://supabase.com/docs/guides/auth)
- [JWT Best Practices](https://tools.ietf.org/html/rfc8725)
- [OWASP Session Management](https://cheatsheetseries.owasp.org/cheatsheets/Session_Management_Cheat_Sheet.html)
- [Content Security Policy](https://developer.mozilla.org/en-US/docs/Web/HTTP/CSP)

## 🤝 Suporte

Para questões sobre implementação, consulte:
- Documentação: `/doc/engineering/session_mod.md`
- Issues: GitHub Issues
- Slack: #engineering channel
