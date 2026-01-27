# Guia de Contribuição - Aevalo

Obrigado por seu interesse em contribuir para o **Aevalo**! Este documento fornece orientações e instruções para ajudar você a colaborar efetivamente com o projeto.

## 🚀 Começando

### Pré-requisitos
- Git configurado em sua máquina
- Conhecimento básico de Git e GitHub
- Familiaridade com o stack do projeto (Rust backend, Vue.js frontend, Supabase)
- Leitura do [README.md](README.md) e documentação técnica em [doc/engineering/](doc/engineering/)

### Configurar seu Ambiente
1. **Fork o repositório** para sua conta GitHub
2. **Clone seu fork** localmente:
   ```bash
   git clone https://github.com/seu-usuario/aevalo.git
   cd aevalo
   ```
3. **Adicione upstream** para sincronizar com o repositório principal:
   ```bash
   git remote add upstream https://github.com/aevalo/aevalo.git
   ```
4. **Instale as dependências** conforme documentado em [doc/engineering/techdesign.md](doc/engineering/techdesign.md)

---

## 📋 Processo de Contribuição

### 1. Escolher ou Reportar uma Issue
- Verifique a aba **Issues** para encontrar tarefas em aberto
- Procure por issues rotuladas com `good first issue` ou `help wanted` se for iniciante
- Não encontrou uma issue? Crie uma descrevendo sua sugestão ou bug

### 2. Comunicar Sua Intenção
- Comente na issue informando que irá trabalhar nela
- Aguarde aprovação dos mantenedores antes de começar trabalho significativo
- Isso evita duplicação de esforços

### 3. Criar uma Branch
Utilize a convenção de nomenclatura:
```bash
git checkout -b tipo/numero-descricao
```
Exemplos:
- `feature/123-llm-integration`
- `bugfix/456-dashboard-pagination`
- `docs/789-update-roadmap`

**Tipos permitidos:** `feature`, `bugfix`, `docs`, `refactor`, `test`, `chore`

### 4. Fazer suas Alterações
- Siga o estilo de código do projeto
- Mantenha commits pequenos e bem documentados
- Escreva testes para novas funcionalidades
- Atualize a documentação conforme necessário

### 5. Sincronizar com Upstream
Antes de fazer push, sincronize com as mudanças mais recentes:
```bash
git fetch upstream
git rebase upstream/main
```

### 6. Push e Criar um Pull Request
```bash
git push origin tipo/numero-descricao
```

**No Pull Request:**
- Use o template fornecido em `.github/PULL_REQUEST_TEMPLATE.md`
- Referencie a issue relacionada: `Closes #123`
- Descreva as mudanças e teste realizadas
- Inclua screenshots se houver mudanças na UI

---

## 🎯 Diretrizes de Qualidade

### Código
- **Formatação:** Use formatadores nativos (rustfmt para Rust, prettier para Vue)
- **Testes:** Toda funcionalidade deve ter testes unitários ou de integração
- **Documentação:** Código complexo deve ter comentários explicativos
- **Linting:** Resolva todos os erros de linting antes do commit

### Commits
```
tipo(escopo): descrição breve

Descrição mais detalhada se necessário.
Pode ter múltiplas linhas.

Closes #123
```

Exemplos:
- `feat(dashboard): add category filter to evaluation list`
- `fix(auth): resolve JWT expiration bug`
- `docs: update installation instructions`

### Pull Requests
- **Uma funcionalidade ou correção por PR**
- **Máximo de 400 linhas** (PRs maiores são mais difíceis de revisar)
- **Descrição clara** do problema e solução
- **Tests coverage:** Novas funcionalidades devem aumentar a cobertura de testes

---

## 📚 Roadmap e Prioridades

Consulte [doc/engineering/roadmap.md](doc/engineering/roadmap.md) para entender as fases do projeto:

1. **Fase 1 (MVP):** Autenticação, Survey Builder básico, Dashboard
2. **Fase 2 (IA):** Integração com LLM, Templates, Escalas avançadas
3. **Fase 3 (Colaboração):** Módulo multi-usuário, Links públicos
4. **Fase 4 (Enterprise):** Relatórios, Notificações, API

Contribuições alinhadas com o roadmap são mais propensas a serem aceitas rapidamente.

---

## 🐛 Reportando Bugs

Use o template de issue em `.github/ISSUE_TEMPLATE/bug_report.md`:

1. **Descrição clara** do bug
2. **Passos para reproduzir**
3. **Comportamento esperado** vs. **atual**
4. **Ambiente** (OS, versão, etc.)
5. **Logs e screenshots** se aplicável

---

## 💡 Sugerindo Melhorias

Use o template de issue em `.github/ISSUE_TEMPLATE/feature_request.md`:

1. **Problema que resolve**
2. **Solução proposta**
3. **Alternativas consideradas**
4. **Contexto adicional**

---

## 🔐 Segurança

Se descobrir uma vulnerabilidade de segurança, **NÃO** a reporte em issues públicas.

Consulte [SECURITY.md](SECURITY.md) para instruções de divulgação responsável.

---

## ❓ Dúvidas?

- 📖 Leia a [documentação técnica](doc/engineering/techdesign.md)
- 💬 Abra uma discussion no GitHub
- 📧 Entre em contato com os mantenedores

---

## 📜 Licença

Ao contribuir para o Aevalo, você concorda que suas contribuições serão licenciadas sob a [GPLv3 License](LICENSE).

---

**Obrigado por contribuir para tornar o Aevalo melhor! 🎉**
