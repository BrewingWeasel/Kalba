<script setup lang="ts">
import { Input } from "@/components/ui/input";
import StyledCombobox from "@/components/StyledCombobox.vue";
import { Dictionary, DictionaryType } from "@/types";
import { watch } from "vue";

const dict = defineModel<Dictionary>({ required: true });

watch(
  () => dict.value.t,
  async (newT, _) => {
    console.log(newT);
    switch (newT) {
      case DictionaryType.File: {
        dict.value.c = {
          file: "",
          filetype: {
            t: "StarDict",
            c: null,
          },
        };
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
</script>

<template>
  <StyledCombobox
    :options="['File', 'Url', 'Command']"
    v-model="dict.t"
    item-being-selected="dictionary type"
  />
  <div v-if="dict.t == 'File' && typeof dict.c !== 'string'">
    <Input
      type="file"
      @change="
        (e: string) => {
          if (typeof dict.c !== 'string') {
            dict.c.file = e;
          }
        }
      "
    />
    <StyledCombobox
      :options="['TextSplitAt', 'StarDict']"
      v-model="dict.c.filetype.t"
      item-being-selected="file dictionary type"
    />
    <Input
      type="text"
      placeholder="Definition separator"
      v-if="dict.c.filetype.t == 'TextSplitAt'"
      v-model="
        //@ts-ignore
        dict.c.filetype.c
      "
    />
  </div>
  <div v-if="typeof dict.c === 'string'">
    <Input type="text" v-model="dict.c" />
  </div>
</template>
