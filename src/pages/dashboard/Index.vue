<script setup lang="ts">
import Button from "@/components/ui/button/Button.vue";
import {
  Card,
  CardContent,
  CardDescription,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import { BarChart } from "@/components/ui/chart-bar";
import { DonutChart } from "@/components/ui/chart-donut";
import { GithubLogoIcon } from "@radix-icons/vue";

import { invoke } from "@tauri-apps/api";
import { BookText, ClipboardPaste, File, Link, Pencil } from "lucide-vue-next";
import { ref, watch } from "vue";

interface TimeSpent {
  days_this_week: { name: string; duration: number }[];
  total_this_week: [string, string];
  this_month: [string, string];
  this_year: [string, string];
  total: [string, string];
  streak: number;
}

type WordLevel = { name: string; amount: number };
type WordsAdded = [number, number, number, number];

const props = defineProps<{ currrentLanguage: string }>();

const timeSpent = ref<TimeSpent | undefined>(undefined);
const wordsLevels = ref<WordLevel[] | undefined>(undefined);
const wordsExportedByTime = ref<WordsAdded | undefined>(undefined);

watch(
  () => props.currrentLanguage,
  async () => {
    timeSpent.value = await invoke<TimeSpent>("time_spent");
    wordsLevels.value = await invoke<WordLevel[]>("get_words_known_at_levels");
    wordsExportedByTime.value = await invoke<WordsAdded>("get_words_added");
  },
  { immediate: true },
);
</script>

<template>
  <div class="flex items-center justify-center p-3">
    <div class="flex gap-3">
      <Card class="w-80 h-[45rem]">
        <CardHeader>
          <CardTitle>Activity</CardTitle>
          <CardDescription>Time spent learning the language</CardDescription>
        </CardHeader>
        <CardContent>
          <div class="flex items-center justify-center">
            <div
              class="p-12 w-48 h-48 bg-accent mb-3 rounded-full flex items-center justify-center flex-col ease-in-out duration-200 hover:scale-110"
            >
              <h1 class="text-center text-5xl font-black text-teal-600">
                {{ timeSpent?.streak }}
              </h1>
              <h3 class="text-center text-xl">day streak</h3>
            </div>
          </div>

          <h4 class="text-xl font-bold mb-1 mt-3">This week</h4>

          <BarChart
            :data="timeSpent?.days_this_week ?? []"
            :categories="['duration']"
            index="name"
            :showLegend="false"
            class="h-48"
            :showYAxis="true"
          />

          <div class="flex items-baseline justify-center mb-4">
            <h2 class="font-bold">
              {{ timeSpent?.total_this_week[0] }}
            </h2>
            <h4>{{ timeSpent?.total_this_week[1] }}</h4>
            <h4>&nbsp;spent learning total</h4>
          </div>

          <div class="bg-accent rounded-md">
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>This month</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent?.this_month[0] }}
                </h2>
                <h4>{{ timeSpent?.this_month[1] }}</h4>
              </div>
            </div>
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>This year</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent?.this_year[0] }}
                </h2>
                <h4>{{ timeSpent?.this_year[1] }}</h4>
              </div>
            </div>
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>All time</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent?.total[0] }}
                </h2>
                <h4>{{ timeSpent?.total[1] }}</h4>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
      <div class="flex flex-col gap-3 h-[45rem]">
        <Card class="w-80 h-fit">
          <CardHeader>
            <CardTitle>Getting Started</CardTitle>
            <CardDescription>Important links</CardDescription>
          </CardHeader>
          <CardContent>
            <div class="flex flex-col items-center justify-center gap-1">
              <a target="_blank" href="https://github.com/BrewingWeasel/kalba">
                <Button class="w-48" variant="outline">
                  <GithubLogoIcon />
                  <h2 class="ml-1">Repository</h2>
                </Button>
              </a>
              <a target="_blank" href="https://kalba.readthedocs.io/en/latest/">
                <Button class="w-48" variant="outline">
                  <BookText />
                  <h2 class="ml-1">Docs</h2>
                </Button>
              </a>
            </div>
          </CardContent>
        </Card>
        <Card class="w-80 h-fit">
          <CardHeader>
            <CardTitle>Words Known</CardTitle>
            <CardDescription
              >Percentages of words at different ratings</CardDescription
            >
          </CardHeader>
          <CardContent>
            <DonutChart
              index="name"
              category="amount"
              :data="wordsLevels ?? []"
              :showTooltip="true"
              :show-legend="true"
              :colors="['#ef5350', '#ffa726', '#ffd54f', 'black']"
            />
          </CardContent>
        </Card>
        <Card class="w-80 h-full">
          <CardHeader>
            <CardTitle>Profile</CardTitle>
            <CardDescription>Switch profiles</CardDescription>
          </CardHeader>
          <CardContent>
            <Button @click="$emit('setProfile')">Set profile</Button>
          </CardContent>
        </Card>
      </div>
      <div class="flex flex-col gap-3 h-[45rem]">
        <Card class="w-80 h-fit">
          <CardHeader>
            <CardTitle>Exported Words</CardTitle>
            <CardDescription>Words added to Anki</CardDescription>
          </CardHeader>
          <CardContent>
            <div class="bg-accent rounded-md" v-if="wordsExportedByTime">
              <div
                class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
              >
                <h4>This week</h4>
                <div class="flex items-baseline">
                  <h2 class="text-xl font-bold px-0.5">
                    {{ wordsExportedByTime[0] }}
                  </h2>
                </div>
              </div>
              <div
                class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
              >
                <h4>This month</h4>
                <div class="flex items-baseline">
                  <h2 class="text-xl font-bold px-0.5">
                    {{ wordsExportedByTime[1] }}
                  </h2>
                </div>
              </div>
              <div
                class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
              >
                <h4>This year</h4>
                <div class="flex items-baseline">
                  <h2 class="text-xl font-bold px-0.5">
                    {{ wordsExportedByTime[2] }}
                  </h2>
                </div>
              </div>
              <div
                class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
              >
                <h4>All time</h4>
                <div class="flex items-baseline">
                  <h2 class="text-xl font-bold px-0.5">
                    {{ wordsExportedByTime[3] }}
                  </h2>
                </div>
              </div>
            </div>
          </CardContent>
        </Card>
        <Card class="w-80 h-full">
          <CardHeader>
            <CardTitle>Reader</CardTitle>
            <CardDescription>Suggested content</CardDescription>
          </CardHeader>
          <CardContent>
            <h3 class="font-semibold text-lg">Builtin inputs</h3>
            <div class="p-2 border-2 rounded-md flex justify-around">
              <Button variant="secondary" as-child>
                <a href="/reader/input"><File :size="18" /></a>
              </Button>
              <Button variant="secondary" as-child>
                <a href="/reader/input"><Link :size="18" /></a>
              </Button>
              <Button variant="secondary" as-child>
                <a href="/reader/input"><ClipboardPaste :size="18" /></a>
              </Button>
              <Button variant="secondary" as-child>
                <a href="/reader/input"><Pencil :size="18" /></a>
              </Button>
            </div>
            <h3 class="font-semibold text-lg mt-2">Suggested sites</h3>
            <div class="p-2 border-2 rounded-md flex flex-col justify-around">
              <Button variant="link" as-child>
                <a href="https://www.wikibooks.org/" target="_blank"
                  >Wikibooks</a
                >
              </Button>
              <Button variant="link" as-child>
                <a href="https://openlibrary.org/languages" target="_blank"
                  >Open Library</a
                >
              </Button>
              <Button variant="link" as-child>
                <a
                  href="https://www.gutenberg.org/browse/languages/en"
                  target="_blank"
                  >Project Gutenburg</a
                >
              </Button>
              <Button variant="link" as-child>
                <a
                  href="https://meta.wikimedia.org/wiki/List_of_Wikipedias"
                  target="_blank"
                  >Wikipedia</a
                >
              </Button>
            </div>
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>
