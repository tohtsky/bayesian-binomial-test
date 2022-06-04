import Vue from "vue";
import Vuetify from "vuetify/lib/framework";
import App from "./App.vue";
import init from "./bayesian-wasm/pkg";

Vue.use(Vuetify);
Vue.config.productionTip = false;

async function main() {
  await init();
  let vuetify = new Vuetify({
    icons: {
      iconfont: "mdi",
    },
  });

  new Vue({
    render: (h) => h(App),
    vuetify,
  }).$mount("#app");
}

main();
