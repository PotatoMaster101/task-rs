<template>
  <div></div>
</template>

<script setup>
import { onMounted, watch } from 'vue'
import { useAuth0 } from '@auth0/auth0-vue'
import { useAuthStore } from '@/stores/auth'

const { idTokenClaims, loginWithRedirect } = useAuth0()
const authStore = useAuthStore()

onMounted(() => {
  loginWithRedirect()
})

watch(idTokenClaims, (newVal, _) => {
  if (newVal) {
    authStore.updateToken(newVal["__raw"])
  }
})
</script>
