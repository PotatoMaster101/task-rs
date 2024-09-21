<template>
  <div>
    <button @click="onLogout">Log Out</button>
  </div>
</template>

<script setup>
import { useAuth0 } from '@auth0/auth0-vue'

const { logout } = useAuth0()

const onLogout = async () => {
  await logout({ logoutParams: { returnTo: window.location.origin } })
}
</script>
