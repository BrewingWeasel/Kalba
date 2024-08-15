<script setup lang="ts">
import { type Ref, computed, onMounted, ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import IndividualWord from "@/components/Word.vue";
import SelectedWordView from "@/components/SelectedWordView.vue";
import type {
  Word,
  Section,
  Definition,
  HistoryItem,
  ExportDetails,
  ParsedWords,
  InputType,
} from "@/types";
import { toast } from "vue-sonner";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import { computedAsync } from "@vueuse/core";
import { Loader2, PanelBottomClose, Redo2, Undo2 } from "lucide-vue-next";
import { useMagicKeys, whenever } from "@vueuse/core";
import { Button } from "@/components/ui/button";
import { Separator } from "@/components/ui/separator";
import BetterTooltip from "@/components/BetterTooltip.vue";
import { readText } from "@tauri-apps/api/clipboard";

const inputText = defineModel<string>("inputText", { required: true });
const props = defineProps<{
  currentLanguage: string;
  inputType: InputType;
}>();
const sections: Ref<Section[] | undefined> = ref(undefined);
const sentences: Ref<string[] | undefined> = ref(undefined);
const selectedWord: Ref<Word | undefined> = ref(undefined);
const selectedSectionIndex: Ref<number> = ref(0);
const selectedWordIndex: Ref<number> = ref(0);

const wordHovered = ref<string | undefined>(undefined);

const sentence = computed(() => {
  if (sentences.value && selectedWord.value) {
    return sentences.value[selectedWord.value.sentence_index];
  }
  return "";
});

async function setWords() {
  const isUrl = props.inputType === "url";
  const command = isUrl ? "parse_url" : "parse_text";
  const args = isUrl ? { url: inputText.value } : { sent: inputText.value };

  const parsedWords = await invoke<ParsedWords>(command, args).catch(
    (error) => {
      toast.error(error);
      return { sections: [], sentences: [] };
    },
  );
  sections.value = parsedWords.sections;
  sentences.value = parsedWords.sentences;
  console.log(sections);
}

const sentenceStats = computed(() => {
  let atEachLevel = [0, 0, 0, 0, 0, 0];
  let words = 0;
  let percentage = 0;
  if (sections.value) {
    sections.value.forEach((section) => {
      if (typeof section.c !== "string") {
        section.c.forEach((word) => {
          if (word.rating === -1) {
            atEachLevel[5]++;
          } else if (word.rating !== undefined) {
            words++;
            percentage += word.rating;
            atEachLevel[word.rating]++;
          }
        });
      }
    });
  }
  return { words, atEachLevel, percentage: percentage / (words * 4) };
});

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
  console.log(`getting on demand definition from dictionary ${dictionary}`);
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
  async (): Promise<Map<string, Definition>> => {
    if (selectedWord.value) {
      const defs = new Map(
        Object.entries(
          await invoke<{ [key: string]: Definition }>("get_defs", {
            lemma: selectedWord.value.lemma,
          }),
        ),
      );

      for (const [_, def] of defs.entries()) {
        if (def.t === "OnDemand" && def.c) {
          onDemandDefinitions.value.set(def.c, undefined);
        }
      }
      return defs;
    }
    return new Map();
  },
  new Map(),
  isComputingDefinition,
);

const separatedDefinitions = ref<string[]>([]);

watch(definitions, () => {
  for (const def of separatedDefinitions.value) {
    getOnDemandDef(def);
  }
});

const exportDetails: Ref<ExportDetails> = ref({
  word: selectedWord.value?.lemma ?? "",
  defs: definitions.value,
  deck: "",
  model: "",
  sentence: "",
  fields: {},
  original_form: selectedWord.value?.text ?? "",
});

watch(sentence, (newSentence) => {
  if (newSentence) {
    exportDetails.value.sentence = newSentence;
  }
});

watch(
  () => selectedWord.value,
  (newWord) => {
    if (newWord) {
      exportDetails.value.word = newWord.lemma;
      exportDetails.value.original_form = newWord.text;
    }
  },
);

watch(
  () => definitions.value,
  (newDefs) => {
    exportDetails.value.defs = newDefs;
  },
);

const history = ref<HistoryItem[]>([]);
const historyIndex = ref(0);
watch(
  () => selectedWord.value?.text,
  () => {
    if (selectedWord.value) {
      history.value = [selectedWord.value.lemma];
      historyIndex.value = 0;
    }
  },
);

function undo() {
  if (historyIndex.value > 0) {
    historyIndex.value--;
    const newLemma = history.value[historyIndex.value];
    if (typeof newLemma === "string" && selectedWord.value) {
      selectedWord.value.lemma = newLemma;
    }
  }
}

function redo() {
  console.log(historyIndex.value, history.value);
  if (historyIndex.value < history.value.length) {
    historyIndex.value++;
    const newLemma = history.value[historyIndex.value];
    if (typeof newLemma === "string" && selectedWord.value) {
      selectedWord.value.lemma = newLemma;
    }
  }
}

const { one, two, three, four, five, zero } = useMagicKeys({
  aliasMap: {
    one: "1",
    two: "2",
    three: "3",
    four: "4",
    five: "5",
    zero: "0",
  },
  passive: false,
  onEventFired(e) {
    if (e.ctrlKey && (e.key === "z" || e.key === "Z") && e.type === "keydown") {
      e.preventDefault();
      if (e.shiftKey) {
        redo();
      } else {
        undo();
      }
    }
  },
});

[one, two, three, four, five].forEach((key, i) => {
  whenever(key, () => {
    if (wordHovered.value) {
      changeRating(i, wordHovered.value, true);
    }
  });
});

whenever(zero, () => {
  if (wordHovered.value) {
    changeRating(-1, wordHovered.value, true);
  }
});

onMounted(async () => {
  await invoke("start_stanza").catch((error) => {
    toast.error(error);
  });
  console.log("Stanza loaded");

  if (props.inputType === "clipboard") {
    console.log("setting up clipboard listener");
    await setWords();

    setInterval(async () => {
      const clipboardText = await readText();
      if (clipboardText && clipboardText !== inputText.value) {
        inputText.value = clipboardText;
        console.log("clipboard text changed", clipboardText);
        setTimeout(setWords, 200);
      }
    }, 600);
  } else {
    await setWords();
    console.log(sections);
  }
});
</script>

<template>
  <ResizablePanelGroup direction="horizontal" class="h-full">
    <ResizablePanel>
      <ResizablePanelGroup direction="vertical">
        <ResizablePanel :min-size="20">
          <div
            class="py-6 px-10 flex flex-col items-center w-full h-[calc(100vh-3.25rem)] overflow-auto"
          >
            <div
              v-for="(section, sectionIndex) in sections"
              class="max-w-[500px] w-[500px] min-w-[350px]"
            >
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
                  v-for="(word, wordIndex) in section.c"
                  :word="word"
                  :rating="word.rating"
                  v-model="wordHovered"
                  @selected="
                    (w) => handle_word_selected(w, sectionIndex, wordIndex)
                  "
                  @set-rating="changeRating"
                />
              </div>
            </div>
          </div>
        </ResizablePanel>
        <template v-for="(definition, index) in separatedDefinitions">
          <ResizableHandle />
          <ResizablePanel :default-size="25">
            <div class="flex items-center justify-between bg-accent px-2 mb-2">
              <h1>{{ definition }}</h1>
              <BetterTooltip tooltip="move panel into definition view">
                <PanelBottomClose
                  class="w-4 h-4"
                  @click="separatedDefinitions.splice(index, 1)"
              /></BetterTooltip>
            </div>
            <div class="px-3 h-full overflow-auto pb-10">
              <span
                v-if="onDemandDefinitions.get(definition)"
                v-html="onDemandDefinitions.get(definition)"
              ></span>
              <Loader2 v-else class="animate-spin" />
            </div>
          </ResizablePanel>
        </template>
        <Separator />
        <div class="flex bg-background px-3 h-8 items-center justify-between">
          <div class="text-xs flex items-center">
            <span class="hover:bg-accent px-1 py-1 h-full"
              >{{ sentenceStats.words }} words</span
            >
            <span class="hover:bg-accent px-1 py-1 h-full mr-2"
              >{{ (sentenceStats.percentage * 100).toFixed(1) }}% known</span
            >
            <template v-if="sentenceStats.atEachLevel[0] != 0">
              <BetterTooltip tooltip="Unknown words">
                <span class="mx-1 rounded-full bg-rose-600 w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[0] }} </span>
              </BetterTooltip>
            </template>
            <template v-if="sentenceStats.atEachLevel[1] != 0">
              <BetterTooltip tooltip="Learning words">
                <span class="mx-1 rounded-full bg-red-400 w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[1] }} </span>
              </BetterTooltip>
            </template>
            <template v-if="sentenceStats.atEachLevel[2] != 0">
              <BetterTooltip tooltip="Recognized words">
                <span class="mx-1 rounded-full bg-orange-400 w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[2] }} </span>
              </BetterTooltip>
            </template>
            <template v-if="sentenceStats.atEachLevel[3] != 0">
              <BetterTooltip tooltip="Familiar words">
                <span class="mx-1 rounded-full bg-amber-300 w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[3] }} </span>
              </BetterTooltip>
            </template>
            <template v-if="sentenceStats.atEachLevel[4] != 0">
              <BetterTooltip tooltip="Known words">
                <span class="mx-1 rounded-full bg-current w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[4] }} </span>
              </BetterTooltip>
            </template>
            <template v-if="sentenceStats.atEachLevel[5] != 0">
              <BetterTooltip tooltip="Ignored words">
                <span class="mx-1 rounded-full bg-gray-500 w-3 h-3"></span>
                <span>{{ sentenceStats.atEachLevel[5] }} </span>
              </BetterTooltip>
            </template>
          </div>
          <div>
            <BetterTooltip tooltip="Undo">
              <Button
                class="mr-1"
                variant="outline"
                size="smallIcon"
                :disabled="history.length === 1 || historyIndex === 0"
                @click="undo"
              >
                <Undo2 class="h-4 w-4" /> </Button
            ></BetterTooltip>
            <BetterTooltip tooltip="Redo">
              <Button
                variant="outline"
                size="smallIcon"
                :disabled="
                  history.length === 1 || historyIndex >= history.length - 1
                "
                @click="redo"
              >
                <Redo2 class="h-4 w-4" />
              </Button>
            </BetterTooltip>
          </div>
        </div>
      </ResizablePanelGroup>
    </ResizablePanel>
    <ResizableHandle />
    <ResizablePanel
      v-if="selectedWord && sections"
      :min-size="20"
      :max-size="70"
      :default-size="32"
      class="max-h-full"
    >
      <Suspense class="h-full">
        <SelectedWordView
          v-model:word="
            sections[selectedSectionIndex].c[selectedWordIndex] as Word
          "
          v-model:history="history"
          v-model:historyIndex="historyIndex"
          v-model:exportDetails="exportDetails"
          :sentence
          :currentLanguage
          :definitions
          :isComputingDefinition
          :onDemandDefinitions
          v-model:separatedDefinitions="separatedDefinitions"
          @set-rating="changeRating"
          @getOnDemandDef="getOnDemandDef"
          class="max-h-full"
        />
      </Suspense>
    </ResizablePanel>
  </ResizablePanelGroup>
</template>
