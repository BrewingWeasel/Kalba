<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { Button } from "@/components/ui/button";
import IndividualDeck from "./Deck.vue";
import type { Deck } from "./Deck.vue";
import { invoke } from "@tauri-apps/api/tauri";

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

function refreshAnki(forceAll: boolean) {
  invoke("refresh_anki", { forceAll });
}
</script>

<template>
  <div class="py-2">
    <Button variant="outline" class="mr-2" @click="refreshAnki(false)"
      >Refresh Anki knowledge</Button
    >
    <Button variant="destructive" @click="refreshAnki(true)"
      >Force refresh Anki knowledge</Button
    >
  </div>

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
