<script setup lang="ts">
import { Input } from "@/components/ui/input";
import StyledCombobox from "@/components/StyledCombobox.vue";
import FilePicker from "@/components/FilePicker.vue";
import {
  type DictionarySpecificSettings,
  DictionaryType,
  type FileType,
} from "@/types";
import { watch } from "vue";
import { Switch } from "@/components/ui/switch";
import { Label } from "@/components/ui/label";

const dict = defineModel<DictionarySpecificSettings>({ required: true });

const props = defineProps<{
  currentLanguage: string;
}>();

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
        dict.value.c = ["en", props.currentLanguage];
        break;
      }
      case DictionaryType.EkalbaBendrines:
      case DictionaryType.EkalbaDabartines: {
        dict.value.c = undefined;
        break;
      }
      case DictionaryType.Url: {
        dict.value.c = ["", true, ""];
        break;
      }
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
  return dictType === "Wiktionary";
}

function isWordReference(
  dictType: DictionaryType,
  _contents: any,
): _contents is [string, string] {
  return dictType === "WordReference";
}

function isUrl(
  dictType: DictionaryType,
  _contents: any,
): _contents is [string, boolean, string] {
  return dictType === "Url";
}

function isFile(
  dictType: DictionaryType,
  _contents: any,
): _contents is [string, FileType] {
  return dictType === "File";
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
      ['WordReference', 'WordReference'],
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
  <div v-else-if="isWordReference(dict.t, dict.c)">
    <Label for="definitionlang">Definition Language (two letter code):</Label>
    <Input type="text" v-model="dict.c[0]" class="w-20" id="definitionlang" />
    <Label for="wordlang">Word Language (two letter code):</Label>
    <Input type="text" v-model="dict.c[1]" class="w-20" id="wordlang" />
  </div>
  <div v-else-if="isUrl(dict.t, dict.c)">
    <Label for="command">Url:</Label>
    <Input type="text" id="command" v-model="dict.c[0]" />

    <Label for="embed">Embed page</Label>
    <Switch id="embed" v-model:checked="dict.c[1]" />

    <div v-if="!dict.c[1]">
      <Label for="selector">CSS selector</Label>
      <Input id="selector" v-model="dict.c[2]" />
    </div>
  </div>
</template>
