#!/usr/bin/env bash
set -euo pipefail

# 🧪 Script de Teste - Validar Login com Senha Correta

echo "🔧 Testando endpoint de login..."
echo ""

# Cores
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Configuração
API_URL="${API_URL:-http://localhost:3000}"
EMAIL="${EMAIL:-user01@aevalo.dev}"
PASSWORD="${PASSWORD:-Password123!}"

echo "📍 Endpoint: $API_URL/auth/login"
echo "📧 Email: $EMAIL"
echo "🔐 Senha: $PASSWORD"
echo ""

# Função para testar login
test_login() {
    local email="$1"
    local password="$2"
    local expected_status="$3"
    
    echo "Testing: $email / $password..."
    
    response=$(curl -s -w "\n%{http_code}" -X POST "$API_URL/auth/login" \
        -H "Content-Type: application/json" \
        -d "{\"email\":\"$email\",\"password\":\"$password\"}")
    
    body=$(echo "$response" | head -n -1)
    status=$(echo "$response" | tail -n 1)
    
    echo "Status: $status"
    echo "Response:"
    echo "$body" | jq . 2>/dev/null || echo "$body"
    echo ""
    
    if [ "$status" = "$expected_status" ]; then
        echo -e "${GREEN}✅ Status esperado: $expected_status${NC}"
    else
        echo -e "${RED}❌ Status inesperado: $status (esperado $expected_status)${NC}"
    fi
    echo ""
}

# Teste 1: Credenciais corretas (200)
echo -e "${YELLOW}=== Teste 1: Credenciais Corretas ===${NC}"
test_login "$EMAIL" "$PASSWORD" "200"

# Teste 2: Senha incorreta (401)
echo -e "${YELLOW}=== Teste 2: Senha Incorreta ===${NC}"
test_login "$EMAIL" "WrongPassword123" "401"

# Teste 3: Email não existe (401)
echo -e "${YELLOW}=== Teste 3: Email Não Existe ===${NC}"
test_login "nonexistent@example.com" "$PASSWORD" "401"

# Teste 4: Email vazio (400)
echo -e "${YELLOW}=== Teste 4: Email Vazio ===${NC}"
test_login "" "$PASSWORD" "400"

# Teste 5: Senha vazia (400)
echo -e "${YELLOW}=== Teste 5: Senha Vazia ===${NC}"
test_login "$EMAIL" "" "400"

echo -e "${GREEN}✨ Testes completados!${NC}"
