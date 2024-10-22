import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      name: 'main',
      component: () => import('../views/main/index.vue')
    },
    {
      path: '/test',
      name: 'test',
      component: () => import('../views/test/index.vue')
    },
    {
      path: '/settings',
      name: 'settings',
      component: () => import('../views/settings/index.vue')
    }
  ]
})

export default router
