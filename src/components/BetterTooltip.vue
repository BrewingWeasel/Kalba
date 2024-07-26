<script setup lang="ts">
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";

defineProps<{
  tooltip: string;
  side?: "top" | "bottom" | "left" | "right";
}>();
</script>

<template>
  <TooltipProvider>
    <Tooltip>
      <TooltipTrigger as-child>
        <slot />
      </TooltipTrigger>
      <TooltipContent :side="side ?? 'top'"> {{ tooltip }} </TooltipContent>
    </Tooltip>
  </TooltipProvider>
</template>
