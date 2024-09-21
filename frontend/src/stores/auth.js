import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useAuthStore = defineStore('auth', () => {
  const token = ref('')

  function isLoggedIn() {
    return token.value != null && token.value !== ''
  }

  function updateToken(newToken) {
    token.value = newToken
  }

  return { token, isLoggedIn, updateToken }
}, { persist: true })
