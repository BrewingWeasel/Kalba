<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { Button } from "@/components/ui/button";
import IndividualDeck from "@/components/settings/Deck.vue";
import { Deck } from "@/components/settings/Deck.vue";

const props = defineProps<{
  decks: Deck[];
  models: string[];
  deckNames: string[];
}>();

function addDeck() {
  props.decks.push({
    name: "",
    notes: [],
  });
}
</script>

<template>
  <Accordion type="single" collapsible>
    <Button @click="addDeck">Add deck</Button>
    <AccordionItem v-for="(deck, index) in decks" :value="index.toString()">
      <AccordionTrigger>
        {{ deck.name ? deck.name : "Select a deck" }}
      </AccordionTrigger>
      <AccordionContent>
        <IndividualDeck :deck :models :decks="deckNames" />
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
