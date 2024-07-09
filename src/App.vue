<script setup lang="ts">
import {
  NavigationMenu,
  NavigationMenuItem,
  NavigationMenuLink,
  NavigationMenuList,
  navigationMenuTriggerStyle,
} from "@/components/ui/navigation-menu";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import { Label } from "@/components/ui/label";
import {
  Popover,
  PopoverContent,
  PopoverTrigger,
} from "@/components/ui/popover";
import { Settings2 } from "lucide-vue-next";
import { Ref, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Toaster } from "@/components/ui/sonner";
import { listen } from "@tauri-apps/api/event";
import { toast } from "vue-sonner";

const languages = ref<string[]>(await invoke("get_languages"));
const currentLanguage = ref<string | null>(await invoke("get_language"));
console.log(currentLanguage.value);

async function updateLanguages() {
  languages.value = await invoke("get_languages");
}

async function setLanguage(language: string) {
  console.log(`setting language to ${language}`);
  await invoke("set_language", { language });
}

const toasters: Ref<Map<string, number | null>> = ref(
  new Map([
    ["stanza_loading", null],
    ["refresh_anki", null],
  ]),
);
for (const toasterEvent of toasters.value.keys()) {
  console.log(toasterEvent);
  listen<{ message: string | null }>(toasterEvent, (event) => {
    console.log(event);
    if (event.payload.message) {
      const startedToaster = toast.info(event.payload.message, {
        duration: 0,
      });
      if (typeof startedToaster == "number") {
        toasters.value.set(toasterEvent, startedToaster);
      }
    } else {
      const toasterId = toasters.value.get(toasterEvent);
      if (toasterId) {
        Toaster.close(toasterId);
      }
    }
  });
}
</script>

<template>
  <Toaster closeButton richColors />
  <div class="flex w-full">
    <NavigationMenu>
      <NavigationMenuList>
        <NavigationMenuItem>
          <NavigationMenuLink href="/" :class="navigationMenuTriggerStyle()">
            Reader
          </NavigationMenuLink>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <NavigationMenuLink
            href="/words"
            :class="navigationMenuTriggerStyle()"
          >
            Dictionary
          </NavigationMenuLink>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <NavigationMenuLink
            href="/corpus"
            :class="navigationMenuTriggerStyle()"
          >
            Corpus
          </NavigationMenuLink>
        </NavigationMenuItem>
        <NavigationMenuItem>
          <NavigationMenuLink
            href="/settings"
            :class="navigationMenuTriggerStyle()"
          >
            Settings
          </NavigationMenuLink>
        </NavigationMenuItem>
      </NavigationMenuList>
    </NavigationMenu>
    <div class="pt-1 pr-8 ml-auto">
      <Popover>
        <PopoverTrigger>
          <div class="flex items-center px-2 rounded-md bg-accent">
            <Settings2 :size="16" class="mr-1" />
            {{ currentLanguage ?? "Select a language" }}
          </div>
        </PopoverTrigger>
        <PopoverContent class="w-60">
          <h4 class="pb-2 font-medium leading-none">Quick Settings</h4>
          <Label for="current-language">Current language:</Label>
          <Select
            id="current-language"
            :modelValue="currentLanguage ?? undefined"
            @update:model-value="
              ($event) => {
                setLanguage($event.value);
                currentLanguage = $event.value;
              }
            "
          >
            <SelectTrigger class="w-[180px]">
              <SelectValue placeholder="Select a language" />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                <SelectItem v-for="language in languages" :value="language">
                  {{ language }}
                </SelectItem>
              </SelectGroup>
            </SelectContent>
          </Select>
        </PopoverContent>
      </Popover>
    </div>
  </div>
  <Suspense>
    <router-view
      :currentLanguage="currentLanguage"
      @settingsChanged="updateLanguages"
    ></router-view>
  </Suspense>
</template>
