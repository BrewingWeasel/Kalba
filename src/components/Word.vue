<script setup lang="ts">
import {
  HoverCard,
  HoverCardContent,
  HoverCardTrigger,
} from '@/components/ui/hover-card'
import { computed } from 'vue';
const props = defineProps(['word'])
const emit = defineEmits(['selected'])

const rating = computed(() => {
  if (props.word.rating == 0) {
    return 'text-rose-600'
  } else if (props.word.rating == 1) {
    return 'text-red-400'
  } else if (props.word.rating == 2) {
    return 'text-orange-400'
  } else if (props.word.rating == 3) {
    return 'text-amber-300'
  } else {
    return 'text-white'
  }
})

function set_selected() {
  console.log('set_selected')
  emit('selected', props.word)
}
</script>

<template>
  <span v-if="props.word.clickable">&nbsp;</span>
  <!-- <span @click="set_selected">{{ props.word.text }}</span> -->
  <div :class="rating" @click="set_selected" style="display: inline">
    <HoverCard>
      <HoverCardTrigger>{{ props.word.text }}</HoverCardTrigger>
      <HoverCardContent>
        {{ props.word.lemma }}
        {{ props.word.morph }}
      </HoverCardContent>
    </HoverCard>
  </div>
</template>
