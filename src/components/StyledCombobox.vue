<script setup lang="ts">
import { ComputedRef, computed, ref } from "vue";
import { CaretSortIcon, CheckIcon } from "@radix-icons/vue";

import { cn } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import {
  Command,
  CommandEmpty,
  CommandGroup,
  CommandInput,
  CommandItem,
  CommandList,
} from "@/components/ui/command";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";

const open = ref(false);

const props = defineProps<{
  itemBeingSelected: string;
  options: string[] | [string, string][];
}>();

const selected = defineModel();

const selections: ComputedRef<{ value: string; label: string }[]> = computed(
  () =>
    props.options.map((option) => {
      if (typeof option == "string") {
        return {
          value: option,
          label: option,
        };
      } else {
        return {
          value: option[0],
          label: option[1],
        };
      }
    }),
);

const searchPrompt = "Search " + props.itemBeingSelected + "...";
const selectPrompt = "Select " + props.itemBeingSelected + "...";
</script>

<template>
  <Popover v-model:open="open">
    <PopoverTrigger as-child>
      <Button
        variant="outline"
        role="combobox"
        :aria-expanded="open"
        class="justify-between w-[350px]"
      >
        {{
          selected
            ? selections.find((opt) => opt.value === selected)?.label
            : selectPrompt
        }}
        <CaretSortIcon class="ml-2 w-4 h-4 opacity-50 shrink-0" />
      </Button>
    </PopoverTrigger>
    <PopoverContent class="p-0 w-[350px]">
      <Command>
        <CommandInput class="h-9" :placeholder="searchPrompt" />
        <CommandEmpty>No selection found.</CommandEmpty>
        <CommandList>
          <CommandGroup>
            <CommandItem
              v-for="selection in selections"
              :key="selection.value"
              :value="selection.value"
              @select="
                (ev) => {
                  if (typeof ev.detail.value === 'string') {
                    selected = ev.detail.value;
                  }
                  open = false;
                }
              "
            >
              {{ selection.label }}
              <CheckIcon
                :class="
                  cn(
                    'ml-auto h-4 w-4',
                    selected === selection.value ? 'opacity-100' : 'opacity-0',
                  )
                "
              />
            </CommandItem>
          </CommandGroup>
        </CommandList>
      </Command>
    </PopoverContent>
  </Popover>
</template>
