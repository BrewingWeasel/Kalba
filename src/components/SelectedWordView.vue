<script setup lang="ts">
import { defineAsyncComponent } from 'vue'
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
import RatingButtons from "@/components/RatingButtons.vue"

const props = defineProps(['word'])


const DefinitionComp = defineAsyncComponent(() =>
  import('@/components/DefinitionView.vue')
)

</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-center">{{ props.word.lemma }}</CardTitle>
      <CardDescription class="text-center"><i>{{ props.word.text }}</i></CardDescription>
    </CardHeader>
    <CardContent>
      <RatingButtons @change-rating="(r) => { $emit('change-rating', r); console.log('lol') }" />
      <Suspense>
        <DefinitionComp :lemma="word.lemma" />

        <template #fallback>
          Loading...
        </template>
      </Suspense>
    </CardContent>
    <CardFooter>
      {{ props.word.morph }}
    </CardFooter>
  </Card>
</template>
