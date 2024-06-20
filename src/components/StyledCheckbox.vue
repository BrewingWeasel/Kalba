<script setup lang='ts'>
import { Label } from "@/components/ui/label";
import { Checkbox } from "@/components/ui/checkbox";

const props = defineProps(["name", "label"]);
</script>

<template>
  <div class="flex items-center space-x-2 pb-1">
    <Checkbox :id="props.name" class="accent-pink-500" />
    <Label :for="props.name"
      class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
      {{ props.label }}
    </Label>
  </div>
</template>
