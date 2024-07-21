<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";
import ReaderView from "./Reader.vue";
import ButtonDialog from "@/components/ButtonDialog.vue";
import FilePicker from "@/components/FilePicker.vue";
import { readTextFile } from "@tauri-apps/api/fs";
import { useRouter } from "vue-router";

const currentSentence = ref("");
const sentence = ref("");
const isUrl = ref(false);

const props = defineProps<{ currentLanguage: string }>();

const router = useRouter();
router.replace("/reader/input");

function set_sentence() {
  sentence.value = currentSentence.value;
}
</script>

<template>
  <div v-if="props.currentLanguage" class="h-full">
    <div
      v-if="sentence.length == 0"
      class="flex flex-wrap py-4 px-10 space-x-5 basis-auto"
    >
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="User Input"
        @submitted="
          router.replace('/reader/custom');
          set_sentence();
        "
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
            router.replace(`/reader/file/${currentSentence}`);
            currentSentence = await readTextFile(currentSentence);
            set_sentence();
          }
        "
      >
        <FilePicker v-model="currentSentence" />
      </ButtonDialog>
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="Url"
        @submitted="
          router.replace('/reader/url');
          isUrl = true;
          set_sentence();
        "
        button-name="Enter a url"
      >
        <Textarea placeholder="Enter url to use" v-model="currentSentence" />
      </ButtonDialog>
    </div>
    <div v-else class="h-full">
      <ReaderView :sentence :currentLanguage :isUrl class="h-full" />
    </div>
  </div>
  <div v-else>
    <p>Select a language to access the reader view</p>
  </div>
</template>
