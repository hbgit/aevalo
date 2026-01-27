# Política de Segurança - Aevalo

## Segurança é Importante para Nós

A Aevalo leva a segurança da comunidade e de nossos usuários muito a sério. Se você descobriu uma vulnerabilidade de segurança no Aevalo, agradecemos seu esforço em divulgá-la de forma responsável.

---

## Relatando Vulnerabilidades

**⚠️ POR FAVOR, NÃO abra uma issue pública para vulnerabilidades de segurança.**

Em vez disso, reporte vulnerabilidades de segurança enviando um email para:
```
security@aevalo.dev
```

Inclua as seguintes informações em seu relatório:

1. **Descrição da vulnerabilidade:** O que é o problema de segurança?
2. **Componente afetado:** Qual parte do código (backend, frontend, banco de dados, etc.)?
3. **Gravidade:** Como você classificaria o risco? (Crítico, Alto, Médio, Baixo)
4. **Passos para reproduzir:** Como alguém pode reproduzir o problema?
5. **Impacto potencial:** Que dados ou funcionalidades poderiam ser comprometidos?
6. **Sugestão de correção (opcional):** Você tem ideias de como corrigir?

---

## O Que Esperar

Após relatar uma vulnerabilidade, você pode esperar:

- **Confirmação de recebimento** dentro de 48 horas
- **Atualização de progresso** a cada 7 dias até a resolução
- **Coordenação de divulgação** para garantir que temos tempo de preparar uma correção
- **Reconhecimento público** de sua descoberta (se desejar) quando a vulnerabilidade for corrigida

---

## Requisitos de Segurança

O Aevalo foi desenvolvido com as seguintes práticas de segurança em mente:

### Autenticação & Autorização
- Implementação de JWT para sessões seguras
- Validação de permissões (role-based access control)
- Proteção contra CSRF e XSS
- Senhas hasheadas com algoritmos modernos

### Dados
- Criptografia em trânsito (HTTPS/TLS)
- Proteção de dados sensíveis em repouso
- Validação rigorosa de entrada (input validation)
- Proteção contra SQL injection e command injection

### Infraestrutura
- Uso de Supabase para segurança gerenciada
- Isolamento de ambientes (dev, staging, prod)
- Logs de auditoria para ações sensíveis
- Monitoramento contínuo de vulnerabilidades

### Stack Seguro
- **Rust:** Linguagem type-safe com memory safety
- **Vue.js:** Framework com proteção XSS integrada
- **Supabase:** Serviço gerenciado com compliance (GDPR, SOC 2)

---

## Divulgação Responsável

Pedimos que você:

✅ **Faça:**
- Relatar vulnerabilidades privadamente por email
- Dar-nos tempo adequado para corrigir antes da divulgação pública
- Ser honesto e cooperativo durante a investigação
- Respeitar a privacidade e segurança dos dados de usuários

❌ **Não Faça:**
- Publicar a vulnerabilidade publicamente antes de coordenar conosco
- Acessar dados de outros usuários
- Degradar ou interromper serviços
- Testar a vulnerabilidade em ambientes de produção sem permissão
- Ameaçar divulgação pública

---

## Critérios de Divulgação

Coordenaremos com você para um prazo apropriado de divulgação, considerando:

- **Criticalidade da vulnerabilidade:** Vulnerabilidades críticas podem ser corrigidas em horas
- **Complexidade da correção:** Alguns problemas exigem mais tempo para resolver
- **Disponibilidade de testes:** Precisamos testar em ambientes simulados

Em geral, tentamos resolver vulnerabilidades relatadas dentro de **30 dias**.

---

## Reconhecimento

Agradecemos aos pesquisadores de segurança que responsavelmente divulgam vulnerabilidades conosco. Quando apropriado:

- Reconheceremos você no [SECURITY_ACKNOWLEDGMENTS.md](SECURITY_ACKNOWLEDGMENTS.md)
- Listaremos seu nome/empresa em nossa página de segurança
- Coordenaremos cobertura de imprensa, se desejado

---

## Atualizações de Segurança

Quando correções de segurança são lançadas:

1. **Anúncio prévio:** Avisamos usuários antes da correção
2. **Patch disponível:** Lançamos a correção com instruções de upgrade
3. **Recomendação de ação:** Explicamos a urgência de atualizar
4. **Relatório pós-incidente:** Publicamos detalhes após confirmação de segurança

---

## Contato de Segurança

| Aspecto | Contato |
|--------|---------|
| **Vulnerabilidades** | security@aevalo.dev |
| **Questões gerais** | hello@aevalo.dev |
| **GitHub Security Advisory** | Através da aba Security no repositório |

---

## Recursos Adicionais

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE - Common Weakness Enumeration](https://cwe.mitre.org/)
- [GitHub Security Best Practices](https://github.com/security)

---

**Obrigado por ajudar a manter o Aevalo seguro! 🔒**
