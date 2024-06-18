<script setup lang="ts">
import { Switch } from "@/components/ui/switch";
import Heading from "@/components/Heading.vue";

import WordKnowledge from "@/components/settings/WordKnowledge.vue";
import Dictionaries from "@/components/settings/Dictionaries.vue";
import FilePicker from "@/components/FilePicker.vue";
import Exporting from "@/components/settings/Exporting.vue";
import SettingsMenu from "@/components/settings/SettingsMenu.vue";
import { SettingsSection } from "@/components/settings/SettingsMenu.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, ref } from "vue";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";
import { Settings } from "@/types";
import { useDark } from "@vueuse/core";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ChevronDown } from "lucide-vue-next";
import { Button } from "@/components/ui/button";

const isDark = useDark();

const settings: Ref<Settings> = ref(await invoke("get_settings"));

const models: string[] = await invoke("get_all_note_names");
const deckNames: string[] = await invoke("get_all_deck_names");

const languagesOpen = ref(true);

const section: Ref<SettingsSection> = ref("Appearance");

async function saveSettings() {
  console.log("trying to write settings", settings.value);
  await invoke("write_settings", { settings: settings.value });
}
</script>

<template>
  <div class="flex px-6">
    <div class="pr-10 w-80">
      <SettingsMenu v-model="section" section="Appearance" />
      <Collapsible class="px-4" v-model:open="languagesOpen">
        <div class="flex justify-between items-center">
          <h4 class="font-semibold">Language Details</h4>
          <CollapsibleTrigger as-child>
            <Button variant="ghost" size="sm" class="p-0 w-9">
              <ChevronDown class="w-4 h-4" />
              <span class="sr-only">Toggle</span>
            </Button>
          </CollapsibleTrigger>
        </div>
        <CollapsibleContent class="pl-4">
          <SettingsMenu v-model="section" section="Exporting" />
          <SettingsMenu v-model="section" section="Word Knowledge" />
          <SettingsMenu v-model="section" section="Dictionaries" />
          <SettingsMenu v-model="section" section="Grammar" />
        </CollapsibleContent>
      </Collapsible>
    </div>
    <div>
      <template v-if="section == 'Appearance'">
        <Heading
          title_id="appearance"
          title="Appearance"
          description="Configure how Sakinyje looks"
        />
        <Switch id="theme" v-model:checked="isDark" />
        <Label for="theme">Use dark mode</Label>
      </template>

      <template v-else-if="section == 'Exporting'">
        <Heading
          title_id="exporting"
          title="Exporting"
          description="Configure the default settings for exporting sentences"
        />
        <Exporting
          :deckNames
          :models
          v-model:deck="settings.deck"
          v-model:model="settings.note_type"
          v-model:fields="settings.note_fields"
        />
      </template>

      <template v-else-if="section == 'Word Knowledge'">
        <Heading
          title_id="wordknowledge"
          title="Word Knowledge"
          description="Automatically synchronize the words you know with Anki"
        />
        <Suspense>
          <WordKnowledge :decks="settings.anki_parser" :models :deckNames />
        </Suspense>
      </template>

      <template v-else-if="section == 'Dictionaries'">
        <Heading
          title_id="dictionaries"
          title="Dictionaries"
          description="Configure dictionaries to use for word lookup"
        />
        <Dictionaries v-model="settings.dicts" />
      </template>

      <template v-else-if="section == 'Grammar'">
        <Heading
          title_id="grammar"
          title="Grammar"
          description="Configure the automatic parsing of grammar"
        />
        <Label for="model">SpaCy model</Label>
        <Input id="model" v-model="settings.model" />
        <Label for="frequencylist">Frequency list</Label>
        <FilePicker v-model="settings.frequency_list" />
        <Label for="freq">Number of words known</Label>
        <Input id="freq" type="number" v-model="settings.words_known_by_freq" />
      </template>

      <br />

      <Button class="mt-2" variant="destructive" @click="saveSettings"
        >Save</Button
      >
    </div>
  </div>
</template>
