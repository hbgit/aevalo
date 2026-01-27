name: Bug Report
description: Reporte um bug ou comportamento inesperado
title: "[BUG] "
labels: ["bug", "triage"]

body:
  - type: markdown
    attributes:
      value: |
        Obrigado por reportar um bug! Por favor, preencha as informações abaixo para nos ajudar a corrigir o problema.

  - type: textarea
    id: description
    attributes:
      label: Descrição do Bug
      description: Descrição clara e concisa do problema
      placeholder: O que aconteceu? Qual era o comportamento esperado?
    validations:
      required: true

  - type: textarea
    id: steps
    attributes:
      label: Passos para Reproduzir
      description: Explique como reproduzir o bug passo a passo
      placeholder: |
        1. Faça login como...
        2. Navegue para...
        3. Clique em...
        4. Observe que...
    validations:
      required: true

  - type: textarea
    id: expected
    attributes:
      label: Comportamento Esperado
      description: O que deveria acontecer?
      placeholder: Esperava que...
    validations:
      required: true

  - type: textarea
    id: actual
    attributes:
      label: Comportamento Atual
      description: O que realmente aconteceu?
      placeholder: Em vez disso, aconteceu...
    validations:
      required: true

  - type: textarea
    id: environment
    attributes:
      label: Ambiente
      description: |
        Informações sobre seu ambiente
      value: |
        - SO: [ex: Windows 11, macOS 13, Ubuntu 22.04]
        - Navegador: [ex: Chrome 120, Firefox 121]
        - Versão do projeto: [ex: v0.1.0]
        - Node/Rust versão: [ex: Node 20.0.0, Rust 1.75]
    validations:
      required: false

  - type: textarea
    id: logs
    attributes:
      label: Logs ou Screenshots
      description: Anexe screenshots, logs de console ou erros relevantes
      placeholder: Cole logs ou arraste screenshots aqui
    validations:
      required: false

  - type: textarea
    id: additional
    attributes:
      label: Contexto Adicional
      description: Qualquer outra informação relevante?
    validations:
      required: false

  - type: checkboxes
    id: checklist
    attributes:
      label: Checklist
      options:
        - label: Confirmo que este é um novo bug e não foi reportado antes
          required: true
        - label: Li a documentação relevante
          required: false
