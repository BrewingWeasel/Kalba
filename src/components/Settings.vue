<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

import { Switch } from "@/components/ui/switch";

import WordKnowledge from "@/components/settings/WordKnowledge.vue";
import Exporting from "@/components/settings/Exporting.vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Ref, ref, watch } from "vue";
import { Deck } from "./settings/Deck.vue";
import { Button } from "@/components/ui/button";
import { Label } from "@/components/ui/label";
import { Input } from "@/components/ui/input";

interface Settings {
  deck: string;
  note_type: string;
  note_fields: { [key: string]: string };
  model: string;
  anki_parser: Deck[];
  dark_mode: boolean;
}

const settings: Ref<Settings> = ref(await invoke("get_settings"));

const models: string[] = await invoke("get_all_note_names", {});
const deckNames: string[] = await invoke("get_all_deck_names", {});

async function saveSettings() {
  await invoke("write_settings", { settings: settings.value });
}

watch(
  () => settings.value.dark_mode,
  async (dark_mode) => {
    if (dark_mode) {
      document.documentElement.classList.add("dark");
    } else {
      document.documentElement.classList.remove("dark");
    }
  },
);
</script>

<template>
  <div class="w-full flex justify-center">
    <Tabs default-value="knowledge" class="object-center">
      <TabsList class="grid w-full grid-cols-6">
        <TabsTrigger value="appearance"> Appearance </TabsTrigger>
        <TabsTrigger value="exporting"> Exporting </TabsTrigger>
        <TabsTrigger value="knowledge"> Word Knowledge </TabsTrigger>
        <TabsTrigger value="dictionaries"> Dictionaries </TabsTrigger>
        <TabsTrigger value="grammar"> Grammar </TabsTrigger>
        <TabsTrigger value="advanced"> Advanced </TabsTrigger>
      </TabsList>
      <TabsContent value="appearance">
        <Card>
          <CardHeader>
            <CardTitle>Appearance</CardTitle>
            <CardDescription> Configure how Sakinyje looks </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2">
            <div class="space-y-1">
              <Label for="theme">Use dark mode</Label>
              <Switch id="theme" v-model:checked="settings.dark_mode" />
            </div>
          </CardContent>
          <CardFooter>
            <Button variant="destructive" @click="saveSettings">Save</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="exporting">
        <Card>
          <CardHeader>
            <CardTitle>Exporting</CardTitle>
            <CardDescription>
              Configure the default settings for exporting sentences
            </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2">
            <div class="py-2">
              <Exporting
                :deckNames
                :models
                :deck="settings.deck"
                :model="settings.note_type"
                :fields="settings.note_fields"
                @set-deck="(deck) => (settings.deck = deck)"
                @set-model="(model) => (settings.note_type = model)"
              />
            </div>
          </CardContent>
        </Card>
      </TabsContent>
      <TabsContent value="knowledge">
        <Card>
          <CardHeader>
            <CardTitle>Word Knowledge</CardTitle>
            <CardDescription>
              Automatically synchronize the words you know with Anki
            </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2">
            <Suspense>
              <WordKnowledge :decks="settings.anki_parser" :models :deckNames />
            </Suspense>
          </CardContent>
          <CardFooter>
            <Button variant="destructive" @click="saveSettings">Save</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="dictionaries">
        <Card>
          <CardHeader>
            <CardTitle>Dictionaries</CardTitle>
            <CardDescription>
              Configure dictionaries to use for word lookup
            </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2"> </CardContent>
        </Card>
      </TabsContent>
      <TabsContent value="grammar">
        <Card>
          <CardHeader>
            <CardTitle>Grammar</CardTitle>
            <CardDescription>
              Configure the automatic parsing of grammar
            </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2">
            <Label for="model">SpaCy model</Label>
            <Input id="model" v-model="settings.model" />
          </CardContent>
          <CardFooter>
            <Button variant="destructive" @click="saveSettings">Save</Button>
          </CardFooter>
        </Card>
      </TabsContent>
      <TabsContent value="advanced">
        <Card>
          <CardHeader>
            <CardTitle>Advanced</CardTitle>
            <CardDescription> Other advanced settings </CardDescription>
          </CardHeader>
          <CardContent class="space-y-2"> </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
  </div>
</template>
