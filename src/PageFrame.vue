<script setup lang="ts">
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

import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbSeparator,
} from "@/components/ui/breadcrumb";

import {
  BookOpenText,
  Home,
  LineChart,
  Search,
  Settings,
  Settings2,
} from "lucide-vue-next";
import { Ref, computed, onMounted, ref } from "vue";
import { invoke } from "@tauri-apps/api/tauri";
import { Toaster } from "@/components/ui/sonner";
import { listen } from "@tauri-apps/api/event";
import { toast } from "vue-sonner";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { Separator } from "@/components/ui/separator";

import { useRoute } from "vue-router";
import { StartupState } from "./types";

const languages = ref<string[]>(await invoke("get_languages"));
const currentLanguage = ref<string | null>(await invoke("get_language"));
console.log(currentLanguage.value);
console.log(languages.value);

const router = useRoute();
const pageNames = computed(() => {
  const path = router.path;
  const pathParts = path.split("/").filter((part) => part !== "");
  if (pathParts.length === 0) {
    return ["dashboard"];
  } else {
    return pathParts;
  }
});

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
    ["added_to_anki", null],
  ]),
);

const props = defineProps<{ startup: StartupState }>();

onMounted(async () => {
  console.log(props.startup.errors);
  for (const error of props.startup.errors) {
    console.log(error);
    if (error.includes("Anki")) {
      toast.error(error, { duration: 6000 });
    } else {
      toast.error(`${error} (state and settings will not be saved)`, {
        duration: 6000,
      });
    }
  }

  for (const toasterEvent of toasters.value.keys()) {
    listen<{ message: string | null }>(toasterEvent, (event) => {
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
});

const profileSetterOpen = ref(false);
</script>

<template>
  <div class="flex min-h-screen w-full flex-col bg-muted/40">
    <aside
      class="fixed inset-y-0 left-0 z-10 hidden w-14 flex-col border-r bg-background sm:flex"
    >
      <nav class="flex flex-col items-center gap-4 px-2 sm:py-5">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <a
                href="/"
                class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              >
                <Home class="h-5 w-5" />
                <span class="sr-only">Dashboard</span>
              </a>
            </TooltipTrigger>
            <TooltipContent side="right"> Dashboard </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <a
                href="/reader"
                class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              >
                <BookOpenText class="h-5 w-5" />
                <span class="sr-only">Reader</span>
              </a>
            </TooltipTrigger>
            <TooltipContent side="right"> Reader </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <a
                href="/browse"
                class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              >
                <Search class="h-5 w-5" />
                <span class="sr-only">Browse</span>
              </a>
            </TooltipTrigger>
            <TooltipContent side="right"> Browse </TooltipContent>
          </Tooltip>
        </TooltipProvider>
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <a
                href="/stats"
                class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              >
                <LineChart class="h-5 w-5" />
                <span class="sr-only">Stats</span>
              </a>
            </TooltipTrigger>
            <TooltipContent side="right"> Stats </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </nav>
      <nav class="mt-auto flex flex-col items-center gap-4 px-2 sm:py-5">
        <TooltipProvider>
          <Tooltip>
            <TooltipTrigger as-child>
              <a
                href="/settings"
                class="flex h-9 w-9 items-center justify-center rounded-lg text-muted-foreground transition-colors hover:text-foreground md:h-8 md:w-8"
              >
                <Settings
                  class="h-5 w-5 hover:rotate-[60deg] ease-in-out duration-300"
                />
                <span class="sr-only">Settings</span>
              </a>
            </TooltipTrigger>
            <TooltipContent side="right"> Settings </TooltipContent>
          </Tooltip>
        </TooltipProvider>
      </nav>
    </aside>
    <div class="flex flex-col sm:pl-14 h-screen max-h-screen">
      <header
        class="sticky top-0 z-30 flex mt-3 h-4 items-center gap-4 border-b bg-background px-4 sm:static sm:h-auto sm:border-0 sm:bg-transparent sm:px-6"
      >
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem>
              <BreadcrumbLink class="capitalize">{{
                pageNames[0]
              }}</BreadcrumbLink>
            </BreadcrumbItem>
            <template v-for="page in pageNames.slice(1)">
              <BreadcrumbSeparator />
              <BreadcrumbItem>
                <BreadcrumbLink class="capitalize">{{ page }}</BreadcrumbLink>
              </BreadcrumbItem>
            </template>
          </BreadcrumbList>
        </Breadcrumb>

        <div class="pr-4 ml-auto">
          <Popover v-model:open="profileSetterOpen">
            <PopoverTrigger>
              <div class="flex items-center px-2 rounded-md bg-accent">
                <Settings2 :size="16" class="mr-1" />
                {{ currentLanguage ?? "Select a profile" }}
              </div>
            </PopoverTrigger>
            <PopoverContent class="w-60">
              <h4 class="pb-2 font-medium leading-none">Quick Settings</h4>
              <Label for="current-language">Current profile:</Label>
              <Select
                id="current-language"
                :modelValue="currentLanguage ?? ''"
                @update:model-value="
                  ($event) => {
                    setLanguage($event);
                    currentLanguage = $event;
                  }
                "
              >
                <SelectTrigger class="w-[180px]">
                  <SelectValue placeholder="Select a profile" />
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
      </header>
      <Separator class="mt-3" />

      <main class="flex-grow">
        <Suspense>
          <router-view
            :currentLanguage="currentLanguage"
            @settingsChanged="updateLanguages"
            @newCurrentLanguage="
              updateLanguages();
              console.log($event);
              currentLanguage = $event;
            "
            @setProfile="profileSetterOpen = true"
          ></router-view>
        </Suspense>
      </main>
    </div>
  </div>
</template>
