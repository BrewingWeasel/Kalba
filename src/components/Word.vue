<script setup lang="ts">
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import GrammarDetails from "@/components/GrammarDetails.vue";

import { computed } from "vue";
const props = defineProps(["word", "rating"]);
const emit = defineEmits(["selected"]);

const rating = computed(() => {
  const startingClass = props.word.clickable ? "pl-1 " : "px-0 ";
  if (props.word.text.includes("\n")) {
    return "basis-full h-0 pb-2";
  }
  if (props.rating == 0) {
    return startingClass + "text-rose-600";
  } else if (props.word.rating == 1) {
    return startingClass + "text-red-400";
  } else if (props.word.rating == 2) {
    return startingClass + "text-orange-400";
  } else if (props.word.rating == 3) {
    return startingClass + "text-amber-300";
  } else {
    return startingClass;
  }
});

function set_selected() {
  if (props.word.clickable) {
    emit("selected", props.word);
  }
}
</script>

<template>
  <div :class="rating" @click="set_selected">
    <HoverCard>
      <HoverCardTrigger>{{ props.word.text }}</HoverCardTrigger>
      <HoverCardContent>
        <h1 class="text-center font-semibold text-lg" :class="rating">
          {{ props.word.lemma }}
        </h1>
        <hr class="py-1" />
        <GrammarDetails :morph="props.word.morph" />
      </HoverCardContent>
    </HoverCard>
  </div>
</template>
