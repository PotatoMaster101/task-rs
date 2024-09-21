<template>
  <div>
    <button @click="router.push('/logout')">Logout</button>
    <TaskList :item="null" />
  </div>
</template>

<script setup>
import TaskList from '@/components/TaskList.vue'
import { ref } from 'vue'
import { useRouter } from 'vue-router'

const router = useRouter()
const lists = ref(null)
</script>
