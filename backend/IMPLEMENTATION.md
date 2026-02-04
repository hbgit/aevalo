# Implementação Backend do Fluxo de Avaliação - Aevalo

Código backend em Rust (Axum) baseado no Diagrama de Sequência: Fluxo Completo de Avaliação do arquivo `seq_diagram.md`.

## 📋 Estrutura dos Handlers

### 1. **handlers/evaluations.rs** - Gerenciamento de Avaliações
Implementa o ciclo de vida completo da avaliação.

#### Endpoints:
- `GET /evaluations` - Lista avaliações do usuário com contagem
  - Retorna: `is_first_time` flag para empty state
  - Verifica: `count(avaliações)` no banco

- `POST /evaluations` - Cria nova avaliação (Draft)
  - Corpo: `{title, description, scale_type, category_id}`
  - Retorna: Evaluation object

- `GET /evaluations/{id}` - Recupera avaliação com questões
  - Retorna: `EvaluationDetail` com lista de questões

- `PATCH /evaluations/{id}` - Atualiza avaliação
  - Validação: status deve ser `Draft`
  - Atualiza: title, description, questões

- `POST /evaluations/{id}/publish` - Publica avaliação
  - Insere questões no banco
  - Muda status para `Open`
  - Gera link público com UUID
  - Retorna: `PublishResponse` com link

- `POST /evaluations/{id}/close` - Fecha avaliação
  - Muda status para `Closed`
  - Desativa link público
  - Dispara processamento de analytics

**Fluxo no diagrama:**
```
Step 1.1-1.3 (Acessa Dashboard) → list_evaluations()
Step 3 (Escolhe criação) → create_evaluation()
Step 4 (Customiza) → update_evaluation()
Step 5 (Publica) → publish_evaluation() + gera link
Step 11 (Finaliza) → close_evaluation()
```

---

### 2. **handlers/public.rs** - Endpoints Públicos
Permite acesso não-autenticado aos avaliadores.

#### Endpoints:
- `GET /public/eval/{uuid}` - Recupera avaliação pública
  - Valida: UUID está ativo e avaliação está `Open`
  - Retorna: Apenas `title, description, questions`
  - Sem autenticação necessária

- `GET /public/eval/{uuid}/stats` - Estatísticas em tempo real
  - Conta: respostas recebidas
  - Calcula: taxa de resposta
  - Atualização em tempo real

**Fluxo no diagrama:**
```
Step 7 (Avaliador acessa link) → get_public_evaluation()
Step 10 (Owner monitora) → get_public_stats()
```

---

### 3. **handlers/responses.rs** - Coleta de Respostas
Gerencia submissão e armazenamento de respostas.

#### Endpoints:
- `POST /responses` - Submete respostas
  - Corpo: `{respondent_id, answers: [{question_id, answer_value}]}`
  - Validações: 
    - Avaliação está `Open`
    - Resposta valida por tipo de escala
  - Armazenamento: anonymizado com respondent_id
  - Retorna: confirmação + count de respostas

- `GET /evaluations/{id}/responses` - Lista todas as respostas (Owner)
  - Retorna: `ResponseDetail[]` com timestamps
  - Requer: autenticação + ownership

- `GET /evaluations/{id}/stats` - Estatísticas de resposta
  - Retorna: total_responses, total_questions, response_rate
  - Dashboard em tempo real

**Validações por escala:**
- `Likert`: 1-5 apenas
- `FixedSum`: soma = 100
- `PairedComparison`: matriz válida
- `Frequency`: valores válidos

**Fluxo no diagrama:**
```
Step 8-9 (Avaliador responde e submete) → submit_responses()
Step 9.1 (Backend valida por escala) → validate_answers()
Step 10.1 (Owner monitora) → get_response_stats()
```

---

### 4. **handlers/ai_generation.rs** - Geração com IA
Integração com Gemini API para gerar itens.

#### Endpoints:
- `POST /evaluations/generate` - Gera itens com IA
  - Corpo: `{description, scale_type?}`
  - Chamada: Gemini API com prompt estruturado
  - Retorna: `GeneratedItem[]` com metadata
  - Streaming de tokens possível

- `POST /evaluations/validate` - Valida estrutura
  - Corpo: `{items, scale_type}`
  - Validações:
    - Mínimo 1 item
    - Ordens únicas
    - Constraints por escala
  - Retorna: `ValidateResponse` com erros

**Estrutura do Prompt:**
```
Descrição da avaliação + Tipo de escala
↓
Gemini API retorna JSON com 5-10 itens
↓
Parse + adiciona metadata padrão se vazio
```

**Fluxo no diagrama:**
```
Step 3.3-3.4 (Caminho B: IA) → generate_items_ai()
Step 3.4 (Backend chama LLM) → call_gemini_api()
Step 6.1-6.2 (Valida estrutura) → validate_items()
```

---

### 5. **handlers/analytics.rs** - Processamento de Resultados
Engine para agregação e análise de respostas.

#### Endpoints:
- `POST /evaluations/{id}/process` - Processa analytics
  - Lê todas as respostas
  - Calcula por escala:
    - **Likert/Frequency**: média, mediana, desvio padrão, histograma
    - **FixedSum**: média ponderada, distribuição
    - **PairedComparison**: rank por vitórias
  - Detecção de outliers
  - Confidence score
  - Armazena em `analytics_results`

- `GET /evaluations/{id}/results` - Recupera resultados
  - Busca: dados calculados do `analytics_results`
  - Fallback: calcula on-demand se não existir
  - Retorna: métricas completas com gráficos

**Cálculos por Questão:**
```json
{
  "question_id": "...",
  "question_text": "...",
  "statistics": {
    "mean": 3.8,
    "median": 4.0,
    "std_dev": 1.2,
    "min": 1.0,
    "max": 5.0,
    "distribution": {"1": 2, "2": 5, "3": 8, "4": 10, "5": 5},
    "confidence_score": 0.95
  }
}
```

**Fluxo no diagrama:**
```
Step 11 (Finaliza avaliação) → close_evaluation() + dispara job
Step 11.2-11.3 (Analytics processa) → compute_analytics()
Step 12.1 (Carrega resultados) → get_results()
Step 12 (Visualiza dashboard) → Retorna métricas formatadas
```

---

### 6. **handlers/auth.rs** - Autenticação
Placeholder para JWT authentication.

#### Endpoints:
- `POST /auth/login` - Login de usuário
  - TODO: Implementar validação com bcrypt
  - TODO: Gerar JWT token

---

## 🗄️ Esquema de Banco de Dados Necessário

```sql
-- Tabelas principais
CREATE TABLE evaluations (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    category_id UUID,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status VARCHAR(20), -- Draft, Open, Closed, Archived
    scale_type VARCHAR(50), -- Likert, Frequency, PairedComparison, FixedSum
    created_at TIMESTAMP,
    updated_at TIMESTAMP,
    published_at TIMESTAMP,
    closed_at TIMESTAMP
);

CREATE TABLE questions (
    id UUID PRIMARY KEY,
    evaluation_id UUID NOT NULL,
    order INT NOT NULL,
    text TEXT NOT NULL,
    scale_type VARCHAR(50),
    metadata JSONB,
    UNIQUE(evaluation_id, order)
);

CREATE TABLE responses (
    id UUID PRIMARY KEY,
    question_id UUID NOT NULL,
    evaluation_id UUID NOT NULL,
    respondent_id VARCHAR(255), -- Anonymized/hashed IP
    answer_value JSONB,
    created_at TIMESTAMP
);

CREATE TABLE public_links (
    id UUID PRIMARY KEY,
    evaluation_id UUID NOT NULL,
    uuid VARCHAR(36) UNIQUE,
    short_url VARCHAR(50),
    is_active BOOLEAN DEFAULT true,
    created_at TIMESTAMP,
    expires_at TIMESTAMP
);

CREATE TABLE analytics_results (
    id UUID PRIMARY KEY,
    evaluation_id UUID NOT NULL UNIQUE,
    total_responses INT,
    response_rate FLOAT,
    metrics JSONB,
    generated_at TIMESTAMP
);
```

---

## 🔐 Fluxo de Segurança

1. **Endpoints autenticados**: Verificam JWT token
   - `GET /evaluations` - Valida user_id do token
   - `POST /evaluations/{id}` - Verifica ownership (user_id == token.user_id)

2. **Endpoints públicos**: Sem autenticação
   - `GET /public/eval/{uuid}` - Valida UUID na tabela public_links
   - Verifica status = "Open"

3. **RLS (Row Level Security)** recomendado no Supabase:
   ```sql
   ALTER TABLE evaluations ENABLE ROW LEVEL SECURITY;
   CREATE POLICY "Users can only see own evaluations"
   ON evaluations FOR SELECT
   USING (auth.uid() = user_id);
   ```

---

## 📊 Fluxo Completo de Exemplo

### Owner cria e publica:
```bash
# 1. Lista (primeiro acesso)
GET /evaluations
← {total: 0, is_first_time: true}

# 2. Cria draft
POST /evaluations
{title: "Team Performance", scale_type: "Likert"}
← {id: "eval-123", status: "Draft"}

# 3. Customiza
PATCH /evaluations/eval-123
{
  title: "Evaluate Team Performance Q1",
  items: [
    {order: 1, text: "Communication skills", metadata: {}},
    {order: 2, text: "Technical knowledge", metadata: {}}
  ]
}

# 4. Publica
POST /evaluations/eval-123/publish
{items: [...]} 
← {status: "Open", public_link: "/public/eval/uuid-xxx", short_url: "eval-abc123"}
```

### Avaliador responde:
```bash
# 5. Acessa link público
GET /public/eval/uuid-xxx
← {title: "...", questions: [...]}

# 6. Submete respostas
POST /responses
{
  respondent_id: "hash-ip-xxxx",
  answers: [
    {question_id: "q1", answer_value: 4},
    {question_id: "q2", answer_value: 5}
  ]
}
← {message: "✓ Obrigado", response_count: 1}
```

### Owner finaliza e visualiza:
```bash
# 7. Monitora progresso
GET /evaluations/eval-123/stats
← {total_responses: 5, response_rate: 100%}

# 8. Finaliza
POST /evaluations/eval-123/close
← {status: "closed", analytics processing started}

# 9. Visualiza resultados
GET /evaluations/eval-123/results
← {
  metrics: {
    by_question: [
      {
        question_id: "q1",
        statistics: {
          mean: 4.2,
          distribution: {1: 0, 2: 1, 3: 1, 4: 2, 5: 1}
        }
      }
    ]
  }
}
```

---

## 🚀 Próximos Passos

1. **Implementar JWT Authentication**
   - Extractor customizado para user_id
   - Middleware de autenticação

2. **Configurar Supabase RLS**
   - Row-level security policies
   - Realtime subscriptions para live updates

3. **Implementar Error Handlers**
   - Axum response converters
   - Logging centralizado

4. **Testes**
   - Unit tests para validações
   - Integration tests para fluxo completo
   - Load testing para múltiplos avaliadores

5. **Monitoring**
   - Prometheus metrics
   - Tracing distribuído
   - Alerting rules

6. **Otimizações**
   - Connection pooling
   - Caching de resultados
   - Índices de banco de dados
