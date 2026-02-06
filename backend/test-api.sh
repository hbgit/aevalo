#!/bin/bash
# Test script for Aevalo Backend API
# Implementação do Diagrama de Sequência: Fluxo Completo de Avaliação

BASE_URL="http://localhost:3000"
USER_ID="550e8400-e29b-41d4-a716-446655440000"
EVAL_ID=""
EVAL_UUID=""
JWT_TOKEN=""
REFRESH_TOKEN=""

echo "🚀 Aevalo Backend API - Test Script"
echo "===================================="
echo ""

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m'

# ==================== STEP 0: Login ====================
echo -e "${BLUE}🔐 STEP 0: Login to get JWT token${NC}"
echo "POST /auth/login"
LOGIN_RESPONSE=$(curl -X POST "${BASE_URL}/auth/login" \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user 1@aevalo.dev",
    "password": "Password123!"
  }' \
  -s)
echo "$LOGIN_RESPONSE" | jq '.'
JWT_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.token // empty' 2>/dev/null)
REFRESH_TOKEN=$(echo "$LOGIN_RESPONSE" | jq -r '.refresh_token // empty' 2>/dev/null)

if [ -z "$JWT_TOKEN" ]; then
  ERROR_MSG=$(echo "$LOGIN_RESPONSE" | jq -r '.error // .message // "Unknown error"' 2>/dev/null)
  echo -e "${YELLOW}⚠️  Login failed: $ERROR_MSG${NC}"
  echo "Full response:"
  echo "$LOGIN_RESPONSE" | jq '.'
  exit 1
fi

echo -e "${GREEN}✓ Login successful${NC}"
echo "Token: ${JWT_TOKEN:0:50}..."
echo ""

# ==================== STEP 1: List Evaluations ====================
echo -e "${BLUE}📋 STEP 1: List evaluations (Dashboard)${NC}"
echo "GET /evaluations"
curl -X GET "${BASE_URL}/evaluations" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

# ==================== STEP 2: Create Evaluation ====================
echo -e "${BLUE}✏️  STEP 2: Create new evaluation (Draft)${NC}"
echo "POST /evaluations"
RESPONSE=$(curl -X POST "${BASE_URL}/evaluations" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -d '{
    "title": "Team Performance Q1 2025",
    "description": "Evaluate team member performance for Q1",
    "scale_type": "Likert",
    "category_id": null
  }' \
  -s)
echo "$RESPONSE" | jq '.'
EVAL_ID=$(echo "$RESPONSE" | jq -r '.id')
echo "Saved EVAL_ID: $EVAL_ID"
echo ""

# ==================== STEP 3: Get Evaluation ====================
echo -e "${BLUE}🔍 STEP 3: Retrieve evaluation details${NC}"
echo "GET /evaluations/{id}"
curl -X GET "${BASE_URL}/evaluations/${EVAL_ID}" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

# ==================== STEP 4: Generate Items with AI ====================
echo -e "${BLUE}🤖 STEP 4: Generate items using AI (Alternative)${NC}"
echo "POST /evaluations/generate"
AI_RESPONSE=$(curl -X POST "${BASE_URL}/evaluations/generate" \
  -H "Content-Type: application/json" \
  -d '{
    "description": "Evaluate team communication, technical skills, and collaboration abilities",
    "scale_type": "Likert"
  }' \
  -s)
echo "$AI_RESPONSE" | jq '.'
echo ""

# ==================== STEP 5: Validate Items ====================
echo -e "${BLUE}✅ STEP 5: Validate generated items${NC}"
echo "POST /evaluations/validate"
curl -X POST "${BASE_URL}/evaluations/validate" \
  -H "Content-Type: application/json" \
  -d '{
    "items": [
      {
        "order": 1,
        "text": "Communication skills are clear and effective",
        "metadata": {}
      },
      {
        "order": 2,
        "text": "Technical knowledge meets role requirements",
        "metadata": {}
      },
      {
        "order": 3,
        "text": "Collaboration with team members is strong",
        "metadata": {}
      }
    ],
    "scale_type": "Likert"
  }' \
  -s | jq '.'
echo ""

# ==================== STEP 6: Update Evaluation ====================
echo -e "${BLUE}📝 STEP 6: Customize evaluation${NC}"
echo "PATCH /evaluations/{id}"
curl -X PATCH "${BASE_URL}/evaluations/${EVAL_ID}" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -d '{
    "title": "Team Performance Q1 2025 - Updated",
    "description": "Updated evaluation",
    "items": [
      {"order": 1, "text": "Communication skills", "metadata": {}},
      {"order": 2, "text": "Technical knowledge", "metadata": {}},
      {"order": 3, "text": "Collaboration", "metadata": {}}
    ]
  }' \
  -s | jq '.'
echo ""

# ==================== STEP 7: Publish Evaluation ====================
echo -e "${BLUE}🚀 STEP 7: Publish evaluation and generate public link${NC}"
echo "POST /evaluations/{id}/publish"
PUBLISH_RESPONSE=$(curl -X POST "${BASE_URL}/evaluations/${EVAL_ID}/publish" \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -d '{
    "items": [
      {"order": 1, "text": "Communication skills", "metadata": {}},
      {"order": 2, "text": "Technical knowledge", "metadata": {}},
      {"order": 3, "text": "Collaboration", "metadata": {}}
    ]
  }' \
  -s)
echo "$PUBLISH_RESPONSE" | jq '.'
EVAL_UUID=$(echo "$PUBLISH_RESPONSE" | jq -r '.public_link' | grep -oP '(?<=/public/eval/)[^/]+')
SHORT_URL=$(echo "$PUBLISH_RESPONSE" | jq -r '.short_url')
echo "Saved PUBLIC_UUID: $EVAL_UUID"
echo "Short URL: $SHORT_URL"
echo ""

# ==================== STEP 8: Get Public Evaluation ====================
echo -e "${BLUE}🌐 STEP 8: Access evaluation via public link (Evaluator)${NC}"
echo "GET /public/eval/{uuid}"
curl -X GET "${BASE_URL}/public/eval/${EVAL_UUID}" \
  -H "Content-Type: application/json" \
  -s | jq '.'
echo ""

# ==================== STEP 9: Get Public Stats ====================
echo -e "${BLUE}📊 STEP 9: Check response statistics (Real-time)${NC}"
echo "GET /public/eval/{uuid}/stats"
curl -X GET "${BASE_URL}/public/eval/${EVAL_UUID}/stats" \
  -s | jq '.'
echo ""

# ==================== STEP 10: Submit Responses ====================
echo -e "${BLUE}✍️  STEP 10: Submit responses (Evaluator)${NC}"
echo "POST /responses"

# Get question IDs (simulated for demo)
Q1_ID="550e8400-e29b-41d4-a716-446655440001"
Q2_ID="550e8400-e29b-41d4-a716-446655440002"
Q3_ID="550e8400-e29b-41d4-a716-446655440003"

curl -X POST "${BASE_URL}/responses" \
  -H "Content-Type: application/json" \
  -d "{
    \"respondent_id\": \"respondent-hash-12345\",
    \"answers\": [
      {\"question_id\": \"${Q1_ID}\", \"answer_value\": 4},
      {\"question_id\": \"${Q2_ID}\", \"answer_value\": 5},
      {\"question_id\": \"${Q3_ID}\", \"answer_value\": 4}
    ]
  }" \
  -s | jq '.'
echo ""

# ==================== STEP 11: Get Response Stats ====================
echo -e "${BLUE}📈 STEP 11: Monitor response progress${NC}"
echo "GET /evaluations/{id}/stats"
curl -X GET "${BASE_URL}/evaluations/${EVAL_ID}/stats" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

# ==================== STEP 12: Close Evaluation ====================
echo -e "${BLUE}🔒 STEP 12: Close evaluation${NC}"
echo "POST /evaluations/{id}/close"
curl -X POST "${BASE_URL}/evaluations/${EVAL_ID}/close" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

# ==================== STEP 13: Process Analytics ====================
echo -e "${BLUE}⚙️  STEP 13: Process analytics${NC}"
echo "POST /evaluations/{id}/process"
curl -X POST "${BASE_URL}/evaluations/${EVAL_ID}/process" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

# ==================== STEP 14: Get Results ====================
echo -e "${BLUE}📊 STEP 14: View final results${NC}"
echo "GET /evaluations/{id}/results"
curl -X GET "${BASE_URL}/evaluations/${EVAL_ID}/results" \
  -H "Authorization: Bearer ${JWT_TOKEN}" \
  -s | jq '.'
echo ""

echo -e "${GREEN}✅ Test script completed!${NC}"
