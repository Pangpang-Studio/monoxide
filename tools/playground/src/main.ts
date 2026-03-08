import { createApp } from 'vue'
import {
  createRouter,
  createWebHashHistory,
  createWebHistory,
} from 'vue-router'
import App from './App.vue'
import './style.css'
import { IS_STATIC_MODE } from './lib/api'

const history = (IS_STATIC_MODE ? createWebHashHistory : createWebHistory)(
  import.meta.env.BASE_URL,
)

const router = createRouter({
  history,
  routes: [
    {
      path: '/',
      component: () => import('./pages/OverviewPage.vue'),
      name: 'home',
    },
    {
      path: '/glyph/:glyphId',
      component: () => import('./pages/GlyphPage.vue'),
      props: true,
      name: 'glyph',
    },
    { path: '/compare', component: () => import('./pages/ComparePage.vue') },
    {
      path: '/:pathMatch(.*)*',
      component: () => import('./pages/404Page.vue'),
    },
  ],
})

createApp(App).use(router).mount('#app')
