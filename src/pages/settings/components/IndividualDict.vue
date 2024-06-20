<script setup lang="ts">
import { Input } from "@/components/ui/input";
import StyledCombobox from "@/components/StyledCombobox.vue";
import FilePicker from "@/components/FilePicker.vue";
import { type Dictionary, DictionaryType, type FileType } from "@/types";
import { watch } from "vue";

const dict = defineModel<Dictionary>({ required: true });

watch(
	() => dict.value.t,
	async (newT, _) => {
		console.log(newT);
		switch (newT) {
			case DictionaryType.File: {
				dict.value.c = [
					"",
					{
						t: "StarDict",
						c: null,
					},
				];
				break;
			}
			case DictionaryType.Wiktionary: {
				dict.value.c = ["en", ""];
				break;
			}
			case DictionaryType.EkalbaBendrines:
			case DictionaryType.EkalbaDabartines: {
				dict.value.c = undefined;
				break;
			}
			case DictionaryType.Url:
			case DictionaryType.Command: {
				dict.value.c = "";
				break;
			}
		}
	},
);

function isWiktionary(
	dictType: DictionaryType,
	_contents: any,
): _contents is [string, string] {
	return dictType == "Wiktionary";
}

function isFile(
	dictType: DictionaryType,
	_contents: any,
): _contents is [string, FileType] {
	return dictType == "File";
}
</script>

<template>
  <Label for="dicttype">Dictionary Type:</Label>
  <StyledCombobox
    :options="[
      ['File', 'File'],
      ['Url', 'Url'],
      ['Command', 'Command'],
      ['Wiktionary', 'Wiktionary'],
      ['EkalbaBendrines', 'Bendrinės lietuvių kalbos žodynas'],
      ['EkalbaDabartines', 'Dabartinės lietuvių kalbos žodynas'],
    ]"
    v-model="dict.t"
    item-being-selected="dictionary type"
    id="dicttype"
  />
  <div v-if="isFile(dict.t, dict.c)">
    <Label for="filepicker">Dictionary File:</Label>
    <FilePicker v-model="dict.c[0]" id="filepicker" />
    <Label for="filetype">File type: </Label>
    <br />
    <StyledCombobox
      :options="['TextSplitAt', 'StarDict']"
      v-model="dict.c[1].t"
      item-being-selected="file dictionary type"
      id="filetype"
    />
    <br />
    <Label for="separator">Definition Separator: </Label>
    <Input
      type="text"
      v-if="dict.c[1].t == 'TextSplitAt' && dict.c[1].c"
      v-model="dict.c[1].c"
      class="w-20"
      id="separator"
    />
  </div>
  <div v-else-if="dict.t == 'Command' && typeof dict.c === 'string'">
    <Label for="command">Command to run for definition:</Label>
    <Input type="text" id="command" v-model="dict.c" />
  </div>
  <div v-else-if="isWiktionary(dict.t, dict.c)">
    <Label for="definitionlang">Definition Language (two letter code):</Label>
    <Input type="text" v-model="dict.c[0]" class="w-20" id="definitionlang" />
    <Label for="wordlang">Word Language:</Label>
    <Input type="text" v-model="dict.c[1]" class="w-100" id="wordlang" />
  </div>
  <div v-else-if="dict.t == 'Url' && typeof dict.c === 'string'">
    <Label for="command">Url:</Label>
    <Input type="text" id="command" v-model="dict.c" />
  </div>
</template>
