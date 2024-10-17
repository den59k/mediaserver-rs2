<template>
  <Chat/>
</template>

<script lang="ts" setup>
import { onMounted, provide } from 'vue';
import { useAppStore } from '../stores/appStore';
import Chat from './Chat/Chat.vue';
import { RoomStore } from '../stores/roomStore';

const appStore = useAppStore()
const roomStore = new RoomStore(appStore.token)

provide("roomStore", roomStore)

</script>

<style lang="sass">

</style>