import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'

const server =  "http://localhost:3000"
// const server = "https://streaming.jt3.ru"

export default defineConfig({
  plugins: [vue()],
  server: {
    proxy: {
      "/rooms": server,
      "/ws": {
        ws: true,
        target: server.replace("http", "ws"),
        changeOrigin: true,
        rewriteWsOrigin: true
      }
    }
  },
})