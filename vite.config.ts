import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { fileURLToPath, URL } from "node:url";

const devApiUrl = process.env.MIONOTE_DEV_API_URL || "http://127.0.0.1:4233";

export default defineConfig({
  plugins: [vue()],
  root: "client",
  base: "",
  resolve: {
    alias: {
      "@": fileURLToPath(new URL("./client/src", import.meta.url)),
    },
  },
  server: {
    // Note: The MIONOTE_PATH_PREFIX environment variable is not supported by the dev server
    port: 5173,
    proxy: {
      "/api/": {
        target: devApiUrl,
        changeOrigin: true,
      },
      "/attachments/": {
        target: devApiUrl,
        changeOrigin: true,
      },
      "/docs": {
        target: devApiUrl,
        changeOrigin: true,
      },
      "/openapi.json": {
        target: devApiUrl,
        changeOrigin: true,
      },
    },
  },
});
