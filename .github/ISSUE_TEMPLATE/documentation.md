name: Documentação
description: Sugira melhorias na documentação
title: "[DOCS] "
labels: ["documentation"]

body:
  - type: markdown
    attributes:
      value: |
        Ajude-nos a melhorar a documentação!

  - type: textarea
    id: description
    attributes:
      label: Descrição
      description: O que está faltando ou precisa ser melhorado?
      placeholder: A documentação sobre... não está clara porque...
    validations:
      required: true

  - type: textarea
    id: suggestion
    attributes:
      label: Sugestão
      description: Como você melhoraria?
      placeholder: Eu sugeriria...
    validations:
      required: false

  - type: textarea
    id: links
    attributes:
      label: Links Relevantes
      description: Links para a documentação ou arquivos mencionados
    validations:
      required: false
