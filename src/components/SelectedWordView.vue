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
import { Button } from "@/components/ui/button";
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

async function exportWord() {
  await invoke("add_to_anki", {
    word: props.word.lemma,
    sent: props.sentence,
    defs: definition.value.map((def) => def.conts),
  });
}
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
    <div class="flex justify-center bottom-0 py-3">
      <Button @click="exportWord" variant="destructive">Export</Button>
    </div>
  </Card>
</template>
