<script setup lang="ts">
import { StartupState } from "./types";
import GettingStarted from "./pages/GettingStarted.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import PageFrame from "./PageFrame.vue";
import { Toaster } from "@/components/ui/sonner";

const startup = ref(await invoke<StartupState>("get_startup_state"));
</script>

<template>
  <Toaster closeButton richColors />
  <template v-if="startup.first_time">
    <GettingStarted v-model="startup.first_time" />
  </template>
  <template v-else>
    <PageFrame :startup />
  </template>
</template>
