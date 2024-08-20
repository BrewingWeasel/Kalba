<script setup lang="ts">
import StyledCheckbox from "@/components/StyledCheckbox.vue";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Note } from "@/types";

import { invoke } from "@tauri-apps/api/core";
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
      v-model="note.handling.remove_everything_in_parens"
    />
    <StyledCheckbox
      label="Only use the first word"
      name="firstword"
      v-model="note.handling.only_first_word_or_line"
    />
    <div class="pt-2">
      <Label for="search">
        Anki search parameters (these use the same syntax as the browse menu in
        Anki)
      </Label>
      <Input id="search" v-model="note.handling.search_params" />
    </div>
  </div>
</template>
