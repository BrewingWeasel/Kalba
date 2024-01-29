<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import IndividualDeck from "@/components/settings/Deck.vue";
import { Deck } from "@/components/settings/Deck.vue";

const models: string[] = await invoke("get_all_note_names", {});

const deckNames: string[] = await invoke("get_all_deck_names", {});

const props = defineProps<{
  decks: Deck[];
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
