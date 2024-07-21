<script setup lang="ts">
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";

export type SettingsSection =
  | "Appearance"
  | "Exporting"
  | "Word Knowledge"
  | "Dictionaries"
  | "Input"
  | "Grammar";

const section = defineModel<SettingsSection>({ required: true });

const props = defineProps<{
  section: SettingsSection;
  rightLanguage: boolean;
}>();
</script>

<template>
  <Button
    :class="
      cn(
        'justify-start w-full text-md text-left',
        section === props.section &&
          props.rightLanguage &&
          'text-accent-foreground bg-accent',
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
