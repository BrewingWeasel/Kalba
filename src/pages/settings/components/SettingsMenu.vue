<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

export type SettingsSection =
  | "Appearance"
  | "Exporting"
  | "Word Knowledge"
  | "Dictionaries"
  | "Grammar";

const section = defineModel<SettingsSection>({ required: true });

const props = defineProps<{
  section: SettingsSection;
}>();
</script>

<template>
  <Button
    :class="
      cn(
        'justify-start w-full text-md text-left',
        section === props.section && 'text-rose-500 bg-accent',
      )
    "
    variant="ghost"
    @click="
      () => {
        section = props.section;
      }
    "
    >{{ props.section }}</Button
  >
</template>
