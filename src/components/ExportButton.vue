<script setup lang="ts">
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import Exporting from "@/components/settings/Exporting.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
import { Settings, ExportDetails } from "@/types";

const props = defineProps<{
  word: string;
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
</script>

<template>
  <Dialog>
    <DialogTrigger as-child>
      <div class="flex justify-center bottom-0 py-3">
        <Button variant="destructive"> Export </Button>
      </div>
    </DialogTrigger>
    <DialogContent class="sm:max-w-[425px]">
      <DialogHeader>
        <DialogTitle>Export word</DialogTitle>
        <DialogDescription>
          Export the word to Anki with information
        </DialogDescription>
      </DialogHeader>
      <Exporting
        :models
        :deckNames
        v-model:deck="exportDetails.deck"
        v-model:model="exportDetails.model"
        v-model:fields="exportDetails.fields"
      />
      <DialogFooter>
        <Button @click="exportWord" type="submit"> Export </Button>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
