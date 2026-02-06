/**
 * @file utils.ts
 * @description Utility functions for authentication
 */

import type { UserContext } from './auth'

/**
 * Decodes JWT payload without verification (for client-side checking only)
 * @param token JWT token
 * @returns Decoded payload
 */
export function decodeJWT(token: string): Record<string, any> {
  try {
    const parts = token.split('.')
    if (parts.length !== 3) {
      throw new Error('Invalid JWT format')
    }

    const decoded = JSON.parse(
      atob(parts[1].replace(/-/g, '+').replace(/_/g, '/'))
    )
    return decoded
  } catch (err) {
    throw new Error('Failed to decode JWT')
  }
}

/**
 * Checks if JWT token is expired
 * @param token JWT token
 * @returns true if expired, false otherwise
 */
export function isTokenExpired(token: string): boolean {
  try {
    const payload = decodeJWT(token)
    const expiryTime = payload.exp * 1000 // Convert to milliseconds
    const currentTime = Date.now()

    // Consider token expired if it expires in less than 1 minute
    return currentTime > expiryTime - 60000
  } catch (err) {
    return true
  }
}

/**
 * Gets time remaining until token expiry (in seconds)
 * @param token JWT token
 * @returns Seconds remaining
 */
export function getTokenTimeRemaining(token: string): number {
  try {
    const payload = decodeJWT(token)
    const expiryTime = payload.exp * 1000
    const currentTime = Date.now()
    return Math.max(0, Math.floor((expiryTime - currentTime) / 1000))
  } catch (err) {
    return 0
  }
}

/**
 * Generates a device fingerprint from browser information
 * @returns Device fingerprint string
 */
export function generateDeviceFingerprint(): string {
  const userAgent = navigator.userAgent
  const platform = navigator.platform
  const language = navigator.language
  const timezone = Intl.DateTimeFormat().resolvedOptions().timeZone
  const screenResolution = `${window.screen.width}x${window.screen.height}`
  const colorDepth = window.screen.colorDepth

  const fingerprint = `${userAgent}|${platform}|${language}|${timezone}|${screenResolution}|${colorDepth}`

  // Simple hash function
  let hash = 0
  for (let i = 0; i < fingerprint.length; i++) {
    const char = fingerprint.charCodeAt(i)
    hash = (hash << 5) - hash + char
    hash = hash & hash // Convert to 32bit integer
  }

  return Math.abs(hash).toString(16)
}

/**
 * Sanitizes a string to prevent XSS attacks
 * @param input String to sanitize
 * @returns Sanitized string
 */
export function sanitizeInput(input: string): string {
  const textarea = document.createElement('textarea')
  textarea.textContent = input
  return textarea.innerHTML
}

/**
 * Validates email format
 * @param email Email to validate
 * @returns true if valid, false otherwise
 */
export function isValidEmail(email: string): boolean {
  const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  return emailRegex.test(email)
}

/**
 * Validates password strength
 * @param password Password to validate
 * @returns Object with validation result and feedback
 */
export function validatePasswordStrength(password: string): {
  isStrong: boolean
  score: number
  feedback: string[]
} {
  const feedback: string[] = []
  let score = 0

  if (password.length >= 8) score++
  else feedback.push('Senha deve ter pelo menos 8 caracteres')

  if (password.length >= 12) score++

  if (/[a-z]/.test(password)) score++
  else feedback.push('Senha deve conter letras minúsculas')

  if (/[A-Z]/.test(password)) score++
  else feedback.push('Senha deve conter letras maiúsculas')

  if (/[0-9]/.test(password)) score++
  else feedback.push('Senha deve conter números')

  if (/[!@#$%^&*]/.test(password)) score++
  else feedback.push('Senha deve conter caracteres especiais (!@#$%^&*)')

  return {
    isStrong: score >= 5,
    score,
    feedback
  }
}
