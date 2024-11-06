<script setup lang="ts">
import { StartupState } from "./types";
import GettingStarted from "./pages/GettingStarted.vue";
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/core";
import { useColorMode } from "@vueuse/core";
import PageFrame from "./PageFrame.vue";
import { Toaster } from "@/components/ui/sonner";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
} from "@/components/ui/alert-dialog";

const startup = ref(await invoke<StartupState>("get_startup_state"));

interface VersionDetails {
  version: string;
  link: string;
  blurb: string;
}

const versionDetails = (await fetch(
  "https://raw.githubusercontent.com/BrewingWeasel/Kalba/main/data/current_version.json",
).then((res) => res.json())) as VersionDetails;

const isNew = ref(
  await invoke<boolean>("check_version", {
    potentiallyNewVersion: versionDetails.version,
  }),
);

let mode = useColorMode({
  modes: {
    "Rose Pine Dawn": "rosePineDawn",
    "Rose Pine Moon": "rosePineMoon",
    "Rose Pine Dark": "rosePine",
    "Simple Dark": "simpleDark",
    "Simple Light": "simpleLight",
  },
});
console.log(mode.value);

switch (mode.value) {
  case "light":
    mode.value = "Simple Light";
    break;
  case "dark":
    mode.value = "Rose Pine Moon";
    break;
  case "auto":
    mode.value = "Simple Light";
}
</script>

<template>
  <Toaster closeButton richColors />
  <AlertDialog v-model:open="isNew">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle
          >New Kalba version ({{ versionDetails.version }})</AlertDialogTitle
        >
        <AlertDialogDescription>
          {{ versionDetails.blurb }}
        </AlertDialogDescription>
      </AlertDialogHeader>
      <AlertDialogFooter>
        <AlertDialogCancel>Ignore</AlertDialogCancel>
        <AlertDialogAction
          ><a :href="versionDetails.link" target="_blank"
            >Check it out</a
          ></AlertDialogAction
        >
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>

  <template v-if="startup.first_time">
    <GettingStarted v-model="startup.first_time" />
  </template>
  <template v-else>
    <PageFrame :startup />
  </template>
</template>
