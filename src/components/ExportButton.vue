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
import { invoke } from "@tauri-apps/api/core";
import type { Settings, ExportDetails } from "@/types";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { toast } from "vue-sonner";
import { ref } from "vue";
import { useMagicKeys, whenever } from "@vueuse/core";

const props = defineProps<{
  word: string;
  sentence: string;
  currentLanguage: string;
}>();

const settings: Settings | undefined = await invoke<Settings>(
  "get_settings",
).catch((e) => {
  toast.error(e);
  return undefined;
});

const models: string[] = await invoke<string[]>("get_all_note_names").catch(
  (e) => {
    toast.error(e);
    return [];
  },
);
const deckNames: string[] = await invoke<string[]>("get_all_deck_names").catch(
  (_) => {
    return [];
  },
);

const emit = defineEmits(["change-rating"]);

const exportDetails = defineModel<ExportDetails>("exportDetails", {
  required: true,
});

exportDetails.value.deck =
  settings?.languages[props.currentLanguage].deck ?? "";
exportDetails.value.model =
  settings?.languages[props.currentLanguage].note_type ?? "";
exportDetails.value.fields =
  settings?.languages[props.currentLanguage].note_fields ?? {};

async function exportWord() {
  console.log(exportDetails.value);
  emit("change-rating", 1, props.word, true);
  await invoke("add_to_anki", { exportDetails: exportDetails.value }).catch(
    (e) => {
      toast.error(e);
    },
  );
}

function selectWord() {
  const selection = window.getSelection();
  if (selection) {
    if (selection.focusNode?.parentElement?.id === "sentence")
      exportDetails.value.sentence = selection.toString();
  }
}

const exportDialogOpen = ref(false);

const keys = useMagicKeys();

whenever(keys.ctrl_e, () => {
  if (!keys.shift.value) {
    exportDialogOpen.value = true;
  }
});

whenever(keys.ctrl_shift_e, async () => {
  await exportWord();
});
</script>

<template>
  <Dialog v-model:open="exportDialogOpen">
    <DialogTrigger
      as-child
      @click.shift="
        exportDialogOpen = false;
        exportWord();
      "
    >
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
                  {{ exportDetails.sentence }}
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
                  :language="props.currentLanguage"
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
