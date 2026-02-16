/**
 * Componentes Base do Sistema Aevalo
 * 
 * Índice centralizado de todos os componentes reutilizáveis
 * Estrutura: UI (genéricos) | Forms (escalas) | Dashboard (específicos) | Shared (layout)
 */

// ==================== UI Components ====================
export * from './ui/index'

// ==================== Form Components ====================
export * from './forms/index'

// ==================== Dashboard Components ====================
export * from './dashboard/index'

// ==================== Shared/Layout Components ====================
export * from './shared/index'

// ==================== Types (opcional) ====================
export type ButtonVariant = 'primary' | 'secondary' | 'ghost' | 'danger'
export type ButtonSize = 'sm' | 'md' | 'lg'
export type BadgeVariant = 'success' | 'warning' | 'danger' | 'info' | 'secondary' | 'primary'
export type ToastType = 'success' | 'error' | 'warning' | 'info' | 'loading'
export type EvaluationStatus = 'open' | 'closed' | 'draft' | 'archived'
export type FieldType = 'text' | 'email' | 'number' | 'likert' | 'fixed_sum' | 'paired_comparison'
