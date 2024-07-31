<script setup lang="ts">
import { Label } from "@/components/ui/label";
import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { computedAsync } from "@vueuse/core";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { CircleDot, Info } from "lucide-vue-next";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import {
  Command,
  CommandDialog,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import { toast } from "vue-sonner";

const props = defineProps<{
  models: string[];
  deckNames: string[];
}>();

const deck = defineModel<string>("deck", { required: true });
const model = defineModel<string>("model", { required: true });
const fields = defineModel<{ [key: string]: string }>("fields", {
  required: true,
});

const openSelectors = ref<boolean[]>([]);

const fieldNames = computedAsync(async () => {
  const fields = await invoke<string[]>("get_note_field_names", {
    model: model.value,
  }).catch((error) => {
    toast.error(error);
  });
  if (fields) {
    openSelectors.value = new Array(fields.length).fill(false);
  }
  return fields;
}, []);

watch(model, async (_) => {
  fields.value = {};
});

function handleOpenChange(index: number) {
  openSelectors.value[index] = !openSelectors.value[index];
}

const builtins = [
  { name: "Word", value: "{word}" },
  { name: "Sentence", value: "{sentence}" },
  { name: "All definitions", value: "{def}" },
];
</script>

<template>
  <Label for="deckselection">Anki deck to be exported to:</Label>
  <StyledCombobox
    :options="props.deckNames"
    v-model="deck"
    item-being-selected="deck"
    id="deckselection"
  />
  <br />
  <Label for="modelselection">Anki model to use for exporting:</Label>
  <StyledCombobox
    :options="props.models"
    v-model="model"
    item-being-selected="model"
    id="modelselection"
  />
  <HoverCard v-if="model">
    <div class="flex items-center">
      <h2 class="pr-1 my-3 text-lg">Note Fields</h2>
      <HoverCardTrigger><Info :size="20" /> </HoverCardTrigger>
    </div>
    <HoverCardContent>
      <p>
        Use variables to make certain content dependent on the card. You can
        also use html.
      </p>
      <p>
        For example, <span class="text-purple-500">{sentence}</span> will be
        replaced with the sentence you mined.
      </p>
      <p>
        Other useful variables are
        <span class="text-purple-500">{sentence}</span> and
        <span class="text-purple-500">{word}</span>.
      </p>
    </HoverCardContent>
  </HoverCard>
  <template v-for="(field, index) in fieldNames">
    <Label :for="index.toString()">{{ field }}</Label>
    <div class="flex gap-2">
      <Input class="w-96" :id="index.toString()" v-model="fields[field]" />
      <Button variant="secondary" @click="handleOpenChange(index)"
        ><CircleDot :size="16"
      /></Button>
      <CommandDialog
        :open="openSelectors[index]"
        @update:open="handleOpenChange(index)"
      >
        <Command>
          <CommandInput class="h-9" placeholder="Search variables" />
          <CommandEmpty>No variable found.</CommandEmpty>
          <CommandList>
            <CommandGroup heading="Builtins">
              <CommandItem
                v-for="builtin in builtins"
                :value="builtin.value"
                @select="
                  if (fields[field]) {
                    fields[field] += builtin.value;
                  } else {
                    fields[field] = builtin.value;
                  }
                  openSelectors[index] = false;
                "
                >{{ builtin.name }}</CommandItem
              >
            </CommandGroup>
          </CommandList>
        </Command>
      </CommandDialog>
    </div>
  </template>
</template>
