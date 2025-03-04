import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";

import App from "./App.vue";
import Settings from "./pages/settings/Index.vue";
import Reader from "./pages/reader/Index.vue";
import Dashboard from "./pages/dashboard/Index.vue";
import ComingSoon from "./pages/ComingSoon.vue";
import "./styles.css";
import { invoke } from "@tauri-apps/api/core";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/dashboard", component: Dashboard, alias: "/" },
    {
      path: "/reader/:_file?/:_?",
      component: Reader,
    },
    {
      path: "/settings/:_lang?/:_?",
      component: Settings,
    },
    {
      path: "/coming-soon",
      component: ComingSoon,
      alias: ["/browse", "/stats"],
    },
  ],
});

router.afterEach(async (_to, _from) => {
  await invoke("switch_page");
  return true;
});

const SyncApp = {
  template: "<Suspense><App /></Suspense>",
  components: { App },
};

const app = createApp(SyncApp);

app.use(router);

app.mount("#app");
