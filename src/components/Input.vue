<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
import { Button } from '@/components/ui/button'

const greetMsg = ref("");
const name = ref("");
const word = ref("labas");
const router = useRouter();

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  greetMsg.value = await invoke("greet", { name: name.value });
}

function goToWord() {
  console.log(word.value);
  router.push(`/words/${word.value}`)
}
</script>

<template>
  <form class="row" @submit.prevent="greet">
    <input id="greet-input" v-model="name" placeholder="Enter a name..." />
    <button type="submit">Greet</button>
  </form>

  <p>{{ greetMsg }}</p>

  <input v-model="word" />

  <Button @click=goToWord>Yes that's true</button>
</template>
