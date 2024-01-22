<script setup lang="ts">
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import Word from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";

interface Word {
  text: string;
  lemma: string;
  morph: any;
  clickable: boolean;
  rating: number;
}

const props = defineProps(["sentence"]);
const words: Ref<[Word] | undefined> = ref(undefined);
const selected_word: Ref<Word | undefined> = ref(undefined);
set_words();

const body = document.querySelector("body");
body.classList.toggle("dark")


async function set_words() {
  words.value = await invoke("parse_text", { sent: props.sentence });
}

function handle_word_selected(word: Word) {
  selected_word.value = word;
}

async function changeRating(rating: number) {
  const attemptedLemma = selected_word.value.lemma;
  words.value.forEach((word, i, vals) => {
    if (word['lemma'] == attemptedLemma) {
      vals[i]['rating'] = rating;
    }
  });
  await invoke("update_word_knowledge", { word: attemptedLemma, rating });
}

</script>

<template>
  <SelectedWordView class="float-right w-96" v-if="selected_word" :word="selected_word" @change-rating="changeRating" />
  <Word v-for="word in words" :word="word" :rating="word.rating" @selected="handle_word_selected" />
</template>
