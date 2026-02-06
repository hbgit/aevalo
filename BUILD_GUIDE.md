# 🚀 Guia de Build - Aevalo Session Management

## 📋 Pré-requisitos

### Frontend
- Node.js 18+
- npm ou yarn

### Backend
- Rust 1.70+
- PostgreSQL 13+
- Supabase account (para autenticação)

## 🛠️ Setup Local

### 1. Frontend Setup

```bash
# Ir para diretório frontend
cd frontend

# Copiar arquivo de configuração
cp .env.example .env.local

# Instalar dependências
npm install

# Iniciar servidor de desenvolvimento
npm run dev

# Build para produção
npm run build

# Type checking
npm run type-check

# Lint código
npm run lint
```

**Arquivo `.env.local` necessário:**
```env
VITE_SUPABASE_URL=https://your-project.supabase.co
VITE_SUPABASE_ANON_KEY=your-anon-key
VITE_API_URL=http://localhost:3000
VITE_GRAPHQL_URL=http://localhost:3000/graphql
```

### 2. Backend Setup

```bash
# Ir para diretório backend
cd backend

# Copiar arquivo de configuração
cp .env.example .env

# Atualizar .env com dados locais
nano .env

# Instalar SQLx CLI (primeira vez)
cargo install sqlx-cli

# Executar migrations
sqlx migrate run

# Iniciar servidor de desenvolvimento
cargo run

# Build para produção
cargo build --release

# Testes
cargo test
```

**Arquivo `.env` necessário:**
```env
DATABASE_URL=postgresql://user:password@localhost:5432/aevalo
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
JWT_SECRET=your-secret-key-min-32-chars
RUST_LOG=debug
```

## 📦 Estrutura do Projeto

```
aevalo/
├── frontend/
│   ├── src/
│   │   ├── types/
│   │   │   ├── auth.ts          # Tipos de autenticação
│   │   │   └── utils.ts         # Utilidades
│   │   ├── stores/
│   │   │   └── auth.ts          # Pinia store
│   │   ├── composables/
│   │   │   ├── useAuthInterceptor.ts
│   │   │   └── useMultiTabSync.ts
│   │   ├── utils/
│   │   │   └── auth-client.ts   # Apollo + REST
│   │   ├── lib/
│   │   │   └── supabase.ts      # Supabase client
│   │   └── main.ts              # Entry point
│   └── package.json             # Dependências
│
├── backend/
│   ├── src/
│   │   ├── middleware/
│   │   │   ├── auth.rs          # Auth middleware
│   │   │   └── security_headers.rs
│   │   ├── modules/
│   │   │   ├── auth.rs          # JWT handling
│   │   │   └── security.rs      # Anomaly detection
│   │   ├── handlers/
│   │   │   └── auth.rs          # Auth handlers
│   │   └── main.rs              # Entry point
│   ├── migrations/
│   │   ├── 003_sessions_and_audit.sql
│   │   └── 004_security_audit.sql
│   └── Cargo.toml               # Dependências
│
└── doc/
    └── engineering/
        ├── session_mod.md       # Documentação de arquitetura
        └── IMPLEMENTATION_GUIDE.md
```

## 🔧 Compilação

### Frontend

```bash
# Desenvolvimento (com hot reload)
npm run dev

# Build otimizado
npm run build

# Preview da build
npm run preview

# Type checking
npm run type-check
```

### Backend

```bash
# Desenvolvimento
cargo run

# Debug build
cargo build

# Release build (otimizado)
cargo build --release

# Com logging detalhado
RUST_LOG=debug cargo run
```

## 🐳 Docker (Opcional)

```bash
# Build imagem backend
docker build -f docker/Dockerfile.backend -t aevalo-backend:latest .

# Build imagem frontend
docker build -f docker/Dockerfile.frontend -t aevalo-frontend:latest .

# Rodar com docker-compose
docker-compose up -d
```

## 📊 Banco de Dados

### Criar banco novo

```bash
# Conectar ao PostgreSQL
psql -U postgres

# Criar banco
CREATE DATABASE aevalo;

# Criar usuário
CREATE USER aevalo WITH PASSWORD 'aevalo_password';
GRANT ALL PRIVILEGES ON DATABASE aevalo TO aevalo;

# Conectar ao banco novo
\c aevalo

# Sair
\q
```

### Executar migrations

```bash
# Com sqlx-cli
sqlx migrate run

# Ou com psql
psql -U aevalo -d aevalo -f backend/migrations/003_sessions_and_audit.sql
psql -U aevalo -d aevalo -f backend/migrations/004_security_audit.sql
```

## ✅ Validação da Build

### Frontend

```bash
# Verificar tipos TypeScript
npm run type-check

# Linter
npm run lint

# Build
npm run build

# Verificar se não há erros
npm run preview
```

### Backend

```bash
# Check de compilação
cargo check

# Clippy (linter)
cargo clippy

# Testes
cargo test

# Build final
cargo build --release
```

## 🚀 Deploy

### Frontend (Vercel)

```bash
# Deploy automático via Vercel
vercel deploy --prod
```

### Backend (Railway/Render)

```bash
# Push para repositório Git
git push origin main

# Railway faz deploy automático
# Ou configure no painel do Render
```

## 🔍 Troubleshooting

### Erro: `DATABASE_URL not found`
```bash
# Certifique-se que .env existe e tem DATABASE_URL
cat backend/.env | grep DATABASE_URL
```

### Erro: `Module not found` (Frontend)
```bash
# Limpar node_modules e reinstalar
rm -rf node_modules package-lock.json
npm install
```

### Erro: `Cargo.lock not found`
```bash
# Regenerar Cargo.lock
cargo generate-lockfile
```

### CORS errors
- Verificar `CORS_ALLOWED_ORIGINS` no `.env` backend
- Verificar `VITE_API_URL` no `.env.local` frontend

### Erro de conexão com banco
```bash
# Testar conexão
psql -h localhost -U aevalo -d aevalo -c "SELECT 1"

# Verificar se PostgreSQL está rodando
sudo service postgresql status
```

## 📚 Documentação Adicional

- [IMPLEMENTATION_GUIDE.md](./doc/engineering/IMPLEMENTATION_GUIDE.md) - Guia completo de implementação
- [session_mod.md](./doc/engineering/session_mod.md) - Arquitetura de sessão
- [Supabase Auth Docs](https://supabase.com/docs/guides/auth)
- [Axum Docs](https://docs.rs/axum/latest/axum/)

## 🤝 Desenvolvimento

### Padrões de Código

- **Frontend:** TypeScript + Vue 3 Composition API
- **Backend:** Rust com Axum framework
- **Database:** PostgreSQL com SQLx

### Scripts Úteis

```bash
# Format código
cargo fmt
npm run lint -- --fix

# Executar migrations em reverso
sqlx migrate revert

# Ver estado das migrations
sqlx migrate info
```

## 📝 Notas Importantes

1. **JWT_SECRET:** Use valor forte em produção (min. 32 caracteres)
2. **HTTPS:** Obrigatório em produção
3. **CORS:** Configure apenas domínios confiáveis
4. **Rate Limiting:** Implemente nas rotas de autenticação
5. **Logging:** Configure níveis apropriados em produção

---

**Última atualização:** Fevereiro 2026
