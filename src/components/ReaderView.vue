<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
// import { useRouter } from "vue-router";
// import { Button } from '@/components/ui/button'
import Word from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";

const props = defineProps(["sentence"]);
const words = ref([]);
const selected_word = ref(null);
set_words();

const body = document.querySelector("body");
body.classList.toggle("dark")


async function set_words() {
  words.value = await invoke("parse_text", { sent: props.sentence });
}

function handle_word_selected(word) {
  selected_word.value = word;
}

</script>

<template>
  <SelectedWordView class="float-right w-96" v-if="selected_word" :word="selected_word" />
  <Word v-for="word in words" :word="word" @selected="handle_word_selected" />
</template>
