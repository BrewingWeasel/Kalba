<script setup lang="ts">
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from '@/components/ui/card'
const props = defineProps(['word'])
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle class="text-center">{{ props.word.lemma }}</CardTitle>
      <CardDescription class="text-center"><i>{{ props.word.text }}</i></CardDescription>
    </CardHeader>
    <CardContent>
      Card Content
    </CardContent>
    <CardFooter>
      {{ props.word.morph }}
    </CardFooter>
  </Card>
</template>

<style>
.Card {
  width: 50%;
}
</style>
