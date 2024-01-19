<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// import { useRouter } from "vue-router";
// import { Button } from '@/components/ui/button'
import Word from "@/components/Word.vue";

const sentence = "Noriu tapti programuotoju.";
const words = ref([]);
const selected_word = ref("");
set_words();


async function set_words() {
  words.value = await invoke("parse_text", { sent: sentence });
}

function handle_word_selected(word) {
  selected_word.value = word;
}

</script>

<template>
  <h1>Reader</h1>
  <h2>Selected word</h2>
  <Word v-for="word in words" :word="word" @selected="handle_word_selected" />
</template>
