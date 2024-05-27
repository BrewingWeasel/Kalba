<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";

import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";

const file = defineModel<string>({ required: true });

async function getFile() {
  const selected = await open({ multiple: false });
  if (selected && !Array.isArray(selected)) {
    file.value = selected;
  }
}
</script>

<template>
  <Button @click="getFile">Select file</Button>
  <Label class="text-xs italic">{{ file }}</Label>
</template>
