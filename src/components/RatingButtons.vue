<script setup lang="ts">
import { Button } from "@/components/ui/button";
import BetterTooltip from "./BetterTooltip.vue";
</script>

<template>
  <div class="flex gap-3 justify-center">
    <BetterTooltip tooltip="Unknown">
      <Button
        size="icon"
        class="bg-rose-600 hover:bg-rose-500 size-7"
        @click="$emit('change-rating', 0)"
      ></Button
    ></BetterTooltip>
    <BetterTooltip tooltip="Learning">
      <Button
        size="icon"
        class="bg-red-400 hover:bg-red-300 size-7"
        @click="$emit('change-rating', 1)"
      ></Button
    ></BetterTooltip>
    <BetterTooltip tooltip="Recognized">
      <Button
        size="icon"
        class="bg-orange-400 hover:bg-orange-300 size-7"
        @click="$emit('change-rating', 2)"
      ></Button>
    </BetterTooltip>
    <BetterTooltip tooltip="Familiar">
      <Button
        size="icon"
        class="bg-amber-300 hover:bg-amber-200 size-7"
        @click="$emit('change-rating', 3)"
      ></Button>
    </BetterTooltip>
    <BetterTooltip tooltip="Known">
      <Button
        size="icon"
        class="size-7"
        @click="$emit('change-rating', 4)"
      ></Button>
    </BetterTooltip>
    <BetterTooltip tooltip="Ignored">
      <Button
        size="icon"
        class="bg-gray-500 size-7 hover:bg-default"
        @click="$emit('change-rating', -1)"
      ></Button>
    </BetterTooltip>
  </div>
</template>
