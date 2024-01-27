<script setup lang="ts">
import StyledCheckbox from "@/components/StyledCheckbox.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";

import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

export interface Note {
  model: string;
  field: string;
  removeParens: boolean;
  firstWordOnly: boolean;
}

const props = defineProps<{
  note: Note;
  models: string[];
}>();

const fields = computedAsync(async (): Promise<string[]> => {
  props.note.field = "";
  return await invoke("get_note_field_names", {
    model: props.note.model,
  });
}, []);
</script>

<template>
  <div class="flex flex-col gap-1 mb-2">
    <StyledCombobox
      :options="props.models"
      v-model="props.note.model"
      item-being-selected="model"
    />
    <StyledCombobox
      :options="fields"
      v-model="props.note.field"
      item-being-selected="field"
    />
  </div>
  <StyledCheckbox
    label="Remove everything in parentheses"
    name="removeparens"
  />
  <StyledCheckbox label="Only use the first word" name="firstword" />
</template>
