<!-- 
🚀 INÍCIO RÁPIDO PARA CONTRIBUIDORES
AEVALO - AVALIAR FICOU INTELIGENTE
-->

# ⚡ Referência Rápida - Contribuindo ao Aevalo

## 5 Passos para Começar

### 1️⃣ Clone e Configure
```bash
git clone https://github.com/SEU-USUARIO/aevalo.git
cd aevalo
git remote add upstream https://github.com/aevalo/aevalo.git
bash setup-contributor.sh
```

### 2️⃣ Procure uma Issue
- 🟢 [Good first issue](https://github.com/aevalo/aevalo/labels/good%20first%20issue)
- 🔵 [Help wanted](https://github.com/aevalo/aevalo/labels/help%20wanted)
- 💡 [Feature request](https://github.com/aevalo/aevalo/issues?q=label%3Aenhancement)

### 3️⃣ Crie uma Branch
```bash
git checkout -b feature/123-descricao
# Exemplo: feature/45-add-export-pdf
```

**Padrão:** `tipo/numero-descricao`
- `feature/` - Nova funcionalidade
- `bugfix/` - Correção de bug
- `docs/` - Documentação
- `refactor/` - Refatoração

### 4️⃣ Faça suas Mudanças
```bash
# Edite os arquivos
npm test        # Rode testes
npm run lint    # Verifique linting
git add .
git commit -m "tipo(escopo): descrição breve"
git push origin feature/123-descricao
```

### 5️⃣ Abra um Pull Request
- Use o template em `.github/PULL_REQUEST_TEMPLATE.md`
- Referencie a issue: `Closes #123`
- Descreva as mudanças
- Aguarde reviews!

---

## 📚 Documentos Principais

| Documento | Para Quem | Conteúdo |
|-----------|-----------|----------|
| [CONTRIBUTING.md](../CONTRIBUTING.md) | Todos | Guia completo de contribuição |
| [COMMUNITY.md](../COMMUNITY.md) | Iniciantes | Hub com tudo que você precisa |
| [CODE_OF_CONDUCT.md](../CODE_OF_CONDUCT.md) | Todos | Padrões de comportamento |
| [SECURITY.md](../SECURITY.md) | Security | Como reportar vulnerabilidades |
| [doc/engineering/](../doc/engineering/) | Técnicos | Arquitetura e roadmap |

---

## 🎯 Roadmap de Fases

```
FASE 1: MVP           → Autenticação, Survey Builder básico
FASE 2: IA & Métodos → LLM, Templates, Escalas avançadas
FASE 3: Colaboração   → Multi-usuário, Links públicos
FASE 4: Enterprise    → Relatórios, Notificações, API
```

Consulte [doc/engineering/roadmap.md](../doc/engineering/roadmap.md) para detalhes.

---

## 💻 Stack do Projeto

```
Backend:  Rust (Performance + Segurança)
Frontend: Vue.js (Reatividade)
Database: Supabase (Gerenciado)
IA:       Gemini API (Gen-AI)
```

Veja [doc/engineering/techdesign.md](../doc/engineering/techdesign.md) para mais.

---

## ✅ Checklist Antes de PR

- [ ] Li CONTRIBUTING.md
- [ ] Segui a convenção de branch `tipo/numero-descricao`
- [ ] Fiz commits pequenos e bem documentados
- [ ] Rodei testes localmente (`npm test`)
- [ ] Rodei linter (`npm run lint`)
- [ ] Atualizei documentação se necessário
- [ ] Meu PR não quebra backward compatibility
- [ ] Usei o template de PR

---

## 🆘 Precisa de Ajuda?

| Tipo | Onde Ir |
|------|---------|
| **Dúvidas gerais** | [COMMUNITY.md](../COMMUNITY.md) |
| **Configuração ambiente** | [CONTRIBUTING.md](../CONTRIBUTING.md#-configurar-seu-ambiente) |
| **Estrutura do projeto** | [doc/engineering/README.md](../doc/engineering/README.md) |
| **Discussões** | [GitHub Discussions](https://github.com/aevalo/aevalo/discussions) |
| **Segurança** | [SECURITY.md](../SECURITY.md) |
| **Email geral** | hello@aevalo.dev |

---

## 🚀 Exemplo Completo

```bash
# 1. Clonar e configurar
git clone https://github.com/seu-usuario/aevalo.git
cd aevalo
git remote add upstream https://github.com/aevalo/aevalo.git

# 2. Atualizar com upstream
git fetch upstream
git checkout main
git merge upstream/main

# 3. Criar feature branch
git checkout -b feature/456-melhorar-dashboard

# 4. Fazer mudanças
# ... edite arquivos ...

# 5. Testar e commitar
npm test
npm run lint
git add .
git commit -m "feat(dashboard): add category filter"

# 6. Push e PR
git push origin feature/456-melhorar-dashboard
# Abrir PR no GitHub com template

# 7. Aguardar review
# Incorporar sugestões se necessário
```

---

## 📋 Tipos de Contribuição

### 💻 Código
- Features do roadmap
- Bugfixes
- Melhorias de performance
- Refactoring

### 📚 Documentação
- Melhorar guias
- Adicionar exemplos
- Corrigir typos
- Traduzir

### 🧪 Testes
- Testes unitários
- Testes de integração
- Melhorar cobertura

### 🐛 Comunidade
- Responder questions
- Revisar PRs
- Reportar bugs
- Sugerir ideias

---

## ⚡ Dicas Rápidas

✅ **Faça:**
- Commits pequenos e focados
- Mensagens de commit claras
- Testes para novas features
- Referências a issues (#123)
- Rebase antes de PR

❌ **Não Faça:**
- Commits gigantescos
- Múltiplas features em um PR
- Mensagens vagas
- Força push (force push)
- Discussões off-topic

---

## 🎓 Aprenda Mais

- 📖 [GitHub - Como Contribuir](https://opensource.guide/pt/how-to-contribute/)
- 🔄 [Git - Guia Prático](https://rogerdudler.github.io/git-guide/index.pt_BR.html)
- 🚀 [Conventional Commits](https://www.conventionalcommits.org/pt-br/)
- 📊 [Contributor Covenant](https://www.contributor-covenant.org/pt-br)

---

## 🎉 Bem-vindo!

Obrigado por considerar contribuir para o **Aevalo**!

Sua contribuição, seja código, documentação ou feedback, é muito valiosa.

**"Avaliar ficou inteligente."** 🚀

---

**Última atualização:** January 27, 2026  
**Versão:** 1.0

Para mais detalhes, consulte [CONTRIBUTING.md](../CONTRIBUTING.md).
