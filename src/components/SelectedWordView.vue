<script setup lang="ts">
const props = defineProps(['word'])
</script>

<template>
  <h1>{{ word.lemma }}</h1>
  <h2><i>{{ word.text }}</i></h2>
</template>
