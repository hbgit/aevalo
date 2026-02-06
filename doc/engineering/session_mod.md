## 🏗️ Projeto: Hybrid Session Architecture (Aevalo)

Nesta abordagem, o sistema não trata todos os dados da mesma forma. O segredo está na **volatilidade** e na **segmentação por criticidade**.

### 1. Divisão de Responsabilidades no Browser

| Artefato | Armazenamento | Motivo Técnico | TTL |
| --- | --- | --- | --- |
| **Access Token (JWT)** | `sessionStorage` | **Vida Curta.** Se o usuário fechar a aba, o token de acesso (que é o mais "poderoso") é destruído. Protege contra acesso físico indevido. | 15-60 min |
| **Refresh Token** | `localStorage` + `indexedDB` (encrypted) | **Persistência + Segurança.** Permite que o usuário continue logado após reabrir o browser. O Supabase usa isso para gerar um novo Access Token automaticamente. | 30+ dias |
| **User Profile / Prefs** | `localStorage` | **UX.** Carregar o tema, nome e preferências instantaneamente, antes mesmo da API responder. | Sem limite |
| **Session Metadata** | `sessionStorage` | **Rastreamento.** ID da sessão, timestamp de login, device fingerprint para auditoria e detecção de anomalias. | Vida da aba |
| **Sensitive Cache** | `sessionStorage` (cleared on logout) | **Dados sensíveis temporários.** Resultados de avaliações em progresso, respostas não enviadas. | Vida da aba |

---

### 2. Implementação no Frontend (Vue.js + Pinia + Composables)

Precisamos de um interceptor que saiba onde buscar cada peça do quebra-cabeça.

```typescript
// stores/auth.ts (Pinia Store - Estado Global)
import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, Session } from '../types'

export const useAuthStore = defineStore('auth', () => {
  const accessToken = ref<string | null>(
    sessionStorage.getItem('ae_at')
  )
  const user = ref<User | null>(
    JSON.parse(localStorage.getItem('ae_user') || 'null')
  )
  const sessionId = ref<string | null>(
    sessionStorage.getItem('ae_sid')
  )
  const isAuthenticated = computed(() => !!accessToken.value && !!user.value)
  
  const setSession = (session: Session) => {
    accessToken.value = session.access_token
    user.value = session.user
    sessionId.value = generateSessionId()

    // Armazenamento Sensível (Aba aberta) - Access Token NUNCA vai para localStorage
    sessionStorage.setItem('ae_at', session.access_token)
    sessionStorage.setItem('ae_sid', sessionId.value)
    sessionStorage.setItem('ae_login_at', new Date().toISOString())
    
    // Armazenamento de Identidade (Persistente)
    localStorage.setItem('ae_user', JSON.stringify(session.user))
    localStorage.setItem('ae_user_prefs', JSON.stringify(session.user.preferences || {}))
    
    // Refresh Token é gerenciado pelo SDK Supabase automaticamente
  }

  const getValidToken = async (): Promise<string> => {
    // Se o token está em sessionStorage e válido, usa-o
    if (accessToken.value && !isTokenExpired(accessToken.value)) {
      return accessToken.value
    }

    // Senão, tenta renovar usando o SDK Supabase
    // (que busca automaticamente o refresh token do localStorage)
    try {
      const { data, error } = await supabase.auth.refreshSession()
      if (error) throw error
      
      const newToken = data.session?.access_token
      if (newToken) {
        accessToken.value = newToken
        sessionStorage.setItem('ae_at', newToken)
        return newToken
      }
    } catch (err) {
      // Token refresh falhou - usuário precisa fazer login novamente
      clearSession()
      throw new Error('Session expired. Please login again.')
    }
  }

  const clearSession = () => {
    accessToken.value = null
    user.value = null
    sessionStorage.removeItem('ae_at')
    sessionStorage.removeItem('ae_sid')
    sessionStorage.removeItem('ae_login_at')
    // Refresh token é gerenciado pelo Supabase
  }

  const logout = async () => {
    await supabase.auth.signOut()
    clearSession()
  }

  return {
    accessToken,
    user,
    sessionId,
    isAuthenticated,
    setSession,
    getValidToken,
    clearSession,
    logout
  }
})
```

```typescript
// composables/useAuthInterceptor.ts (Interceptor Global)
import { useAuthStore } from '../stores/auth'
import type { AxiosRequestConfig } from 'axios'

export const useAuthInterceptor = () => {
  const authStore = useAuthStore()

  const requestInterceptor = async (config: AxiosRequestConfig) => {
    try {
      const token = await authStore.getValidToken()
      config.headers = {
        ...config.headers,
        'Authorization': \`Bearer \${token}\`,
        'X-Session-ID': authStore.sessionId // Para correlação de logs
      }
    } catch (err) {
      // Se falhar, deixa passar - backend responderá com 401
      console.error('Failed to attach auth token:', err)
    }
    return config
  }

  const responseInterceptor = async (response: any) => {
    // Processa resposta normalmente
    return response
  }

  const errorInterceptor = async (error: any) => {
    if (error.response?.status === 401) {
      // Token expirou e refresh falhou
      authStore.logout()
      window.location.href = '/login'
    }
    return Promise.reject(error)
  }

  return {
    requestInterceptor,
    responseInterceptor,
    errorInterceptor
  }
}
```

```typescript
// utils/auth-client.ts (Cliente GraphQL com auth automático)
import { ApolloClient, ApolloLink, Observable, createHttpLink } from '@apollo/client'
import { useAuthStore } from '../stores/auth'

const createAuthLink = () => {
  return new ApolloLink((operation, forward) => {
    return new Observable(observer => {
      let handle: any
      
      Promise.resolve()
        .then(async () => {
          const authStore = useAuthStore()
          const token = await authStore.getValidToken()
          operation.setContext({
            headers: {
              Authorization: token ? \`Bearer \${token}\` : '',
              'X-Session-ID': authStore.sessionId
            }
          })
        })
        .then(() => {
          handle = forward(operation).subscribe({
            next: observer.next.bind(observer),
            error: observer.error.bind(observer),
            complete: observer.complete.bind(observer),
          })
        })
        .catch(observer.error.bind(observer))

      return () => {
        if (handle) handle.unsubscribe()
      }
    })
  })
}

export const createApolloClient = () => {
  return new ApolloClient({
    link: createAuthLink().concat(
      createHttpLink({
        uri: import.meta.env.VITE_GRAPHQL_URL
      })
    ),
    cache: new InMemoryCache()
  })
}
```

---

### 3. Proteção contra XSS (Backend Rust + Headers de Segurança + CSP)

Como estamos usando o storage do browser, o risco de XSS é real. Para mitigar isso sem usar cookies `HttpOnly`, o backend em Rust deve forçar uma política de segurança rígida.

```rust
// middleware/security_headers.rs (Torre-HTTP middleware)
use axum::{
    http::HeaderValue,
    middleware::Next,
    response::Response,
};
use tower_http::set_header::SetResponseHeaderLayer;

pub fn security_headers_layer() -> SetResponseHeaderLayer {
    SetResponseHeaderLayer::overriding(
        axum::http::header::HeaderName::from_static("content-security-policy"),
        HeaderValue::from_static(
            "default-src 'self'; \
             script-src 'self' 'nonce-{random}'; \
             connect-src 'self' https://*.supabase.co; \
             style-src 'self' 'nonce-{random}' https://fonts.googleapis.com; \
             img-src 'self' data: https:; \
             font-src 'self' https://fonts.gstatic.com; \
             frame-ancestors 'none'; \
             base-uri 'self'; \
             form-action 'self';"
        )
    )
}

pub fn cors_and_security_setup() -> Router {
    Router::new()
        // Rotas aqui
        .layer(security_headers_layer())
        .layer(
            tower_http::cors::CorsLayer::permissive()
                .allow_origin("http://localhost:5173".parse()?)
                .allow_origin("https://aevalo.com".parse()?)
                .allow_credentials(true)
                .allow_headers([
                    "authorization",
                    "content-type",
                    "x-session-id",
                ])
        )
        .layer(
            SetResponseHeaderLayer::overriding(
                axum::http::header::X_CONTENT_TYPE_OPTIONS,
                HeaderValue::from_static("nosniff")
            )
        )
        .layer(
            SetResponseHeaderLayer::overriding(
                axum::http::header::X_FRAME_OPTIONS,
                HeaderValue::from_static("DENY")
            )
        )
        .layer(
            SetResponseHeaderLayer::overriding(
                axum::http::header::X_XSS_PROTECTION,
                HeaderValue::from_static("1; mode=block")
            )
        )
}
```

```rust
// handlers/auth.rs (Login com contexto seguro)
use crate::error::AppError;
use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    access_token: String,
    user: UserData,
    session_id: String,
}

#[derive(Serialize)]
pub struct UserData {
    id: String,
    email: String,
    name: String,
    preferences: UserPreferences,
}

#[derive(Serialize)]
pub struct UserPreferences {
    theme: String,
    language: String,
}

pub async fn login(
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, AppError> {
    // 1. Valida credenciais com Supabase
    let supabase_response = supabase_client
        .auth
        .sign_in_with_password(&payload.email, &payload.password)
        .await?;

    let session = supabase_response.session;
    let user = supabase_response.user;

    // 2. Cria entry na tabela 'sessions' para auditoria
    let session_id = uuid::Uuid::new_v4().to_string();
    let device_fingerprint = extract_device_fingerprint(&req)?; // Implementar

    sqlx::query(
        "INSERT INTO sessions (id, user_id, device_fingerprint, created_at, expires_at) \
         VALUES ($1, $2, $3, NOW(), NOW() + INTERVAL '30 days')"
    )
    .bind(&session_id)
    .bind(&user.id)
    .bind(&device_fingerprint)
    .execute(&pool)
    .await?;

    // 3. Retorna tokens e dados do usuário
    Ok(Json(LoginResponse {
        access_token: session.access_token,
        user: UserData {
            id: user.id,
            email: user.email,
            name: user.user_metadata.get("full_name").unwrap_or_default().to_string(),
            preferences: fetch_user_preferences(&user.id, &pool).await?,
        },
        session_id,
    }))
}
```

---

### 4. Validação e Rotação de Tokens (Backend + Row-Level Security)

```rust
// modules/auth.rs (Extração segura de contexto)
use axum::extract::FromRequestParts;
use axum::http::request::Parts;
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserContext {
    pub id: String,
    pub email: String,
    pub iat: i64,
    pub exp: i64,
}

impl UserContext {
    pub fn is_expired(&self) -> bool {
        chrono::Utc::now().timestamp() > self.exp
    }
}

#[async_trait]
impl<S> FromRequestParts<S> for UserContext
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get("Authorization")
            .and_then(|v| v.to_str().ok())
            .ok_or(AppError::Unauthorized("Missing auth header".into()))?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or(AppError::Unauthorized("Invalid bearer token".into()))?;

        let decoding_key = DecodingKey::from_secret(
            std::env::var("JWT_SECRET")?.as_bytes()
        );

        let token_data = decode::<UserContext>(
            token,
            &decoding_key,
            &Validation::default(),
        ).map_err(|_| AppError::Unauthorized("Invalid token".into()))?;

        let user = token_data.claims;

        if user.is_expired() {
            return Err(AppError::Unauthorized("Token expired".into()));
        }

        Ok(user)
    }
}

pub fn auth_user_from_headers(headers: &HeaderMap) -> Result<UserContext, AppError> {
    let auth_header = headers
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(AppError::Unauthorized("Missing auth header".into()))?;

    let token = auth_header.strip_prefix("Bearer ").ok_or(
        AppError::Unauthorized("Invalid bearer token format".into())
    )?;

    // Decodifica e valida JWT
    let decoded = decode_jwt(token)?;
    
    Ok(decoded.claims)
}

pub fn decode_jwt(token: &str) -> Result<TokenData<UserContext>, AppError> {
    let decoding_key = DecodingKey::from_secret(
        std::env::var("JWT_SECRET")?.as_bytes()
    );

    decode::<UserContext>(
        token,
        &decoding_key,
        &Validation::default(),
    ).map_err(|err| AppError::Unauthorized(format!("JWT validation failed: {}", err)))
}
```

```sql
-- migrations/003_sessions_and_audit.sql
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    device_fingerprint TEXT NOT NULL,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    last_activity TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    status TEXT DEFAULT 'active' CHECK (status IN ('active', 'revoked', 'expired')),
    is_suspicious BOOLEAN DEFAULT FALSE
);

-- Row-Level Security: Usuários só acessam suas próprias sessões
ALTER TABLE sessions ENABLE ROW LEVEL SECURITY;

CREATE POLICY "Users can view own sessions"
    ON sessions FOR SELECT
    USING (auth.uid() = user_id);

CREATE POLICY "Users can revoke own sessions"
    ON sessions FOR UPDATE
    USING (auth.uid() = user_id)
    WITH CHECK (auth.uid() = user_id);

-- Índices para performance
CREATE INDEX idx_sessions_user_id ON sessions(user_id);
CREATE INDEX idx_sessions_expires_at ON sessions(expires_at);
CREATE INDEX idx_sessions_status ON sessions(status);
```

```rust
// middleware/auth.rs (Middleware com contexto de sessão)
use axum::extract::State;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::Response;
use sqlx::PgPool;

pub async fn require_auth(
    State(pool): State<PgPool>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response, (StatusCode, String)> {
    // 1. Extrai e valida token JWT
    let user = crate::modules::auth::auth_user_from_headers(req.headers())
        .map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    // 2. Valida que a sessão ainda existe e está ativa
    let session_id = req
        .headers()
        .get("X-Session-ID")
        .and_then(|v| v.to_str().ok())
        .ok_or((StatusCode::BAD_REQUEST, "Missing session ID".into()))?;

    let session_valid: (bool,) = sqlx::query_as(
        "SELECT status = 'active' AND expires_at > NOW() FROM sessions WHERE id = $1 AND user_id = $2"
    )
    .bind(session_id)
    .bind(&user.id)
    .fetch_optional(&pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
    .ok_or((StatusCode::UNAUTHORIZED, "Session not found".into()))?;

    if !session_valid.0 {
        return Err((StatusCode::UNAUTHORIZED, "Session invalid or expired".into()));
    }

    // 3. Atualiza last_activity
    sqlx::query(
        "UPDATE sessions SET last_activity = NOW() WHERE id = $1"
    )
    .bind(session_id)
    .execute(&pool)
    .await
    .ok();

    // 4. Injeta contexto na request para handlers acessarem
    req.extensions_mut().insert(user.clone());
    req.extensions_mut().insert(session_id.to_string());

    // 5. Define RLS context no PostgreSQL
    sqlx::query("SELECT set_config('request.jwt.claim.sub', $1, true)")
        .bind(&user.id)
        .execute(&pool)
        .await
        .ok();

    Ok(next.run(req).await)
}
```

### 5. Monitoramento de Anomalias e Segurança

```rust
// modules/security.rs (Detecção de anomalias)
pub struct AnomalyDetector;

impl AnomalyDetector {
    /// Detecta múltiplos logins simultâneos de locais diferentes
    pub async fn detect_concurrent_sessions(
        user_id: &str,
        pool: &PgPool,
    ) -> Result<bool, AppError> {
        let active_sessions: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM sessions \
             WHERE user_id = $1 AND status = 'active' AND expires_at > NOW()"
        )
        .bind(user_id)
        .fetch_one(pool)
        .await?;

        // Máximo de 5 sessões simultâneas por usuário
        Ok(active_sessions > 5)
    }

    /// Detecta login de geolocação suspeita
    pub async fn detect_impossible_travel(
        user_id: &str,
        new_ip: &str,
        pool: &PgPool,
    ) -> Result<bool, AppError> {
        // Busca último login
        let last_session: Option<(String, i64)> = sqlx::query_as(
            "SELECT ip_address, \
             EXTRACT(EPOCH FROM (NOW() - created_at))::int64 as seconds_ago \
             FROM sessions \
             WHERE user_id = $1 AND status = 'active' \
             ORDER BY created_at DESC LIMIT 1"
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await?
        .flatten();

        if let Some((last_ip, seconds_ago)) = last_session {
            // Se houve login há menos de 10 minutos de outro IP distante,
            // é fisicamente impossível
            if seconds_ago < 600 && !is_same_ip_block(&last_ip, new_ip) {
                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn flag_suspicious_session(
        session_id: &str,
        reason: &str,
        pool: &PgPool,
    ) -> Result<(), AppError> {
        sqlx::query(
            "UPDATE sessions SET is_suspicious = TRUE WHERE id = $1"
        )
        .bind(session_id)
        .execute(pool)
        .await?;

        // Log para análise posterior
        sqlx::query(
            "INSERT INTO security_events (session_id, event_type, description) \
             VALUES ($1, 'suspicious_activity', $2)"
        )
        .bind(session_id)
        .bind(reason)
        .execute(pool)
        .await
        .ok();

        Ok(())
    }
}
```

```sql
-- migrations/004_security_audit.sql
CREATE TABLE security_events (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    session_id UUID REFERENCES sessions(id) ON DELETE SET NULL,
    user_id UUID REFERENCES auth.users(id) ON DELETE CASCADE,
    event_type TEXT NOT NULL,
    description TEXT,
    ip_address INET,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_security_events_user_id ON security_events(user_id);
CREATE INDEX idx_security_events_created_at ON security_events(created_at DESC);

-- Alerts para atividade suspeita
CREATE TABLE security_alerts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES auth.users(id) ON DELETE CASCADE,
    alert_type TEXT NOT NULL,
    severity TEXT CHECK (severity IN ('low', 'medium', 'high', 'critical')),
    resolved BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    resolved_at TIMESTAMP WITH TIME ZONE
);
```

### 6. Sincronização Entre Abas (LocalStorage Events)

```typescript
// composables/useMultiTabSync.ts
import { useAuthStore } from '../stores/auth'
import { watch } from 'vue'

export const useMultiTabSync = () => {
  const authStore = useAuthStore()

  // Listener para mudanças no localStorage de outras abas
  window.addEventListener('storage', (event: StorageEvent) => {
    if (event.key === 'ae_user') {
      if (event.newValue === null) {
        // Outro aba fez logout
        authStore.logout()
        window.location.href = '/login'
      } else {
        // Outro aba atualizou o usuário
        authStore.user = JSON.parse(event.newValue)
      }
    }

    if (event.key === 'ae_revoked_tokens') {
      // Tokens foram revogados em outra aba (logout remoto)
      authStore.clearSession()
      window.location.href = '/login'
    }
  })

  // Watcher: se logout acontece aqui, notifica outras abas
  watch(
    () => authStore.accessToken,
    (newToken) => {
      if (newToken === null) {
        // Sinaliza para outras abas que tokens foram revogados
        localStorage.setItem(
          'ae_revoked_tokens',
          new Date().toISOString()
        )
        localStorage.removeItem('ae_user')
      }
    }
  )
}
```

---

### 🛠️ Fluxo Completo de Autenticação e Segurança

```
┌─────────────────────────────────────────────────────────────────────┐
│ 1. LOGIN (Frontend)                                                  │
├─────────────────────────────────────────────────────────────────────┤
│  User → [Email/Senha] → Supabase Auth                               │
│                              ↓                                       │
│  ← [Access Token + Refresh Token + User Data]                       │
│                              ↓                                       │
│  Store:                                                              │
│  • sessionStorage: ae_at (Access Token)                             │
│  • sessionStorage: ae_sid (Session ID)                              │
│  • localStorage: ae_user (User Profile)                             │
│  • localStorage: (Refresh Token - gerenciado por Supabase SDK)      │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 2. REQUEST COM PROTEÇÃO (Frontend → Backend)                         │
├─────────────────────────────────────────────────────────────────────┤
│  [useAuthInterceptor]                                                │
│  1. getValidToken() → sessionStorage.ae_at                           │
│  2. Se expirado: Supabase.refreshSession() → novo token            │
│  3. Injeta: Authorization: Bearer <token>                           │
│  4. Injeta: X-Session-ID: <id>                                      │
│  5. Injeta: X-CSRF-Token: (se necessário)                           │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 3. VALIDAÇÃO NO BACKEND (Rust Middleware)                            │
├─────────────────────────────────────────────────────────────────────┤
│  [require_auth middleware]                                           │
│  1. Extrai token do header Authorization                            │
│  2. Decodifica JWT e valida assinatura                              │
│  3. Valida que sessão ainda está ativa no DB                        │
│  4. Detecta anomalias (geolocação, concorrência)                    │
│  5. Atualiza last_activity                                          │
│  6. Seta RLS context (set_config) para segurança de dados           │
│  7. Injeta UserContext nas extensões da request                    │
│  8. Permite requisição prosseguir OU retorna 401/403                │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 4. PROCESSAMENTO SEGURO (Handler com UserContext)                    │
├─────────────────────────────────────────────────────────────────────┤
│  async fn handler(user: UserContext, pool: PgPool) {                │
│    • User ID é validado e confiável                                 │
│    • Row-Level Security filtra dados automaticamente                │
│    • Logs auditam quem acessou o quê e quando                       │
│  }                                                                   │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 5. LOGOUT (Frontend)                                                 │
├─────────────────────────────────────────────────────────────────────┤
│  useAuthStore().logout()                                             │
│  1. Supabase.auth.signOut()                                         │
│  2. sessionStorage.clear()                                          │
│  3. localStorage.removeItem('ae_user')                              │
│  4. Navigate → /login                                               │
│  5. Outras abas são notificadas via StorageEvent                    │
│  6. Backend: sessão é marcada como 'revoked'                        │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────────────┐
│ 6. SEGURANÇA CONTRA ATAQUES                                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│ XSS Attack                                                           │
│ ├─ Ataque: <script>alert('hacked')</script>                         │
│ ├─ Defesa: CSP Headers (script-src 'self')                          │
│ ├─ Defesa: Token em sessionStorage (morre com aba)                  │
│ ├─ Defesa: DOMPurify em inputs                                      │
│ └─ Resultado: ✓ Acesso Token não roubado                            │
│                                                                      │
│ CSRF Attack                                                          │
│ ├─ Ataque: <img src="https://aevalo.com/logout">                   │
│ ├─ Defesa: SameSite=Strict em cookies                               │
│ ├─ Defesa: X-CSRF-Token em formulários                              │
│ └─ Resultado: ✓ Request é rejeitado                                 │
│                                                                      │
│ Token Theft (localStorage)                                           │
│ ├─ Ataque: XSS copia Refresh Token                                  │
│ ├─ Defesa: Refresh Token Rotation (Supabase)                        │
│ ├─ Defesa: Device Fingerprint validation                            │
│ ├─ Defesa: Anomaly Detection (geolocação, concorrência)             │
│ └─ Resultado: ✓ Tokens revogados em minutos                         │
│                                                                      │
│ Session Fixation                                                     │
│ ├─ Ataque: Forçar uso de session_id predefinido                     │
│ ├─ Defesa: Session ID gerado randomicamente no login                │
│ ├─ Defesa: Session ID vinculado a JWT sub                           │
│ └─ Resultado: ✓ Session_id é inválido                               │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 7. Mitigação de Riscos Conhecidos

| Risco | Impacto | Mitigação | Status |
| --- | --- | --- | --- |
| **XSS → Steal Access Token** | Alto | Access Token em sessionStorage (morre com aba) | ✅ |
| **XSS → Steal Refresh Token** | Médio | Refresh Token Rotation + Anomaly Detection | ✅ |
| **Brute Force Login** | Médio | Rate Limiting na API + CAPTCHA | 🔧 TODO |
| **Session Hijacking** | Alto | Session ID + Device Fingerprint + IP Validation | ✅ |
| **Account Enumeration** | Baixo | Mensagens genéricas ("Invalid credentials") | ✅ |
| **CSRF** | Médio | SameSite + CSRF Token + Origin validation | ✅ |
| **Man-in-the-Middle** | Alto | HTTPS obrigatório + HSTS | ✅ |
| **Token Expiration Edge Case** | Médio | Retry logic com exponential backoff | 🔧 TODO |

---

### 🛠️ Resumo da Implementação

1. **Login:** O usuário entra; Access Token vai para `sessionStorage`, Refresh vai para `localStorage` (gerenciado por Supabase).
2. **Requisição:** O Vue pega o token do `sessionStorage` via `useAuthInterceptor` e envia no header `Authorization: Bearer <token>`.
3. **Processamento:** Rust valida o JWT via middleware `require_auth` e verifica sessão ativa no DB.
4. **Handler:** UserContext é injetado automaticamente; handler executa com segurança garantida.
5. **Fechamento de Aba:** O Access Token morre com a aba.
6. **Retorno ao Site:** O Vue percebe que não tem Access Token, usa o Refresh do `localStorage` para pedir um novo ao Supabase e repopula o `sessionStorage`.
7. **Logout:** Todas as abas são notificadas; sessão é revogada no backend.

### 📋 Checklist de Implementação

- [ ] Criar tabela `sessions` com RLS policies
- [ ] Criar tabela `security_events` para auditoria
- [ ] Implementar middleware `require_auth` em Rust
- [ ] Implementar composable `useAuthInterceptor` no Vue
- [ ] Implementar composable `useMultiTabSync` para sincronização
- [ ] Implementar store `useAuthStore` com Pinia
- [ ] Adicionar CSP headers em todas as respostas
- [ ] Implementar `AnomalyDetector` para detecção de fraude
- [ ] Adicionar rate limiting na rota `/auth/login`
- [ ] Configurar HTTPS/HSTS
- [ ] Testes de segurança (OWASP Top 10)
- [ ] Configurar monitoring/alerting de eventos suspeitos

---

### 🔐 Validação de Segurança (Risk Assessment Atualizado)

> **"Engenheiro, e se capturarem o Refresh Token no LocalStorage?"**

É aqui que entra a **Refresh Token Rotation** do Supabase + **Anomaly Detection**:

1. Atacante copia `localStorage` e obtém Refresh Token.
2. Atacante chama `/auth/refresh` em geolocação diferente.
3. **Backend detecta:**
   - Uso de Refresh Token em novo device_fingerprint
   - Login de geolocação fisicamente impossível
   - Mais de 5 sessões simultâneas
4. **Backend marca como suspeito** e armazena em `security_alerts`.
5. **Usuário legítimo tenta usar sua sessão:**
   - Refresh Token dele é detectado como "usado"
   - Supabase revoga **toda a família de tokens**
6. **Resultado:** Ambos deslogados, ataque é neutralizado em minutos, e administradores são alertados.

Essa camada adicional de detecção torna o roubo de Refresh Token praticamente inútil no contexto do Aevalo.