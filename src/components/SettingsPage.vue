<script setup lang="ts">
import { Suspense } from "vue";
import Settings from "./Settings.vue";
import { invoke } from "@tauri-apps/api/tauri";

if (invoke("get_dark_mode")) {
  document.documentElement.classList.add("dark");
}
</script>

<template>
  <Suspense><Settings /></Suspense>
</template>
