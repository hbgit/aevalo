#!/bin/bash

# Setup script for Aevalo — supports DEV and PROD (release) modes
# Usage:
#   ./setup-dev.sh --dev    (or -d)    Development mode
#   ./setup-dev.sh --prod   (or -p)    Production / release mode

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
BOLD='\033[1m'
NC='\033[0m' # No Color

# ---------------------------------------------------------------------------
# Mode selection
# ---------------------------------------------------------------------------
MODE=""

for arg in "$@"; do
    case "$arg" in
        --dev|-d)   MODE="dev"  ;;
        --prod|-p)  MODE="prod" ;;
        --help|-h)
            echo -e "${BLUE}Usage:${NC} $0 [--dev|-d | --prod|-p]"
            echo "  --dev,  -d   Set up the development environment (default if interactive)"
            echo "  --prod, -p   Set up the production / release environment"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown argument: $arg${NC}"
            echo "Use --help for usage information."
            exit 1
            ;;
    esac
done

# If no flag provided, ask interactively
if [ -z "$MODE" ]; then
    echo -e "${CYAN}${BOLD}Select setup mode:${NC}"
    echo "  [1] DEV   — Development (debug logging, local ports, hot-reload)"
    echo "  [2] PROD  — Production / Release (optimised build, strict security)"
    echo ""
    read -p "$(echo -e "${YELLOW}Enter choice [1/2]:${NC} ")" -n 1 -r MODE_CHOICE
    echo
    case "$MODE_CHOICE" in
        1) MODE="dev"  ;;
        2) MODE="prod" ;;
        *)
            echo -e "${RED}Invalid choice. Exiting.${NC}"
            exit 1
            ;;
    esac
fi

# ---------------------------------------------------------------------------
# Mode-specific variables
# ---------------------------------------------------------------------------
if [ "$MODE" = "dev" ]; then
    MODE_LABEL="DEVELOPMENT"
    MODE_COLOR="$CYAN"
    COMPOSE_FILE=""                        # use default docker-compose.yml
    COMPOSE_PROFILE=""
    RUST_LOG_LEVEL="debug"
    REQUIRE_HTTPS="false"
    CORS_ORIGINS="http://localhost:5173,http://localhost:3000"
    DOCKER_BUILD_FLAG=""                   # incremental build
    FRONTEND_URL="http://localhost:5173"
    BACKEND_URL="http://localhost:3000"
    SHOW_GRAFANA=true
else
    MODE_LABEL="PRODUCTION"
    MODE_COLOR="$RED"
    COMPOSE_FILE="-f docker-compose.yml -f docker-compose.prod.yml"
    COMPOSE_PROFILE="--profile production"
    RUST_LOG_LEVEL="warn"
    REQUIRE_HTTPS="true"
    CORS_ORIGINS="https://your-domain.com"
    DOCKER_BUILD_FLAG="--no-cache"         # clean build for release
    FRONTEND_URL="https://your-domain.com"
    BACKEND_URL="https://api.your-domain.com"
    SHOW_GRAFANA=true
fi

echo -e "\n${MODE_COLOR}${BOLD}════════════════════════════════════════════════════${NC}"
echo -e "${MODE_COLOR}${BOLD}  🚀 Aevalo Setup — ${MODE_LABEL} mode${NC}"
echo -e "${MODE_COLOR}${BOLD}════════════════════════════════════════════════════${NC}\n"

# ---------------------------------------------------------------------------
# Check prerequisites
# ---------------------------------------------------------------------------
echo -e "${YELLOW}📋 Checking prerequisites...${NC}"

# Check Docker
if ! command -v docker &> /dev/null; then
    echo -e "${RED}❌ Docker is not installed. Please install Docker.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Docker $(docker --version)${NC}"

# Check Docker Compose
if ! command -v docker compose &> /dev/null; then
    echo -e "${RED}❌ Docker Compose is not installed. Please install Docker Compose.${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Docker Compose${NC}"

# Check Node.js (optional, for local dev)
if command -v node &> /dev/null; then
    echo -e "${GREEN}✅ Node.js $(node --version)${NC}"
else
    echo -e "${YELLOW}⚠️  Node.js not found (OK for Docker-only setup)${NC}"
fi

# Check Rust (optional, for local dev)
if command -v cargo &> /dev/null; then
    echo -e "${GREEN}✅ Rust $(rustc --version | cut -d' ' -f2)${NC}"
else
    echo -e "${YELLOW}⚠️  Rust not found (OK for Docker-only setup)${NC}"
fi

# Check PostgreSQL client (optional)
if command -v psql &> /dev/null; then
    echo -e "${GREEN}✅ PostgreSQL client $(psql --version | cut -d' ' -f3)${NC}"
else
    echo -e "${YELLOW}⚠️  PostgreSQL client not found (optional)${NC}"
fi

# Production: warn if openssl is missing (needed to generate secrets)
if [ "$MODE" = "prod" ]; then
    if command -v openssl &> /dev/null; then
        echo -e "${GREEN}✅ openssl $(openssl version)${NC}"
    else
        echo -e "${YELLOW}⚠️  openssl not found — you must supply JWT_SECRET manually${NC}"
    fi
fi


# ---------------------------------------------------------------------------
# Create / validate environment files
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}📝 Setting up environment files [${MODE_LABEL}]...${NC}"

# ---- Backend .env ----
if [ ! -f backend/.env ]; then
    if [ -f backend/.env.example ]; then
        cp backend/.env.example backend/.env
        echo -e "${GREEN}✅ Created backend/.env from example${NC}"
    else
        if [ "$MODE" = "dev" ]; then
            cat > backend/.env << EOF
DATABASE_URL=postgres://aevalo:aevalo_password@localhost:5432/aevalo_db
JWT_SECRET=dev-jwt-secret-change-this-value-for-production-32chars
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
SESSION_MAX_AGE_SECONDS=3600
RUST_LOG=${RUST_LOG_LEVEL}
REQUIRE_HTTPS=${REQUIRE_HTTPS}
CORS_ALLOWED_ORIGINS=${CORS_ORIGINS}
EOF
        else
            # PROD: generate a random secret if openssl is available
            GENERATED_SECRET=""
            if command -v openssl &> /dev/null; then
                GENERATED_SECRET=$(openssl rand -hex 32)
            else
                GENERATED_SECRET="REPLACE-WITH-A-STRONG-RANDOM-SECRET-MIN-64-CHARS"
            fi
            cat > backend/.env << EOF
DATABASE_URL=postgres://aevalo:CHANGE_ME@db:5432/aevalo_db
JWT_SECRET=${GENERATED_SECRET}
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
SESSION_MAX_AGE_SECONDS=900
RUST_LOG=${RUST_LOG_LEVEL}
REQUIRE_HTTPS=${REQUIRE_HTTPS}
CORS_ALLOWED_ORIGINS=${CORS_ORIGINS}
EOF
        fi
        echo -e "${GREEN}✅ Created backend/.env with ${MODE_LABEL} defaults${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  backend/.env already exists${NC}"
fi

# ---- Frontend .env ----
ENV_FRONTEND_FILE="frontend/.env.local"
[ "$MODE" = "prod" ] && ENV_FRONTEND_FILE="frontend/.env.production"

if [ ! -f "$ENV_FRONTEND_FILE" ]; then
    if [ -f frontend/.env.example ]; then
        cp frontend/.env.example "$ENV_FRONTEND_FILE"
        echo -e "${GREEN}✅ Created ${ENV_FRONTEND_FILE} from example${NC}"
    else
        if [ "$MODE" = "dev" ]; then
            cat > "$ENV_FRONTEND_FILE" << EOF
VITE_SUPABASE_URL=https://your-project.supabase.co
VITE_SUPABASE_ANON_KEY=your-anon-key
VITE_API_URL=http://localhost:3000
VITE_GRAPHQL_URL=http://localhost:3000/graphql
VITE_APP_ENV=development
EOF
        else
            cat > "$ENV_FRONTEND_FILE" << EOF
VITE_SUPABASE_URL=https://your-project.supabase.co
VITE_SUPABASE_ANON_KEY=your-anon-key
VITE_API_URL=https://api.your-domain.com
VITE_GRAPHQL_URL=https://api.your-domain.com/graphql
VITE_APP_ENV=production
EOF
        fi
        echo -e "${GREEN}✅ Created ${ENV_FRONTEND_FILE} with ${MODE_LABEL} defaults${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  ${ENV_FRONTEND_FILE} already exists${NC}"
fi

# ---- Root .env for Docker Compose ----
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${GREEN}✅ Created .env from example${NC}"
    else
        if [ "$MODE" = "dev" ]; then
            cat > .env << EOF
# Docker Compose — DEVELOPMENT
DB_USER=aevalo
DB_PASSWORD=aevalo_password
DB_NAME=aevalo_db
JWT_SECRET=dev-jwt-secret-change-this-value-for-production-32chars
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
RUST_LOG=debug
EOF
        else
            PROD_SECRET=""
            if command -v openssl &> /dev/null; then
                PROD_SECRET=$(openssl rand -hex 32)
            else
                PROD_SECRET="REPLACE-WITH-A-STRONG-RANDOM-SECRET-MIN-64-CHARS"
            fi
            cat > .env << EOF
# Docker Compose — PRODUCTION
DB_USER=aevalo
DB_PASSWORD=CHANGE_ME_STRONG_PASSWORD
DB_NAME=aevalo_db
JWT_SECRET=${PROD_SECRET}
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
CORS_ALLOWED_ORIGINS=https://your-domain.com
RUST_LOG=warn
REQUIRE_HTTPS=true
EOF
        fi
        echo -e "${GREEN}✅ Created .env with ${MODE_LABEL} defaults${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  .env already exists${NC}"
fi

# ---------------------------------------------------------------------------
# Production: enforce security requirements before continuing
# ---------------------------------------------------------------------------
if [ "$MODE" = "prod" ]; then
    echo -e "\n${RED}${BOLD}🔒 Production security validation...${NC}"
    PROD_OK=true

    JWT_VAL=$(grep -E '^JWT_SECRET=' .env | cut -d'=' -f2-)
    if [[ "$JWT_VAL" == *"REPLACE"* ]] || [[ ${#JWT_VAL} -lt 32 ]]; then
        echo -e "${RED}  ❌ JWT_SECRET is insecure or not set in .env (min 32 chars)${NC}"
        PROD_OK=false
    else
        echo -e "${GREEN}  ✅ JWT_SECRET looks adequate${NC}"
    fi

    DB_PW=$(grep -E '^DB_PASSWORD=' .env | cut -d'=' -f2-)
    if [[ "$DB_PW" == "CHANGE_ME"* ]] || [[ -z "$DB_PW" ]]; then
        echo -e "${RED}  ❌ DB_PASSWORD must be changed in .env${NC}"
        PROD_OK=false
    else
        echo -e "${GREEN}  ✅ DB_PASSWORD is set${NC}"
    fi

    CORS_VAL=$(grep -E '^CORS_ALLOWED_ORIGINS=' .env | cut -d'=' -f2-)
    if [[ "$CORS_VAL" == *"localhost"* ]]; then
        echo -e "${YELLOW}  ⚠️  CORS_ALLOWED_ORIGINS still contains localhost — update for production${NC}"
    else
        echo -e "${GREEN}  ✅ CORS_ALLOWED_ORIGINS does not expose localhost${NC}"
    fi

    if [ "$PROD_OK" = false ]; then
        echo -e "\n${RED}❌ Production security checks failed. Fix the issues above before deploying.${NC}"
        exit 1
    fi
    echo -e "${GREEN}✅ Security validation passed${NC}"
fi


# ---------------------------------------------------------------------------
# Configuration checklist reminder
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}⚙️  Configuration checklist [${MODE_LABEL}]:${NC}"
echo "   📄 backend/.env"
echo "      - DATABASE_URL"
echo "      - JWT_SECRET (min 32 chars)"
echo "      - SUPABASE_URL and SUPABASE_API_KEY"
echo "      - SESSION_MAX_AGE_SECONDS"
if [ "$MODE" = "prod" ]; then
    echo "      - REQUIRE_HTTPS=true"
    echo "      - RUST_LOG=warn"
fi
echo ""
echo "   📄 ${ENV_FRONTEND_FILE}"
echo "      - VITE_SUPABASE_URL"
echo "      - VITE_SUPABASE_ANON_KEY"
echo "      - VITE_API_URL"
echo "      - VITE_GRAPHQL_URL"
echo ""
echo "   📄 .env (Docker Compose)"
echo "      - DB_USER, DB_PASSWORD, DB_NAME"
echo "      - JWT_SECRET"
if [ "$MODE" = "prod" ]; then
    echo "      - CORS_ALLOWED_ORIGINS (no localhost!)"
fi
echo ""

read -p "$(echo -e "${YELLOW}Have you configured the .env files? [y/N]:${NC} ")" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "\n${YELLOW}⚠️  Please configure the .env files before continuing.${NC}"
    echo "   Run this script again after configuration."
    exit 1
fi


# ---------------------------------------------------------------------------
# Build Docker images
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}🐳 Building Docker images [${MODE_LABEL}]...${NC}"
if [ "$MODE" = "dev" ]; then
    docker compose build
else
    # Production: clean build with no cache
    docker compose $COMPOSE_FILE build $DOCKER_BUILD_FLAG
fi

# ---------------------------------------------------------------------------
# Start services
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}🚀 Starting services [${MODE_LABEL}]...${NC}"
if [ "$MODE" = "dev" ]; then
    docker compose up -d
else
    docker compose $COMPOSE_FILE up -d $COMPOSE_PROFILE
fi


# ---------------------------------------------------------------------------
# Wait for database and run migrations
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}⏳ Waiting for database to be ready...${NC}"
sleep 5

if docker compose ps | grep -q "aevalo-db.*Up"; then
    echo -e "${GREEN}✅ Database is running${NC}"
    
    echo -e "\n${YELLOW}🗄️  Running database migrations...${NC}"
    for migration in \
        "001_initial_schema.sql" \
        "002_rls_policies.sql" \
        "003_sessions_and_audit.sql (Session Management)" \
        "004_security_audit.sql (Security & Audit)"; do
        file=$(echo "$migration" | awk '{print $1}')
        [ -f "backend/migrations/$file" ] && echo "   - $migration"
    done

    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/001_initial_schema.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/002_rls_policies.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/003_sessions_and_audit.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/004_security_audit.sql 2>/dev/null || true

    echo -e "${GREEN}✅ Migrations completed (check logs if errors occurred)${NC}"

    # DEV only: optionally seed database with test data
    if [ "$MODE" = "dev" ]; then
        if [ -f backend/scripts/seed_db.sh ]; then
            read -p "$(echo -e "${YELLOW}Seed database with development test data? [y/N]:${NC} ")" -n 1 -r
            echo
            if [[ $REPLY =~ ^[Yy]$ ]]; then
                echo -e "${YELLOW}🌱 Seeding database...${NC}"
                bash backend/scripts/seed_db.sh && echo -e "${GREEN}✅ Database seeded${NC}"
            fi
        fi
    fi
else
    echo -e "${YELLOW}⚠️  Database might not be ready yet. Run migrations manually if needed.${NC}"
fi

# ---------------------------------------------------------------------------
# Service status
# ---------------------------------------------------------------------------
echo -e "\n${YELLOW}🔍 Checking service status...${NC}"
sleep 2
docker compose ps

# ---------------------------------------------------------------------------
# Summary
# ---------------------------------------------------------------------------
echo ""
echo -e "${MODE_COLOR}${BOLD}════════════════════════════════════════════════════${NC}"
echo -e "${MODE_COLOR}${BOLD}  ✅ Setup complete — Aevalo [${MODE_LABEL}] is ready${NC}"
echo -e "${MODE_COLOR}${BOLD}════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}📍 Access points:${NC}"
echo "   🌐 Frontend:    ${FRONTEND_URL}"
echo "   🔧 Backend API: ${BACKEND_URL}"
echo "   📊 GraphQL:     ${BACKEND_URL}/graphql"
if [ "$MODE" = "dev" ]; then
    echo "   💾 Database:    localhost:5432"
    echo "   📈 Prometheus:  http://localhost:9090"
    echo "   📊 Grafana:     http://localhost:3001 (admin/admin)"
fi
echo ""
echo -e "${BLUE}🔐 Session Management Features:${NC}"
echo "   ✓ Hybrid storage (sessionStorage + localStorage)"
echo "   ✓ JWT token validation with refresh"
echo "   ✓ Multi-tab synchronization"
echo "   ✓ Anomaly detection (concurrent sessions, impossible travel)"
echo "   ✓ Security headers (CSP, HSTS, X-Frame-Options)"
echo "   ✓ Row-level security (RLS) policies"
echo "   ✓ Audit logging and security events"
echo ""
echo -e "${BLUE}📚 Documentation:${NC}"
echo "   📖 BUILD_GUIDE.md              - Build & deployment guide"
echo "   📖 IMPLEMENTATION_GUIDE.md     - Implementation details"
echo "   📖 session_mod.md              - Session architecture"
echo ""
echo -e "${BLUE}🛠️  Useful commands:${NC}"
echo "   ${GREEN}# View logs${NC}"
echo "   docker compose logs -f backend"
echo "   docker compose logs -f frontend"
echo "   docker compose logs -f db"
echo ""
echo "   ${GREEN}# Service management${NC}"
echo "   docker compose ps                # Check status"
echo "   docker compose down              # Stop all services"
echo "   docker compose restart backend   # Restart backend"
echo ""
echo "   ${GREEN}# Database${NC}"
echo "   docker compose exec db psql -U aevalo -d aevalo_db"
echo "   docker compose exec db pg_dump -U aevalo aevalo_db > backup.sql"
echo ""
echo "   ${GREEN}# Run migrations manually${NC}"
echo "   docker compose exec backend sqlx migrate run"
echo ""
echo -e "${BLUE}🚨 Security Checklist:${NC}"
echo "   ⚠️  Change JWT_SECRET to a strong random value (min 32 chars)"
echo "   ⚠️  Configure Supabase credentials in .env files"
if [ "$MODE" = "prod" ]; then
    echo "   ⚠️  Confirm CORS_ALLOWED_ORIGINS does not contain localhost"
    echo "   ⚠️  HTTPS is enforced (REQUIRE_HTTPS=true)"
    echo "   ⚠️  Review rate limiting and firewall rules before going live"
else
    echo "   ⚠️  Review CORS_ALLOWED_ORIGINS for production"
    echo "   ⚠️  Enable HTTPS in production (REQUIRE_HTTPS=true)"
    echo "   ⚠️  Configure rate limiting on auth endpoints"
fi
echo ""
echo -e "${YELLOW}💡 Next steps:${NC}"
if [ "$MODE" = "dev" ]; then
    echo "   1. Test authentication: http://localhost:5173/login"
    echo "   2. Check health endpoint: http://localhost:3000/health"
    echo "   3. Review security events in database"
    echo "   4. Start developing! 🎉"
else
    echo "   1. Point your DNS to this server"
    echo "   2. Configure your reverse proxy / TLS certificates"
    echo "   3. Test authentication: ${FRONTEND_URL}/login"
    echo "   4. Check health endpoint: ${BACKEND_URL}/health"
    echo "   5. Monitor logs and security events 🚀"
fi
echo ""
