<template>
  <div>
    <DataView :value="props.item" layout="list" />
  </div>
</template>

<script setup>
import DataView from 'primevue/dataview'

const props = defineProps({
  item: Object,
})
</script>
