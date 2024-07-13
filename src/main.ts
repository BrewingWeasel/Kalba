import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { useDark } from "@vueuse/core";

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
      alias: ["/reader/:id", "/reader/file/:file"],
    },
    {
      path: "/settings",
      component: Settings,
      alias: ["/settings/:tab", "/settings/:language/:tab"],
    },
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
