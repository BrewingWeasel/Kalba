<script setup lang="ts">
import { DialogClose } from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";

const props = defineProps<{
  isDialog: boolean;
  existingSelection: string;
}>();

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
  <div>
    <component
      :is="props.isDialog ? DialogClose : 'div'"
      as-child
      v-for="[language, tags] in languages"
    >
      <div
        @click="$emit('langSelected', language)"
        :class="
          cn(
            'flex items-center p-2 rounded-md hover:bg-accent',
            props.existingSelection === language
              ? 'bg-indigo-300 dark:bg-indigo-700 hover:bg-indigo-200 dark:hover:bg-indigo-800'
              : '',
          )
        "
      >
        <h2 class="grow">{{ language }}</h2>
        <Badge v-for="tag in tags" class="m-1" :class="tagColors.get(tag)">{{
          tag
        }}</Badge>
      </div>
    </component>
  </div>
</template>
