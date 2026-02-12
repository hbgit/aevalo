import { createRouter, createWebHistory } from 'vue-router'
import DashboardPage from '../pages/Dashboard.vue'
import { NotFound404, ServerError500 } from '../pages/errors'

const routes = [
  {
    path: '/',
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
