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
import { BookText } from "lucide-vue-next";

interface TimeSpent {
  days_this_week: { name: string; duration: number }[];
  total_this_week: [string, string];
  this_month: [string, string];
  this_year: [string, string];
  total: [string, string];
  streak: number;
}

const timeSpent = await invoke<TimeSpent>("time_spent");
console.log(timeSpent);

const wordsLevels = await invoke<{ name: string; amount: number }[]>(
  "get_words_known_at_levels",
);
console.log(wordsLevels);
</script>

<template>
  <div class="flex items-center justify-center p-3">
    <div class="flex gap-3">
      <Card class="w-80">
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
                {{ timeSpent.streak }}
              </h1>
              <h3 class="text-center text-xl">day streak</h3>
            </div>
          </div>

          <h4 class="text-xl font-bold mb-1 mt-3">This week</h4>

          <BarChart
            :data="timeSpent.days_this_week"
            :categories="['duration']"
            index="name"
            :showLegend="false"
            class="h-48"
            :showYAxis="true"
          />

          <div class="flex items-baseline justify-center mb-4">
            <h2 class="font-bold">
              {{ timeSpent.total_this_week[0] }}
            </h2>
            <h4>{{ timeSpent.total_this_week[1] }}</h4>
            <h4>&nbsp;spent learning total</h4>
          </div>

          <div class="bg-accent rounded-md">
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>This month</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent.this_month[0] }}
                </h2>
                <h4>{{ timeSpent.this_month[1] }}</h4>
              </div>
            </div>
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>This year</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent.this_year[0] }}
                </h2>
                <h4>{{ timeSpent.this_year[1] }}</h4>
              </div>
            </div>
            <div
              class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-background"
            >
              <h4>All time</h4>
              <div class="flex items-baseline">
                <h2 class="text-xl font-bold px-0.5">
                  {{ timeSpent.total[0] }}
                </h2>
                <h4>{{ timeSpent.total[1] }}</h4>
              </div>
            </div>
          </div>
        </CardContent>
      </Card>
      <div class="flex flex-col gap-3">
        <Card class="w-80 h-fit">
          <CardHeader>
            <CardTitle>Getting Started</CardTitle>
            <CardDescription>Important links</CardDescription>
          </CardHeader>
          <CardContent>
            <div class="flex flex-col items-center justify-center gap-1">
              <a
                target="_blank"
                href="https://github.com/BrewingWeasel/sakinyje"
              >
                <Button class="w-48" variant="outline">
                  <GithubLogoIcon />
                  <h2 class="ml-1">Repository</h2>
                </Button>
              </a>
              <a
                target="_blank"
                href="https://github.com/BrewingWeasel/sakinyje/wiki"
              >
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
              :data="wordsLevels"
              :showTooltip="true"
              :show-legend="true"
              :colors="['#ef5350', '#ffa726', '#ffd54f', 'black']"
            />
          </CardContent>
        </Card>
      </div>
    </div>
  </div>
</template>
