name: Feature Request
description: Sugira uma nova funcionalidade ou melhoria
title: "[FEATURE] "
labels: ["enhancement", "feature-request"]

body:
  - type: markdown
    attributes:
      value: |
        Obrigado pela sugestão! Por favor, preencha as informações abaixo para descrever sua ideia.

  - type: textarea
    id: problem
    attributes:
      label: Problema ou Caso de Uso
      description: Qual problema essa funcionalidade resolveria?
      placeholder: |
        Estou enfrentando o problema de... 
        Isto acontece quando eu...
    validations:
      required: true

  - type: textarea
    id: solution
    attributes:
      label: Solução Proposta
      description: Como você imagina que esta funcionalidade funcionaria?
      placeholder: Gostaria que o sistema...
    validations:
      required: true

  - type: textarea
    id: alternatives
    attributes:
      label: Alternativas Consideradas
      description: Existem outras abordagens que considerou?
      placeholder: Poderia ser também...
    validations:
      required: false

  - type: dropdown
    id: phase
    attributes:
      label: Alinhamento com Roadmap
      description: Esta funcionalidade se alinha com qual fase do roadmap?
      options:
        - "Fase 1 (MVP: Fundação e Core Engine)"
        - "Fase 2 (IA: Inteligência e Metodologia)"
        - "Fase 3 (Escala: Colaboração e Viralidade)"
        - "Fase 4 (Enterprise: Insights)"
        - "Não se aplica / Fora do roadmap"
    validations:
      required: false

  - type: dropdown
    id: impact
    attributes:
      label: Impacto Estimado
      description: Qual seria o impacto desta funcionalidade?
      options:
        - "Crítico (Bloqueador do produto)"
        - "Alto (Melhora significativa)"
        - "Médio (Nice to have)"
        - "Baixo (Conveniência)"
    validations:
      required: false

  - type: textarea
    id: context
    attributes:
      label: Contexto Adicional
      description: Screenshots, links, referências, ou qualquer informação relevante
    validations:
      required: false

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: Verifiquei se não existe uma feature request similar
          required: true
        - label: Li o roadmap do projeto
          required: false
