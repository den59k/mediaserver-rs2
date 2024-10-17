<template>
  <div v-if="appStore.page === 'init'"></div>
  <EnterRoomPage v-else-if="appStore.page === 'selectRoom'"/>
  <RoomPage v-else-if="appStore.page === 'room'"/>
</template>

<script lang="ts" setup>
import { onMounted } from 'vue';
import EnterRoomPage from './components/EnterRoomPage.vue'
import RoomPage from './components/RoomPage.vue'
import { useAppStore } from './stores/appStore';

const appStore = useAppStore()

onMounted(() => {
  const roomId = window.location.pathname.slice(1)
  if (roomId) {
    appStore.openRoom(roomId)
  } else {
    appStore.page = 'selectRoom'
  }
})

</script>

<style lang="sass">

</style>