# GitHub Configuração e Templates

Este diretório contém configurações, templates e workflows para o repositório GitHub do Aevalo.

## 📁 Estrutura

```
.github/
├── ISSUE_TEMPLATE/
│   ├── bug_report.md        # Template para reportar bugs
│   ├── feature_request.md   # Template para sugerir features
│   └── documentation.md     # Template para melhorias de docs
├── workflows/
│   └── test.yml            # CI/CD: Testes automatizados
├── CODEOWNERS              # Proprietários de código por área
├── PULL_REQUEST_TEMPLATE.md # Template para pull requests
└── github-settings.yml     # Configurações do repositório

```

## 📝 Templates de Issues

Os templates ajudam contribuidores a fornecer informações estruturadas:

- **[bug_report.md](ISSUE_TEMPLATE/bug_report.md)** - Para reportar bugs com contexto completo
- **[feature_request.md](ISSUE_TEMPLATE/feature_request.md)** - Para sugerir funcionalidades alinhadas com o roadmap
- **[documentation.md](ISSUE_TEMPLATE/documentation.md)** - Para melhorias na documentação

## 🔄 Pull Requests

[PULL_REQUEST_TEMPLATE.md](PULL_REQUEST_TEMPLATE.md) fornece um checklist e campos estruturados para PRs:

- Descrição das mudanças
- Issue relacionada
- Tipo de mudança
- Checklist de qualidade
- Instruções de teste
- Impacto em performance e segurança

## 👥 Code Owners

[CODEOWNERS](CODEOWNERS) define quem deve revisar mudanças em áreas específicas:

```
* @aevalo/maintainers          # Default para tudo
/doc/ @aevalo/maintainers
/SECURITY.md @aevalo/security
# ... mais configurações quando o projeto crescer
```

## 🔄 Workflows

### test.yml
Executa automaticamente em push e PR:
- Instala dependências
- Roda testes
- Executa linter
- Upload de cobertura

Para adicionar novos workflows, crie arquivos `.yml` nesta pasta.

## ⚙️ Configurações do Repositório

[github-settings.yml](github-settings.yml) documenta as configurações recomendadas:
- Issues habilitadas
- Discussions habilitadas
- Wiki desabilitado
- Pages desabilitado

---

## 🔗 Recursos Relacionados

- [CONTRIBUTING.md](../CONTRIBUTING.md) - Guia de contribuição
- [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) - Código de conduta
- [SECURITY.md](../SECURITY.md) - Política de segurança
- [COMMUNITY.md](../COMMUNITY.md) - Hub de comunidade

---

**Para contribuir, consulte [CONTRIBUTING.md](../CONTRIBUTING.md)** 🚀
