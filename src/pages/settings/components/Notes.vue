<script setup lang="ts">
import StyledCheckbox from "@/components/StyledCheckbox.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { Note } from "@/types";

import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

const note = defineModel<Note>({ required: true });

const props = defineProps<{
  models: string[];
}>();

const fields = computedAsync(async (): Promise<string[]> => {
  return await invoke("get_note_field_names", {
    model: note.value.model,
  });
}, []);
</script>

<template>
  <div>
    <div class="flex flex-col gap-1 mb-2 pt-2">
      <StyledCombobox
        :options="props.models"
        v-model="note.model"
        item-being-selected="note type"
      />
      <StyledCombobox
        :options="fields"
        v-model="note.handling.field_to_use"
        item-being-selected="field"
      />
    </div>
    <StyledCheckbox
      label="Remove everything in parentheses"
      name="removeparens"
    />
    <StyledCheckbox label="Only use the first word" name="firstword" />
  </div>
</template>
