<script setup lang="ts">
import {
	Accordion,
	AccordionContent,
	AccordionItem,
	AccordionTrigger,
} from "@/components/ui/accordion";

import { Button } from "@/components/ui/button";
import Notes from "./Notes.vue";
import { Note } from "./Notes.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";

export interface Deck {
	name: string;
	notes: Note[];
}

const props = defineProps<{
	deck: Deck;
	models: string[];
	decks: string[];
}>();

function addNote() {
	props.deck.notes.push({
		model: "",
		handling: {
			field_to_use: "",
			only_first_word_or_line: false,
			remove_everything_in_parens: false,
			tags_wanted: [],
		},
	});
}
</script>

<template>
  <div class="pl-5 rounded-md bg-slate-100 dark:bg-slate-900">
    <div class="py-2">
      <StyledCombobox
        :options="props.decks"
        v-model="props.deck.name"
        item-being-selected="deck"
      />
    </div>
    <Button @click="addNote">Add note</Button>
    <Accordion type="single" collapsible>
      <AccordionItem
        v-for="(note, index) in props.deck.notes"
        :value="index.toString()"
      >
        <AccordionTrigger>
          {{ note.model ? note.model : "Select a model" }}
        </AccordionTrigger>
        <AccordionContent>
          <Notes :note :models />
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  </div>
</template>
