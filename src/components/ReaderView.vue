<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
import { Button } from '@/components/ui/button'

const words = ref([{ display: "Labas", }, "kaip", "gyveni"]);
const router = useRouter();

</script>

<template>
  <v-for="word in words">
</template>
