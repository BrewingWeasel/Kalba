<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";
import ReaderView from "./Reader.vue";
import ButtonDialog from "@/components/ButtonDialog.vue";
import FilePicker from "@/components/FilePicker.vue";
import { readTextFile } from "@tauri-apps/api/fs";

const currentSentence = ref("");
const sentence = ref("");

const props = defineProps<{ currentLanguage: string }>();

function set_sentence() {
  sentence.value = currentSentence.value;
}
</script>

<template>
  <div v-if="props.currentLanguage">
    <div
      v-if="sentence.length == 0"
      class="flex flex-wrap py-4 px-10 space-x-5 basis-auto"
    >
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="User Input"
        @submitted="set_sentence"
        button-name="Input content"
      >
        <Textarea
          placeholder="Enter text to analyze"
          v-model="currentSentence"
        />
      </ButtonDialog>
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="File Input"
        button-name="Select file"
        @submitted="
          async () => {
            currentSentence = await readTextFile(currentSentence);
            set_sentence();
          }
        "
      >
        <FilePicker v-model="currentSentence" />
      </ButtonDialog>
    </div>
    <div v-else>
      <ReaderView :sentence :currentLanguage />
    </div>
  </div>
  <div v-else>
    <p>Select a language to access the reader view</p>
  </div>
</template>
