<script setup lang="ts">
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { computedAsync } from "@vueuse/core";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { watch } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Info } from "lucide-vue-next";
import {
	HoverCard,
	HoverCardContent,
	HoverCardTrigger,
} from "@/components/ui/hover-card";
import { toast } from 'vue-sonner';

const props = defineProps<{
	models: string[];
	deckNames: string[];
}>();

const deck = defineModel<string>("deck", { required: true });
const model = defineModel<string>("model", { required: true });
const fields = defineModel<{ [key: string]: string }>("fields", {
	required: true,
});

const fieldNames = computedAsync(
	async () =>
      await invoke<string[]>("get_note_field_names", {
			model: model.value,
		}).catch((error) => {
         toast.error(error);
   }),
	[],
);

watch(model, async (_) => {
	fields.value = {};
});
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
    <Input :id="index.toString()" v-model="fields[field]" />
  </template>
</template>
