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

interface Definition {
	t: string;
	conts: string;
}

const props = defineProps(["sentence"]);
const word = defineModel<Word>({ required: true });

const emit = defineEmits<(e: "set-rating", rating: number, lemma: string, modifiable?: boolean) => void>();

const definition = computedAsync(async (): Promise<Definition[]> => {
	return await invoke("get_defs", { lemma: word.value.lemma });
}, []);

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
      <CardTitle
        ><Input
          @change="updateLemma"
          class="text-lg text-center border-0 hover:border-2 focus:border-2"
          v-model="word.lemma"
        ></Input
      ></CardTitle>
      <CardDescription class="text-center"
        ><i>{{ word.text }}</i></CardDescription
      >
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
            $emit('set-rating', r, word.lemma, true);
          }
        "
      />
    </Suspense>
  </Card>
</template>
