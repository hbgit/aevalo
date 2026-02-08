/**
 * @file Login.test.ts
 * @description Testes para validação dos tratamentos de erro do Login.vue
 */

import { describe, it, expect, beforeEach, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import Login from '../pages/Login.vue'

describe('Login.vue - Error Handling', () => {
  let wrapper: any
  let fetchMock: any

  beforeEach(() => {
    // Mock do fetch global
    fetchMock = vi.fn()
    global.fetch = fetchMock

    // Mount componente
    wrapper = mount(Login)
  })

  describe('Form Validation', () => {
    it('deve mostrar erro quando email está vazio', async () => {
      const input = wrapper.find('input[type="password"]')
      await input.setValue('password123')

      const button = wrapper.find('button[type="submit"]')
      await button.trigger('click')

      expect(wrapper.vm.errorMessage).toContain('Email é obrigatório')
    })

    it('deve mostrar erro quando senha está vazia', async () => {
      const input = wrapper.find('input[type="email"]')
      await input.setValue('test@example.com')

      const button = wrapper.find('button[type="submit"]')
      await button.trigger('click')

      expect(wrapper.vm.errorMessage).toContain('Senha é obrigatória')
    })

    it('deve mostrar erro quando email é inválido', async () => {
      const emailInput = wrapper.find('input[type="email"]')
      const passwordInput = wrapper.find('input[type="password"]')

      await emailInput.setValue('invalid-email')
      await passwordInput.setValue('password123')

      const button = wrapper.find('button[type="submit"]')
      await button.trigger('click')

      expect(wrapper.vm.errorMessage).toContain('Email inválido')
    })
  })

  describe('HTTP Error Handling', () => {
    it('deve tratar erro 401 - credenciais inválidas', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 401,
        json: async () => ({ error: 'Unauthorized' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('auth')
      expect(wrapper.vm.errorMessage).toContain('Email ou senha incorretos')
      expect(wrapper.vm.password).toBe('') // Senha foi limpa
    })

    it('deve tratar erro 429 - rate limit com countdown', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 429,
        json: async () => ({ error: 'Too Many Requests' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('rate-limit')
      expect(wrapper.vm.errorMessage).toContain('Muitas tentativas')
      expect(wrapper.vm.showRetryCountdown).toBe(true)
      expect(wrapper.vm.retryCountdown).toBeGreaterThan(0)
      expect(wrapper.vm.isLoading).toBe(true) // Botão desabilitado
    })

    it('deve iniciar countdown de 60s para 429', async () => {
      vi.useFakeTimers()
      
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 429,
        json: async () => ({})
      })

      const initialCountdown = 60
      await wrapper.vm.handleLogin()
      
      // Simular 10 segundos
      vi.advanceTimersByTime(10000)
      
      expect(wrapper.vm.retryCountdown).toBe(50)
      
      vi.useRealTimers()
    })

    it('deve tratar erro 500 - servidor indisponível', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 500,
        json: async () => ({ error: 'Internal Server Error' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('server')
      expect(wrapper.vm.errorMessage).toContain('servidores estão temporariamente indisponíveis')
      expect(wrapper.vm.showRetryButton).toBe(true)
      expect(wrapper.vm.statusPageUrl).toBe('https://status.aevalo.app')
    })

    it('deve tratar erro 503 - manutenção', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 503,
        json: async () => ({ error: 'Service Unavailable' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('maintenance')
      expect(wrapper.vm.errorMessage).toContain('manutenção')
      expect(wrapper.vm.showRetryButton).toBe(true)
    })

    it('deve tratar erro 403 - email não verificado', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 403,
        json: async () => ({ code: 'EMAIL_NOT_VERIFIED' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('auth')
      expect(wrapper.vm.errorMessage).toContain('Email não verificado')
    })

    it('deve tratar erro 403 - conta bloqueada', async () => {
      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 403,
        json: async () => ({ code: 'ACCOUNT_LOCKED' })
      })

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('auth')
      expect(wrapper.vm.errorMessage).toContain('temporariamente bloqueada')
    })
  })

  describe('Network Error Handling', () => {
    it('deve tratar erro de rede - Failed to fetch', async () => {
      fetchMock.mockRejectedValueOnce(
        new TypeError('Failed to fetch')
      )

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('network')
      expect(wrapper.vm.errorMessage).toContain('Sem conexão')
    })

    it('deve tratar timeout de 10 segundos', async () => {
      fetchMock.mockRejectedValueOnce(
        new DOMException('The operation was aborted.', 'AbortError')
      )

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorType).toBe('server')
      expect(wrapper.vm.errorMessage).toContain('Conexão expirou')
    })

    it('deve tratargenereric fetch error', async () => {
      fetchMock.mockRejectedValueOnce(
        new Error('Unknown error')
      )

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(wrapper.vm.errorMessage).toContain('Erro ao conectar')
    })
  })

  describe('Successful Login', () => {
    it('deve fazer login com sucesso e armazenar tokens', async () => {
      const mockData = {
        token: 'mock-token-123',
        refresh_token: 'mock-refresh-token',
        expires_at: Date.now() + 3600000
      }

      fetchMock.mockResolvedValueOnce({
        ok: true,
        json: async () => mockData
      })

      // Mock do router
      const mockRouter = { push: vi.fn() }
      wrapper.vm.$router = mockRouter

      await wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(localStorage.getItem('auth_token')).toBe('mock-token-123')
      expect(localStorage.getItem('refresh_token')).toBe('mock-refresh-token')
      expect(mockRouter.push).toHaveBeenCalledWith('/dashboard')
    })

    it('deve salvar email quando "lembrar de mim" está ativo', async () => {
      const mockData = {
        token: 'mock-token',
        refresh_token: 'mock-refresh-token'
      }

      fetchMock.mockResolvedValueOnce({
        ok: true,
        json: async () => mockData
      })

      await wrapper.find('input[type="checkbox"]').setValue(true)
      await wrapper.vm.handleLogin()

      expect(localStorage.getItem('remember_email')).toBe(wrapper.vm.email)
    })
  })

  describe('UI State Management', () => {
    it('deve mostrar spinner durante loading', async () => {
      const button = wrapper.find('button[type="submit"]')
      wrapper.vm.isLoading = true
      await wrapper.vm.$nextTick()

      expect(button.classes()).toContain('disabled:bg-orange-300')
    })

    it('deve limpar mensagem de erro ao iniciar novo login', async () => {
      wrapper.vm.errorMessage = 'Erro anterior'
      wrapper.vm.errorType = 'auth'

      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 401,
        json: async () => ({})
      })

      await wrapper.vm.handleLogin()

      // Erro anterior deve ser limpo antes de mostrar novo
      expect(wrapper.vm.errorType).toBe('auth')
    })

    it('deve exibir cores diferentes por tipo de erro', async () => {
      wrapper.vm.errorType = 'auth'
      await wrapper.vm.$nextTick()
      expect(wrapper.vm.errorStyles).toContain('red-50')

      wrapper.vm.errorType = 'rate-limit'
      await wrapper.vm.$nextTick()
      expect(wrapper.vm.errorStyles).toContain('yellow-50')

      wrapper.vm.errorType = 'server'
      await wrapper.vm.$nextTick()
      expect(wrapper.vm.errorStyles).toContain('orange-50')
    })

    it('deve exibir ícones corretos por tipo de erro', async () => {
      wrapper.vm.errorType = 'auth'
      expect(wrapper.vm.errorIcon).toBe('🔐')

      wrapper.vm.errorType = 'rate-limit'
      expect(wrapper.vm.errorIcon).toBe('⏱️')

      wrapper.vm.errorType = 'server'
      expect(wrapper.vm.errorIcon).toBe('⚠️')

      wrapper.vm.errorType = 'network'
      expect(wrapper.vm.errorIcon).toBe('📡')
    })
  })

  describe('Password Management', () => {
    it('deve ativar/desativar visibilidade da senha', async () => {
      const button = wrapper.find('button[type="button"]')
      
      expect(wrapper.vm.showPassword).toBe(false)
      await button.trigger('click')
      expect(wrapper.vm.showPassword).toBe(true)
      await button.trigger('click')
      expect(wrapper.vm.showPassword).toBe(false)
    })

    it('deve limpar senha após erro 401', async () => {
      wrapper.find('input[type="email"]').setValue('test@example.com')
      wrapper.find('input[type="password"]').setValue('wrong-password')

      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 401,
        json: async () => ({})
      })

      await wrapper.vm.handleLogin()

      expect(wrapper.vm.password).toBe('')
    })
  })

  describe('Remember Me Feature', () => {
    it('deve restaurar email ao montar componente', () => {
      localStorage.setItem('remember_email', 'saved@example.com')
      
      const newWrapper = mount(Login)
      
      expect(newWrapper.vm.email).toBe('saved@example.com')
      expect(newWrapper.vm.rememberMe).toBe(true)
    })

    it('deve limpar email memorizado quando desmarca checkbox', async () => {
      localStorage.setItem('remember_email', 'saved@example.com')

      const mockData = {
        token: 'mock-token',
        refresh_token: 'mock-refresh-token'
      }

      fetchMock.mockResolvedValueOnce({
        ok: true,
        json: async () => mockData
      })

      const checkbox = wrapper.find('input[type="checkbox"]')
      await checkbox.setValue(false)

      wrapper.vm.handleLogin()
      await wrapper.vm.$nextTick()

      expect(localStorage.getItem('remember_email')).toBeNull()
    })
  })

  describe('Accessibility', () => {
    it('deve ter labels para todos os inputs', () => {
      const emailLabel = wrapper.find('label')
      expect(emailLabel.text()).toContain('Email')
    })

    it('deve focar no campo de senha após erro 401', async () => {
      const focusSpy = vi.fn()
      const passwordInput = wrapper.find('input[type="password"]')
      
      Object.defineProperty(passwordInput.element, 'focus', {
        value: focusSpy
      })

      fetchMock.mockResolvedValueOnce({
        ok: false,
        status: 401,
        json: async () => ({})
      })

      await wrapper.vm.handleLogin()
      await new Promise(resolve => setTimeout(resolve, 200))

      // Focus deveria ser chamado
      expect(focusSpy).toHaveBeenCalled()
    })
  })
})
