<template>
  <div class="enter-room-page">
    <VCard tag="form" class="enter-room-page__form" @submit="apply">
      <template #header>Введите адрес комнаты</template>
      <div class="form-column">
        <VInput v-bind="register('room')" label="Адрес комнаты"/>
        <VButton @click="apply">Войти</VButton>
      </div>
    </VCard>
  </div>
</template>

<script lang="ts" setup>
import VButton from './VButton.vue';
import VCard from './VCard.vue';
import VInput from './VInput.vue'
import { useAppStore } from '../stores/appStore'
import { useForm } from 'vuesix'

const { register, handleSubmit } = useForm({
  room: ""
})
const appStore = useAppStore()
const apply = handleSubmit(async (values) => {
  await appStore.openRoom(values.room)
  window.history.pushState({}, "", `/${values.room}`)
})

</script>

<style lang="sass">
.enter-room-page
  height: 100vh
  display: flex
  flex-direction: column
  justify-content: center
  align-items: center

.enter-room-page__form
  width: 400px
  background-color: var(--paper-color)
  border-radius: 16px

  .v-card__content
    padding-top: 4px

</style>