/**
 * Error and System Pages
 * 
 * Centralized exports for all error and system status pages
 * Based on interface_flow.md specifications
 */

export { default as NotFound404 } from './NotFound404.vue'
export { default as ServerError500 } from './ServerError500.vue'

// TODO: Implement additional error pages as per interface_flow.md:
// - ServiceUnavailable503.vue (Maintenance Mode)
// - Unauthorized403.vue (Access Denied)
// - StatusPage.vue (System Status - external subdomain)
