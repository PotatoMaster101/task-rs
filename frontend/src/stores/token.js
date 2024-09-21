import { defineStore } from 'pinia'
import { ref } from 'vue'

export const useTokenStore = defineStore('token', () => {
  const token = ref('')

  function updateToken(newToken) {
    token.value = newToken
  }

  return { token, updateToken }
})
