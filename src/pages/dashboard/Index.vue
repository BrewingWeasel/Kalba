<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

import { BarChart } from "@/components/ui/chart-bar";

import { invoke } from "@tauri-apps/api";

interface TimeSpent {
  days_this_week: { name: string; duration: number }[];
  total_this_week: [string, string];
  this_month: [string, string];
  this_year: [string, string];
  total: [string, string];
}

const timeSpent = await invoke<TimeSpent>("time_spent");
console.log(timeSpent);
</script>

<template>
  <Card class="w-80">
    <CardHeader>
      <CardTitle>Activity</CardTitle>
      <!-- <CardDescription></CardDescription> -->
    </CardHeader>
    <CardContent>
      <BarChart
        :data="timeSpent.days_this_week"
        :categories="['duration']"
        index="name"
        :showLegend="false"
        class="h-48"
        :showYAxis="true"
      />

      <div
        class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-accent"
      >
        <h4>This Week</h4>
        <div class="flex items-baseline">
          <h2 class="text-xl font-bold px-0.5">
            {{ timeSpent.total_this_week[0] }}
          </h2>
          <h4>{{ timeSpent.total_this_week[1] }}</h4>
        </div>
      </div>
      <div
        class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-accent"
      >
        <h4>This Month</h4>
        <div class="flex items-baseline">
          <h2 class="text-xl font-bold px-0.5">
            {{ timeSpent.this_month[0] }}
          </h2>
          <h4>{{ timeSpent.this_month[1] }}</h4>
        </div>
      </div>
      <div
        class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-accent"
      >
        <h4>This Year</h4>
        <div class="flex items-baseline">
          <h2 class="text-xl font-bold px-0.5">
            {{ timeSpent.this_year[0] }}
          </h2>
          <h4>{{ timeSpent.this_year[1] }}</h4>
        </div>
      </div>
      <div
        class="flex justify-between items-baseline px-2 py-1 rounded-md hover:bg-accent"
      >
        <h4>All time</h4>
        <div class="flex items-baseline">
          <h2 class="text-xl font-bold px-0.5">
            {{ timeSpent.total[0] }}
          </h2>
          <h4>{{ timeSpent.total[1] }}</h4>
        </div>
      </div>
    </CardContent>
  </Card>
</template>
