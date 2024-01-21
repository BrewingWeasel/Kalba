<script setup lang="ts">
import { ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { useRouter } from "vue-router";
import { Button } from '@/components/ui/button'
import { Textarea } from '@/components/ui/textarea'
import ReaderView from '@/components/ReaderView.vue'

const currentSentence = ref("");
const sentence = ref("");

function set_sentence() {
  sentence.value = currentSentence.value;
}
</script>

<template>
  <div v-if="sentence.length == 0">
    <div class="grid w-full gap-2">
      <Textarea placeholder="Enter text to change" v-model="currentSentence" />
      <Button @click="set_sentence">Submit</Button>
    </div>
  </div>
  <div v-else>
    <ReaderView :sentence />
  </div>
</template>
