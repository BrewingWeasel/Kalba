<script setup lang="ts">
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from '@/components/ui/hover-card'
const props = defineProps(['word'])
const emit = defineEmits(['selected'])

const rating = `rating-${props.word.rating}`

function set_selected() {
  console.log('set_selected')
  emit('selected', props.word)
}
</script>

<template>
  <span v-if="props.word.clickable">&nbsp;</span>
  <!-- <span @click="set_selected">{{ props.word.text }}</span> -->
  <div class="word" @click="set_selected" style="display: inline">
    <HoverCard :class="rating">
      <HoverCardTrigger>{{ props.word.text }}</HoverCardTrigger>
      <HoverCardContent>
        {{ props.word.lemma }}
        {{ props.word.morph }}
      </HoverCardContent>
    </HoverCard>
  </div>
</template>
