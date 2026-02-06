/**
 * @file composables/useMultiTabSync.ts
 * @description Synchronization of auth state between browser tabs
 */

import { onMounted, onUnmounted } from 'vue'
import { useAuthStore } from '../stores/auth'

export const useMultiTabSync = () => {
  const authStore = useAuthStore()

  let storageListener: ((event: StorageEvent) => void) | null = null

  /**
   * Initializes multi-tab synchronization
   */
  const initSync = () => {
    storageListener = (event: StorageEvent) => {
      // User profile updated in another tab
      if (event.key === 'ae_user') {
        if (event.newValue === null) {
          // Another tab logged out
          authStore.clearSession()
          window.location.href = '/login'
        } else {
          // Another tab updated user profile
          try {
            authStore.updateUserProfile(JSON.parse(event.newValue))
          } catch (err) {
            console.error('Failed to sync user profile:', err)
          }
        }
      }

      // Tokens were revoked in another tab
      if (event.key === 'ae_revoked_tokens') {
        authStore.clearSession()
        window.location.href = '/login'
      }

      // User preferences updated in another tab
      if (event.key === 'ae_user_prefs') {
        try {
          const prefs = JSON.parse(event.newValue || '{}')
          authStore.updateUserPreferences(prefs)
        } catch (err) {
          console.error('Failed to sync user preferences:', err)
        }
      }
    }

    window.addEventListener('storage', storageListener)
  }

  /**
   * Cleans up event listeners
   */
  const cleanup = () => {
    if (storageListener) {
      window.removeEventListener('storage', storageListener)
      storageListener = null
    }
  }

  /**
   * Broadcasts logout to other tabs
   */
  const broadcastLogout = () => {
    localStorage.setItem('ae_revoked_tokens', new Date().toISOString())
    localStorage.removeItem('ae_user')
  }

  /**
   * Broadcasts user profile update to other tabs
   */
  const broadcastUserUpdate = (user: any) => {
    localStorage.setItem('ae_user', JSON.stringify(user))
  }

  /**
   * Broadcasts preference update to other tabs
   */
  const broadcastPrefsUpdate = (prefs: any) => {
    localStorage.setItem('ae_user_prefs', JSON.stringify(prefs))
  }

  onMounted(() => {
    initSync()
  })

  onUnmounted(() => {
    cleanup()
  })

  return {
    broadcastLogout,
    broadcastUserUpdate,
    broadcastPrefsUpdate,
    cleanup
  }
}
