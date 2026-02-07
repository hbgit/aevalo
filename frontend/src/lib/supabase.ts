/**
 * @file lib/supabase.ts
 * @description Supabase client initialization
 */

import { createClient } from '@supabase/supabase-js'

const supabaseUrl = import.meta.env.VITE_SUPABASE_URL || ''
const supabaseAnonKey = import.meta.env.VITE_SUPABASE_ANON_KEY || ''

// For development, allow missing credentials with a warning
if (!supabaseUrl || !supabaseAnonKey) {
  console.warn('⚠️ Supabase environment variables not properly configured. Using development mode.')
  console.warn('Set VITE_SUPABASE_URL and VITE_SUPABASE_ANON_KEY in .env.local')
}

// Use dummy values for development if not set
const url = supabaseUrl || 'https://dummy.supabase.co'
const key = supabaseAnonKey || 'dummy-key'

export const supabase = createClient(url, key, {
  auth: {
    persistSession: true,
    storage: localStorage,
    autoRefreshToken: true,
    detectSessionInUrl: true
  }
})
