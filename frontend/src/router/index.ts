import { createRouter, createWebHistory } from 'vue-router'
import DashboardPage from '../pages/Dashboard.vue'

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
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

export default router
