# 📦 Diagrama de Classes UML - Aevalo

Documentação da arquitetura de classes do sistema Aevalo usando UML.

---

## 🏗️ Diagrama de Classes: Modelo de Dados Completo

```mermaid
classDiagram
    %% ==================== CORE ENTITIES ====================
    
    class User {
        -id: UUID
        -email: String
        -password_hash: String
        -name: String
        -created_at: DateTime
        -updated_at: DateTime
        +login() void
        +logout() void
        +getEvaluations() List~Evaluation~
        +createEvaluation() Evaluation
        +getCategories() List~Category~
    }
    
    class Category {
        -id: UUID
        -user_id: UUID
        -name: String
        -description: String
        -color: String
        -created_at: DateTime
        +addEvaluation() void
        +removeEvaluation() void
        +getEvaluations() List~Evaluation~
    }
    
    class Evaluation {
        -id: UUID
        -user_id: UUID
        -category_id: UUID
        -title: String
        -description: String
        -status: EvaluationStatus
        -scale_type: ScaleType
        -created_at: DateTime
        -updated_at: DateTime
        -published_at: DateTime
        -closed_at: DateTime
        +publish() void
        +close() void
        +addQuestion() void
        +removeQuestion() void
        +getQuestions() List~Question~
        +generatePublicLink() PublicLink
        +getResponses() List~Response~
        +calculateResults() AnalyticsResult
    }
    
    class PublicLink {
        -id: UUID
        -evaluation_id: UUID
        -uuid: String
        -short_url: String
        -created_at: DateTime
        -expires_at: DateTime
        -is_active: Boolean
        +isValid() Boolean
        +deactivate() void
    }
    
    class Question {
        -id: UUID
        -evaluation_id: UUID
        -order: Integer
        -text: String
        -scale_type: ScaleType
        -metadata: JSON
        +moveUp() void
        +moveDown() void
        +getResponses() List~Response~
    }
    
    class Response {
        -id: UUID
        -question_id: UUID
        -evaluation_id: UUID
        -respondent_id: String (anon)
        -answer_value: JSON
        -created_at: DateTime
        +validate() Boolean
    }
    
    %% ==================== SCALE TYPES (INHERITANCE) ====================
    
    class Scale {
        <<abstract>>
        -id: UUID
        -question_id: UUID
        +validate(answer: JSON) Boolean*
        +calculateStatistics(responses: List~Response~) Statistics*
    }
    
    class LikertScale {
        -min_value: Integer
        -max_value: Integer
        -labels: List~String~
        +validate(answer: Integer) Boolean
        +calculateStatistics() Statistics
        +calculateMean() Float
        +calculateMedian() Float
        +calculateStdDev() Float
    }
    
    class FrequencyScale {
        -categories: List~String~
        -frequency_type: String
        +validate(answer: String) Boolean
        +calculateStatistics() Statistics
        +calculateDistribution() Map~String,Integer~
    }
    
    class PairedComparisonScale {
        -items: List~String~
        -comparison_matrix: Matrix
        +validate(answer: Pair) Boolean
        +calculateStatistics() Statistics
        +rankItems() List~String~
        +calculateStrength() Map~String,Float~
    }
    
    class FixedSumScale {
        -total_sum: Integer
        -items: List~String~
        +validate(answer: Map) Boolean
        +calculateStatistics() Statistics
        +calculateMean() Map~String,Float~
        +calculateVariance() Map~String,Float~
    }
    
    %% ==================== TEMPLATES ====================
    
    class Template {
        -id: UUID
        -name: String
        -description: String
        -scale_type: ScaleType
        -structure: JSON
        -created_at: DateTime
        +toEvaluation() Evaluation
        +duplicate() Template
        +getQuestions() List~Question~
    }
    
    %% ==================== COLLABORATION ====================
    
    class Collaborator {
        -id: UUID
        -evaluation_id: UUID
        -user_id: UUID
        -role: String
        -added_at: DateTime
        +updateRole() void
        +remove() void
    }
    
    %% ==================== RESULTS & ANALYTICS ====================
    
    class AnalyticsResult {
        -id: UUID
        -evaluation_id: UUID
        -total_responses: Integer
        -response_rate: Float
        -generated_at: DateTime
        -metrics: Map~String,Float~
        -insights: String
        +getMetrics() Map~String,Float~
        +getInsights() String
        +exportPDF() File
        +exportCSV() File
    }
    
    class Statistics {
        -mean: Float
        -median: Float
        -std_dev: Float
        -min: Float
        -max: Float
        -distribution: Map~String,Integer~
        -confidence_score: Float
        +calculateOutliers() List~Float~
        +getQualityScore() Float
    }
    
    %% ==================== ENUMERATIONS (Reference) ====================
    %% EvaluationStatus: DRAFT | OPEN | CLOSED | ARCHIVED
    %% ScaleType: LIKERT | FREQUENCY | PAIRED_COMPARISON | FIXED_SUM
    %% CollaboratorRole: OWNER | EDITOR | VIEWER
    
    %% ==================== RELATIONSHIPS ====================
    
    User "1" --> "*" Evaluation : creates
    User "1" --> "*" Category : owns
    Category "1" --> "*" Evaluation : contains
    Evaluation "1" --> "0..*" PublicLink : generates
    Evaluation "1" --> "*" Question : contains
    Evaluation "1" --> "*" Response : collects
    Evaluation "1" --> "1" AnalyticsResult : produces
    Evaluation "1" --> "0..*" Collaborator : allows
    
    Question "1" --> "1" Scale : uses
    Question "1" --> "*" Response : receives
    
    Template "1" --> "*" Question : defines
    
    Collaborator "*" --> "1" User : references
    
    Scale <|-- LikertScale : extends
    Scale <|-- FrequencyScale : extends
    Scale <|-- PairedComparisonScale : extends
    Scale <|-- FixedSumScale : extends
    
    Response "1" --> "1" Question : answers
    
    AnalyticsResult "1" --> "*" Statistics : contains
    
    %% ==================== STYLING ====================
    
    class User:::userClass
    class Category:::categoryClass
    class Evaluation:::evaluationClass
    class PublicLink:::linkClass
    class Question:::questionClass
    class Response:::responseClass
    class Scale:::scaleClass
    class LikertScale:::scaleImpl
    class FrequencyScale:::scaleImpl
    class PairedComparisonScale:::scaleImpl
    class FixedSumScale:::scaleImpl
    class Template:::templateClass
    class Collaborator:::collaboratorClass
    class AnalyticsResult:::analyticsClass
    class Statistics:::statsClass
    
    classDef userClass fill:#4CAF50,stroke:#2E7D32,stroke-width:2px,color:#fff
    classDef categoryClass fill:#2196F3,stroke:#1565C0,stroke-width:2px,color:#fff
    classDef evaluationClass fill:#FF6F00,stroke:#E65100,stroke-width:2px,color:#fff
    classDef linkClass fill:#9C27B0,stroke:#6A1B9A,stroke-width:2px,color:#fff
    classDef questionClass fill:#F44336,stroke:#C62828,stroke-width:2px,color:#fff
    classDef responseClass fill:#FF9800,stroke:#E65100,stroke-width:2px,color:#fff
    classDef scaleClass fill:#3F51B5,stroke:#1A237E,stroke-width:2px,color:#fff
    classDef scaleImpl fill:#5C6BC0,stroke:#3F51B5,stroke-width:2px,color:#fff
    classDef templateClass fill:#00BCD4,stroke:#006064,stroke-width:2px,color:#fff
    classDef collaboratorClass fill:#8BC34A,stroke:#558B2F,stroke-width:2px,color:#fff
    classDef analyticsClass fill:#FFC107,stroke:#F57F17,stroke-width:2px,color:#fff
    classDef statsClass fill:#FFEB3B,stroke:#F57F17,stroke-width:2px,color:#000
```

---

## 📋 Descrição das Classes

### 🔑 Core Entities

#### **User**
Representa o proprietário/criador de avaliações
- **Atributos:** ID, email, senha hasheada, nome, timestamps
- **Métodos:** Login/logout, gerenciar avaliações e categorias
- **Relações:** Possui múltiplas avaliações e categorias

#### **Category**
Organiza avaliações por tópicos personalizados
- **Atributos:** ID, user_id, nome, descrição, cor
- **Métodos:** Adicionar/remover avaliações, listar avaliações
- **Relações:** Pertence a um usuário, contém múltiplas avaliações

#### **Evaluation**
Representação principal de uma avaliação
- **Atributos:** ID, tipo de escala, status, timestamps
- **Métodos:** Publicar, fechar, gerenciar questões, gerar link público
- **Relações:** Pertence a um usuário/categoria, contém questões e respostas

#### **Question**
Questão individual dentro de uma avaliação
- **Atributos:** ID, texto, tipo de escala, ordem, metadados
- **Métodos:** Reordenar, obter respostas
- **Relações:** Pertence a uma avaliação, recebe múltiplas respostas

#### **Response**
Resposta individual de um respondente
- **Atributos:** ID, valor da resposta (JSON), timestamp, respondente (anônimo)
- **Métodos:** Validar resposta
- **Relações:** Responde a uma questão

---

### 📏 Scale Types (Polimorfismo)

#### **Scale** (Classe Abstrata)
Classe base para todos os tipos de escala

#### **LikertScale**
Escala 1-5 com labels personalizados
- **Métodos:** Validar valor inteiro, calcular média/mediana/desvio padrão

#### **FrequencyScale**
Escala categórica com frequência
- **Métodos:** Validar categoria, calcular distribuição

#### **PairedComparisonScale**
Comparação "A vs B" com ranking
- **Métodos:** Validar pares, rankear itens, calcular força relativa

#### **FixedSumScale**
Distribuição de pontos com somatório fixo
- **Métodos:** Validar somatório, calcular média ponderada

---

### 🔗 Collaboration & Access

#### **PublicLink**
Link compartilhável para avaliar sem login
- **Atributos:** UUID, short URL, data de expiração, status ativo
- **Métodos:** Validar link, desativar link
- **Relações:** Referencia uma avaliação

#### **Collaborator**
Define role de colaboradores em uma avaliação
- **Atributos:** Avaliação, usuário, role (owner/editor/viewer)
- **Métodos:** Atualizar role, remover colaborador
- **Roles:** OWNER (controle total), EDITOR (editar), VIEWER (ler apenas)

---

### 📊 Results & Analytics

#### **AnalyticsResult**
Resultados agregados de uma avaliação finalizada
- **Atributos:** Taxa de resposta, métricas, insights, data de geração
- **Métodos:** Exportar PDF/CSV, obter insights
- **Relações:** Contém múltiplas estatísticas

#### **Statistics**
Estatísticas calculadas por questão
- **Atributos:** Média, mediana, desvio padrão, distribuição, score de qualidade
- **Métodos:** Detectar outliers, calcular qualidade

---

### 📋 Templates

#### **Template**
Modelo pré-definido para acelerar criação
- **Atributos:** Nome, descrição, estrutura JSON, tipo de escala
- **Métodos:** Converter para avaliação, duplicar
- **Relações:** Define questões padrão

---

## 🔗 Relacionamentos Principais

| De | Para | Tipo | Cardinalidade | Descrição |
|---|---|---|---|---|
| **User** | **Evaluation** | Ownership | 1 : * | Um usuário cria múltiplas avaliações |
| **User** | **Category** | Ownership | 1 : * | Um usuário possui múltiplas categorias |
| **Category** | **Evaluation** | Composition | 1 : * | Uma categoria contém múltiplas avaliações |
| **Evaluation** | **Question** | Composition | 1 : * | Uma avaliação contém múltiplas questões |
| **Question** | **Response** | Aggregation | 1 : * | Uma questão recebe múltiplas respostas |
| **Evaluation** | **PublicLink** | Association | 1 : 0..* | Uma avaliação gera múltiplos links públicos |
| **Evaluation** | **AnalyticsResult** | Composition | 1 : 1 | Uma avaliação produz um resultado |
| **Scale** | **LikertScale** | Inheritance | 1 : 1 | Herança de tipo de escala |
| **Collaborator** | **User** | Reference | * : 1 | Múltiplos colaboradores por usuário |

---

## 💾 Serialização e Persistência

### Evaluation
```json
{
  "id": "uuid-eval-001",
  "user_id": "uuid-user-001",
  "category_id": "uuid-cat-001",
  "title": "Avaliação de Desempenho",
  "status": "OPEN",
  "scale_type": "LIKERT",
  "created_at": "2026-01-29T10:00:00Z"
}
```

### Question
```json
{
  "id": "uuid-q-001",
  "evaluation_id": "uuid-eval-001",
  "text": "Como você avalia a qualidade?",
  "scale_type": "LIKERT",
  "metadata": {
    "min": 1,
    "max": 5,
    "labels": ["Muito Ruim", "Ruim", "Neutro", "Bom", "Muito Bom"]
  }
}
```

### Response
```json
{
  "id": "uuid-resp-001",
  "question_id": "uuid-q-001",
  "answer_value": 4,
  "created_at": "2026-01-29T11:00:00Z"
}
```

---

## 🎯 Padrões de Design Utilizados

| Padrão | Uso | Exemplo |
|---|---|---|
| **Strategy** | Diferentes tipos de escala | Scale abstrata com implementations |
| **Template Method** | Cálculo de estatísticas | Cada escala implementa seu algoritmo |
| **Observer** | Realtime updates | Supabase notifica mudanças |
| **Builder** | Criar avaliações complexas | Evaluation builder com fluent API |
| **Repository** | Persistência de dados | Supabase query abstraction |

---

## 📈 Evolução Futura

Possíveis extensões da arquitetura:

- **Nested Questions:** Questões condicionais baseadas em respostas anteriores
- **Multi-language:** Suporte a múltiplos idiomas
- **Versioning:** Histórico de versões de avaliações
- **Permissions:** Sistema granular de permissões
- **Webhooks:** Notificações para sistemas externos
- **Custom Scales:** Extensão para tipos de escala customizados

---

**Última atualização:** January 29, 2026  
**Versão:** 1.0
