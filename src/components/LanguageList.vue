<script setup lang="ts">
import { DialogClose } from "@/components/ui/dialog";
import { Badge } from "@/components/ui/badge";
import { cn } from "@/lib/utils";
import { toast } from "vue-sonner";

const props = defineProps<{
  isDialog: boolean;
  existingSelection: string;
}>();

const tagColors = new Map<string, string>([
  ["Indo-European", "bg-rose-500"],
  ["Baltic", "bg-pink-500"],
  ["Full Support", "bg-emerald-500"],
]);

interface TemplateDetails {
  language: string;
  tags: string[];
}

async function getLanguages() {
  try {
    const response = await fetch(
      "https://raw.githubusercontent.com/BrewingWeasel/Kalba/main/data/templates.json",
    );
    if (!response.ok) {
      throw new Error(`Response status: ${response.status}`);
    }

    const json = (await response.json()) as TemplateDetails[];
    return json;
  } catch (error) {
    toast.error(error as string);
    return [];
  }
}

const languages = await getLanguages();
</script>

<template>
  <div>
    <component
      :is="props.isDialog ? DialogClose : 'div'"
      as-child
      v-for="template in languages"
    >
      <div
        @click="$emit('langSelected', template.language)"
        :class="
          cn(
            'flex items-center p-2 rounded-md hover:bg-accent',
            props.existingSelection === template.language
              ? 'bg-indigo-300 dark:bg-indigo-700 hover:bg-indigo-200 dark:hover:bg-indigo-800'
              : '',
          )
        "
      >
        <h2 class="grow">{{ template.language }}</h2>
        <Badge
          v-for="tag in template.tags"
          class="m-1"
          :class="tagColors.get(tag)"
          >{{ tag }}</Badge
        >
      </div>
    </component>
  </div>
</template>
