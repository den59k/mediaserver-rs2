<template>
  <div class="fixed-chat">
    <div class="chat__header">
      Чат в комнате
    </div>
    <div ref="scrollRef" class="chat__view scroll">
      <div class="flex-spacer"></div>
      <div v-for="item in roomStore.messages" class="chat-message-item">
        {{ item.text }}
      </div>
    </div>
    <div class="chat__input-wrapper">
      <VInput v-model="input" multiline placeholder="Введите сообщение" @keydown="onKeyDown"/>
    </div>
  </div>
</template>

<script lang="ts" setup>
import { inject, nextTick, ref, watch } from 'vue';
import VInput from '../VInput.vue';
import { RoomStore } from '../../stores/roomStore';

const input = ref("")
const roomStore = inject("roomStore") as RoomStore

const onKeyDown = (e: KeyboardEvent) => {
  if (e.code === "Enter" && !e.shiftKey) {
    roomStore.sendMessage(input.value)
    input.value = ""
    e.preventDefault()
  }
}

const scrollRef = ref<HTMLDivElement>()
watch(roomStore.messages, () => {
  nextTick(() => {
    if (!scrollRef.value) return
    scrollRef.value.scrollTop = 9999
  })
})

</script>

<style lang="sass">
.fixed-chat
  position: fixed
  width: 400px
  border: 1px solid var(--border-color)
  border-radius: 12px
  right: 30px
  bottom: 30px

.chat__input-wrapper
  padding: 16px

.chat__view
  height: 400px
  backgorund-color: #1A1A20
  display: flex
  overflow-y: auto
  padding: 16px
  box-sizing: border-box
  flex-direction: column
  gap: 8px
  align-items: flex-start
  margin-right: 16px
  padding-right: 4px

  .flex-spacer
    flex: 1 1 auto

.chat-message-item
  padding: 8px 12px
  background-color: var(--paper-color)
  border-radius: 8px
  white-space: pre-wrap
  

.chat__header
  height: 52px
  display: flex
  align-items: center
  padding: 0 16px

</style>