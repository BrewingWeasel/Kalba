<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";

import { Button } from "@/components/ui/button";
import { invoke } from "@tauri-apps/api/core";
import { toast } from "vue-sonner";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { ref } from "vue";
import { Loader2, Pencil, Plus, X } from "lucide-vue-next";
import Notes from "./Notes.vue";
import { Deck } from "@/types";
import BetterTooltip from "@/components/BetterTooltip.vue";

const props = defineProps<{
  decks: Deck[];
  models: string[];
  deckNames: string[];
}>();

const notesOpen = ref<boolean[][]>([]);
props.decks.forEach((deck) => {
  notesOpen.value.push(Array(deck.notes.length).fill(false));
});

function addDeck() {
  props.decks.push({
    name: "",
    notes: [],
  });
  notesOpen.value[props.decks.length - 1] = [];
}

function addNote(deck_index: number) {
  props.decks[deck_index].notes.push({
    model: "",
    handling: {
      field_to_use: "",
      only_first_word_or_line: false,
      remove_everything_in_parens: false,
      search_params: "",
    },
  });
  notesOpen.value[deck_index].push(true);
}

const isRefreshing = ref(false);
const canClose = ref(false);

async function refreshAnki(forceAll: boolean) {
  isRefreshing.value = true;
  await invoke("refresh_anki", { forceAll }).catch((e) => {
    toast.error(e);
  });
  canClose.value = true;
}
</script>

<template>
  <AlertDialog v-model:open="isRefreshing">
    <AlertDialogContent>
      <AlertDialogHeader>
        <AlertDialogTitle>Refresh Anki</AlertDialogTitle>
      </AlertDialogHeader>
      <div v-if="!canClose">
        <Loader2 class="animate-spin" />
      </div>
      <AlertDialogFooter>
        <AlertDialogAction v-if="canClose">Continue</AlertDialogAction>
      </AlertDialogFooter>
    </AlertDialogContent>
  </AlertDialog>
  <div class="py-2">
    <Button variant="outline" class="mr-2" @click="refreshAnki(false)"
      >Refresh Anki knowledge</Button
    >
    <Button variant="destructive" @click="refreshAnki(true)"
      >Force refresh Anki knowledge</Button
    >
  </div>

  <Table class="max-w-5xl table-fixed">
    <TableCaption>
      <Button variant="ghost" class="text-xl" @click="addDeck">+</Button>
    </TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead class="w-1/3">Deck</TableHead>
        <TableHead class="w-8 text-left">Configure</TableHead>
        <TableHead class="w-8 text-left">Remove</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <template v-for="(deck, index) in decks" :key="index">
        <TableRow>
          <TableCell>
            <StyledCombobox
              :options="props.deckNames"
              v-model="deck.name"
              item-being-selected="deck"
            />
          </TableCell>
          <TableCell>
            <BetterTooltip tooltip="Add note type parser">
              <Plus
                :size="16"
                class="float-right cursor-pointer hover:stroke-green-400"
                @click="addNote(index)"
              />
            </BetterTooltip>
          </TableCell>
          <TableCell>
            <BetterTooltip tooltip="Remove deck parser">
              <X
                @click="
                  () => {
                    decks.splice(index, 1);
                  }
                "
                class="float-right cursor-pointer hover:stroke-rose-500"
                :size="16"
              />
            </BetterTooltip>
          </TableCell>
        </TableRow>
        <TableRow v-for="(note, noteIndex) in deck.notes" class="bg-accent">
          <TableCell class="px-16">
            {{ note.model }}
            {{ note.model ? " > " : "No note selected" }}
            <i>{{ note.handling.field_to_use }}</i>
          </TableCell>
          <TableCell>
            <AlertDialog v-model:open="notesOpen[index][noteIndex]">
              <AlertDialogTrigger class="w-full">
                <BetterTooltip tooltip="Edit note type parser">
                  <Pencil
                    :size="16"
                    class="float-right cursor-pointer hover:stroke-teal-400"
                  />
                </BetterTooltip>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Edit note</AlertDialogTitle>
                </AlertDialogHeader>
                <Notes v-model="decks[index].notes[noteIndex]" :models />
                <AlertDialogFooter>
                  <AlertDialogAction>Save</AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
          </TableCell>
          <TableCell>
            <BetterTooltip tooltip="Remove note type parser">
              <X
                @click="
                  () => {
                    deck.notes.splice(noteIndex, 1);
                  }
                "
                class="float-right cursor-pointer hover:stroke-rose-500"
                :size="16"
            /></BetterTooltip>
          </TableCell>
        </TableRow>
      </template>
    </TableBody>
  </Table>
</template>
