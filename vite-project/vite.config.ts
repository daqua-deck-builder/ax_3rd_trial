import {defineConfig} from 'vite'
import vue from '@vitejs/plugin-vue'
import path from 'path';


// https://vitejs.dev/config/
export default defineConfig({
    plugins: [vue()],
    build: {
        rollupOptions: {
            input: {
                main: path.resolve(__dirname, 'index.html'),
            }
        }
    },
    server: {
        host: '127.0.0.1',
        port: 3001,

        proxy: {
            '/websocket': {
                target: 'http://localhost:3000',
                changeOrigin: true,
                ws: true
            },
            '/api': {
                target: 'http://localhost:3000',
                changeOrigin: true,
            },
        }
    }
})
