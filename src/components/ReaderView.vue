<script setup lang="ts">
import { Ref, computed, ref } from "vue";
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

if (await invoke("get_dark_mode")) {
  document.documentElement.classList.add("dark");
}

const props = defineProps(["sentence"]);
const words: Ref<[Word] | undefined> = ref(undefined);
const selected_word: Ref<Word | undefined> = ref(undefined);
const selected_index: Ref<number> = ref(0);

set_words();

const DEFAULT_WORDS_AROUND = 25;

const sentence = computed(() => {
  let intendedSent = "";
  for (
    let i = selected_index.value - DEFAULT_WORDS_AROUND;
    i < selected_index.value + DEFAULT_WORDS_AROUND;
    i++
  ) {
    const curWord = words.value![i];
    if (curWord) {
      if (curWord.clickable) {
        intendedSent += " " + curWord.text;
      } else {
        intendedSent += curWord.text;
      }
    }
  }
  return intendedSent;
});

async function set_words() {
  words.value = await invoke("parse_text", { sent: props.sentence });
}

function handle_word_selected(word: Word, index: number) {
  selected_word.value = word;
  selected_index.value = index;
}

async function changeRating(rating: number, attemptedLemma: string) {
  words.value!.forEach((word, i, vals) => {
    if (word["lemma"] == attemptedLemma) {
      vals[i]["rating"] = rating;
    }
  });
  await invoke("update_word_knowledge", { word: attemptedLemma, rating });
}
</script>

<template>
  <Suspense>
    <SelectedWordView
      class="float-right w-96 m-3"
      v-if="selected_word"
      :word="selected_word"
      :sentence
      @set-rating="changeRating"
    />
  </Suspense>
  <div class="flex flex-wrap px-6 py-3">
    <Word
      v-for="(word, index) in words"
      :word="word"
      :rating="word.rating"
      @selected="(w) => handle_word_selected(w, index)"
      @set-rating="changeRating"
    />
  </div>
</template>
