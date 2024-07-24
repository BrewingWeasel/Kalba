<script setup lang="ts">
import RatingButtons from "@/components/RatingButtons.vue";
import GrammarDetails from "@/components/GrammarDetails.vue";
import DefinitionView from "@/components/DefinitionView.vue";
import { Input } from "@/components/ui/input";
import ExportButton from "@/components/ExportButton.vue";
import { invoke } from "@tauri-apps/api/tauri";
import type { Definition, Word } from "@/types";
import { Button } from "@/components/ui/button";
import { ref, watch } from "vue";
import { Loader2, Redo2, Undo2 } from "lucide-vue-next";

const separatedDefinitions = defineModel<string[]>("separatedDefinitions", {
  required: true,
});

const props = defineProps<{
  sentence: string;
  currentLanguage: string;
  definitions: Definition[];
  isComputingDefinition: boolean;
  onDemandDefinitions: Map<string, undefined | string>;
}>();
const word = defineModel<Word>({ required: true });
console.log(word.value);

const emit = defineEmits<{
  (e: "set-rating", rating: number, lemma: string, modifiable?: boolean): void;
  (e: "getOnDemandDef", definition: string): void;
}>();

const word_history = ref([word.value.lemma]);
const word_history_index = ref(0);
watch(
  () => word.value.text,
  () => {
    word_history.value = [];
    word_history_index.value = 0;
  },
);

async function updateLemma() {
  const rating: number = await invoke("get_rating", {
    lemma: word.value.lemma,
  });
  emit("set-rating", rating, word.value.lemma);
}
</script>

<template>
  <div class="px-7 bg-accent w-full h-full flex flex-col">
    <br />
    <div class="p-2 bg-border rounded-lg mb-2">
      <div class="flex justify-center gap-1 items-center">
        <Button
          variant="outline"
          size="icon"
          :disabled="word_history.length === 1 || word_history_index === 0"
          @click="
            word_history_index--;
            word.lemma = word_history[word_history_index];
            updateLemma();
          "
        >
          <Undo2 />
        </Button>
        <Input
          @change="updateLemma"
          class="text-lg text-center border-0 hover:border-2 focus:border-2 max-w-64"
          v-model="word.lemma"
        />
        <Button
          variant="outline"
          size="icon"
          :disabled="
            word_history.length === 1 ||
            word_history_index === word_history.length - 1
          "
          @click="
            word_history_index++;
            word.lemma = word_history[word_history_index];
            updateLemma();
          "
        >
          <Redo2 />
        </Button>
      </div>
      <div class="flex justify-center gap-3 items-center mt-1">
        <Button
          variant="secondary"
          class="text-sm"
          size="sm"
          v-for="form in word.other_forms.filter((f) => f !== word.lemma)"
          @click="
            word_history.push(form);
            word_history_index++;
            word.lemma = form;
            updateLemma();
          "
        >
          {{ form }}
        </Button>
      </div>
      <p class="flex justify-center items-center py-1">
        <i>{{ word.text }}</i>
      </p>
      <RatingButtons
        class="pb-3"
        @change-rating="
          (r) => {
            $emit('set-rating', r, word.lemma);
          }
        "
      />
    </div>
    <Suspense>
      <DefinitionView
        v-if="!props.isComputingDefinition"
        :definitions="props.definitions"
        :lemma="word.lemma"
        :onDemandDefinitions="props.onDemandDefinitions"
        :separatedDefinitions
        @getOnDemandDef="
          (definition) => {
            $emit('getOnDemandDef', definition);
          }
        "
      />
      <div v-else><Loader2 class="animate-spin" /></div>

      <template #fallback><Loader2 class="animate-spin" /></template>
    </Suspense>
    <div class="mt-auto">
      <GrammarDetails :morph="word.morph" separator="true" />
      <Suspense>
        <ExportButton
          :defs="definitions.filter((v) => v.t == 'Text').map((v) => v.c!)"
          :word="word.lemma"
          :sentence="props.sentence"
          :currentLanguage
          @change-rating="
            (r) => {
              $emit('set-rating', r, word.lemma, true);
            }
          "
        />
      </Suspense>
    </div>
  </div>
</template>
