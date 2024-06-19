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
import IndividualDict from "@/components/settings/IndividualDict.vue";
import { Dictionary, DictionaryType } from "@/types";
import { Pencil, X } from "lucide-vue-next";

const dicts = defineModel({ type: Array<Dictionary>, required: true });

function addDictionary() {
  dicts.value.push({
    t: DictionaryType.File,
    c: [
      "",
      {
        t: "StarDict",
        c: null,
      },
    ],
  });
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
        <TableCell><Input /></TableCell>
        <TableCell>{{ dict.t }}</TableCell>
        <TableCell class="text-right">
          <AlertDialog>
            <AlertDialogTrigger><Pencil :size="16" /></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Edit dictionary</AlertDialogTitle>
              </AlertDialogHeader>
              <IndividualDict v-model="dicts[index]" />
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
