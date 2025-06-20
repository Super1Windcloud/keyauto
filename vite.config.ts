import {defineConfig} from "vite";
import react from "@vitejs/plugin-react";
import path from "path";
import tailwindcss from '@tailwindcss/vite'

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
    plugins: [react() , tailwindcss()],
    // 1. prevent vite from obscuring rust errors
    clearScreen: false,
    // 2. tauri expects a fixed port, fail if that port is not available
    server: {
        port: 3333,
        strictPort: true,
        host: host || false,
        hmr: host
            ? {
                protocol: "ws",
                host,
                port: 3333,
            }
            : undefined,
        watch: {
            // 3. tell vite to ignore watching `src-tauri`
            ignored: ["**/src-tauri/**"],
        },
    },
    resolve: {
        alias: {
            "@": path.resolve(path.join(__dirname, "src"))
            , "~": path.resolve(path.join(__dirname, "src-tauri"))
        }
    }
}));
