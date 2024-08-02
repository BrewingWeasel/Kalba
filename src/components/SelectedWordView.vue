<script setup lang="ts">
import RatingButtons from "@/components/RatingButtons.vue";
import GrammarDetails from "@/components/GrammarDetails.vue";
import DefinitionView from "@/components/DefinitionView.vue";
import { Input } from "@/components/ui/input";
import ExportButton from "@/components/ExportButton.vue";
import { invoke } from "@tauri-apps/api/tauri";
import {
  ExportDetails,
  type Definition,
  type HistoryItem,
  type Word,
} from "@/types";
import { Button } from "@/components/ui/button";
import { Tags, CheckCircle2, Loader2 } from "lucide-vue-next";
import { ref, watch } from "vue";
import BetterTooltip from "./BetterTooltip.vue";

const separatedDefinitions = defineModel<string[]>("separatedDefinitions", {
  required: true,
});

const props = defineProps<{
  sentence: string;
  currentLanguage: string;
  definitions: Map<string, Definition>;
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

const updatingLemma = ref(word.value.lemma);
watch(
  () => word.value.lemma,
  (newLemma) => {
    updatingLemma.value = newLemma;
  },
);

async function updateLemma() {
  history.value.push(updatingLemma.value);
  historyIndex.value++;
  word.value.lemma = updatingLemma.value;
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

const exportDetails = defineModel<ExportDetails>("exportDetails", {
  required: true,
});
</script>

<template>
  <div
    class="px-7 bg-accent w-full h-[calc(100vh-3.25rem)] flex flex-col overflow-auto"
  >
    <br />
    <div class="p-2 bg-border rounded-lg mb-2">
      <div class="flex justify-center gap-1 items-center">
        <Button variant="outline" size="icon" :disabled="true">
          <BetterTooltip tooltip="Edit tags">
            <Tags />
          </BetterTooltip>
        </Button>
        <Input
          @change="updateLemma"
          class="text-lg text-center max-w-64"
          v-model="updatingLemma"
        />
        <Button
          variant="outline"
          size="icon"
          :disabled="history.length === 1 || historyIndex === 0"
          @click="alwaysChangeLemma"
        >
          <BetterTooltip
            :tooltip="`Always change '${history[0]}' to '${word.lemma}'`"
          >
            <CheckCircle2
          /></BetterTooltip>
        </Button>
      </div>
      <div class="flex justify-center gap-3 items-center mt-1">
        <Button
          variant="secondary"
          class="text-sm"
          size="sm"
          v-for="form in word.other_forms.filter((f) => f !== word.lemma)"
          @click="
            updatingLemma = form;
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
        :definitions
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
        <BetterTooltip :tooltip="`Save ${word.lemma} to Anki`">
          <ExportButton
            v-model:exportDetails="exportDetails"
            :definitions
            :word="word.lemma"
            :sentence="props.sentence"
            :currentLanguage
            @change-rating="
              (r) => {
                $emit('set-rating', r, word.lemma, true);
              }
            "
        /></BetterTooltip>
      </Suspense>
    </div>
  </div>
</template>
