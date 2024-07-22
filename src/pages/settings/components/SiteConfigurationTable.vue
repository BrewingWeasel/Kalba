<script setup lang="ts">
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";
import {
  AlertDialog,
  AlertDialogDescription,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
} from "@/components/ui/alert-dialog";
import {
  TagsInput,
  TagsInputInput,
  TagsInputItem,
  TagsInputItemDelete,
  TagsInputItemText,
} from "@/components/ui/tags-input";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

import { Button } from "@/components/ui/button";
import { type SiteConfiguration } from "@/types";
import { Pencil, X } from "lucide-vue-next";
import { ref } from "vue";
import Badge from "@/components/ui/badge/Badge.vue";

const sites = defineModel<{ [key: string]: SiteConfiguration }>({
  required: true,
});

const siteConfigOpen = ref<{ [key: string]: boolean }>({});
for (const site in sites) {
  siteConfigOpen.value[site] = false;
}

function newSite() {
  sites.value["New site"] = {
    sites: [],
    main_section: "main",
    title_selector: "",
    subtitle_selector: "",
    image_selector: "img",
    caption_selector: "",
    paragraph_selector: "",
  };
  siteConfigOpen.value["New site"] = true;
}
</script>

<template>
  <Table class="max-w-5xl">
    <TableCaption>
      <Button variant="ghost" class="text-xl" @click="newSite">+</Button>
    </TableCaption>
    <TableHeader>
      <TableRow>
        <TableHead class="w-1/3">Name</TableHead>
        <TableHead>Sites</TableHead>
        <TableHead class="text-right">Configure</TableHead>
        <TableHead class="w-1 text-right">Remove</TableHead>
      </TableRow>
    </TableHeader>
    <TableBody>
      <TableRow v-for="(_, site) in sites">
        <TableCell
          ><Input
            :modelValue="site"
            @update:model-value="
              ($event) => {
                sites[$event] = sites[site];
                delete sites[site];
              }
            "
          ></Input
        ></TableCell>
        <TableCell
          ><Badge v-for="currentSite in sites[site].sites" class="mr-1">{{
            currentSite
          }}</Badge></TableCell
        >
        <TableCell class="text-right">
          <AlertDialog v-model:open="siteConfigOpen[site]">
            <AlertDialogTrigger><Pencil :size="16" /></AlertDialogTrigger>
            <AlertDialogContent>
              <AlertDialogHeader>
                <AlertDialogTitle>Edit site</AlertDialogTitle>
              </AlertDialogHeader>
              <AlertDialogDescription
                >Configure the css selectors used to parse the
                site</AlertDialogDescription
              >

              <TagsInput v-model="sites[site].sites">
                <TagsInputItem
                  v-for="item in sites[site].sites"
                  :key="item"
                  :value="item"
                >
                  <TagsInputItemText />
                  <TagsInputItemDelete />
                </TagsInputItem>
                <TagsInputInput placeholder="Sites" />
              </TagsInput>

              <div>
                <Label for="main-section">Main section</Label>
                <Input v-model="sites[site].main_section" id="main-section" />
                <Label for="title-selector">Title Selector</Label>
                <Input
                  v-model="sites[site].title_selector"
                  id="title-selector"
                />
                <Label for="subtitle-selector">Subtitle Selector</Label>
                <Input
                  v-model="sites[site].subtitle_selector"
                  id="subtitle-selector"
                />
                <Label for="caption-selector">Caption Selector</Label>
                <Input
                  v-model="sites[site].caption_selector"
                  id="caption-selector"
                />
                <Label for="image-selector">Image Selector</Label>
                <Input
                  v-model="sites[site].image_selector"
                  id="image-selector"
                />
              </div>

              <AlertDialogFooter>
                <AlertDialogCancel>Cancel</AlertDialogCancel>
                <AlertDialogAction>Save</AlertDialogAction>
              </AlertDialogFooter>
            </AlertDialogContent>
          </AlertDialog>
        </TableCell>
        <TableCell
          ><X
            @click="
              () => {
                delete sites[site];
              }
            "
            class="float-right transition duration-200 cursor-pointer hover:scale-110 hover:stroke-rose-500"
            :size="16"
        /></TableCell>
      </TableRow>
    </TableBody>
  </Table>
</template>
