import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes: [
    {
      path: '/',
      component: () => import('../views/MainView.vue'),
      children: [
        {
          path: '',
          redirect: '/home',
        },
        {
          path: 'home',
          component: () => import('../views/HomeView.vue'),
        },
        {
          path: 'about',
          component: () => import('../views/AboutView.vue'),
        },
        {
          path: 'add',
          component: () => import('../views/AddView.vue'),
        },
        {
          path: 'import',
          component: () => import('../views/ImportView.vue'),
        },
        {
          path: 'help',
          component: () => import('../views/HelpView.vue'),
        },
        {
          path: 'setting',
          component: () => import('../views/SettingView.vue'),
        },
        {
          path: 'test',
          component: () => import('../views/TestView.vue'),
        },

      ]
    },
    {
      path: '/upload',
      name: 'upload',
      component: () => import('../views/UploadView.vue'),
    },
    {
      path: '/keyboard',
      name: 'keyboard',
      component: () => import('../views/KeyboardView.vue'),
    },
    {
      path: '/quickly-chat',
      name: 'quickly-chat',
      component: () => import('../views/QuicklyChatView.vue'),
    },


  ],
})

export default router
