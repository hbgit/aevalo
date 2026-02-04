# Backend - Implementação do Fluxo de Avaliação

Implementação completa em **Rust com Axum** baseada no "Diagrama de Sequência: Fluxo Completo de Avaliação" do arquivo `doc/engineering/uml/seq_diagram.md`.

## 📁 Estrutura de Arquivos

```
src/
├── main.rs                  # Servidor Axum com roteamento
├── error.rs                 # Tipos de erro customizados
├── models/                  # Structs de dados
│   └── mod.rs              # User, Evaluation, Question, Response, etc.
├── handlers/               # Endpoints da API
│   ├── mod.rs
│   ├── evaluations.rs      # CRUD de avaliações
│   ├── responses.rs        # Submissão de respostas
│   ├── public.rs           # Endpoints públicos (sem auth)
│   ├── ai_generation.rs    # Integração com Gemini API
│   ├── analytics.rs        # Processamento de resultados
│   └── auth.rs             # JWT authentication
├── modules/                # Lógica de negócio
└── db/                     # Conexão com banco de dados

IMPLEMENTATION.md           # Documentação técnica detalhada
test-api.sh                # Script de teste com curl
```

## 🚀 Fluxo de Avaliação Implementado

### Fase 1: Criação (Owner)
```
1️⃣ GET /evaluations
   └─ Valida first-time user com count
   
2️⃣ POST /evaluations
   └─ Cria avaliação em status Draft
   
3️⃣ PATCH /evaluations/{id}
   └─ Customiza título, descrição, questões
   
4️⃣ POST /evaluations/{id}/publish
   └─ Muda para Open, gera link público, retorna UUID
```

### Fase 2: Geração com IA (Optional)
```
🤖 POST /evaluations/generate
   ├─ Recebe descrição
   ├─ Chama Gemini API com prompt estruturado
   └─ Retorna 5-10 itens com metadata

✅ POST /evaluations/validate
   └─ Valida itens por tipo de escala
```

### Fase 3: Coleta de Respostas (Avaliadores)
```
🌐 GET /public/eval/{uuid}
   └─ Acessa avaliação pública (sem auth)
   
✍️ POST /responses
   ├─ Submete respostas (anonymizado)
   ├─ Valida por escala (Likert 1-5, FixedSum=100, etc)
   └─ Retorna confirmação
```

### Fase 4: Finalização (Owner)
```
🔒 POST /evaluations/{id}/close
   ├─ Muda status para Closed
   ├─ Desativa link público
   └─ Dispara processamento
   
⚙️ POST /evaluations/{id}/process
   ├─ Carrega todas respostas
   ├─ Calcula estatísticas por escala
   └─ Armazena em analytics_results
   
📊 GET /evaluations/{id}/results
   └─ Retorna métricas formatadas
```

## 🔧 Configuração

### Requisitos
- **Rust 1.70+**
- **PostgreSQL/Supabase**
- **Gemini API key** (para IA)

### Variáveis de Ambiente (.env)
```bash
DATABASE_URL=postgresql://user:password@localhost/aevalo
GEMINI_API_KEY=your-api-key-here
RUST_LOG=info
```

### Build
```bash
cargo build --release
```

### Run
```bash
cargo run
# Servidor inicia em http://0.0.0.0:3000
```

## 📡 Endpoints

### Avaliações (Autenticado)
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/evaluations` | Lista com count |
| POST | `/evaluations` | Cria draft |
| GET | `/evaluations/{id}` | Recupera com questões |
| PATCH | `/evaluations/{id}` | Atualiza draft |
| POST | `/evaluations/{id}/publish` | Publica + gera link |
| POST | `/evaluations/{id}/close` | Fecha |

### Respostas
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| POST | `/responses` | Submete respostas (público) |
| GET | `/evaluations/{id}/responses` | Lista respostas (owner) |
| GET | `/evaluations/{id}/stats` | Conta e taxa (owner) |

### IA & Validação
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| POST | `/evaluations/generate` | Gera com IA |
| POST | `/evaluations/validate` | Valida estrutura |

### Público (Sem Auth)
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| GET | `/public/eval/{uuid}` | Acessa avaliação |
| GET | `/public/eval/{uuid}/stats` | Estatísticas real-time |

### Analytics
| Método | Endpoint | Descrição |
|--------|----------|-----------|
| POST | `/evaluations/{id}/process` | Processa analytics |
| GET | `/evaluations/{id}/results` | Retorna resultados |

## 🧪 Testes

### Script Automático
```bash
chmod +x test-api.sh
./test-api.sh
```

Executa todo o fluxo automaticamente com curl:
1. Cria avaliação
2. Customiza
3. Publica
4. Acessa como público
5. Submete respostas
6. Fecha e processa
7. Visualiza resultados

### Teste Manual (curl)
```bash
# 1. Criar avaliação
curl -X POST http://localhost:3000/evaluations \
  -H "Content-Type: application/json" \
  -d '{"title":"Test", "scale_type":"Likert"}'

# 2. Publicar
curl -X POST http://localhost:3000/evaluations/{id}/publish \
  -H "Content-Type: application/json" \
  -d '{"items":[{"order":1,"text":"Q1","metadata":{}}]}'

# 3. Acessar publicamente
curl http://localhost:3000/public/eval/{uuid}

# 4. Submeter respostas
curl -X POST http://localhost:3000/responses \
  -H "Content-Type: application/json" \
  -d '{
    "respondent_id":"hash-123",
    "answers":[{"question_id":"q1","answer_value":4}]
  }'
```

## 📊 Validações por Escala

**Likert (1-5)**
```rust
if value < 1 || value > 5 { error!() }
```

**FixedSum (=100)**
```rust
if answers.sum() != 100 { error!() }
```

**PairedComparison**
```rust
// Matriz simétrica e sem autoconsistência
validate_pair_matrix()
```

**Frequency**
```rust
// Categorias válidas do metadata
validate_categories()
```

## 🔐 Segurança

### Autenticação
- JWT token no header `Authorization: Bearer {token}`
- Extrai `user_id` do token

### Autorização
- Endpoints de escrita: Verificam `user_id == token.user_id`
- Endpoints de leitura: Verificam ownership via RLS

### Publicamente Acessível
- Validação de UUID na tabela `public_links`
- Verifica `status = "Open"`
- Anonymização de respondents com hash IP

## 📈 Analytics - Cálculos

### Likert/Frequency
```
Mean: Σ valores / N
Median: valor central dos valores ordenados
StdDev: √(Σ(x-mean)²/N)
Distribution: histograma com contagens
```

### FixedSum
```
Mean Ponderada: Σ(valor × peso) / Σ pesos
Distribuição: padrões de alocação
```

### PairedComparison
```
Ranking: quantidade de vitórias por item
Win Rate: vitórias / comparações totais
```

## 🔌 Integração Gemini API

### Prompt Estruturado
```
Descrição + Tipo de Escala
→ Gemini gera JSON com 5-10 itens
→ Parse e adiciona metadata padrão
```

### Exemplo de Resposta
```json
[
  {
    "order": 1,
    "text": "Communication skills are effective",
    "metadata": {
      "min_value": 1,
      "max_value": 5,
      "labels": ["Strongly Disagree", ..., "Strongly Agree"]
    }
  }
]
```

## 🗄️ Banco de Dados

Consulte [IMPLEMENTATION.md](./IMPLEMENTATION.md) para esquema completo.

Tabelas principais:
- `evaluations` - Avaliações e metadados
- `questions` - Questões com ordem e metadata
- `responses` - Respostas anônimas
- `public_links` - UUIDs para acesso público
- `analytics_results` - Resultados agregados

## 🚨 Tratamento de Erros

```rust
// Implementado em error.rs
enum AppError {
    DatabaseError,
    ValidationError,
    NotFound,
    AuthError,
    InternalServerError,
}

// HTTP Status Codes
400 - ValidationError
401 - AuthError
404 - NotFound
500 - DatabaseError / InternalServerError
```

## 📋 TODO - Próximas Implementações

- [ ] JWT token extractor
- [ ] RLS policies no Supabase
- [ ] Prometheus metrics
- [ ] Connection pooling otimizado
- [ ] Caching de resultados
- [ ] Supabase Realtime para live updates
- [ ] Unit tests
- [ ] Integration tests
- [ ] Load testing
- [ ] Documentação OpenAPI/Swagger

## 📚 Documentação Adicional

- [IMPLEMENTATION.md](./IMPLEMENTATION.md) - Detalhes técnicos completos
- [../doc/engineering/uml/seq_diagram.md](../doc/engineering/uml/seq_diagram.md) - Diagramas UML
- [Cargo.toml](./Cargo.toml) - Dependências

## ❓ FAQ

**P: Como adiciono autenticação real?**
A: Implemente JWT extractor em `handlers/auth.rs` e adicione middleware nos endpoints protegidos.

**P: Posso usar outro LLM além de Gemini?**
A: Sim! Modifique `ai_generation.rs` para chamar sua API preferida (OpenAI, Claude, etc).

**P: Como escalo para muitos respondentes?**
A: Use índices no banco, caching de resultados, e considere worker jobs assíncronos para analytics.

**P: Posso estender com mais escalas?**
A: Sim! Adicione no enum `ScaleType` em `models/mod.rs` e implemente validação em `responses.rs`.
