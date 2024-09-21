import { createAuth0 } from '@auth0/auth0-vue'
import { createPinia } from 'pinia'
import { createApp } from 'vue'
import { createMemoryHistory, createRouter } from 'vue-router'
import App from '@/App.vue'
import Aura from '@primevue/themes/aura'
import PrimeVue from 'primevue/config'

const routes = [
  { name: 'home', path: '/', component: () => import('@/views/Home.vue') },
]

const app = createApp(App)
app.use(createPinia())
app.use(createRouter({
  history: createMemoryHistory(),
  routes,
}))

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
