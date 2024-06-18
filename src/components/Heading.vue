<script setup lang="ts">
const props = defineProps({
  title: {
    type: String,
    required: true,
  },
  description: {
    type: String,
    required: true,
  },
  title_id: {
    type: String,
    required: true,
  },
});
</script>

<template>
  <div class="pt-8 pb-3">
    <h1 :id="props.title_id" class="text-xl font-semibold">
      {{ props.title }}
    </h1>
    <Label :for="props.title_id" class="italic">{{ props.description }}</Label>
    <hr class="mb-2 border" />
  </div>
</template>
