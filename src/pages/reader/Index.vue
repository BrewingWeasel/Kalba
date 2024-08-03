<script setup lang="ts">
import { ref } from "vue";
import { Textarea } from "@/components/ui/textarea";
import ReaderView from "./Reader.vue";
import ButtonDialog from "@/components/ButtonDialog.vue";
import FilePicker from "@/components/FilePicker.vue";
import { readTextFile } from "@tauri-apps/api/fs";
import { useRouter } from "vue-router";
import { InputType } from "@/types";
import { readText } from "@tauri-apps/api/clipboard";

const currentInput = ref("");
const inputText = ref("");
const inputType = ref<InputType>("normal");

const props = defineProps<{ currentLanguage: string }>();

const router = useRouter();
router.replace("/reader/input");

function set_sentence() {
  inputText.value = currentInput.value;
}
</script>

<template>
  <div v-if="props.currentLanguage" class="h-full">
    <div
      v-if="inputText.length == 0"
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
        <Textarea placeholder="Enter text to analyze" v-model="currentInput" />
      </ButtonDialog>
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="File Input"
        button-name="Select file"
        @submitted="
          async () => {
            router.replace(`/reader/file/${currentInput}`);
            currentInput = await readTextFile(currentInput);
            set_sentence();
          }
        "
      >
        <FilePicker v-model="currentInput" />
      </ButtonDialog>
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="Url"
        @submitted="
          router.replace('/reader/url');
          inputType = 'url';
          set_sentence();
        "
        button-name="Enter a url"
      >
        <Textarea placeholder="Enter url to use" v-model="currentInput" />
      </ButtonDialog>
      <ButtonDialog
        class="flex-1 my-2 max-w-md"
        title="Clipboard"
        @submitted="
          async () => {
            router.replace('/reader/clipboard');
            inputType = 'clipboard';
            currentInput = (await readText()) ?? 'Empty clipboard';
            set_sentence();
          }
        "
        button-name="Use clipboard"
      >
      </ButtonDialog>
    </div>
    <div v-else class="h-full">
      <ReaderView
        v-model:inputText="inputText"
        :currentLanguage
        :inputType
        class="h-full"
      />
    </div>
  </div>
  <div v-else>
    <p>Select a language to access the reader view</p>
  </div>
</template>
