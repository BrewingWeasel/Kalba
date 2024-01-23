<script setup lang="ts">
const props = defineProps(['morph'])
import { Separator } from '@/components/ui/separator'
</script>

<template>
  <div class="flex flex-wrap justify-center">
    <div v-for="(value, grammar) in props.morph" class="flex">
      <p class="text-xs"><i>{{ grammar }}:&nbsp;</i> {{ value }}</p>
      <Separator orientation="vertical" class="mx-2" />
    </div>
  </div>
</template>
