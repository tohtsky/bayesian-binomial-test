import vue from "@vitejs/plugin-vue";
import { defineConfig } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";

export default defineConfig({
  build: {
    assetsInlineLimit: 100000000,
  },
  plugins: [
    vue(),
    viteSingleFile({
      removeViteModuleLoader: true,
    }),
  ],
});
