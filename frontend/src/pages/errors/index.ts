/**
 * Error and System Pages
 * 
 * Centralized exports for all error and system status pages
 * Based on interface_flow.md specifications
 */

export { default as NotFound404 } from './NotFound404.vue'
export { default as ServerError500 } from './ServerError500.vue'
export { default as ServiceUnavailable503 } from './ServiceUnavailable503.vue'
export { default as Unauthorized403 } from './Unauthorized403.vue'

// TODO: Implement additional error pages as per interface_flow.md:
// - StatusPage.vue (System Status - external subdomain)
