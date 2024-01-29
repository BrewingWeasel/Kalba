<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { Button } from "@/components/ui/button";
import IndividualDeck from "@/components/settings/Deck.vue";
import { Deck } from "@/components/settings/Deck.vue";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { computedAsync } from "@vueuse/core";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { ref, watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";

const props = defineProps<{
  models: string[];
  deckNames: string[];
  deck: string;
  model: string;
  fields: { [key: string]: string };
}>();

const deck = ref(props.deck);
const model = ref(props.model);
const fields = ref(props.fields);

const fieldNames = ref(
  props.model == ""
    ? []
    : await invoke("get_note_field_names", {
        model: model.value,
      }),
);

const emit = defineEmits(["set-deck", "set-model", "set-fields"]);

watch(deck, (deck) => {
  emit("set-deck", deck);
});

watch(model, async (model) => {
  fieldNames.value = await invoke("get_note_field_names", {
    model,
  });
  fields.value = {};
  emit("set-model", model);
});

watch(fields, (fields) => {
  emit("set-fields", model);
});
</script>

<template>
  <StyledCombobox
    :options="props.deckNames"
    v-model="deck"
    item-being-selected="deck"
  />
  <StyledCombobox
    :options="props.models"
    v-model="model"
    item-being-selected="model"
  />
  <div v-for="(field, index) in fieldNames">
    <Label :for="index.toString()">{{ field }}</Label>
    <Input :id="index.toString()" v-model="props.fields[field]" />
  </div>
</template>
