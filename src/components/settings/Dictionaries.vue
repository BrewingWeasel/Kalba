<script setup lang="ts">
import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";

import { Button } from "@/components/ui/button";
import IndividualDict from "@/components/settings/IndividualDict.vue";
import { Dictionary, DictionaryType } from "@/types";

const dicts = defineModel({ type: Array<Dictionary>, required: true });

function addDictionary() {
  dicts.value.push({
    t: DictionaryType.File,
    c: [
      "",
      {
        t: "StarDict",
        c: null,
      },
    ],
  });
}
</script>

<template>
  <Accordion type="single" collapsible>
    <Button @click="addDictionary">Add dictionary</Button>
    <AccordionItem v-for="(_dict, index) in dicts" :value="index.toString()">
      <AccordionTrigger>
        {{ index }}
      </AccordionTrigger>
      <AccordionContent>
        <IndividualDict v-model="dicts[index]" />
      </AccordionContent>
    </AccordionItem>
  </Accordion>
</template>
