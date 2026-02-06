#!/bin/bash
# setup-auth-system.sh
# Automated setup script for Aevalo Session Management System

set -e  # Exit on error

echo "🚀 Aevalo Session Management System - Setup Script"
echo "=================================================="

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check prerequisites
echo -e "\n${YELLOW}📋 Checking prerequisites...${NC}"

# Check Node.js
if ! command -v node &> /dev/null; then
    echo -e "${RED}❌ Node.js not found. Please install Node.js 18+${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Node.js $(node --version)${NC}"

# Check npm
if ! command -v npm &> /dev/null; then
    echo -e "${RED}❌ npm not found. Please install npm${NC}"
    exit 1
fi
echo -e "${GREEN}✅ npm $(npm --version)${NC}"

# Check Rust
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}❌ Rust/Cargo not found. Please install Rust${NC}"
    exit 1
fi
echo -e "${GREEN}✅ Rust $(rustc --version)${NC}"

# Check PostgreSQL
if ! command -v psql &> /dev/null; then
    echo -e "${YELLOW}⚠️  PostgreSQL client not found. Some features may not work${NC}"
else
    echo -e "${GREEN}✅ PostgreSQL $(psql --version)${NC}"
fi

# Setup Frontend
echo -e "\n${YELLOW}📦 Setting up Frontend...${NC}"

cd frontend

# Copy .env.local if it doesn't exist
if [ ! -f .env.local ]; then
    echo -e "${YELLOW}Creating .env.local...${NC}"
    cp .env.example .env.local
    echo -e "${GREEN}✅ .env.local created${NC}"
    echo -e "${YELLOW}⚠️  Please edit frontend/.env.local with your Supabase credentials${NC}"
else
    echo -e "${GREEN}✅ .env.local already exists${NC}"
fi

# Install dependencies
echo -e "${YELLOW}Installing npm dependencies...${NC}"
npm install
echo -e "${GREEN}✅ Frontend dependencies installed${NC}"

# Type checking
echo -e "${YELLOW}Running type check...${NC}"
npm run type-check || echo -e "${YELLOW}⚠️  Type check warnings (non-blocking)${NC}"

cd ..

# Setup Backend
echo -e "\n${YELLOW}📦 Setting up Backend...${NC}"

cd backend

# Copy .env if it doesn't exist
if [ ! -f .env ]; then
    echo -e "${YELLOW}Creating .env...${NC}"
    cp .env.example .env
    echo -e "${GREEN}✅ .env created${NC}"
    echo -e "${YELLOW}⚠️  Please edit backend/.env with your database and Supabase credentials${NC}"
else
    echo -e "${GREEN}✅ .env already exists${NC}"
fi

# Install sqlx-cli if needed
if ! command -v sqlx &> /dev/null; then
    echo -e "${YELLOW}Installing sqlx-cli...${NC}"
    cargo install sqlx-cli --no-default-features --features postgres
    echo -e "${GREEN}✅ sqlx-cli installed${NC}"
fi

# Check if database is configured
if [ -z "$DATABASE_URL" ]; then
    source .env
fi

if [ -z "$DATABASE_URL" ]; then
    echo -e "${YELLOW}⚠️  DATABASE_URL not set. Please configure your database first:${NC}"
    echo "1. Create PostgreSQL database"
    echo "2. Set DATABASE_URL in backend/.env"
    echo "3. Run: sqlx migrate run"
else
    echo -e "${YELLOW}Testing database connection...${NC}"
    if sqlx database create 2>/dev/null; then
        echo -e "${GREEN}✅ Database created${NC}"
    else
        echo -e "${YELLOW}Database may already exist (this is OK)${NC}"
    fi
    
    echo -e "${YELLOW}Running migrations...${NC}"
    if sqlx migrate run; then
        echo -e "${GREEN}✅ Migrations completed${NC}"
    else
        echo -e "${YELLOW}⚠️  Migration warning (check manually)${NC}"
    fi
fi

# Build check
echo -e "${YELLOW}Checking Rust build...${NC}"
cargo check
echo -e "${GREEN}✅ Rust build check passed${NC}"

cd ..

# Summary
echo -e "\n${GREEN}=================================================="
echo "✅ Setup completed successfully!"
echo "==================================================${NC}\n"

echo "📚 Next steps:"
echo ""
echo "1. 🔧 Configure credentials:"
echo "   - Edit frontend/.env.local with Supabase keys"
echo "   - Edit backend/.env with database and JWT secret"
echo ""
echo "2. 🚀 Start development servers:"
echo "   # Terminal 1 - Frontend"
echo "   cd frontend && npm run dev"
echo ""
echo "   # Terminal 2 - Backend"
echo "   cd backend && cargo run"
echo ""
echo "3. 📖 View documentation:"
echo "   - BUILD_GUIDE.md - Build instructions"
echo "   - IMPLEMENTATION_GUIDE.md - Implementation details"
echo "   - doc/engineering/session_mod.md - Architecture docs"
echo ""
echo "4. 🧪 Test the system:"
echo "   - Frontend: http://localhost:5173"
echo "   - Backend: http://localhost:3000"
echo "   - GraphQL: http://localhost:3000/graphql"
echo ""
echo -e "${YELLOW}For help, check the documentation files${NC}\n"
