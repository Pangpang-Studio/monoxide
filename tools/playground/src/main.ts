import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import { createRouter, createWebHistory } from 'vue-router'

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: '/',
      component: () => import('./pages/Overview.vue'),
      name: 'home',
    },
    {
      path: '/glyph/:glyphId',
      component: () => import('./pages/Glyph.vue'),
      props: true,
      name: 'glyph',
    },
    { path: '/compare', component: () => import('./pages/Compare.vue') },
    { path: '/:pathMatch(.*)*', component: () => import('./pages/404.vue') },
  ],
})

createApp(App).use(router).mount('#app')
