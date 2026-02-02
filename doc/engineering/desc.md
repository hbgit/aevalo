# Product Description

## 📋 Proposta de Solução: Plataforma Inteligente de Gestão de Avaliações

Esta solução visa otimizar o ciclo de coleta de feedbacks e métricas de desempenho através de uma interface intuitiva e recursos de Inteligência Artificial, permitindo que gestores transformem percepções subjetivas em dados acionáveis.

### 1. Experiência do Usuário e Business Intelligence

#### 🎯 Dashboard Centralizador: Interface Intuitiva e Orientada a Dados

A jornada inicia-se em um **Dashboard Centralizador**, projetado com princípios de *progressive disclosure* e *information hierarchy* para oferecer visão imediata do status operacional sem sobrecarregar o usuário com dados não-essenciais.

##### **Arquitetura de Layout**

O dashboard é organizado em **três camadas visuais principais**, utilizando a metodologia de design *card-based UI* com Tailwind CSS:

1. **Barra Superior Inteligente (Header):**
   - Logo e branding da plataforma no canto superior esquerdo
   - Barra de **busca global com autocomplete** (destacada em primeiro plano)
   - Menu de perfil e notificações no canto superior direito
   - Indicador visual de status (conectado/sincronizando)

2. **Painel de Métricas em Tempo Real (Above the Fold):**
   - **Status Tracker Card:** Um destaque visual proeminente para avaliações com `status == 'open'`, permitindo acesso rápido. Exibe:
     - Número de avaliações ativas
     - Prazo mais próximo (com indicador visual de urgência)
     - CTA (*Call-to-Action*) para iniciar uma nova avaliação ou visualizar pendências
   - **Resumo de Categoria:** Cards compactos mostrando contagem de avaliações por categoria principal, com cores codificadas por categoria

3. **Área Principal com Searchable List e Analytics:**
   - **Tabela Interativa (Left Side):** 
     * Paginação *server-side* para performance em grandes volumes
     * Colunas principais: Título, Status (badge com cores), Categoria, Data de Criação, Ações (editar/compartilhar/fechar)
     * Filtros via query params (status, categoria, intervalo de datas)
     * Linhas clicáveis que expandem informações em tempo real
   - **Painel de Analytics (Right Side):**
     * **Gráfico de Barras Dinâmico:** Agregação automática de `evaluations` por `category_id`, com tooltips interativos mostrando contagens exatas e tendências
     * **Breakdown por Status:** Mini-gráfico mostrando distribuição (open/closed/draft) para dar contexto rápido

##### **Funcionalidades de Interação e UX**

* **Gestão Ágil via Busca Preditiva:** 
  - A lista dinâmica com busca preditiva permite localizar qualquer avaliação em segundos
  - Sugestões em tempo real baseadas em título, categoria ou conteúdo de perguntas (alimentadas por queries GraphQL otimizadas)
  - Histórico de buscas recentes para acesso rápido
  - Filtros avançados com syntax visual (ex: `category:NPS status:open`)

* **Visualização de Dados Contextual:**
  - Gráficos de barras automáticos segmentam os resultados por categorias personalizadas, permitindo identificar gargalos ou sucessos por área
  - Ao clicar em uma barra do gráfico, a tabela se sincroniza automaticamente filtrando para aquela categoria
  - Hover tooltips mostram detalhes sem necessidade de clique
  - Exportação de dados em CSV/PDF com um clique

* **Onboarding Inteligente para Novos Usuários:**
  - **Empty State Contextualizado:** Para novos usuários (quando `count == 0`), o dashboard injeta um componente de *Onboarding CTA* no lugar da lista vazia
  - Fluxo guiado com três passos visuais:
    1. "Crie sua primeira avaliação" → botão destacado levando a formulário de criação
    2. "Escolha um template ou deixe a IA gerar" → cards interativos mostrando opções
    3. "Convide seus avaliadores" → interface de share com URL copiável e QR code
  - Tooltips contextuais aparecem ao passar mouse sobre elementos principais
  - Ícones de ajuda (?) abrem modais educacionais sem interromper a jornada

##### **Design System e Acessibilidade**

- **Tailwind CSS:** Utiliza utilitários de spacing, cores e tipografia consistentes
- **States Visuais:** Estados de hover, focus, active e disabled bem definidos em todos os componentes interativos
- **Responsividade:** Layout adapta-se para mobile (colapsível lateral), tablet (two-column com gráfico reduzido) e desktop (three-column com full analytics)
- **Acessibilidade:** 
  - Contraste WCAG AA mínimo em todos os textos
  - Navegação por teclado completa
  - ARIA labels em componentes dinâmicos
  - Modo escuro nativo para reduzir fadiga visual

##### **Performance e Realtime**

- **Atualização em Tempo Real:** Quando um colaborador submete uma resposta, o dashboard atualiza instantaneamente via conexão Supabase Realtime sem refresh manual
- **Otimização GraphQL:** Queries minimistas trazem apenas os campos necessários para renderização, reduzindo carga de rede
- **Carregamento Progressivo:** Tabela renderiza primeiras 20 linhas imediatamente, restante carrega em background

### 2. Metodologias Científicas de Avaliação

O diferencial técnico da plataforma reside na flexibilidade metodológica. O sistema suporta quatro modelos consagrados de coleta de dados (baseados nos padrões *MeasuringU*):

* **Likert & Frequency Scales:** Para medição de atitudes e recorrência.
* **Paired Comparison Scale:** Para definições de prioridade e preferência relativa.
* **Fixed Sum:** Para análise de importância e alocação de peso/valor.

### 3. Criação Híbrida e Inteligência Artificial

Para maximizar a produtividade, a criação de artefatos de avaliação segue dois caminhos:

1. **Caminho Curado:** Uso de modelos (templates) validados e existentes.
2. **Caminho Assistido (AI-Driven):** O usuário fornece o contexto e uma **LLM (Inteligência Artificial)** gera automaticamente os itens da avaliação, garantindo relevância e coesão textual.

> *Ambos os caminhos permitem customização total pelo usuário antes da publicação.*

### 4. Ecossistema Colaborativo

A plataforma transcende o uso individual, permitindo **Avaliações em Cooperação**:

* **Convite via Link Público:** Agilidade no recrutamento de avaliadores externos ou internos.
* **Governança de Dados:** Controle centralizado para encerramento de ciclos de avaliação, garantindo a integridade dos relatórios finais.

---

### Por que esta estrutura valida o negócio?

* **Foco no Problema:** Mostra que você resolve a demora em criar avaliações (via AI).
* **Foco no Rigor:** O uso das escalas do *MeasuringU* traz autoridade técnica.
* **Foco na Escala:** A colaboração via link público mostra que o sistema aguenta múltiplos usuários.

