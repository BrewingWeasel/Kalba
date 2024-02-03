<script setup lang="ts">
defineProps(["definition"]);
</script>

<template>
  <div v-for="def in definition">
    <span v-if="def['t'] == 'Ok'" v-html="def['conts']"></span>
    <span v-else class="text-pink-600">{{ def.conts }}</span>
  </div>
</template>
