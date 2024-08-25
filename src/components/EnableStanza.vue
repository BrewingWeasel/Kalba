<script setup lang="ts">
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Info } from "lucide-vue-next";
import { invoke } from "@tauri-apps/api/core";
import { ref } from "vue";
import { listen } from "@tauri-apps/api/event";
import { toast } from "vue-sonner";
import { Label } from "@/components/ui/label";
import { Switch } from "@/components/ui/switch";
import { Button } from "@/components/ui/button";

const isInstalled = defineModel<boolean>("installed", { required: true });
const enabled = defineModel<boolean>("enabled", { required: true });

const installMessage = ref("Downloading...");
const finishedInstall = ref(false);

const errored = ref(false);

await listen<{ message: string }>("stanzaDownloadUpdate", (event) => {
  installMessage.value = event.payload.message;
});

async function installStanza() {
  await invoke("setup_stanza").catch((e) => {
    toast.error(e);
    errored.value = true;
  });
  installMessage.value = errored.value
    ? "Installation failed."
    : "Installation complete.";
  finishedInstall.value = true;
}

async function uninstallStanza() {
  await invoke("uninstall_stanza").catch((e) => {
    toast.error(e);
  });
  isInstalled.value = false;
  finishedInstall.value = false;
  toast.info("Stanza uninstalled.");
}
</script>

<template>
  <div v-if="isInstalled">
    <Label for="stanza-enabled">Enable Stanza</Label>
    <Switch id="stanza-enabled" v-model:checked="enabled" />
    <br />
    <Button class="mt-2" @click="uninstallStanza">Uninstall stanza</Button>
  </div>
  <div v-else>
    <AlertDialog>
      <AlertDialogTrigger>
        <Button @click="installStanza" variant="outline">Install Stanza</Button>
      </AlertDialogTrigger>
      <AlertDialogContent>
        <AlertDialogHeader>
          <AlertDialogTitle>Install Stanza</AlertDialogTitle>
        </AlertDialogHeader>
        {{ installMessage }}
        <AlertDialogFooter>
          <AlertDialogAction
            @click="
              if (!errored) {
                isInstalled = true;
                enabled = true;
              }
            "
            v-if="finishedInstall"
            >Continue</AlertDialogAction
          >
        </AlertDialogFooter>
      </AlertDialogContent>
    </AlertDialog>
    <Alert class="mt-4 w-fit" variant="destructive">
      <Info class="h-4 w-4" />
      <AlertTitle>Stanza Installation</AlertTitle>
      <AlertDescription>
        Installing Stanza will take several minutes and use multiple gigabytes
        of disk space.
      </AlertDescription>
    </Alert>
  </div>
  <Alert class="mt-4 w-fit">
    <Info class="h-4 w-4" />
    <AlertTitle>Stanza Usage</AlertTitle>
    <AlertDescription>
      Stanza can be used to automatically parse grammar and determine the root
      word for over 70 languages. Some languages with writing systems that less
      closely match the Latin script (ex: Japanese) require stanza.
    </AlertDescription>
  </Alert>
</template>
