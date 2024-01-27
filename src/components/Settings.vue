<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  // CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";

import { Switch } from "@/components/ui/switch";

import WordKnowledge from "@/components/settings/WordKnowledge.vue";

function toggleDarkMode() {
  const body = document.querySelector("body");
  body!.classList.toggle("dark");
}
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
              <Switch id="theme" @click="toggleDarkMode" />
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
              <WordKnowledge />
            </Suspense>
          </CardContent>
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
          <CardContent class="space-y-2"> </CardContent>
        </Card>
      </TabsContent>
    </Tabs>
  </div>
</template>
