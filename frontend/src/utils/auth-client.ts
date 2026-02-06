/**
 * @file utils/auth-client.ts
 * @description GraphQL Apollo client with automatic authentication
 */

import {
  ApolloClient,
  InMemoryCache,
  ApolloLink,
  Observable,
  createHttpLink,
  ApolloError
} from '@apollo/client'
import { useAuthStore } from '../stores/auth'

/**
 * Creates an ApolloLink that handles authentication
 */
const createAuthLink = () => {
  return new ApolloLink((operation, forward) => {
    return new Observable((observer) => {
      let handle: any

      Promise.resolve()
        .then(async () => {
          const authStore = useAuthStore()
          
          try {
            const token = await authStore.getValidToken()
            operation.setContext({
              headers: {
                Authorization: token ? `Bearer ${token}` : '',
                'X-Session-ID': authStore.sessionId || undefined
              }
            })
          } catch (err) {
            console.error('Failed to get auth token:', err)
            // Continue without token
          }
        })
        .then(() => {
          handle = forward(operation).subscribe({
            next: observer.next.bind(observer),
            error: observer.error.bind(observer),
            complete: observer.complete.bind(observer)
          })
        })
        .catch(observer.error.bind(observer))

      return () => {
        if (handle) handle.unsubscribe()
      }
    })
  })
}

/**
 * Creates error link to handle auth errors
 */
const createErrorLink = () => {
  return new ApolloLink((operation, forward) => {
    return forward(operation).map((response) => {
      return response
    }).catch((error: ApolloError) => {
      if (error.graphQLErrors) {
        error.graphQLErrors.forEach((err: any) => {
          if (err.extensions?.code === 'UNAUTHENTICATED') {
            const authStore = useAuthStore()
            authStore.clearSession()
            window.location.href = '/login'
          }
        })
      }

      if (error.networkError) {
        const networkError = error.networkError as any
        if (networkError.status === 401) {
          const authStore = useAuthStore()
          authStore.clearSession()
          window.location.href = '/login'
        }
      }

      return Promise.reject(error)
    })
  })
}

/**
 * Creates and configures Apollo Client
 */
export const createApolloClient = () => {
  const httpLink = createHttpLink({
    uri: import.meta.env.VITE_GRAPHQL_URL || 'http://localhost:3000/graphql',
    credentials: 'include'
  })

  return new ApolloClient({
    link: ApolloLink.from([
      createErrorLink(),
      createAuthLink(),
      httpLink
    ]),
    cache: new InMemoryCache(),
    connectToDevTools: true
  })
}

/**
 * REST API client with auth
 */
export const createAuthenticatedFetcher = () => {
  const authStore = useAuthStore()

  return async (url: string, options: RequestInit = {}) => {
    try {
      const token = await authStore.getValidToken()
      const headers = {
        ...options.headers,
        'Authorization': `Bearer ${token}`,
        'X-Session-ID': authStore.sessionId || undefined
      }

      const response = await fetch(url, {
        ...options,
        headers
      })

      if (response.status === 401) {
        authStore.clearSession()
        window.location.href = '/login'
        throw new Error('Unauthorized')
      }

      return response
    } catch (err) {
      console.error('Fetch error:', err)
      throw err
    }
  }
}
