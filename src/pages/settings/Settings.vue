<script setup lang="ts">
import { Switch } from "@/components/ui/switch";
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from "@/components/ui/hover-card";
import Heading from "@/components/Heading.vue";
import FilePicker from "@/components/FilePicker.vue";

import WordKnowledge from "./components/WordKnowledge.vue";
import Dictionaries from "./components/Dictionaries.vue";
import Exporting from "@/components/ExportingConfiguration.vue";
import SettingsMenu from "./components/SettingsMenu.vue";
import type { SettingsSection } from "./components/SettingsMenu.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { type Ref, ref, watch, reactive, nextTick } from "vue";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import type { Settings } from "@/types";
import { useDark } from "@vueuse/core";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ChevronDown, Info, X } from "lucide-vue-next";
import { Button } from "@/components/ui/button";
import NewLanguage from "@/components/generated/NewLanguage.vue";
import { toast } from "vue-sonner";
import Grammar from "./components/Grammar.vue";
import { useRouter } from "vue-router";
import SiteConfigurationTable from "./components/SiteConfigurationTable.vue";
import { Alert, AlertDescription, AlertTitle } from "@/components/ui/alert";

import {
  NumberField,
  NumberFieldContent,
  NumberFieldDecrement,
  NumberFieldIncrement,
  NumberFieldInput,
} from "@/components/ui/number-field";
import EnableStanza from "@/components/EnableStanza.vue";

const isDark = useDark();
const router = useRouter();

const installed = await invoke<boolean>("check_stanza_installed");
const settings = reactive<Settings>(await invoke("get_settings"));

const models = await invoke<string[]>("get_all_note_names").catch((error) => {
  toast.error(error);
  return [];
});
const deckNames = await invoke<string[]>("get_all_deck_names").catch(
  (error) => {
    toast.error(error);
    return [];
  },
);

const props = defineProps<{
  currentLanguage: string | null;
}>();

const languagesOpen: Ref<{ [key: string]: boolean }> = ref({});
for (const language in settings.languages) {
  if (props.currentLanguage === language) {
    languagesOpen.value[language] = true;
  } else {
    languagesOpen.value[language] = false;
  }
}

const allLanguageMenuOpen = ref(true);
const section: Ref<SettingsSection> = ref("Appearance");
const selectedLang: Ref<string | null> = ref(null);

watch(
  () => [selectedLang.value, section.value],
  ([newLang, newSection]) => {
    if (newLang !== null) {
      router.replace(`/settings/${newLang}/${newSection}`);
    } else {
      router.replace(`/settings/${newSection}`);
    }
  },
);

const emit = defineEmits(["settingsChanged"]);

console.log(settings);

async function saveSettings() {
  emit("settingsChanged");
  await invoke("write_settings", { settings: settings }).catch(async (_) => {
    // Errors in settings could potentially occur before the contents of a dictionary are updated,
    // to handle this we simply try writing the settings again one tick later
    await nextTick();
    invoke("write_settings", { settings: settings }).catch(async (error) => {
      toast.error(error);
    });
  });
}
watch(settings, saveSettings, { deep: true });

async function newLanguage(language: string) {
  await invoke("new_language_from_template", { language });
  const updated: Settings = await invoke("get_settings");
  settings.languages = updated.languages;
}
</script>

<template>
  <div class="flex px-6">
    <div class="pr-10 w-1/3">
      <SettingsMenu
        v-model="section"
        section="Appearance"
        :rightLanguage="true"
      />
      <SettingsMenu v-model="section" section="Input" :rightLanguage="true" />
      <SettingsMenu v-model="section" section="Stanza" :rightLanguage="true" />
      <Collapsible class="px-4" v-model:open="allLanguageMenuOpen">
        <div class="flex justify-between items-center">
          <h4 class="font-semibold">Languages</h4>
          <CollapsibleTrigger as-child>
            <Button variant="ghost" size="sm" class="p-0 w-9">
              <ChevronDown class="w-4 h-4" />
              <span class="sr-only">Toggle</span>
            </Button>
          </CollapsibleTrigger>
        </div>
        <CollapsibleContent class="pl-4">
          <Collapsible
            v-for="(_language_settings, language) in settings.languages"
            class="px-4"
            v-model:open="languagesOpen[language]"
          >
            <div class="flex justify-between items-center">
              <h4 class="font-semibold grow">
                {{ language }}
              </h4>
              <Button
                variant="ghost"
                size="sm"
                class="p-0 w-9"
                @click="delete settings.languages[language]"
              >
                <X class="w-4 h-4" />
                <span class="sr-only">Remove</span>
              </Button>
              <CollapsibleTrigger as-child>
                <Button variant="ghost" size="sm" class="p-0 w-9">
                  <ChevronDown class="w-4 h-4" />
                  <span class="sr-only">Toggle</span>
                </Button>
              </CollapsibleTrigger>
            </div>
            <CollapsibleContent
              class="pl-4"
              v-if="typeof language === 'string'"
              @click="
                () => {
                  selectedLang = language;
                }
              "
            >
              <SettingsMenu
                v-model="section"
                :rightLanguage="selectedLang === language"
                section="Exporting"
              />
              <SettingsMenu
                v-model="section"
                :rightLanguage="selectedLang === language"
                section="Word Knowledge"
              />
              <SettingsMenu
                v-model="section"
                :rightLanguage="selectedLang === language"
                section="Dictionaries"
              />
              <SettingsMenu
                v-model="section"
                :rightLanguage="selectedLang === language"
                section="Grammar"
              />
            </CollapsibleContent>
          </Collapsible>
          <div class="flex justify-center w-full">
            <NewLanguage @langSelected="newLanguage" />
          </div>
        </CollapsibleContent>
      </Collapsible>
    </div>
    <div class="w-full lg:pr-1/3">
      <template v-if="section == 'Appearance'">
        <Heading
          title_id="appearance"
          title="Appearance"
          description="Configure how Sakinyje looks"
        />
        <Switch id="theme" v-model:checked="isDark" />
        <Label for="theme">Use dark mode</Label>
      </template>

      <template v-if="section == 'Input'">
        <Heading
          title_id="input"
          title="Input"
          description="Configure how text and pages are processed"
        />
        <SiteConfigurationTable v-model="settings.site_configurations" />
      </template>

      <template v-if="section == 'Stanza'">
        <Heading
          title_id="stanza"
          title="Stanza"
          description="Configure and enable grammar parsing with Stanza"
        />
        <div v-if="installed">
          <Label for="stanza-enabled">Enable Stanza</Label>
          <Switch
            id="stanza-enabled"
            v-model:checked="settings.stanza_enabled"
          />
        </div>
        <div v-else>
          <EnableStanza
            v-model:installed="installed"
            v-model:enabled="settings.stanza_enabled"
          />
        </div>
        <Alert class="mt-4 w-fit">
          <Info class="h-4 w-4" />
          <AlertTitle>Stanza Usage</AlertTitle>
          <AlertDescription>
            Stanza can be used to automatically parse grammar and determine the
            root word for over 70 languages.</AlertDescription
          >
        </Alert>
      </template>

      <template v-else-if="section == 'Exporting' && selectedLang != null">
        <Heading
          title_id="exporting"
          title="Exporting"
          description="Configure the default settings for exporting sentences"
        />
        <Exporting
          :deckNames
          :models
          v-model:deck="settings.languages[selectedLang].deck"
          v-model:model="settings.languages[selectedLang].note_type"
          v-model:fields="settings.languages[selectedLang].note_fields"
        />
      </template>

      <template v-else-if="section == 'Word Knowledge' && selectedLang != null">
        <Heading
          title_id="wordknowledge"
          title="Word Knowledge"
          description="Automatically synchronize the words you know with Anki"
        />
        <HoverCard>
          <div class="flex items-center">
            <Label for="freq">Words known by frequency </Label>
            <HoverCardTrigger
              ><Info class="ml-1 mt-2" :size="16"
            /></HoverCardTrigger>
          </div>
          <HoverCardContent>
            <p>
              The most frequent words within this range will be marked as known
              automatically, referencing the frequency list provided.
            </p>
          </HoverCardContent>
        </HoverCard>
        <NumberField
          id="freq"
          v-model="settings.languages[selectedLang].words_known_by_freq"
          class="w-48"
        >
          <NumberFieldContent>
            <NumberFieldDecrement />
            <NumberFieldInput />
            <NumberFieldIncrement />
          </NumberFieldContent>
        </NumberField>
        <br />

        <Suspense>
          <WordKnowledge
            :decks="settings.languages[selectedLang].anki_parser"
            :models
            :deckNames
          />
        </Suspense>
      </template>

      <template v-else-if="section == 'Dictionaries' && selectedLang != null">
        <Heading
          title_id="dictionaries"
          title="Dictionaries"
          description="Configure dictionaries to use for word lookup"
        />
        <Dictionaries
          :current-language="selectedLang"
          v-model="settings.languages[selectedLang].dicts"
        />
      </template>

      <template v-else-if="section == 'Grammar' && selectedLang != null">
        <Heading
          title_id="grammar"
          title="Grammar"
          description="Configure the automatic parsing of grammar and other language details"
        />
        <template v-if="settings.stanza_enabled">
          <HoverCard>
            <div class="flex items-center">
              <Label for="model" class="pr-1">Stanza model</Label>
              <HoverCardTrigger
                ><Info class="mt-2" :size="16"
              /></HoverCardTrigger>
            </div>
            <HoverCardContent>
              <p>
                Stanza models are used to automatically determine the grammar of
                the language so that words with the same root are automatically
                considered the same
              </p>
            </HoverCardContent>
          </HoverCard>
          <Input id="model" v-model="settings.languages[selectedLang].model" />
        </template>
        <Label for="frequencylist">Frequency list</Label>
        <FilePicker v-model="settings.languages[selectedLang].frequency_list" />

        <br />
        <Grammar
          v-model:parser="settings.languages[selectedLang].grammar_parser"
          v-model:runOnLemmas="settings.languages[selectedLang].run_on_lemmas"
          v-model:suggestOnLemmas="
            settings.languages[selectedLang].suggest_on_lemmas
          "
        />
      </template>
      <br />
    </div>
  </div>
</template>
