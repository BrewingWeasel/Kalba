<script setup lang="ts">
import RatingButtons from "@/components/RatingButtons.vue";
import GrammarDetails from "@/components/GrammarDetails.vue";
import DefinitionView from "@/components/DefinitionView.vue";
import { Input } from "@/components/ui/input";
import ExportButton from "@/components/ExportButton.vue";
import { invoke } from "@tauri-apps/api/tauri";
import type { Definition, HistoryItem, Word } from "@/types";
import { Button } from "@/components/ui/button";
import { Tags, CheckCircle2, Loader2 } from "lucide-vue-next";

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

const word = defineModel<Word>("word", { required: true });
console.log(word.value);

const history = defineModel<HistoryItem[]>("history", { required: true });
const historyIndex = defineModel<number>("historyIndex", { required: true });

const emit = defineEmits<{
  (e: "set-rating", rating: number, lemma: string, modifiable?: boolean): void;
  (e: "getOnDemandDef", definition: string): void;
}>();

async function updateLemma() {
  const rating: number = await invoke("get_rating", {
    lemma: word.value.lemma,
  });
  emit("set-rating", rating, word.value.lemma);
}

async function alwaysChangeLemma() {
  await invoke("always_change_lemma", {
    lemma: history.value[0],
    updatedLemma: word.value.lemma,
  });
}
</script>

<template>
  <div
    class="px-7 bg-accent w-full h-[calc(100vh-3.25rem)] flex flex-col overflow-auto"
  >
    <br />
    <div class="p-2 bg-border rounded-lg mb-2">
      <div class="flex justify-center gap-1 items-center">
        <Button variant="outline" size="icon" :disabled="true">
          <Tags />
        </Button>
        <Input
          @change="updateLemma"
          class="text-lg text-center max-w-64"
          v-model="word.lemma"
        />
        <Button
          variant="outline"
          size="icon"
          :disabled="history.length === 1 || historyIndex === 0"
          @click="alwaysChangeLemma"
        >
          <CheckCircle2 />
        </Button>
      </div>
      <div class="flex justify-center gap-3 items-center mt-1">
        <Button
          variant="secondary"
          class="text-sm"
          size="sm"
          v-for="form in word.other_forms.filter((f) => f !== word.lemma)"
          @click="
            history.push(form);
            historyIndex++;
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
