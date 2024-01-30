<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Button } from "@/components/ui/button";
import { Textarea } from "@/components/ui/textarea";
import ReaderView from "@/components/ReaderView.vue";

if (await invoke("get_dark_mode")) {
  document.documentElement.classList.add("dark");
}

const currentSentence = ref("");
const sentence = ref("");

function set_sentence() {
  sentence.value = currentSentence.value;
}
</script>

<template>
  <div v-if="sentence.length == 0">
    <div class="grid px-10 py-3 gap-2">
      <Textarea placeholder="Enter text to analyze" v-model="currentSentence" />
      <Button @click="set_sentence">Submit</Button>
    </div>
  </div>
  <div v-else>
    <ReaderView :sentence />
  </div>
</template>
