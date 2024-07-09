<script setup lang="ts">
import { type Ref, computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import IndividualWord from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";
import type { Word } from "@/types";
import { toast } from "vue-sonner";

const props = defineProps(["sentence"]);
const words: Ref<Word[] | undefined> = ref(undefined);
const selected_word: Ref<Word | undefined> = ref(undefined);
const selected_index: Ref<number> = ref(0);

await invoke("start_stanza").catch((error) => {
  toast.error(error);
});
console.log("Stanza loaded");

set_words();

const DEFAULT_WORDS_AROUND = 25;

const sentence = computed(() => {
  let intendedSent = "";
  for (
    let i = selected_index.value - DEFAULT_WORDS_AROUND;
    i < selected_index.value + DEFAULT_WORDS_AROUND;
    i++
  ) {
    const curWord = words.value?.[i];
    if (curWord) {
      if (curWord.clickable) {
        intendedSent += ` ${curWord.text}`;
      } else {
        intendedSent += curWord.text;
      }
    }
  }
  return intendedSent;
});

async function set_words() {
  words.value = await invoke<Word[]>("parse_text", {
    sent: props.sentence,
  }).catch((error) => {
    toast.error(error);
    return [];
  });
}

function handle_word_selected(word: Word, index: number) {
  selected_word.value = word;
  selected_index.value = index;
}

async function changeRating(
  rating: number,
  attemptedLemma: string,
  modifiable = false,
) {
  console.log(attemptedLemma);
  words.value?.forEach((word, i, vals) => {
    if (word.lemma === attemptedLemma) {
      vals[i].rating = rating;
    }
  });
  await invoke("update_word_knowledge", {
    word: attemptedLemma,
    rating,
    modifiable,
  });
}
</script>

<template>
  <Suspense>
    <SelectedWordView
      class="float-right m-3 w-96"
      v-if="selected_word"
      v-model="words![selected_index]"
      :sentence
      @set-rating="changeRating"
    />
  </Suspense>
  <div class="flex flex-wrap py-3 px-6">
    <IndividualWord
      v-for="(word, index) in words"
      :word="word"
      :rating="word.rating"
      @selected="(w) => handle_word_selected(w, index)"
      @set-rating="changeRating"
    />
  </div>
</template>
