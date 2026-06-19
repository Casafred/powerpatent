import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      redirect: '/input',
    },
    {
      path: '/input',
      name: 'input',
      component: () => import('../views/InputView.vue'),
      meta: { step: 1, title: '输入材料' },
    },
    {
      path: '/config',
      name: 'config',
      component: () => import('../views/ConfigView.vue'),
      meta: { step: 2, title: '模式与板块' },
    },
    {
      path: '/generate',
      name: 'generate',
      component: () => import('../views/GenerateView.vue'),
      meta: { step: 3, title: '生成与重跑' },
    },
    {
      path: '/export',
      name: 'export',
      component: () => import('../views/ExportView.vue'),
      meta: { step: 4, title: '预览与导出' },
    },
  ],
})

export default router
