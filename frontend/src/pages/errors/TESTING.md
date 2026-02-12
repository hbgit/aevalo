# 🧪 Testando Páginas de Erro

Este guia mostra como testar as páginas de erro do sistema no ambiente de desenvolvimento.

---

## 🎯 Método 1: Acessar a Rota Diretamente

### Erro 500 (Server Error)
```bash
# Inicie o servidor de desenvolvimento
npm run dev

# Acesse no navegador:
http://localhost:5173/error/500
```

### Erro 404 (Not Found)
```bash
# Qualquer rota inexistente
http://localhost:5173/pagina-que-nao-existe
http://localhost:5173/abc123
http://localhost:5173/test/nested/404
```

---

## 🔧 Método 2: Navegação Programática (Vue DevTools)

1. Instale a extensão **Vue DevTools** no navegador
2. Abra o DevTools (F12) e vá para a aba Vue
3. No console, execute:

```javascript
// Erro 500
$router.push({ name: 'ServerError' })

// Erro 404
$router.push('/pagina-invalida')

// Voltar
$router.go(-1)
```

---

## 🌐 Método 3: Interceptor de Erros (Recomendado para Produção)

Adicione um interceptor Axios para capturar erros 500 automaticamente:

### Criar arquivo: `src/utils/errorHandler.ts`

```typescript
import { Router } from 'vue-router'
import axios from 'axios'

export const setupErrorInterceptor = (router: Router) => {
  axios.interceptors.response.use(
    response => response,
    error => {
      if (error.response) {
        switch (error.response.status) {
          case 500:
            router.push({ name: 'ServerError' })
            break
          case 404:
            router.push({ name: 'NotFound' })
            break
          case 403:
            // TODO: Implementar página 403
            console.error('Unauthorized access')
            break
          case 503:
            // TODO: Implementar página 503
            console.error('Service unavailable')
            break
        }
      } else if (error.request) {
        // Network error
        console.error('Network error:', error)
        // Opcional: mostrar toast de erro de conexão
      }
      
      return Promise.reject(error)
    }
  )
}
```

### Usar no `src/main.ts`:

```typescript
import { setupErrorInterceptor } from './utils/errorHandler'

const app = createApp(App)
const router = createRouter(/* ... */)

app.use(router)
setupErrorInterceptor(router)

app.mount('#app')
```

---

## 🧩 Método 4: Componente de Teste

Crie um botão de teste em qualquer página:

```vue
<template>
  <div>
    <h1>Testing Error Pages</h1>
    
    <button @click="triggerError(500)">
      Test Error 500
    </button>
    
    <button @click="triggerError(404)">
      Test Error 404
    </button>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()

const triggerError = (code: number) => {
  if (code === 500) {
    router.push({ name: 'ServerError' })
  } else if (code === 404) {
    router.push('/invalid-route')
  }
}
</script>
```

---

## 🔨 Método 5: Simular Erro Real de API

### Criar endpoint de teste no backend:

```rust
// backend/src/handlers/test.rs
#[get("/api/test/error500")]
async fn test_error_500() -> Result<HttpResponse, Error> {
    Err(Error::InternalServer("Test error".to_string()))
}
```

### Fazer requisição no frontend:

```typescript
// Em qualquer componente
const testServerError = async () => {
  try {
    await axios.get('/api/test/error500')
  } catch (error) {
    // Interceptor redirecionará automaticamente para /error/500
  }
}
```

---

## 🎨 Método 6: Vue Router Navigation Guards

Adicione um guard para simular erros:

```typescript
// src/router/index.ts

router.beforeEach((to, from, next) => {
  // Simular erro 500 em rotas específicas (apenas dev)
  if (import.meta.env.DEV && to.query.simulateError === '500') {
    next({ name: 'ServerError' })
    return
  }
  
  next()
})
```

Usar:
```bash
http://localhost:5173/dashboard?simulateError=500
```

---

## 🧪 Método 7: Teste Automatizado (Vitest + Testing Library)

```typescript
// src/pages/errors/__tests__/ServerError500.spec.ts

import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import { createRouter, createMemoryHistory } from 'vue-router'
import ServerError500 from '../ServerError500.vue'

describe('ServerError500', () => {
  it('renders error code and message', () => {
    const wrapper = mount(ServerError500, {
      global: {
        plugins: [
          createRouter({
            history: createMemoryHistory(),
            routes: []
          })
        ]
      }
    })
    
    expect(wrapper.find('h1').text()).toBe('500')
    expect(wrapper.text()).toContain('Algo deu errado')
  })
  
  it('generates unique error ID', () => {
    const wrapper = mount(ServerError500, {
      global: {
        plugins: [
          createRouter({
            history: createMemoryHistory(),
            routes: []
          })
        ]
      }
    })
    
    const errorIdText = wrapper.text()
    expect(errorIdText).toMatch(/#ERR-\d{4}-\d{2}-\d{2}-\d{4}/)
  })
  
  it('triggers retry on button click', async () => {
    const wrapper = mount(ServerError500, {
      global: {
        plugins: [
          createRouter({
            history: createMemoryHistory(),
            routes: []
          })
        ]
      }
    })
    
    const retryButton = wrapper.find('button')
    await retryButton.trigger('click')
    
    // Verificar que isRetrying é verdadeiro
    expect(wrapper.text()).toContain('Tentando...')
  })
})
```

Executar:
```bash
npm run test
```

---

## 🎭 Método 8: Mock Service Worker (MSW)

Para testes mais realistas, use MSW para simular respostas de API:

```typescript
// src/mocks/handlers.ts
import { rest } from 'msw'

export const handlers = [
  rest.get('/api/evaluations', (req, res, ctx) => {
    // Simular erro 500
    return res(
      ctx.status(500),
      ctx.json({ error: 'Internal Server Error' })
    )
  })
]
```

```typescript
// src/mocks/browser.ts
import { setupWorker } from 'msw'
import { handlers } from './handlers'

export const worker = setupWorker(...handlers)
```

```typescript
// src/main.ts
if (import.meta.env.DEV) {
  import('./mocks/browser').then(({ worker }) => {
    worker.start()
  })
}
```

---

## 📊 Método 9: Chrome DevTools - Network Throttling

1. Abra DevTools (F12)
2. Vá para Network
3. Selecione "Offline" ou "Slow 3G"
4. Tente fazer uma requisição
5. O timeout disparará o erro 500

---

## 🚀 Método 10: Link Direto no Menu de Desenvolvimento

Adicione um menu de debug no desenvolvimento:

```vue
<!-- src/components/DevMenu.vue -->
<template>
  <div v-if="isDev" class="fixed bottom-4 right-4 bg-black text-white p-4 rounded-lg shadow-2xl">
    <h3 class="font-bold mb-2">🛠️ Dev Menu</h3>
    <div class="space-y-2">
      <button @click="goto500" class="block w-full text-left">
        Test 500 Error
      </button>
      <button @click="goto404" class="block w-full text-left">
        Test 404 Error
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useRouter } from 'vue-router'

const router = useRouter()
const isDev = import.meta.env.DEV

const goto500 = () => router.push({ name: 'ServerError' })
const goto404 = () => router.push('/invalid')
</script>
```

Adicione no `App.vue`:
```vue
<template>
  <div>
    <RouterView />
    <DevMenu />
  </div>
</template>
```

---

## ✅ Checklist de Testes

Ao testar a página 500, verifique:

- [ ] Código 500 é exibido corretamente
- [ ] ID de erro único é gerado (formato: #ERR-YYYY-MM-DD-XXXX)
- [ ] Timestamp é exibido
- [ ] Botão "Tentar Novamente" funciona
- [ ] Loading state aparece durante retry
- [ ] Botão "Voltar" funciona
- [ ] Botão "Status" abre nova aba (status.aevalo.app)
- [ ] Botão "Reportar" navega/abre modal
- [ ] Card de status do sistema é exibido
- [ ] Auto-retry countdown funciona (5s)
- [ ] Progress bar anima corretamente
- [ ] Auto-retry para após 3 tentativas
- [ ] Dark mode funciona
- [ ] Responsivo em mobile
- [ ] Animações rodam suavemente (engrenagem + explosão)
- [ ] Console.log mostra dados do erro
- [ ] Sentry captura erro (se configurado)

---

## 🎯 Recomendação

**Para desenvolvimento diário:** Use o **Método 1** (acesso direto via URL)
```bash
http://localhost:5173/error/500
```

**Para produção:** Use o **Método 3** (interceptor de erros)

**Para CI/CD:** Use o **Método 7** (testes automatizados)

---

*Última atualização: 11 de fevereiro de 2026*
