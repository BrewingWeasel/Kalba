import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";
import { useDark, useToggle } from '@vueuse/core'

import App from "./App.vue";
import Settings from "./components/SettingsPage.vue";
import WordView from "./components/WordView.vue";
import LoadingPage from "./components/LoadingPage.vue";
import "./styles.css";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: "/", component: () => import("./components/Input.vue") },
    {
      path: "/reader",
      component: () => import("./components/ReaderView.vue"),
    },
    { path: "/settings", component: Settings },
    { path: "/words/:word", component: WordView },
  ],
});

const app = createApp(App);

app.use(router);

app.mount("#app");

useDark();
