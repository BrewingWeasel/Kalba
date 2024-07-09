<script setup lang="ts">
import { Suspense } from "vue";
import Settings from "./Settings.vue";

const props = defineProps<{
  currentLanguage: string | null;
}>();
</script>

<template>
  <Suspense><Settings :current-language="props.currentLanguage" /></Suspense>
</template>
