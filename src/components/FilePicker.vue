<script setup lang="ts">
import { open } from "@tauri-apps/api/dialog";
import { Label } from "@/components/ui/label";
import { Upload } from "lucide-vue-next";

const file = defineModel<string>({ required: true });

async function getFile() {
  const selected = await open({ multiple: false });
  if (selected && !Array.isArray(selected)) {
    file.value = selected;
  }
}
console.log(file);
</script>

<template>
  <div
    @click="getFile"
    class="flex flex-col justify-center items-center m-3 w-5/6 h-36 rounded-md border-2 border-dashed border-slate-500"
  >
    <Upload />
    <Label class="text-xs italic">{{ file || "Click to select a file" }}</Label>
  </div>
</template>
