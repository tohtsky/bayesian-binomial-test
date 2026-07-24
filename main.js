import { createApp } from "vue";
import { createVuetify } from "vuetify";
import {
  VApp,
  VAppBar,
  VBtn,
  VCard,
  VCardText,
  VCardTitle,
  VCol,
  VContainer,
  VIcon,
  VMain,
  VRow,
  VSpacer,
  VTextField,
  VToolbarTitle,
} from "vuetify/components";
import { aliases, mdi } from "vuetify/iconsets/mdi";
import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";
import App from "./App.vue";
import init from "./bayesian-wasm/pkg";

async function main() {
  await init();

  const vuetify = createVuetify({
    components: {
      VApp,
      VAppBar,
      VBtn,
      VCard,
      VCardText,
      VCardTitle,
      VCol,
      VContainer,
      VIcon,
      VMain,
      VRow,
      VSpacer,
      VTextField,
      VToolbarTitle,
    },
    icons: {
      defaultSet: "mdi",
      aliases,
      sets: {
        mdi,
      },
    },
  });

  createApp(App).use(vuetify).mount("#app");
}

main();
