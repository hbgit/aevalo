# Diagramas de Sequência UML - Aevalo

Documentação visual dos fluxos do sistema Aevalo usando diagramas UML.

---

## 🔄 Diagrama de Sequência: Fluxo Completo de Avaliação

```mermaid
sequenceDiagram
    actor User as 👤 Usuário<br/>(Owner)
    participant Dashboard as 🖥️ Dashboard
    participant DB as 🗄️ Supabase<br/>(DB)
    participant LLM as 🤖 Gemini API<br/>(LLM)
    participant LinkGen as 🔗 Link Generator
    actor Evaluators as 👥 Avaliadores<br/>(Colaboradores)
    participant Analytics as 📊 Analytics Engine

    User->>Dashboard: 1. Acessa Dashboard
    Dashboard->>DB: 1.1 Valida contagem de avaliações
    DB-->>Dashboard: n = count(avaliações)
    
    alt Primeira Vez (n == 0)
        Dashboard->>Dashboard: 1.2 Exibe Empty State CTA
        Dashboard-->>User: Botão "Criar Primeira Avaliação"
    else Retorno (n > 0)
        Dashboard->>DB: 1.3 Carrega lista de avaliações
        DB-->>Dashboard: Lista + gráficos por categoria
        Dashboard-->>User: Dashboard com histórico
    end
    
    User->>Dashboard: 2. Clica "Criar Avaliação"
    Dashboard-->>User: Exibe diálogo de criação
    
    User->>Dashboard: 3. Escolhe método de criação
    
    alt Caminho A: Template Curado
        User->>Dashboard: 3.1 Seleciona template pré-existente
        Dashboard->>DB: 3.2 Carrega estrutura do template
        DB-->>Dashboard: Template JSON (Likert/Fixed Sum/etc)
        Dashboard-->>User: Preview com questões
    else Caminho B: Assistido por IA
        User->>Dashboard: 3.3 Insere descrição da avaliação
        Dashboard-->>User: "Gerando itens com IA..."
        Dashboard->>LLM: 3.4 Prompt estruturado com contexto
        LLM-->>Dashboard: JSON com itens gerados
        Dashboard-->>User: Preview com itens IA
    end
    
    User->>Dashboard: 4. Customiza avaliação<br/>(se necessário)
    Dashboard-->>User: Editor inline com validações
    
    alt Escala Tipo
        note right of Dashboard: Validação por tipo
        Dashboard->>Dashboard: Likert: 1-5 apenas
        Dashboard->>Dashboard: Fixed Sum: somatório = 100
        Dashboard->>Dashboard: Paired Comparison: matriz validada
    end
    
    User->>Dashboard: 5. Publica avaliação
    Dashboard->>DB: 5.1 Salva avaliação (status: 'open')
    DB-->>Dashboard: ✓ ID_eval criado
    Dashboard->>LinkGen: 5.2 Gera link público
    LinkGen-->>Dashboard: UUID único com short-url
    Dashboard-->>User: Link copiar & compartilhar
    
    User->>Dashboard: 6. Seleciona "Compartilhar"
    Dashboard-->>User: 📋 Link público + preview
    User->>User: Compartilha link com avaliadores
    
    Evaluators->>Dashboard: 7. Acessa link público
    Dashboard->>DB: 7.1 Valida UUID e status='open'
    DB-->>Dashboard: ✓ Avaliação acessível
    Dashboard-->>Evaluators: Exibe questões
    
    loop Para cada avaliador
        Evaluators->>Dashboard: 8. Responde questões
        Dashboard->>Dashboard: Valida respostas por escala
        Evaluators->>Dashboard: 9. Submete respostas
        Dashboard->>DB: 9.1 Salva response (anonymizado)
        DB-->>Dashboard: ✓ Response registrada
        Dashboard-->>Evaluators: ✓ Obrigado por responder
    end
    
    User->>Dashboard: 10. Monitora progresso
    Dashboard->>DB: 10.1 Query responses count
    DB-->>Dashboard: Estatísticas em tempo real
    Dashboard-->>User: Progress bar + respostas
    
    User->>Dashboard: 11. Finaliza avaliação
    Dashboard->>DB: 11.1 ALTER status = 'closed'
    DB-->>Dashboard: ✓ Link desabilitado
    Dashboard->>Analytics: 11.2 Dispara processamento
    
    Analytics->>DB: 11.3 Agregação de respostas
    Analytics->>Analytics: Cálculos por escala
    Analytics-->>DB: ✓ Resultados processados
    
    Dashboard-->>User: 📊 Relatório gerado
    User->>Dashboard: 12. Visualiza resultados
    Dashboard->>DB: 12.1 Carrega resultados agregados
    DB-->>Dashboard: Métricas + gráficos
    Dashboard-->>User: Dashboard analytics
    
    User->>Dashboard: 13. Exporta/Arquiva
    Dashboard->>DB: Status = 'archived'
    Dashboard-->>User: ✓ Avaliação encerrada
```

---

## 🎯 Diagrama de Sequência: Fluxo Alternativo - Criação via IA (Detalhado)

```mermaid
sequenceDiagram
    actor User as 👤 Usuário
    participant UI as 🖥️ Frontend Vue
    participant Backend as ⚡ Rust API
    participant DB as 🗄️ Supabase
    participant LLM as 🤖 Gemini API
    participant Monitor as 📊 Prometheus

    User->>UI: 1. Descreve avaliação<br/>"Avaliar desempenho da equipe"
    UI->>UI: 1.1 Valida input
    UI-->>User: Loading indicator ⏳

    UI->>Backend: 2. POST /evaluations/generate<br/>(user_id, description)
    Backend->>Monitor: 2.1 Registra início (latência)
    
    Backend->>Backend: 2.2 Estrutura prompt
    note right of Backend: Contexto + constraints<br/>Seleciona escala ótima
    
    Backend->>LLM: 2.3 Stream POST /generateContent<br/>(prompt estruturado)
    LLM-->>Backend: 3. Streama tokens
    Backend->>Monitor: 3.1 Métrica: latência_llm
    
    Backend-->>UI: 3.2 SSE stream items
    UI->>UI: 3.3 Renderiza items em tempo real
    UI-->>User: "Gerando..." + progresso
    
    LLM->>LLM: 4. Completa geração
    LLM-->>Backend: JSON com 5-10 items + metadata
    
    Backend->>Backend: 4.1 Valida schema
    Backend->>Backend: 4.2 Mapeia para struct Rust
    Backend->>Monitor: 4.3 Métrica: items_gerados
    
    Backend-->>UI: 5. ✓ Geração completa
    UI->>UI: 5.1 Popula form com items
    UI-->>User: Editor para customizar
    
    User->>UI: 6. Edita/Valida items
    UI->>Backend: 6.1 POST /evaluations/validate<br/>(items, scale_type)
    
    Backend->>Backend: 6.2 Validações por escala
    alt Likert (1-5)
        Backend->>Backend: ✓ Validação simples
    else Fixed Sum
        Backend->>Backend: Calcula somatório
        Backend->>Backend: Alerta se ≠ 100
    else Paired Comparison
        Backend->>Backend: Valida matriz simétrica
    end
    
    Backend-->>UI: 7. Resultado validação
    alt Válido
        UI-->>User: ✓ Pronto para publicar
    else Inválido
        UI-->>User: ❌ Erros encontrados
        User->>UI: Corrige
        UI->>Backend: Revalida
    end
    
    User->>UI: 7. Clica "Publicar"
    UI->>Backend: 7.1 POST /evaluations<br/>(items, metadata)
    Backend->>DB: 7.2 INSERT avaliação<br/>(status='open')
    DB-->>Backend: ✓ ID_eval + timestamp
    
    Backend->>Monitor: 7.3 Registra métrica:<br/>evaluations_created
    Backend-->>UI: 8. Response com ID
    
    UI->>UI: 8.1 Gera link público
    UI-->>User: ✓ Link pronto para compartilhar
```

---

## 📈 Diagrama de Sequência: Coleta de Respostas (Multi-usuário)

```mermaid
sequenceDiagram
    participant LinkSystem as 🔗 Link Public<br/>Generator
    actor Eval1 as 👤 Avaliador 1
    actor Eval2 as 👤 Avaliador 2
    participant UI as 🖥️ Frontend Vue
    participant Backend as ⚡ Rust API
    participant DB as 🗄️ Supabase
    participant Realtime as 🔄 Realtime<br/>(Supabase)
    participant Owner as 👥 Owner<br/>(Dashboard)

    LinkSystem-->>Eval1: Compartilha link público
    LinkSystem-->>Eval2: Compartilha link público
    
    Eval1->>UI: 1. Acessa link UUID
    UI->>Backend: 1.1 GET /public/eval/{uuid}
    Backend->>DB: 1.2 SELECT * WHERE uuid=?
    DB-->>Backend: ✓ Avaliação (status='open')
    Backend-->>UI: 1.3 Retorna questões
    UI-->>Eval1: Exibe escala interativa
    
    Eval2->>UI: 2. Acessa link UUID (paralelo)
    UI->>Backend: 2.1 GET /public/eval/{uuid}
    Backend->>DB: 2.2 SELECT * (query paralelo)
    DB-->>Backend: ✓ Mesma avaliação
    Backend-->>UI: 2.3 Retorna questões
    UI-->>Eval2: Exibe escala interativa
    
    par Respostas Simultâneas
        Eval1->>UI: 3A. Responde Q1-Q5
        Eval2->>UI: 3B. Responde Q1-Q5
        
        UI->>Backend: 3A.1 POST /responses<br/>(eval_id, answers, ip_hash)
        UI->>Backend: 3B.1 POST /responses<br/>(eval_id, answers, ip_hash)
        
        Backend->>Backend: 3A.2 / 3B.2<br/>Valida por escala
        Backend->>DB: 3A.3 INSERT response
        Backend->>DB: 3B.3 INSERT response
        DB-->>Backend: ✓ Response 1 OK
        DB-->>Backend: ✓ Response 2 OK
        
        Backend->>Realtime: 3A.4 / 3B.4<br/>Emite eventos
        Realtime->>Owner: Notifica update
    end
    
    Backend-->>UI: 4A/4B ✓ Respostas salvas
    UI-->>Eval1: Mensagem de sucesso
    UI-->>Eval2: Mensagem de sucesso
    
    Owner->>UI: 5. Monitora dashboard
    UI->>Backend: 5.1 GET /evaluations/{id}/stats
    Backend->>DB: 5.2 SELECT COUNT(responses)
    DB-->>Backend: responses: 2/n
    Backend-->>UI: 5.3 Retorna estatísticas
    UI-->>Owner: Atualiza em tempo real
    
    Realtime-->>Owner: 5.4 Notificação<br/>Nova resposta recebida
```

---

## 🔐 Diagrama de Sequência: Segurança e Controle de Acesso

```mermaid
sequenceDiagram
    actor User as 👤 Usuário
    participant UI as 🖥️ Frontend
    participant Auth as 🔐 Auth Module<br/>(JWT)
    participant Backend as ⚡ Rust API
    participant DB as 🗄️ Supabase<br/>(RLS)
    participant Logger as 📋 Audit Logger

    User->>Auth: 1. Login (email/password)
    Auth->>Backend: 1.1 POST /auth/login
    Backend->>DB: 1.2 SELECT user WHERE email=?
    DB-->>Backend: user_id + password_hash
    Backend->>Backend: 1.3 Valida bcrypt
    Backend->>Auth: 1.4 Gera JWT (user_id)
    Auth->>Logger: 1.5 Log: login_success
    Auth-->>UI: 2. Retorna token + refresh
    UI->>UI: 2.1 Armazena token (localStorage)
    UI-->>User: ✓ Autenticado

    User->>UI: 3. Acessa avaliação (ID: 456)
    UI->>Backend: 3.1 GET /evaluations/456<br/>(Authorization: JWT)
    Backend->>Backend: 3.2 Extrai user_id do JWT
    Backend->>DB: 3.3 SELECT * WHERE id=456<br/>AND user_id=? (RLS)
    
    alt Proprietário
        DB-->>Backend: ✓ Dados completos (editable)
        Backend->>Logger: 3.4 Log: access_owner
        Backend-->>UI: 3.5 Retorna full data
        UI-->>User: Exibe modo edição
    else Colaborador
        DB-->>Backend: ✓ Dados parciais (readonly)
        Backend->>Logger: 3.6 Log: access_collaborator
        Backend-->>UI: 3.7 Retorna data restrita
        UI-->>User: Exibe modo leitura
    else Sem Permissão
        DB-->>Backend: ✗ Query retorna vazio
        Backend->>Logger: 3.8 Log: access_denied
        Backend-->>UI: 401 Unauthorized
        UI-->>User: ❌ Acesso negado
    end

    User->>UI: 4. Tenta ação (editar/excluir)
    UI->>Backend: 4.1 PATCH /evaluations/456
    Backend->>Backend: 4.2 Verifica role (owner/collab/evaluator)
    
    alt Owner
        Backend->>DB: 4.3 UPDATE avaliação
        DB-->>Backend: ✓ Updated
        Backend->>Logger: 4.4 Log: edit_survey (user_id, changes)
    else Não-Owner
        Backend->>Logger: 4.5 Log: unauthorized_edit_attempt
        Backend-->>UI: 403 Forbidden
        UI-->>User: ❌ Sem permissão
    end
```

---

## 📊 Diagrama de Sequência: Processamento de Resultados

```mermaid
sequenceDiagram
    actor Owner as 👤 Owner
    participant UI as 🖥️ Dashboard
    participant Backend as ⚡ Rust API
    participant DB as 🗄️ Supabase
    participant Analytics as 📊 Analytics<br/>Engine
    participant Monitor as 📊 Prometheus

    Owner->>UI: 1. Clica "Finalizar Avaliação"
    UI->>Backend: 1.1 POST /evaluations/{id}/close
    Backend->>DB: 1.2 UPDATE status='closed'
    DB-->>Backend: ✓ Status updated

    Backend->>Analytics: 2. Dispara job<br/>(eval_id, responses[])
    Backend->>Monitor: 2.1 Métrica: evaluation_closed

    Analytics->>DB: 3. Carrega todas as responses
    DB-->>Analytics: responses[] (anonymizado)

    Analytics->>Analytics: 4. Processa por escala_tipo

    alt Likert/Frequency
        Analytics->>Analytics: 4A. Calcula média, mediana, desvio
        Analytics->>Analytics: Gera histograma
    else Paired Comparison
        Analytics->>Analytics: 4B. Rank itens por vitórias
        Analytics->>Analytics: Calcula força relativa
    else Fixed Sum
        Analytics->>Analytics: 4C. Média ponderada
        Analytics->>Analytics: Identifica padrões de alocação
    end

    Analytics->>Analytics: 5. Validações de qualidade
    Analytics->>Analytics: Outlier detection
    Analytics->>Analytics: Confidence score

    Analytics->>DB: 6. INSERT resultados_agregados
    DB-->>Analytics: ✓ Results saved
    Analytics->>Monitor: 6.1 Métrica: processing_duration_ms

    Analytics-->>Backend: 7. ✓ Processamento completo
    Backend->>Monitor: 7.1 Métrica: results_ready
    Backend-->>UI: 8. Notifica front

    UI-->>Owner: 8.1 "Resultados prontos"
    Owner->>UI: 9. Clica "Ver Resultados"
    
    UI->>Backend: 9.1 GET /evaluations/{id}/results
    Backend->>DB: 9.2 SELECT resultados_agregados
    DB-->>Backend: Estatísticas + insights
    Backend-->>UI: 9.3 Retorna JSON
    UI->>UI: 9.4 Renderiza gráficos + tabelas
    UI-->>Owner: 📊 Dashboard analytics
```

---

## Legenda de Componentes

| Símbolo | Componente | Descrição |
|---------|-----------|-----------|
| 👤 | Usuário/Owner | Criador da avaliação |
| 👥 | Avaliadores | Colaboradores respondendo |
| 🖥️ | Frontend Vue | Interface React com Vite |
| ⚡ | Rust API | Backend com Axum/Actix |
| 🗄️ | Supabase | PostgreSQL + Auth + RLS |
| 🤖 | Gemini API | LLM para geração de itens |
| 🔗 | Link Generator | UUID + short URLs |
| 🔐 | Auth Module | JWT + Segurança |
| 📊 | Analytics | Processamento de resultados |
| 📊 | Prometheus | Métricas e monitoramento |
| 🔄 | Realtime | Supabase Realtime |

---

**Nota:** Todos os diagramas são gerados em Mermaid e podem ser editados/expandidos conforme necessário durante o desenvolvimento.
