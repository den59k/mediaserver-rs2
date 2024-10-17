import { defineStore } from "pinia";
import { ref } from "vue";
import { request } from "../utils/request";

export const useAppStore = defineStore('app', () => {

  const page = ref("init")
  const token = ref("")

  const openRoom = async (roomId: string) => {

    const resp = await request(`/rooms/${roomId}/register-user`, {})
    token.value = resp.token
    page.value = "room"
  }

  return {
    page,
    token,
    openRoom
  }
})