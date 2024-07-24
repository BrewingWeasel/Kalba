<script setup lang="ts">
import { Definition } from "@/types";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { ref } from "vue";
import { ChevronDown, Loader2, PanelRightOpen } from "lucide-vue-next";

const emit = defineEmits<{
  (e: "getOnDemandDef", definition: string): void;
}>();

const separatedDefinitions = defineModel<string[]>("separatedDefinitions", {
  required: true,
});
const props = defineProps<{
  definitions: Definition[];
  lemma: string;
  onDemandDefinitions: Map<string, undefined | string>;
}>();

const collapsiblesOpen = ref<{ [key: string]: boolean }>({});

for (const def of props.definitions) {
  if (def.t === "OnDemand" && def.c) {
    collapsiblesOpen.value[def.c] = false;
  }
}
</script>

<template>
  <div v-for="def in props.definitions">
    <span v-if="def.t == 'Text'" v-html="def.c"></span>
    <div
      v-else-if="
        def.t == 'OnDemand' && def.c && !separatedDefinitions.includes(def.c)
      "
      class="p-1 my-1 bg-background rounded-md"
    >
      <Collapsible v-model:open="collapsiblesOpen[def.c]">
        <CollapsibleTrigger as-child
          ><div
            class="flex justify-between items-center"
            @click="$emit('getOnDemandDef', def.c)"
          >
            <h1>{{ def.c }}</h1>
            <div class="flex items-center gap-1">
              <PanelRightOpen
                @click="separatedDefinitions.push(def.c)"
                class="w-4 h-4"
              />
              <ChevronDown class="w-4 h-4" />
            </div>
          </div>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <span
            v-if="props.onDemandDefinitions.get(def.c)"
            v-html="props.onDemandDefinitions.get(def.c)"
          ></span>
          <Loader2 v-else class="animate-spin" />
        </CollapsibleContent>
      </Collapsible>
    </div>
  </div>
</template>
