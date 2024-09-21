<template>
  <Menubar :model="routes">
    <template #item="{ item, props }">
      <router-link :to="item.route" v-slot="{ navigate }">
        <button @click="navigate" v-bind="props">
          {{ item.label }}
        </button>
      </router-link>
    </template>
  </Menubar>
</template>

<script setup>
import Menubar from "primevue/menubar"
import { ref } from "vue"

const routes = ref([
  {
    label: "Tasks",
    route: "/",
  }
])
</script>
