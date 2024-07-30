<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  AlertDialog,
  AlertDialogAction,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import { Input } from "@/components/ui/input";

import { Button } from "@/components/ui/button";
import IndividualDict from "./IndividualDict.vue";
import { type Dictionary, DictionaryType } from "@/types";
import { Pencil, X } from "lucide-vue-next";
import { ref } from "vue";
import { Switch } from "@/components/ui/switch";
import StyledCombobox from "@/components/StyledCombobox.vue";

const dicts = defineModel({
  type: Array<Dictionary>,
  required: true,
});

const props = defineProps<{
  currentLanguage: string;
}>();

const dictSettings = ref<{ [key: string]: boolean }>({});
const dictIsReplacement = ref<{ [key: string]: boolean }>({});
for (const dict of dicts.value) {
  dictSettings.value[dict.name] = false;
  dictIsReplacement.value[dict.name] = dict.run_when_not ? true : false;
}

function addDictionary() {
  dicts.value.push({
    name: "New dictionary",
    fetch_by_default: true,
    specific_settings: {
      t: DictionaryType.File,
      c: [
        "",
        {
          t: "StarDict",
          c: null,
        },
      ],
    },
  });
  dictSettings.value["New dictionary"] = true;
  dictIsReplacement.value["New dictionary"] = false;
}
</script>

<template>
  <Table class="max-w-5xl">
    <TableCaption>
      <Button variant="ghost" class="text-xl" @click="addDictionary">+</Button>
    </TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead class="w-1/3">Name</TableHead>
        <TableHead>Type</TableHead>
        <TableHead class="text-right">Configure</TableHead>
        <TableHead class="w-1 text-right">Remove</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="(dict, index) in dicts" :key="index">
        <TableCell><Input v-model="dict.name" /></TableCell>
        <TableCell>{{ dict.specific_settings.t }}</TableCell>
        <TableCell class="text-right">
          <AlertDialog v-model:open="dictSettings[dict.name]">
            <AlertDialogTrigger><Pencil :size="16" /></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Edit dictionary</AlertDialogTitle>
              </AlertDialogHeader>
              <IndividualDict
                :currentLanguage="props.currentLanguage"
                v-model="dicts[index].specific_settings"
              />
              <Label for="toggle-replacement">Backup dictionary</Label>
              <Switch
                id="toggle-replacement"
                v-model:checked="dictIsReplacement[dict.name]"
                @update:checked="
                  if (!$event) {
                    dicts[index].run_when_not = undefined;
                  }
                "
              />
              <div v-if="dictIsReplacement[dict.name]">
                <Label for="dict-replacement">Dictionary to replace</Label>
                <StyledCombobox
                  id="dict-replacement"
                  v-model="dicts[index].run_when_not"
                  :options="
                    dicts.map((d) => d.name).filter((n) => n !== dict.name)
                  "
                  item-being-selected="dictionary to replace"
                />
              </div>
              <Label for="fetch">Fetch by default</Label>
              <Switch
                id="fetch"
                v-model:checked="dicts[index].fetch_by_default"
              />
              <AlertDialogFooter>
                <AlertDialogAction>Save</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </TableCell>
        <TableCell
          ><X
            @click="
              () => {
                dicts.splice(index, 1);
              }
            "
            class="float-right transition duration-200 cursor-pointer hover:scale-110 hover:stroke-rose-500"
            :size="16"
        /></TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
