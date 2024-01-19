<script setup lang="ts">
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from '@/components/ui/hover-card'
const props = defineProps(['word'])

const rating = `rating-${props.word.rating}`

function set_selected() {
  console.log('set_selected')
  this.$emit('selected', props.word)
}
</script>

<template @click.native="set_selected">
  <span v-if="props.word.clickable">&nbsp;</span>
  <HoverCard :class="rating">
    <HoverCardTrigger>{{ props.word.text }}</HoverCardTrigger>
    <HoverCardContent>
      {{ props.word.lemma }}
      {{ props.word.morph }}
    </HoverCardContent>
  </HoverCard>
</template>
