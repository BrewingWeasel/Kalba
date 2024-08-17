<script setup lang="ts">
import LanguageList from "@/components/LanguageList.vue";
import Button from "@/components/ui/button/Button.vue";
import {
  Stepper,
  StepperDescription,
  StepperIndicator,
  StepperItem,
  StepperSeparator,
  StepperTitle,
  StepperTrigger,
} from "@/components/ui/stepper";
import {
  BrainCog,
  Check,
  GraduationCap,
  Hand,
  Languages,
  RotateCcw,
} from "lucide-vue-next";
import { computed, ref } from "vue";
import WordKnowledge from "./settings/components/WordKnowledge.vue";
import { invoke } from "@tauri-apps/api/tauri";
import EnableStanza from "@/components/EnableStanza.vue";
import { Deck } from "@/types";
import { toast } from "vue-sonner";

const stepIndex = ref(1);

const selectedLang = ref("");
const decks = ref<Deck[]>([]);

const deckNames = ref<undefined | string[]>(undefined);
const models = ref<undefined | string[]>(undefined);
async function setDeckNames() {
  deckNames.value = await invoke<string[]>("get_all_deck_names").catch(
    () => undefined,
  );
  models.value = await invoke<string[]>("get_all_note_names").catch(
    () => undefined,
  );
}

setDeckNames();

const nextName = computed(() => {
  if (!models.value && stepIndex.value === 3) {
    return "Skip";
  }
  if (stepIndex.value === 5) {
    return "Finish";
  }
  return "Next";
});

const stanzaEnabled = ref(false);
const stanzaInstalled = ref(false);

const firstTime = defineModel<boolean>();

async function finishWizard() {
  console.log("Finished wizard");
  firstTime.value = false;
  await invoke("get_started", {
    starting: {
      template: selectedLang.value,
      decks: decks.value,
      stanza_enabled: stanzaEnabled.value,
    },
  }).catch((e) => {
    toast.error(e);
  });
}
</script>

<template>
  <div class="flex flex-col gap-2 w-full h-screen justify-center items-center">
    <Stepper v-model="stepIndex" class="w-[40rem]">
      <StepperItem :step="1">
        <StepperTrigger>
          <StepperIndicator><Hand /></StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle> Welcome </StepperTitle>
            <StepperDescription> Get started with Kalba </StepperDescription>
          </div>
        </StepperTrigger>
        <StepperSeparator />
      </StepperItem>
      <StepperItem :step="2">
        <StepperTrigger>
          <StepperIndicator><Languages /></StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle> Profile </StepperTitle>
            <StepperDescription> Set up a target language </StepperDescription>
          </div>
        </StepperTrigger>
      </StepperItem>
      <StepperItem :step="3">
        <StepperTrigger>
          <StepperIndicator><GraduationCap /></StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle> Word Knowledge </StepperTitle>
            <StepperDescription> Configure word knowledge </StepperDescription>
          </div>
        </StepperTrigger>
      </StepperItem>
      <StepperItem :step="4">
        <StepperTrigger>
          <StepperIndicator><BrainCog /></StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle> Stanza </StepperTitle>
            <StepperDescription> Configure Stanza </StepperDescription>
          </div>
        </StepperTrigger>
      </StepperItem>
      <StepperItem :step="5">
        <StepperTrigger>
          <StepperIndicator><Check /></StepperIndicator>
          <div class="flex flex-col">
            <StepperTitle> Finish </StepperTitle>
            <StepperDescription> Complete the setup </StepperDescription>
          </div>
        </StepperTrigger>
      </StepperItem>
    </Stepper>
    <div class="p-10 border-2 rounded-md w-[40rem]">
      <template v-if="stepIndex === 1">
        <h1 class="text-xl font-semibold">Welcome to Kalba</h1>
        <p>
          This installation wizard will walk you through setting up a target
          language.
        </p>
        <p>All of these settings can be changed at any time</p>
        <p>
          For more information about any of the settings, see
          <a
            class="underline decoration-pink-400 decoration-2 underline-offset-2 hover:underline-offset-1"
            href="https://kalba.readthedocs.io/en/latest/"
            target="_blank"
          >
            the docs.</a
          >
        </p>
      </template>
      <template v-if="stepIndex === 2">
        <h1 class="text-xl font-semibold">Select a starter template</h1>
        <p>
          Templates offer a quick way to get started with certain languages.
          They can always be modified later.
        </p>
        <p>
          Languages that do not have a tempalte can still be used by editing the
          custom template to include dictionaries and grammar information
        </p>
        <LanguageList
          @langSelected="
            (template) => {
              selectedLang = template;
              console.log(selectedLang);
              stepIndex++;
            }
          "
          :isDialog="false"
          :existingSelection="selectedLang"
        />
      </template>
      <template v-if="stepIndex === 3">
        <h1 class="text-xl font-semibold">Customize Word Knowledge</h1>
        <template v-if="models && deckNames">
          <WordKnowledge :models :deckNames :decks />
        </template>
        <div v-else class="flex items-center flex-col mt-3">
          <h2>Anki is not available</h2>
          <p>
            Anki may not be open or the AnkiConnect extension may not be
            installed
          </p>
          <Button variant="outline" @click="setDeckNames"
            ><RotateCcw :size="16"
          /></Button>
        </div>
      </template>
      <template v-if="stepIndex === 4">
        <h1 class="text-xl font-semibold">Configure Stanza</h1>
        <EnableStanza
          v-model:installed="stanzaInstalled"
          v-model:enabled="stanzaEnabled"
        />
      </template>
      <template v-if="stepIndex === 5">
        <h1 class="text-xl font-semibold">Next steps</h1>
        <p>To get started with reading texts, check out the reader page.</p>
        <p>
          You may also want to customize the template's grammar and dictionary
          settings, especially if your target language did not have a template.
        </p>
      </template>
      <Button
        class="float-end mt-2"
        :disabled="!selectedLang && stepIndex === 2"
        @click="
          if (stepIndex < 5) {
            stepIndex++;
          } else {
            finishWizard();
          }
        "
        >{{ nextName }}</Button
      >
    </div>
  </div>
</template>
