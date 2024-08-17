<script setup lang="ts">
import BetterTagInput from "@/components/BetterTagInput.vue";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Textarea } from "@/components/ui/textarea";
import { invoke } from "@tauri-apps/api/tauri";
import { AlertCircle } from "lucide-vue-next";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";
import { Ref, ref } from "vue";

const parser = defineModel<string>("parser", { required: true });
const runOnLemmas = defineModel<string[]>("runOnLemmas", { required: true });
const suggestOnLemmas = defineModel<string[]>("suggestOnLemmas", {
  required: true,
});

const functions: Ref<string[]> = ref([]);
const error: Ref<string | null> = ref(null);

console.log(runOnLemmas.value);
console.log(suggestOnLemmas.value);

async function checkSpyglys() {
  await invoke<string[]>("get_spyglys_functions", {
    spyglysGrammar: parser.value,
  })
    .then((response) => {
      functions.value = response;
      error.value = null;
    })
    .catch((e) => {
      error.value = e;
    });
}

async function formatSpyglys() {
  await invoke<string>("format_spyglys", { spyglysGrammar: parser.value })
    .then((formmated) => {
      parser.value = formmated;
    })
    .catch((e) => {
      error.value = e;
    });
}

await checkSpyglys();
</script>

<template>
  <Alert variant="destructive" v-if="error">
    <AlertCircle class="w-4 h-4" />
    <AlertTitle>Error</AlertTitle>
    <AlertDescription>
      {{ error }}
    </AlertDescription>
  </Alert>

  <div class="flex justify-between">
    <Label for="grammarparser">Grammar Parser</Label>
    <div>
      <Button
        @click="formatSpyglys"
        variant="secondary"
        size="sm"
        class="mr-2 mb-1"
        >Format</Button
      >
      <Button @click="checkSpyglys" variant="secondary" size="sm">Check</Button>
    </div>
  </div>
  <Textarea id="grammarparser" class="h-64" v-model="parser" />
  <br />

  <Label for="runlemmas">Functions to run on lemmas</Label>
  <BetterTagInput
    class="w-2/3"
    id="runlemmas"
    v-model="runOnLemmas"
    placeholder="spyglys functions"
    :options="functions"
  />
  <br />

  <Label for="suggestlemmas">Functions to suggest on lemmas</Label>
  <BetterTagInput
    class="w-2/3"
    id="suggestlemmas"
    v-model="suggestOnLemmas"
    placeholder="spyglys functions"
    :options="functions"
  />
</template>
