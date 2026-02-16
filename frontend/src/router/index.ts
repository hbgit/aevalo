import { createRouter, createWebHistory } from 'vue-router'
import DashboardPage from '../pages/Dashboard.vue'
import { NotFound404, ServerError500, ServiceUnavailable503, Unauthorized403, StatusPage } from '../pages/errors'

const routes = [
  {
    path: '/',
    name: 'Landing',
    component: () => import('../pages/Landing.vue'),
    meta: { layout: 'landing' },
  },
  {
    path: '/dashboard',
    name: 'Dashboard',
    component: DashboardPage,
  },
  {
    path: '/evaluation/:id',
    name: 'EvaluationDetail',
    component: () => import('../pages/EvaluationDetail.vue'),
  },
  {
    path: '/create',
    name: 'CreateEvaluation',
    component: () => import('../pages/CreateEvaluation.vue'),
  },
  {
    path: '/login',
    name: 'Login',
    component: () => import('../pages/Login.vue'),
    meta: { layout: 'auth' },
  },
  {
    path: '/register',
    name: 'Register',
    component: () => import('../pages/Register.vue'),
    meta: { layout: 'auth' },
  },
  // Error pages (for testing and programmatic use)
  {
    path: '/error/500',
    name: 'ServerError',
    component: ServerError500,
    meta: { layout: 'minimal' },
  },
  {
    path: '/error/503',
    name: 'ServiceUnavailable',
    component: ServiceUnavailable503,
    meta: { layout: 'minimal' },
  },
  {
    path: '/error/403',
    name: 'Unauthorized',
    component: Unauthorized403,
    meta: { layout: 'minimal' },
  },
  {
    path: '/status',
    name: 'Status',
    component: StatusPage,
    meta: { layout: 'minimal' },
  },
  // Admin Routes
  {
    path: '/admin',
    name: 'AdminDashboard',
    component: () => import('../pages/AdminDashboard.vue'),
    meta: { layout: 'admin', requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/admin/users',
    name: 'AdminUsers',
    component: () => import('../pages/AdminUsers.vue'),
    meta: { layout: 'admin', requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/admin/evaluations',
    name: 'AdminEvaluations',
    component: () => import('../pages/AdminEvaluations.vue'),
    meta: { layout: 'admin', requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/admin/billing',
    name: 'AdminBilling',
    component: () => import('../pages/AdminBilling.vue'),
    meta: { layout: 'admin', requiresAuth: true, requiresAdmin: true },
  },
  {
    path: '/admin/settings',
    name: 'AdminSettings',
    component: () => import('../pages/AdminSettings.vue'),
    meta: { layout: 'admin', requiresAuth: true, requiresAdmin: true },
  },
  // 404 Catch-all route - must be last
  {
    path: '/:pathMatch(.*)*',
    name: 'NotFound',
    component: NotFound404,
    meta: { layout: 'minimal' },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
