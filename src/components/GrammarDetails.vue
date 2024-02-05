<script setup lang="ts">
const props = defineProps(["morph", "separator"]);
import { Separator } from "@/components/ui/separator";
</script>

<template>
  <div class="flex flex-wrap justify-center">
    <div v-for="(value, grammar) in props.morph" class="flex">
      <p class="text-xs">
        <i>{{ grammar }}:&nbsp;</i> {{ value }}
      </p>
      <Separator v-if="props.separator" orientation="vertical" class="mx-2" />
      <span v-else>&nbsp;</span>
    </div>
  </div>
</template>
