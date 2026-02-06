/**
 * @file composables/useAuthInterceptor.ts
 * @description Auth interceptor for HTTP requests
 */

import { useAuthStore } from '../stores/auth'
import type { AxiosInstance, AxiosRequestConfig, AxiosResponse, AxiosError } from 'axios'

export const useAuthInterceptor = (axiosInstance: AxiosInstance) => {
  const authStore = useAuthStore()

  /**
   * Request interceptor - adds auth headers
   */
  const requestInterceptor = async (config: AxiosRequestConfig) => {
    try {
      const token = await authStore.getValidToken()
      if (token) {
        config.headers = {
          ...config.headers,
          'Authorization': `Bearer ${token}`,
          'X-Session-ID': authStore.sessionId || undefined
        }
      }
    } catch (err) {
      console.error('Failed to attach auth token:', err)
      // Continue without token - backend will return 401
    }
    return config
  }

  /**
   * Response interceptor - handles errors
   */
  const responseInterceptor = (response: AxiosResponse) => {
    return response
  }

  /**
   * Error interceptor - handles auth errors
   */
  const errorInterceptor = async (error: AxiosError) => {
    // Handle 401 Unauthorized
    if (error.response?.status === 401) {
      authStore.clearSession()
      window.location.href = '/login'
      return Promise.reject(error)
    }

    // Handle 403 Forbidden
    if (error.response?.status === 403) {
      console.error('Access forbidden')
      return Promise.reject(error)
    }

    return Promise.reject(error)
  }

  /**
   * Setup interceptors on axios instance
   */
  const setupInterceptors = () => {
    axiosInstance.interceptors.request.use(requestInterceptor, (error) => {
      return Promise.reject(error)
    })

    axiosInstance.interceptors.response.use(responseInterceptor, errorInterceptor)
  }

  return {
    setupInterceptors,
    requestInterceptor,
    responseInterceptor,
    errorInterceptor
  }
}
