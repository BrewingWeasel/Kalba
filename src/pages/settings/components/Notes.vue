<script setup lang="ts">
import StyledCheckbox from "@/components/StyledCheckbox.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";

import { invoke } from "@tauri-apps/api/tauri";
import { computedAsync } from "@vueuse/core";

export interface Note {
  model: string;
  handling: NoteToWordHandling;
}

interface NoteToWordHandling {
  field_to_use: string;
  only_first_word_or_line: boolean;
  remove_everything_in_parens: boolean;
  tags_wanted: string[];
}

const props = defineProps<{
  note: Note;
  models: string[];
}>();

const fields = computedAsync(async (): Promise<string[]> => {
  props.note.handling.field_to_use = "";
  return await invoke("get_note_field_names", {
    model: props.note.model,
  });
}, []);
</script>

<template>
  <div class="pl-5 bg-slate-200 dark:bg-slate-800 rounded-md">
    <div class="flex flex-col gap-1 mb-2 pt-2">
      <StyledCombobox
        :options="props.models"
        v-model="props.note.model"
        item-being-selected="model"
      />
      <StyledCombobox
        :options="fields"
        v-model="props.note.handling.field_to_use"
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
