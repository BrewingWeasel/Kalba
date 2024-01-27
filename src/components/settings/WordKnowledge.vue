<script setup lang="ts">
import { ref } from "vue";

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import StyledCheckbox from "@/components/StyledCheckbox.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";

import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

const selectedTemplate = ref("");
const selectedField = ref("");

const models: string[] = await invoke("get_all_note_names", {});
const fields = computedAsync(async (): Promise<string[]> => {
  return await invoke("get_note_field_names", {
    model: selectedTemplate.value,
  });
}, []);
</script>

<template>
  <Accordion type="single" collapsible>
    <AccordionItem value="item-1">
      <AccordionTrigger>Language learning card</AccordionTrigger>
      <AccordionContent>
        <StyledCombobox
          :options="models"
          v-model="selectedTemplate"
          item-being-selected="model"
        />
        <StyledCombobox
          :options="fields"
          v-model="selectedField"
          item-being-selected="field"
        />
        <StyledCheckbox
          label="Remove everything in parentheses"
          name="removeparens"
        />
        <StyledCheckbox label="Only use the first word" name="firstword" />
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
