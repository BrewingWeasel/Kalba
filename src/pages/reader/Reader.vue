<script setup lang="ts">
import { type Ref, computed, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import IndividualWord from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";
import type { Word, Section, Definition } from "@/types";
import { toast } from "vue-sonner";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { computedAsync } from "@vueuse/core";
import { Loader2, PanelBottomClose } from "lucide-vue-next";

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

const onDemandDefinitions = ref(new Map<string, undefined | string>());

async function getOnDemandDef(dictionary: string) {
  if (onDemandDefinitions.value.get(dictionary) || !selectedWord.value) {
    return;
  }
  console.log(`getting on demand definiton from dictionary ${dictionary}`);
  onDemandDefinitions.value.set(
    dictionary,
    await invoke<Definition>("get_definition_on_demand", {
      dictionary,
      lemma: selectedWord.value.lemma,
    }).then((def) => def.c!),
  );
}

const isComputingDefinition = ref(false);

const definitions = computedAsync(
  async (): Promise<Definition[]> => {
    if (selectedWord.value) {
      const defs = await invoke<Definition[]>("get_defs", {
        lemma: selectedWord.value.lemma,
      });

      for (const def of defs) {
        if (def.t === "OnDemand" && def.c) {
          onDemandDefinitions.value.set(def.c, undefined);
        }
      }
      return defs;
    }
    return [];
  },
  [],
  isComputingDefinition,
);

const separatedDefinitions = ref<string[]>([]);

watch(definitions, () => {
  for (const def of separatedDefinitions.value) {
    getOnDemandDef(def);
  }
});
</script>

<template>
  <ResizablePanelGroup direction="horizontal" class="h-full">
    <ResizablePanel>
      <ResizablePanelGroup direction="vertical">
        <ResizablePanel :min-size="20">
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
        <template v-for="(definition, index) in separatedDefinitions">
          <ResizableHandle />
          <ResizablePanel :default-size="25">
            <div class="flex items-center justify-between bg-accent px-2">
              <h1>{{ definition }}</h1>
              <PanelBottomClose
                class="w-4 h-4"
                @click="separatedDefinitions.splice(index, 1)"
              />
            </div>
            <div class="p-3 h-full">
              <span
                v-if="onDemandDefinitions.get(definition)"
                v-html="onDemandDefinitions.get(definition)"
              ></span>
              <Loader2 v-else class="animate-spin" />
            </div>
          </ResizablePanel>
        </template>
      </ResizablePanelGroup>
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel
      v-if="selectedWord"
      :min-size="20"
      :max-size="70"
      :default-size="32"
      class="h-full"
    >
      <Suspense class="h-full">
        <SelectedWordView
          v-model="sections![selectedSectionIndex].c[selectedWordIndex]"
          :sentence
          :currentLanguage
          :definitions
          :isComputingDefinition
          :onDemandDefinitions
          :separatedDefinitions
          @set-rating="changeRating"
          @getOnDemandDef="getOnDemandDef"
          class="h-full"
        />
      </Suspense>
    </ResizablePanel>
  </ResizablePanelGroup>
</template>
