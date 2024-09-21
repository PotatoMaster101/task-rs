<template>
  <div></div>
</template>

<script setup>
import { onMounted } from 'vue';
import { useAuth0 } from '@auth0/auth0-vue';
import { useAuthStore } from '@/stores/auth'

const { logout } = useAuth0();
const authStore = useAuthStore()

onMounted(() => {
  logout({ returnTo: window.location.origin })
  authStore.updateToken('')
})
</script>
