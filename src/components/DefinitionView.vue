<script setup lang="ts">
import { Definition } from "@/types";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
import { ChevronDown, Loader2 } from "lucide-vue-next";

const props = defineProps<{ definitions: Definition[]; lemma: string }>();

const onDemandDefinitions = ref(new Map<string, undefined | string>());
const collapsiblesOpen = ref<{ [key: string]: boolean }>({});

for (const def of props.definitions) {
  if (def.t === "OnDemand" && def.c) {
    onDemandDefinitions.value.set(def.c, undefined);
    collapsiblesOpen.value[def.c] = false;
  }
}

async function getOnDemandDef(dictionary: string) {
  if (onDemandDefinitions.value.get(dictionary)) {
    return;
  }
  console.log(`getting on demand definiton from dictionary ${dictionary}`);
  onDemandDefinitions.value.set(
    dictionary,
    await invoke<Definition>("get_definition_on_demand", {
      dictionary,
      lemma: props.lemma,
    }).then((def) => def.c!),
  );
}
</script>

<template>
  <div v-for="def in props.definitions">
    <span v-if="def.t == 'Text'" v-html="def.c"></span>
    <div
      v-else-if="def.t == 'OnDemand' && def.c"
      class="p-1 my-1 bg-background rounded-md"
    >
      <Collapsible v-model:open="collapsiblesOpen[def.c]">
        <CollapsibleTrigger as-child
          ><div class="flex justify-between items-center">
            <h1 @click="getOnDemandDef(def.c)">{{ def.c }}</h1>
            <ChevronDown class="w-4 h-4" />
          </div>
        </CollapsibleTrigger>
        <CollapsibleContent>
          <span
            v-if="onDemandDefinitions.get(def.c)"
            v-html="onDemandDefinitions.get(def.c)"
          ></span>
          <Loader2 v-else class="animate-spin" />
        </CollapsibleContent>
      </Collapsible>
    </div>
  </div>
</template>
