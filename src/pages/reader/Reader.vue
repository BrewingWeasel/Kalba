<script setup lang="ts">
import { type Ref, computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import IndividualWord from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";
import type { Word, Section } from "@/types";
import { toast } from "vue-sonner";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";

const props = defineProps<{
  sentence: string;
  currentLanguage: string;
  isUrl: boolean;
}>();
const sections: Ref<Section[] | undefined> = ref(undefined);
const selectedWord: Ref<Word | undefined> = ref(undefined);
const selectedSectionIndex: Ref<number> = ref(0);
const selectedWordIndex: Ref<number> = ref(0);

await invoke("start_stanza").catch((error) => {
  toast.error(error);
});
console.log("Stanza loaded");

await set_words();
console.log(sections);

const DEFAULT_WORDS_AROUND = 25;

const sentence = computed(() => {
  let intendedSent = "";

  const section = sections.value?.[selectedSectionIndex.value];
  if (section && typeof section.c != "string") {
    for (
      let i = selectedWordIndex.value - DEFAULT_WORDS_AROUND;
      i < selectedWordIndex.value + DEFAULT_WORDS_AROUND;
      i++
    ) {
      const curWord = section.c[i];
      if (curWord) {
        if (curWord.clickable) {
          intendedSent += ` ${curWord.text}`;
        } else {
          intendedSent += curWord.text;
        }
      }
    }
  }
  return intendedSent;
});

async function set_words() {
  if (props.isUrl) {
    sections.value = await invoke<Section[]>("parse_url", {
      url: props.sentence,
    }).catch((error) => {
      toast.error(error);
      return [];
    });
  } else {
    sections.value = await invoke<Section[]>("parse_text", {
      sent: props.sentence,
    }).catch((error) => {
      toast.error(error);
      return [];
    });
  }
}

function handle_word_selected(word: Word, s_index: number, w_index: number) {
  console.log(word);
  selectedWord.value = word;
  selectedSectionIndex.value = s_index;
  selectedWordIndex.value = w_index;
}

async function changeRating(
  rating: number,
  attemptedLemma: string,
  modifiable = false,
) {
  console.log(attemptedLemma);
  sections.value?.forEach((section) => {
    if (typeof section.c !== "string") {
      section.c.forEach((word, i, vals) => {
        if (word.lemma === attemptedLemma) {
          vals[i].rating = rating;
        }
      });
    }
  });

  await invoke("update_word_knowledge", {
    word: attemptedLemma,
    rating,
    modifiable,
  });
}

const sectionStyling = new Map<string, string>([
  ["Paragraph", "m-1"],
  ["Title", "text-2xl"],
  ["Subtitle", "text-lg font-bold pt-2 pb-1"],
  ["Caption", "text-sm mb-1"],
]);
</script>

<template>
  <ResizablePanelGroup direction="horizontal">
    <ResizablePanel>
      <div class="py-3 px-10 w-1/2">
        <div v-for="(section, s_index) in sections">
          <div v-if="section.t == 'Image' && typeof section.c == 'string'">
            <img :src="section.c" class="mt-1" />
          </div>
          <div
            v-else
            :class="sectionStyling.get(section.t)"
            class="flex flex-wrap"
          >
            <IndividualWord
              v-if="typeof section.c != 'string'"
              v-for="(word, w_index) in section.c"
              :word="word"
              :rating="word.rating"
              @selected="(w) => handle_word_selected(w, s_index, w_index)"
              @set-rating="changeRating"
            />
          </div>
        </div>
      </div>
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel :min-size="20" :max-size="70" :default-size="32">
      <Suspense>
        <SelectedWordView
          v-if="selectedWord"
          v-model="sections![selectedSectionIndex].c[selectedWordIndex]"
          :sentence
          :currentLanguage
          @set-rating="changeRating"
        />
      </Suspense>
    </ResizablePanel>
  </ResizablePanelGroup>
</template>
