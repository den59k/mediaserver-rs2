import { reactive } from "vue"

export class RoomStore {

  messages = reactive<{ text: string }[]>([])
  ws: WebSocket

  constructor(token: string) {
    this.ws = new WebSocket(window.location.origin.replace("http", "ws")+"/ws?token="+token)
    this.ws.addEventListener("message", (e) => {
      const msg = JSON.parse(e.data)
      if (msg.type === "message") {
        this.messages.push({ text: msg.text })
      }
    })
    this.ws.addEventListener("close", () => {
      console.log("WebSocket closed")
    })
  }

  sendMessage(text: string) {
    this.ws.send(JSON.stringify({ type: "message", text }))
  }

}