<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogClose,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import Exporting from "@/components/settings/Exporting.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
import { Settings, ExportDetails } from "@/types";
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

const props = defineProps<{
  word: string;
  sentence: string;
  defs: string[];
}>();

const settings: Settings = await invoke("get_settings");

const models: string[] = await invoke("get_all_note_names");
const deckNames: string[] = await invoke("get_all_deck_names");

const exportDetails: Ref<ExportDetails> = ref({
  word: props.word,
  defs: props.defs,
  deck: settings.deck,
  model: settings.note_type,
  sentence: "",
  fields: settings.note_fields,
});

async function exportWord() {
  await invoke("add_to_anki", { exportDetails: exportDetails.value });
}

function selectWord() {
  const selection = window.getSelection();
  if (selection) {
    if (selection.focusNode?.parentElement?.id === "sentence")
      exportDetails.value.sentence = selection.toString();
  }
}
</script>

<template>
  <Dialog>
    <DialogTrigger as-child>
      <div class="flex justify-center bottom-0 py-3">
        <Button variant="destructive"> Export </Button>
      </div>
    </DialogTrigger>
    <DialogContent class="select-none">
      <DialogHeader>
        <DialogTitle>Export word</DialogTitle>
        <DialogDescription>
          Export the word to Anki with information
        </DialogDescription>
      </DialogHeader>
      <Accordion type="multiple" collapsible>
        <AccordionItem value="location">
          <AccordionTrigger>Export location</AccordionTrigger>
          <AccordionContent>
            <div class="select-auto">
              <Exporting
                :models
                :deckNames
                v-model:deck="exportDetails.deck"
                v-model:model="exportDetails.model"
                v-model:fields="exportDetails.fields"
              />
            </div>
          </AccordionContent>
        </AccordionItem>
        <AccordionItem value="context">
          <AccordionTrigger>Export context</AccordionTrigger>
          <AccordionContent>
            <p v-if="exportDetails.sentence == ''">
              Select the context to export
            </p>
            <p v-else>
              {{ exportDetails.sentence }}
            </p>
            <div class="border-2 border-slate-800 p-2 mt-2 rounded-md">
              <p
                @mouseup="selectWord"
                @mousedown="selectWord"
                @mouseleave="selectWord"
                id="sentence"
                class="selection:bg-pink-300 select-auto"
              >
                {{ props.sentence }}
              </p>
            </div>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
      <DialogFooter>
        <DialogClose as-child>
          <div class="flex justify-center bottom-0 py-3">
            <Button @click="exportWord" type="submit"> Export </Button>
          </div>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
