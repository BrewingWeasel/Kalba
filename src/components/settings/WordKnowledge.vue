<script setup lang="ts">
import { Ref, ref } from "vue";

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import Notes from "@/components/settings/Notes.vue";
import { Note } from "@/components/settings/Notes.vue";

const notes: Ref<Array<Note>> = ref([]);

// const selectedTemplate = ref("");
// const selectedField = ref("");

const models: string[] = await invoke("get_all_note_names", {});

function addNote() {
  notes.value.push({
    model: "",
    field: "",
    removeParens: false,
    firstWordOnly: false,
  });
}
</script>

<template>
  <Accordion type="single" collapsible>
    <Button @click="addNote">Add note</Button>
    <AccordionItem v-for="(note, index) in notes" :value="index.toString()">
      <AccordionTrigger>
        {{ note.model ? note.model : "Select a model" }}
      </AccordionTrigger>
      <AccordionContent>
        <Notes :note :models />
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
