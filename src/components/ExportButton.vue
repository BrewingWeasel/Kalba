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
import Exporting from "@/components/ExportingConfiguration.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { ref, Ref } from "vue";
import { Settings, ExportDetails } from "@/types";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

const props = defineProps<{
  word: string;
  sentence: string;
  defs: string[];
}>();

const settings: Settings = await invoke("get_settings");

const models: string[] = await invoke("get_all_note_names");
const deckNames: string[] = await invoke("get_all_deck_names");

const emit = defineEmits(["change-rating"]);

const exportDetails: Ref<ExportDetails> = ref({
  word: props.word,
  defs: props.defs,
  deck: settings.deck,
  model: settings.note_type,
  sentence: "",
  fields: settings.note_fields,
});

async function exportWord() {
  emit("change-rating", 1, props.word, true);
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
      <div class="flex bottom-0 justify-center py-3">
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
      <Tabs default-value="context" class="w-[400px]">
        <TabsList>
          <TabsTrigger value="context"> Context </TabsTrigger>
          <TabsTrigger value="location"> Location </TabsTrigger>
        </TabsList>
        <TabsContent value="context">
          <Card>
            <CardHeader>
              <CardTitle>Context</CardTitle>
              <CardDescription
                >Highlight the context to be exported along with the
                sentence</CardDescription
              >
            </CardHeader>
            <CardContent>
              <p v-if="exportDetails.sentence == ''">
                Select the context to export
              </p>
              <p v-else>
                {{ exportDetails.sentence }}
              </p>
              <div class="p-2 mt-2 rounded-md border-2 border-slate-800">
                <p
                  @mouseup="selectWord"
                  @mousedown="selectWord"
                  @mouseleave="selectWord"
                  id="sentence"
                  class="select-auto selection:bg-pink-300"
                >
                  {{ props.sentence }}
                </p>
              </div>
            </CardContent>
          </Card>
        </TabsContent>
        <TabsContent value="location">
          <Card>
            <CardHeader>
              <CardTitle>Location</CardTitle>
              <CardDescription
                >Edit the settings for exporting to Anki</CardDescription
              >
            </CardHeader>
            <CardContent>
              <div class="select-auto">
                <Exporting
                  :models
                  :deckNames
                  v-model:deck="exportDetails.deck"
                  v-model:model="exportDetails.model"
                  v-model:fields="exportDetails.fields"
                />
              </div>
            </CardContent>
          </Card>
        </TabsContent>
      </Tabs>
      <DialogFooter>
        <DialogClose as-child>
          <div class="flex bottom-0 justify-center py-3">
            <Button @click="exportWord" type="submit"> Export </Button>
          </div>
        </DialogClose>
      </DialogFooter>
    </DialogContent>
  </Dialog>
</template>
