<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

const props = defineProps(["lemma"]);

interface Definition {
  t: string;
  conts: string;
}

const definition = computedAsync(async (): Promise<Definition[]> => {
  return await invoke("get_defs", { lemma: props.lemma });
}, []);
</script>

<template>
  <div v-for="def in definition">
    <span v-if="def['t'] == 'Ok'" v-html="def['conts']"></span>
    <span v-else class="text-pink-600">{{ def.conts }}</span>
  </div>
</template>
