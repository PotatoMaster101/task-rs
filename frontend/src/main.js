import App from '@/App.vue'
import Aura from '@primevue/themes/aura'
import piniaPluginPersistedstate from 'pinia-plugin-persistedstate'
import PrimeVue from 'primevue/config'
import { createApp } from 'vue'
import { createAuth0 } from '@auth0/auth0-vue'
import { createMemoryHistory, createRouter } from 'vue-router'
import { createPinia } from 'pinia'
import { useAuthStore } from '@/stores/auth'

const routes = [
  {
    component: () => import('@/views/Home.vue'),
    meta: { requiresAuth: true },
    name: 'home',
    path: '/',
  },
  {
    component: () => import('@/views/Login.vue'),
    name: 'login',
    path: '/login',
  },
  {
    component: () => import('@/views/Logout.vue'),
    name: 'logout',
    path: '/logout',
  },
]

const router = createRouter({
  history: createMemoryHistory(),
  routes,
})

router.beforeEach((to, from, next) => {
  const authStore = useAuthStore()

  if (to.path !== '/login' && !authStore.isLoggedIn()) {
    next({ path: '/login' })
  } else {
    next()
  }
})

const pinia = createPinia()
pinia.use(piniaPluginPersistedstate)

const app = createApp(App)
app.use(pinia)
app.use(router)

app.use(PrimeVue, {
  theme: {
    options: {
      cssLayer: false,
      darkModeSelector: 'system',
      prefix: 'p',
    },
    preset: Aura,
  },
})

app.use(createAuth0({
  authorizationParams: {
    redirect_uri: window.location.origin,
  },
  clientId: import.meta.env.VITE_CLIENT_ID,
  domain: import.meta.env.VITE_DOMAIN,
}))

app.mount('#app')
