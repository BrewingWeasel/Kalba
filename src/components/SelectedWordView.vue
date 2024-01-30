<script setup lang="ts">
import { defineAsyncComponent } from "vue";
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

const props = defineProps(["word"]);

const DefinitionComp = defineAsyncComponent(
  () => import("@/components/DefinitionView.vue"),
);
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
        <DefinitionComp :lemma="word.lemma" />

        <template #fallback> Loading... </template>
      </Suspense>
    </CardContent>
    <CardFooter>
      <GrammarDetails :morph="word.morph" separator="true" />
    </CardFooter>
  </Card>
</template>
