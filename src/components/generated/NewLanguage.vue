<script setup lang="ts">
import {
  Dialog,
  DialogClose,
  DialogScrollContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";

const tagColors = new Map<string, string>([
  ["Indo-European", "bg-rose-500"],
  ["Baltic", "bg-pink-500"],
  ["Full Support", "bg-emerald-500"],
]);
// TAGS OVER

const languages: [string, string[]][] = [
  ["Custom", []],
  // START
  ["Lithuanian", ["Indo-European", "Baltic", "Full Support"]],
  // END
];
</script>

<template>
  <Dialog>
    <DialogTrigger as-child>
      <Button variant="ghost" class="text-xl text-center">+</Button>
    </DialogTrigger>
    <DialogScrollContent>
      <DialogHeader>
        <DialogTitle>Add a language</DialogTitle>
        <DialogDescription>
          Click to select a template to use for the new language
        </DialogDescription>
      </DialogHeader>
      <div class="">
        <DialogClose as-child v-for="[language, tags] in languages">
          <div
            @click="$emit('langSelected', language)"
            class="flex items-center p-2 rounded-md hover:bg-accent"
          >
            <h2 class="grow">{{ language }}</h2>
            <Badge
              v-for="tag in tags"
              class="m-1"
              :class="tagColors.get(tag)"
              >{{ tag }}</Badge
            >
          </div>
        </DialogClose>
      </div>
    </DialogScrollContent>
  </Dialog>
</template>
