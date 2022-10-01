import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";
import { VueQueryPlugin } from "vue-query";
import configJson from "./config.json";

import "./assets/main.css";

import appStore from "./store";

const app = createApp(App);

app.use(router).use(VueQueryPlugin);

fetch(configJson.wellKnownConfigUrl)
  .then((response) => response.json())
  .then((result) => {
    appStore.apiOrganization = result["apiOrganization"];
    app.mount("#app");
  });
