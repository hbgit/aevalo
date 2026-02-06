/**
 * @file auth.ts
 * @description Type definitions for authentication and session management
 */

export interface User {
  id: string
  email: string
  name: string
  preferences: UserPreferences
  created_at: string
  updated_at: string
}

export interface UserPreferences {
  theme: 'light' | 'dark' | 'auto'
  language: 'pt-BR' | 'en-US' | 'es-ES'
  notifications_enabled: boolean
}

export interface Session {
  access_token: string
  refresh_token: string
  user: User
  expires_in: number
  token_type: string
}

export interface LoginRequest {
  email: string
  password: string
}

export interface LoginResponse {
  access_token: string
  user: User
  session_id: string
  expires_in: number
}

export interface RegisterRequest {
  email: string
  password: string
  name: string
}

export interface RefreshTokenRequest {
  refresh_token: string
}

export interface UserContext {
  id: string
  email: string
  iat: number
  exp: number
}

export interface AuthError {
  code: string
  message: string
  details?: Record<string, any>
}

export interface SecurityEvent {
  id: string
  session_id: string
  user_id: string
  event_type: string
  description: string
  ip_address: string
  created_at: string
}

export interface SecurityAlert {
  id: string
  user_id: string
  alert_type: string
  severity: 'low' | 'medium' | 'high' | 'critical'
  resolved: boolean
  created_at: string
  resolved_at: string | null
}
