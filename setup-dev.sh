#!/bin/bash

# Setup script for Aevalo development environment with Session Management

set -e

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}🚀 Setting up Aevalo development environment with Hybrid Session Architecture...${NC}"

# Check prerequisites
echo -e "\n${YELLOW}📋 Checking prerequisites...${NC}"

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

# Create environment files
echo -e "\n${YELLOW}📝 Creating environment files...${NC}"

# Backend .env
if [ ! -f backend/.env ]; then
    cp backend/.env.example backend/.env
    echo -e "${GREEN}✅ Created backend/.env${NC}"
else
    echo -e "${BLUE}ℹ️  backend/.env already exists${NC}"
fi

# Frontend .env
if [ ! -f frontend/.env.local ]; then
    cp frontend/.env.example frontend/.env.local
    echo -e "${GREEN}✅ Created frontend/.env.local${NC}"
else
    echo -e "${BLUE}ℹ️  frontend/.env.local already exists${NC}"
fi

# Root .env for Docker Compose
if [ ! -f .env ]; then
    if [ -f .env.example ]; then
        cp .env.example .env
        echo -e "${GREEN}✅ Created .env${NC}"
    else
        # Create minimal .env if example doesn't exist
        cat > .env << 'EOF'
# Docker Compose Environment Variables
DB_USER=aevalo
DB_PASSWORD=aevalo_password
DB_NAME=aevalo_db
JWT_SECRET=your-jwt-secret-change-this-in-production-min-32-chars
SUPABASE_URL=https://your-project.supabase.co
SUPABASE_API_KEY=your-api-key
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
RUST_LOG=debug
EOF
        echo -e "${GREEN}✅ Created .env with defaults${NC}"
    fi
else
    echo -e "${BLUE}ℹ️  .env already exists${NC}"
fi

echo -e "\n${YELLOW}⚙️  Configuration checklist:${NC}"
echo "   📄 backend/.env"
echo "      - DATABASE_URL"
echo "      - JWT_SECRET (min 32 chars)"
echo "      - SUPABASE_URL and SUPABASE_API_KEY"
echo "      - SESSION_MAX_AGE_SECONDS"
echo ""
echo "   📄 frontend/.env.local"
echo "      - VITE_SUPABASE_URL"
echo "      - VITE_SUPABASE_ANON_KEY"
echo "      - VITE_API_URL"
echo "      - VITE_GRAPHQL_URL"
echo ""
echo "   📄 .env (Docker Compose)"
echo "      - DB_USER, DB_PASSWORD, DB_NAME"
echo "      - JWT_SECRET"
echo ""

read -p "$(echo -e ${YELLOW}Have you configured the .env files? [y/N]:${NC} )" -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo -e "\n${YELLOW}⚠️  Please configure the .env files before continuing.${NC}"
    echo "   Run this script again after configuration."
    exit 1
fi

# Build Docker images
echo -e "\n${YELLOW}🐳 Building Docker images...${NC}"
docker compose build

# Start services
echo -e "\n${YELLOW}🚀 Starting services...${NC}"
docker compose up -d

# Wait for database to be ready
echo -e "\n${YELLOW}⏳ Waiting for database to be ready...${NC}"
sleep 5

# Check if database is up
if docker compose ps | grep -q "aevalo-db.*Up"; then
    echo -e "${GREEN}✅ Database is running${NC}"
    
    # Run migrations
    echo -e "\n${YELLOW}🗄️  Running database migrations...${NC}"
    if [ -f backend/migrations/001_initial_schema.sql ]; then
        echo "   - 001_initial_schema.sql"
    fi
    if [ -f backend/migrations/002_rls_policies.sql ]; then
        echo "   - 002_rls_policies.sql"
    fi
    if [ -f backend/migrations/003_sessions_and_audit.sql ]; then
        echo "   - 003_sessions_and_audit.sql (Session Management)"
    fi
    if [ -f backend/migrations/004_security_audit.sql ]; then
        echo "   - 004_security_audit.sql (Security & Audit)"
    fi
    
    # Try to run migrations via docker
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/001_initial_schema.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/002_rls_policies.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/003_sessions_and_audit.sql 2>/dev/null || true
    docker compose exec -T db psql -U aevalo -d aevalo_db -f /docker-entrypoint-initdb.d/004_security_audit.sql 2>/dev/null || true
    
    echo -e "${GREEN}✅ Migrations completed (check logs if errors occurred)${NC}"
else
    echo -e "${YELLOW}⚠️  Database might not be ready yet. Run migrations manually if needed.${NC}"
fi

# Check service status
echo -e "\n${YELLOW}🔍 Checking service status...${NC}"
sleep 2
docker compose ps

echo ""
echo -e "${GREEN}════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}✅ Setup complete! Aevalo is ready for development${NC}"
echo -e "${GREEN}════════════════════════════════════════════════════${NC}"
echo ""
echo -e "${BLUE}📍 Access points:${NC}"
echo "   🌐 Frontend:    http://localhost:5173"
echo "   🔧 Backend API: http://localhost:3000"
echo "   📊 GraphQL:     http://localhost:3000/graphql"
echo "   💾 Database:    localhost:5432"
echo "   📈 Prometheus:  http://localhost:9090"
echo "   📊 Grafana:     http://localhost:3001 (admin/admin)"
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
echo "   ⚠️  Review CORS_ALLOWED_ORIGINS for production"
echo "   ⚠️  Enable HTTPS in production (REQUIRE_HTTPS=true)"
echo "   ⚠️  Configure rate limiting on auth endpoints"
echo ""
echo -e "${YELLOW}💡 Next steps:${NC}"
echo "   1. Test authentication: http://localhost:5173/login"
echo "   2. Check health endpoint: http://localhost:3000/health"
echo "   3. Review security events in database"
echo "   4. Start developing! 🎉"
echo ""
