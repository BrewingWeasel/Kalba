<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import RatingButtons from "@/components/RatingButtons.vue";
import GrammarDetails from "@/components/GrammarDetails.vue";
import DefinitionView from "@/components/DefinitionView.vue";
import { Input } from "@/components/ui/input";
import ExportButton from "@/components/ExportButton.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";
import type { Word } from "@/types";
import { Button } from "@/components/ui/button";
import { ref, watch } from "vue";
import { Loader2, Redo2, Undo2 } from "lucide-vue-next";

interface Definition {
  t: string;
  conts: string;
}

const props = defineProps(["sentence"]);
const word = defineModel<Word>({ required: true });
console.log(word.value);

const emit =
  defineEmits<
    (
      e: "set-rating",
      rating: number,
      lemma: string,
      modifiable?: boolean,
    ) => void
  >();

const word_history = ref([word.value.lemma]);
const word_history_index = ref(0);
watch(
  () => word.value.text,
  () => {
    word_history.value = [];
    word_history_index.value = 0;
  },
);

const isComputingDefinition = ref(false);

const definition = computedAsync(
  async (): Promise<Definition[]> => {
    return await invoke("get_defs", { lemma: word.value.lemma });
  },
  [],
  isComputingDefinition,
);

async function updateLemma() {
  const rating: number = await invoke("get_rating", {
    lemma: word.value.lemma,
  });
  emit("set-rating", rating, word.value.lemma);
}
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>
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
            class="text-lg text-center border-0 hover:border-2 focus:border-2"
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
        <div class="flex justify-center gap-3 items-center">
          <Button
            variant="outline"
            class="text-sm"
            size="sm"
            v-for="form in word.other_forms"
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
      </CardTitle>
      <CardDescription class="text-center">
        <i>{{ word.text }}</i>
      </CardDescription>
    </CardHeader>
    <CardContent>
      <RatingButtons
        class="pb-3"
        @change-rating="
          (r) => {
            $emit('set-rating', r, word.lemma);
          }
        "
      />
      <Suspense>
        <DefinitionView v-if="!isComputingDefinition" :definition />
        <div v-else><Loader2 class="animate-spin" /></div>

        <template #fallback><Loader2 class="animate-spin" /></template>
      </Suspense>
    </CardContent>
    <CardFooter>
      <GrammarDetails :morph="word.morph" separator="true" />
    </CardFooter>
    <Suspense>
      <ExportButton
        :defs="definition.map((v) => v.conts)"
        :word="word.lemma"
        :sentence="props.sentence"
        @change-rating="
          (r) => {
            $emit('set-rating', r, word.lemma, true);
          }
        "
      />
    </Suspense>
  </Card>
</template>
