<template>
  <div v-if="isAuthenticated">
    <Logout />
    {{ `Token: ${userStore.token}` }}
  </div>
  <div v-else>
    <Login />
  </div>
</template>

<script setup>
import Login from '@/components/Login.vue'
import Logout from '@/components/Logout.vue'
import { useAuth0 } from '@auth0/auth0-vue'
import { useTokenStore } from '@/stores/token'
import { watch } from 'vue'

const { idTokenClaims, isAuthenticated } = useAuth0()
const userStore = useTokenStore()

watch(idTokenClaims, async (newVal, _) => {
  if (newVal) {
    userStore.updateToken(newVal["__raw"])
  }
})
</script>
