# Projeto Aevalo

Potencializamos a coleta de feedbacks transformando descrições simples em avaliações estruturadas via IA. Nossa solução automatiza o levantamento de métricas de desempenho, garantindo precisão metodológica e insights baseados em dados para uma gestão orientada a resultados.

## Objetivo:
Desenvolver um MicroSaaS escalável para a criação e gestão de avaliações multidimensionais (eventos, indivíduos e produtos), integrando Inteligência Artificial para a geração automatizada de quesitos e utilizando metodologias psicométricas para garantir a precisão dos dados coletados.

## Brainstorm:

O usuário deverá realizar o login no sistema. Após a validação do acesso, será apresentado um **dashboard** contendo informações sobre suas avaliações recentes. Esse painel incluirá:

* uma lista das avaliações criadas, acompanhada de uma barra de pesquisa;
* um gráfico de barras que contabiliza as avaliações por categoria;
* a indicação de avaliações em aberto.

Caso o usuário ainda não possua avaliações cadastradas, o sistema deverá exibir um botão para **criação de uma nova avaliação**.

O usuário poderá criar avaliações **individuais** ou **colaborativas**. Nas avaliações colaborativas, o sistema deverá gerar um **link público** para convite dos avaliadores, além de disponibilizar um botão para **finalizar a avaliação**. As avaliações deverão ser organizadas por **categorias personalizáveis**, definidas pelo próprio usuário.

O sistema de avaliação adotará diferentes tipos de escalas, conforme descrito em: [https://measuringu.com/rating-scales/](https://measuringu.com/rating-scales/), incluindo:

* Escala Likert;
* Escalas de Frequência (*Frequency Scales*);
* Escala de Comparação Pareada (*Paired Comparison Scale*);
* Escala de Soma Fixa (*Fixed Sum*).

Ao criar uma avaliação, o usuário poderá optar por:

1. utilizar um **modelo de avaliação pré-existente**; ou
2. fornecer uma **breve descrição** do que será avaliado, permitindo que uma **LLM** gere automaticamente os itens da avaliação.

Em ambos os casos, a avaliação poderá ser **customizada** pelo usuário antes de sua aplicação.

---

## 📚 Sumário Executivo dos Documentos do Projeto

Este diretório contém a documentação técnica completa para o desenvolvimento do **Aevalo**. Consulte os arquivos abaixo para uma compreensão detalhada de cada aspecto do projeto:

### 📄 [desc.md](desc.md) - Descrição do Produto
Documento que detalha a proposta de solução, experiência do usuário, metodologias científicas de avaliação e o ecossistema colaborativo. Inclui justificativas de negócio e diferenciais competitivos.

### 🗺️ [roadmap.md](roadmap.md) - Roadmap de Evolução
Apresenta o plano de desenvolvimento em 4 fases:
- **Fase 1:** Fundação e Core Engine (MVP)
- **Fase 2:** Inteligência e Metodologia (Diferenciais)
- **Fase 3:** Colaboração e Viralidade (Escala)
- **Fase 4:** Insights e Enterprise (Maturação)

Inclui matriz de prioridade para orientar decisões de desenvolvimento.

### 🛠️ [techdesign.md](techdesign.md) - Especificação Técnica
Define a arquitetura técnica, stack adotada e especificações de implementação para:
- Autenticação e onboarding
- Dashboard de gestão
- Motor de avaliação polimórfico
- Integração com IA (LLM)
- Sistema de colaboração e acesso público

---

**Documento Raiz:** Consulte [../../README.md](../../README.md) para informações gerais do projeto.

