import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { useDark, useToggle } from "@vueuse/core";

import App from "./App.vue";
import Settings from "./pages/settings/Index.vue";
import Reader from "./pages/reader/Index.vue";
import "./styles.css";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: Reader },
    {
      path: "/reader",
      component: Reader,
    },
    { path: "/settings", component: Settings },
  ],
});

const SyncApp = {
  template: "<Suspense><App /></Suspense>",
  components: { App },
};

const app = createApp(SyncApp);

app.use(router);

app.mount("#app");

useDark();
