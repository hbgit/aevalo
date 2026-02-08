# 🔐 Tratamento de Erros - Tela de Login

> Documentação completa dos tratamentos de erros HTTP implementados no componente Login.vue

## 📋 Resumo das Melhorias

O componente Login.vue foi atualizado para implementar **tratamento robusto de erros de API** conforme especificado no documento `interface_flow.md`. 

### ✨ Recursos Implementados

| Recurso | Status | Descrição |
|---------|--------|-----------|
| **Diferenciação por código HTTP** | ✅ | Cada erro HTTP recebe tratamento específico |
| **Countdown de retry (429)** | ✅ | Timer visual para rate limit de 60s |
| **Limpeza contextual** | ✅ | Limpa campo de senha após erro 401 |
| **Mensagens descritivas** | ✅ | Mensagem principal + detalhes adicionais |
| **Ícones visuais** | ✅ | Emoji que muda conforme tipo de erro |
| **Validação de form** | ✅ | Validação antes de enviar |
| **Timeout automático** | ✅ | 10 segundos de timeout na requisição |
| **Recuperação de rede** | ✅ | Detecta e trata erros de conectividade |
| **Links contextuais** | ✅ | Status page e outras ações por tipo de erro |

---

## 🛠️ Tratamento de Erros Implementados

### 1️⃣ Erro 401 - Credenciais Inválidas

**Quando ocorre:** Email ou senha incorretos

**Mensagem:**
```
🔐 Email ou senha incorretos. Tente novamente.
Verifique seus dados de acesso e tente novamente.
```

**Ações do Sistema:**
- Limpa campo de senha (segurança)
- Mantém email preenchido
- Foca no campo de senha para facilitar correção
- Exibe botão "Tentar Novamente"

**Código HTTP:** `401 Unauthorized`

---

### 2️⃣ Erro 429 - Rate Limit

**Quando ocorre:** Muitas tentativas de login

**Mensagem:**
```
⏱️ Muitas tentativas de login. Por favor, aguarde.
Sua conta foi temporariamente bloqueada por segurança.
Tente novamente em 60s [════░░░░░░░░]
```

**Ações do Sistema:**
- Exibe countdown regressivo de 60 segundos
- Desabilita botão de login
- Após countdown, ativa botão "Tentar Novamente"
- Indicador visual do progresso

**Código HTTP:** `429 Too Many Requests`

**Implementação:**
```typescript
const startRetryCountdown = (seconds: number = 60) => {
  retryCountdown.value = seconds
  showRetryCountdown.value = true
  isLoading.value = true
  
  retryTimer = window.setInterval(() => {
    retryCountdown.value--
    if (retryCountdown.value <= 0) {
      clearInterval(retryTimer)
      showRetryButton.value = true
      isLoading.value = false
    }
  }, 1000)
}
```

---

### 3️⃣ Erro 500 - Servidor Indisponível

**Quando ocorre:** Erro interno no servidor backend

**Mensagem:**
```
⚠️ Nossos servidores estão temporariamente indisponíveis.
Já estamos trabalhando para resolver o problema. 
Tente novamente em instantes.
[ Tentar Novamente ]  [ Ver Status ]
```

**Ações do Sistema:**
- Exibe link para status page
- Oferece opção de retry
- Mensagem de contexto sobre o problema

**Código HTTP:** `500 Internal Server Error`

---

### 4️⃣ Erro 503 - Manutenção

**Quando ocorre:** Sistema em manutenção programada

**Mensagem:**
```
🔧 Sistema em manutenção programada.
Retornaremos em breve com melhorias. 
Obrigado pela paciência!
[ Tentar Novamente ]
```

**Ações do Sistema:**
- Informa usuário sobre manutenção
- Oferece retry (sistema pode estar de volta)
- Tom mais amigável

**Código HTTP:** `503 Service Unavailable`

---

### 5️⃣ Erro 403 - Conta Bloqueada ou Email Não Verificado

**Cenário A: Email não verificado**
```
🔐 Email não verificado.
Verifique seu email para ativar sua conta. 
Não encontrou? Clique para reenviar.
[ Reenviar Email ]
```

**Cenário B: Conta bloqueada**
```
🔐 Sua conta foi temporariamente bloqueada por segurança.
Verifique seu email para instruções de recuperação.
```

**Código HTTP:** `403 Forbidden`

---

### 6️⃣ Erro de Rede - Sem Conexão

**Quando ocorre:** Usuário sem conexão com internet ou servidor não responde

**Mensagem:**
```
📡 Sem conexão com a internet.
Verifique sua conexão de rede e tente novamente.
[ Tentar Novamente ]
```

**Ações do Sistema:**
- Detecta `TypeError: Failed to fetch`
- Oferece retry quando conexão for restaurada
- Mensagem clara sobre o problema

---

### 7️⃣ Timeout - Conexão Expirada

**Quando ocorre:** Requisição demora mais de 10 segundos

**Mensagem:**
```
⚠️ Conexão expirou. Tente novamente.
A requisição demorou muito tempo. 
Verifique sua conexão.
[ Tentar Novamente ]
```

**Implementação:**
```typescript
fetch('...', {
  signal: AbortSignal.timeout(10000) // 10 segundos
})
```

---

## 📊 Estados Visuais por Tipo de Erro

### Cores e Ícones

| Tipo | Cor | Ícone | Classe CSS |
|------|-----|-------|-----------|
| **Auth** | Vermelho | 🔐 | `bg-red-50 border-red-200 text-red-700` |
| **Rate Limit** | Amarelo | ⏱️ | `bg-yellow-50 border-yellow-200 text-yellow-700` |
| **Server** | Laranja | ⚠️ | `bg-orange-50 border-orange-200 text-orange-700` |
| **Maintenance** | Laranja | 🔧 | `bg-orange-50 border-orange-200 text-orange-700` |
| **Network** | Cinza | 📡 | `bg-slate-50 border-slate-200 text-slate-700` |

---

## 🔍 Validação de Formulário

Antes de enviar a requisição, o sistema valida:

```typescript
const validateForm = (): boolean => {
  // 1. Email obrigatório
  if (!email.value) {
    errorMessage.value = 'Email é obrigatório.'
    return false
  }

  // 2. Senha obrigatória
  if (!password.value) {
    errorMessage.value = 'Senha é obrigatória.'
    return false
  }

  // 3. Formato de email válido
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  if (!emailRegex.test(email.value)) {
    errorMessage.value = 'Email inválido.'
    return false
  }

  return true
}
```

---

## 🎯 Fluxo Completo de Login

```
┌─────────────────┐
│   Usuário clica │
│   "Entrar"      │
└────────┬────────┘
         │
         ▼
┌─────────────────────────┐
│ Validar Formulário      │
│ (email, senha, format)  │
└────────┬────────────────┘
         │
         ▼ OK
┌─────────────────────────┐
│ POST /auth/login        │
│ (com timeout de 10s)    │
└────────┬────────────────┘
         │
         ▼ Resposta
    ┌────────────────┐
    │ Status OK?     │
    └─┬──────────────┘
      │
      ├─ Não: 401 ──► Limpa senha, foca input
      │
      ├─ Não: 429 ──► Countdown 60s
      │
      ├─ Não: 500 ──► Status page link
      │
      ├─ Não: 503 ──► Mensagem manutenção
      │
      ├─ Não: 403 ──► Email verificado?
      │
      └─ Sim ────────► Store tokens
                       Redirect /dashboard
```

---

## 💾 Dados Armazenados

**localStorage após sucesso:**
```javascript
{
  auth_token: "eyJ0eXAiOiJKV1QiLCJhbGc...",
  refresh_token: "eyJ0eXAiOiJKV1QiLCJhbGc...",
  token_expires_at: 1707388234567,
  remember_email: "user@example.com" // opcional
}
```

---

## 🧪 Testando os Tratamentos

### Teste 401 - Credenciais Inválidas
```bash
curl -X POST http://localhost:3000/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"wrong"}'
# Esperar: 401 Unauthorized
```

### Teste 429 - Rate Limit
```bash
# Fazer múltiplas requisições em sequência
for i in {1..10}; do
  curl -X POST http://localhost:3000/auth/login \
    -H "Content-Type: application/json" \
    -d '{"email":"test@example.com","password":"wrong"}'
done
# Esperar: 429 Too Many Requests após X tentativas
```

### Teste Timeout
```bash
# Mock de delay no backend para testar timeout de 10s
# Sistema deve mostrar "Conexão expirou"
```

---

## 📱 Responsividade

O componente está totalmente responsivo:

- **Mobile:** Stack vertical, mensagens truncadas com "..."
- **Tablet:** Mesmo layout do desktop
- **Desktop:** Largura máxima 28rem (400px)

---

## ♿ Acessibilidade

- ✅ Todos os inputs com labels associadas
- ✅ Focus management correto (foca no campo com erro)
- ✅ Mensagens de erro com roles ARIA
- ✅ Cores + ícones (não depende só de cor)
- ✅ Navegação por teclado (Tab, Enter)

---

## 🚀 Próximos Passos

### Fase 1 (MVP)
- [ ] Testar com diferentes códigos HTTP
- [ ] Implementar retry automático para network errors
- [ ] Adicionar analytics de erros (Sentry)

### Fase 2 (Refinamento)
- [ ] Social login (Google, GitHub)
- [ ] Autenticação multi-fator
- [ ] Biometria (em mobile)

### Fase 3 (Avançado)
- [ ] Sincronização com status page
- [ ] Notificações de manutenção programada
- [ ] Recovery automático de tokens expirados

---

## 📚 Referências

- Documentação: [interface_flow.md](../../interface_flow.md#autenticação-e-onboarding)
- Specs: [Autenticação e Onboarding](../../interface_flow.md#11-tela-de-loginacadastro-modo-toggle)
- Matriz de erros: [HTTP Error Handling](../../interface_flow.md#camada-de-autenticação)

---

*Última atualização: 08 de fevereiro de 2026*  
*Maintido por: Equipe Aevalo - Frontend*
