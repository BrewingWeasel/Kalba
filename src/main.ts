import { createApp } from "vue";
import { createRouter, createWebHistory } from "vue-router";

import "./styles.css";
import App from "./App.vue";
import Input from "./components/Input.vue";
import Settings from "./components/Settings.vue";
import WordView from "./components/WordView.vue";
import ReaderView from "./components/ReaderView.vue";

const router = createRouter({
	history: createWebHistory(),
	routes: [
		{ path: "/", component: Input },
		{ path: "/reader", component: ReaderView },
		{ path: "/settings", component: Settings },
		{ path: "/words/:word", component: WordView },
		// { path: "/reader", component: Reader },
	],
});

const app = createApp(App);

app.use(router);

app.mount("#app");
