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
import ExportButton from "@/components/ExportButton.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

interface Definition {
  t: string;
  conts: string;
}

const props = defineProps(["word", "sentence"]);

const definition = computedAsync(async (): Promise<Definition[]> => {
  return await invoke("get_defs", { lemma: props.word.lemma });
}, []);
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-center">{{ props.word.lemma }}</CardTitle>
      <CardDescription class="text-center"
        ><i>{{ props.word.text }}</i></CardDescription
      >
    </CardHeader>
    <CardContent>
      <RatingButtons
        class="pb-3"
        @change-rating="
          (r) => {
            $emit('set-rating', r, props.word.lemma);
          }
        "
      />
      <Suspense>
        <DefinitionView :definition />

        <template #fallback> Loading... </template>
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
            $emit('set-rating', r, props.word.lemma, true);
          }
        "
      />
    </Suspense>
  </Card>
</template>
