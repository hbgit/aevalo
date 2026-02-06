/**
 * @file stores/auth.ts
 * @description Pinia store for authentication and session management
 */

import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import type { User, Session, LoginRequest, LoginResponse } from '../types/auth'
import { isTokenExpired, generateDeviceFingerprint } from '../types/utils'
import { supabase } from '../lib/supabase' // Você precisa criar esse arquivo

const STORAGE_KEYS = {
  ACCESS_TOKEN: 'ae_at',
  SESSION_ID: 'ae_sid',
  LOGIN_AT: 'ae_login_at',
  USER: 'ae_user',
  USER_PREFS: 'ae_user_prefs',
  DEVICE_FP: 'ae_device_fp'
}

export const useAuthStore = defineStore('auth', () => {
  // State
  const accessToken = ref<string | null>(
    sessionStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN)
  )
  const user = ref<User | null>(
    JSON.parse(localStorage.getItem(STORAGE_KEYS.USER) || 'null')
  )
  const sessionId = ref<string | null>(
    sessionStorage.getItem(STORAGE_KEYS.SESSION_ID)
  )
  const isLoading = ref(false)
  const error = ref<string | null>(null)

  // Computed
  const isAuthenticated = computed(() => !!accessToken.value && !!user.value)
  const userEmail = computed(() => user.value?.email || null)
  const userName = computed(() => user.value?.name || null)
  const userTheme = computed(() => user.value?.preferences.theme || 'auto')

  /**
   * Generates a unique session ID
   */
  function generateSessionId(): string {
    const timestamp = Date.now().toString(36)
    const randomStr = Math.random().toString(36).substring(2, 15)
    return `${timestamp}-${randomStr}`
  }

  /**
   * Sets the current session with tokens and user data
   */
  function setSession(response: LoginResponse): void {
    accessToken.value = response.access_token
    user.value = response.user
    sessionId.value = response.session_id

    // Store in sessionStorage (Access Token - dies with tab)
    sessionStorage.setItem(STORAGE_KEYS.ACCESS_TOKEN, response.access_token)
    sessionStorage.setItem(STORAGE_KEYS.SESSION_ID, response.session_id)
    sessionStorage.setItem(STORAGE_KEYS.LOGIN_AT, new Date().toISOString())

    // Store in localStorage (User Profile - persistent)
    localStorage.setItem(STORAGE_KEYS.USER, JSON.stringify(response.user))
    localStorage.setItem(
      STORAGE_KEYS.USER_PREFS,
      JSON.stringify(response.user.preferences)
    )
    localStorage.setItem(STORAGE_KEYS.DEVICE_FP, generateDeviceFingerprint())

    error.value = null
  }

  /**
   * Gets a valid access token, refreshing if necessary
   */
  async function getValidToken(): Promise<string> {
    // If token is in sessionStorage and still valid, use it
    if (accessToken.value && !isTokenExpired(accessToken.value)) {
      return accessToken.value
    }

    // Try to refresh token using Supabase SDK
    try {
      const { data, error: refreshError } = await supabase.auth.refreshSession()

      if (refreshError || !data.session?.access_token) {
        throw new Error('Token refresh failed')
      }

      accessToken.value = data.session.access_token
      sessionStorage.setItem(STORAGE_KEYS.ACCESS_TOKEN, data.session.access_token)

      return data.session.access_token
    } catch (err) {
      // Token refresh failed - clear session
      clearSession()
      error.value = 'Sessão expirada. Por favor, faça login novamente.'
      throw new Error('Session expired')
    }
  }

  /**
   * Performs login
   */
  async function login(credentials: LoginRequest): Promise<void> {
    isLoading.value = true
    error.value = null

    try {
      // Call backend login endpoint
      const response = await fetch('/api/auth/login', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(credentials)
      })

      if (!response.ok) {
        const errorData = await response.json()
        throw new Error(errorData.message || 'Falha no login')
      }

      const loginResponse: LoginResponse = await response.json()
      setSession(loginResponse)
    } catch (err) {
      error.value = (err as Error).message || 'Erro ao fazer login'
      throw err
    } finally {
      isLoading.value = false
    }
  }

  /**
   * Performs logout
   */
  async function logout(): Promise<void> {
    try {
      // Sign out from Supabase
      await supabase.auth.signOut()

      // Notify other tabs
      localStorage.setItem('ae_revoked_tokens', new Date().toISOString())

      // Clear session
      clearSession()
    } catch (err) {
      console.error('Logout error:', err)
      clearSession()
    }
  }

  /**
   * Clears all session data
   */
  function clearSession(): void {
    accessToken.value = null
    user.value = null
    sessionId.value = null
    error.value = null

    // Clear sessionStorage
    sessionStorage.removeItem(STORAGE_KEYS.ACCESS_TOKEN)
    sessionStorage.removeItem(STORAGE_KEYS.SESSION_ID)
    sessionStorage.removeItem(STORAGE_KEYS.LOGIN_AT)

    // Keep localStorage for UX (theme, etc.)
  }

  /**
   * Checks if current session is still valid
   */
  function isSessionValid(): boolean {
    return !!(accessToken.value && user.value && sessionId.value) &&
      !isTokenExpired(accessToken.value)
  }

  /**
   * Restores session from storage (on app initialization)
   */
  function restoreSession(): void {
    const storedToken = sessionStorage.getItem(STORAGE_KEYS.ACCESS_TOKEN)
    const storedUser = localStorage.getItem(STORAGE_KEYS.USER)
    const storedSessionId = sessionStorage.getItem(STORAGE_KEYS.SESSION_ID)

    if (storedToken && storedUser && storedSessionId) {
      accessToken.value = storedToken
      user.value = JSON.parse(storedUser)
      sessionId.value = storedSessionId

      if (isTokenExpired(storedToken)) {
        clearSession()
      }
    }
  }

  /**
   * Updates user profile
   */
  function updateUserProfile(updates: Partial<User>): void {
    if (user.value) {
      user.value = { ...user.value, ...updates }
      localStorage.setItem(STORAGE_KEYS.USER, JSON.stringify(user.value))
    }
  }

  /**
   * Updates user preferences
   */
  function updateUserPreferences(prefs: Partial<User['preferences']>): void {
    if (user.value) {
      user.value.preferences = {
        ...user.value.preferences,
        ...prefs
      }
      localStorage.setItem(
        STORAGE_KEYS.USER_PREFS,
        JSON.stringify(user.value.preferences)
      )
      localStorage.setItem(STORAGE_KEYS.USER, JSON.stringify(user.value))
    }
  }

  return {
    // State
    accessToken,
    user,
    sessionId,
    isLoading,
    error,

    // Computed
    isAuthenticated,
    userEmail,
    userName,
    userTheme,

    // Methods
    setSession,
    getValidToken,
    login,
    logout,
    clearSession,
    isSessionValid,
    restoreSession,
    updateUserProfile,
    updateUserPreferences
  }
})
