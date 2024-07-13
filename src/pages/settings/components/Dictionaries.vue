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
  AlertDialogCancel,
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

const dicts = defineModel({
  type: Array<[string, Dictionary]>,
  required: true,
});

const props = defineProps<{
  currentLanguage: string;
}>();

const dictSettings = ref<{ [key: string]: boolean }>({});
for (const dict of dicts.value) {
  dictSettings.value[dict[0]] = false;
}

function addDictionary() {
  dicts.value.push([
    "New dictionary",
    {
      t: DictionaryType.File,
      c: [
        "",
        {
          t: "StarDict",
          c: null,
        },
      ],
    },
  ]);
  dictSettings.value["New dictionary"] = true;
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
        <TableCell><Input v-model="dict[0]" /></TableCell>
        <TableCell>{{ dict[1].t }}</TableCell>
        <TableCell class="text-right">
          <AlertDialog v-model:open="dictSettings[dict[0]]">
            <AlertDialogTrigger><Pencil :size="16" /></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Edit dictionary</AlertDialogTitle>
              </AlertDialogHeader>
              <IndividualDict
                :currentLanguage="props.currentLanguage"
                v-model="dicts[index][1]"
              />
              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
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
