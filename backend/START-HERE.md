# 📚 ARQUIVO DE INÍCIO - Backend Aevalo

**Bem-vindo! Este é seu ponto de entrada para entender o projeto.**

**Backend Rust completo** para o sistema de avaliação no Aevalo 

---

## 🎯 FLUXO IMPLEMENTADO

**12 steps do diagrama de sequência:**

```
1️⃣  Acessa Dashboard
    ↓
2️⃣  Cria Avaliação
    ↓
3️⃣  Customiza (ou usa IA)
    ↓
4️⃣  Publica com link público
    ↓
5️⃣  Avaliadores acessam
    ↓
6️⃣  Submetem respostas
    ↓
7️⃣  Owner monitora
    ↓
8️⃣  Finaliza avaliação
    ↓
9️⃣  Sistema processa
    ↓
🔟 Visualiza resultados
```

---

## 📊 18 ENDPOINTS

| Fase | Método | Endpoint | Tipo |
|------|--------|----------|------|
| Criação | GET | `/evaluations` | List |
| | POST | `/evaluations` | Create |
| | GET | `/evaluations/{id}` | Read |
| | PATCH | `/evaluations/{id}` | Update |
| | POST | `/evaluations/{id}/publish` | Publish |
| IA | POST | `/evaluations/generate` | Generate |
| | POST | `/evaluations/validate` | Validate |
| Respostas | POST | `/responses` | Submit |
| | GET | `/evaluations/{id}/responses` | List |
| | GET | `/evaluations/{id}/stats` | Stats |
| Público | GET | `/public/eval/{uuid}` | Access |
| | GET | `/public/eval/{uuid}/stats` | Stats |
| Análise | POST | `/evaluations/{id}/process` | Process |
| | GET | `/evaluations/{id}/results` | Results |
| Finalização | POST | `/evaluations/{id}/close` | Close |
| Auth | POST | `/auth/login` | Login |
| | GET | `/health` | Health |

---

## 🧪 COMO TESTAR

### Teste Rápido
```bash
curl http://localhost:3000/health
```

### Teste Automático (Fluxo Completo)
```bash
chmod +x test-api.sh
./test-api.sh
```

### Teste Manual com Curl
Veja exemplos em **API-EXAMPLES.json**

--- 

🎊 **Bem-vindo ao Aevalo Backend!** 🎊
