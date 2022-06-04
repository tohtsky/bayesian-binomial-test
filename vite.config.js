import { createVuePlugin } from "vite-plugin-vue2";
import { defineConfig } from "vite";
import { viteSingleFile } from "vite-plugin-singlefile";
import { VuetifyResolver } from "unplugin-vue-components/resolvers";
import Components from "unplugin-vue-components/vite";

export default defineConfig({
  build: {
    assetsInlineLimit: 100000000,
  },
  plugins: [
    createVuePlugin(),
    Components({
      resolvers: [
        // Vuetify
        VuetifyResolver(),
      ],
    }),
    viteSingleFile({
      removeViteModuleLoader: true,
    }),
  ],
});
