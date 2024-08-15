<script setup lang="ts">
import {
  Dialog,
  DialogScrollContent,
  DialogDescription,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Button } from "@/components/ui/button";
import LanguageList from "@/components/LanguageList.vue";
</script>

<template>
  <Dialog>
    <DialogTrigger as-child>
      <Button variant="ghost" class="text-xl text-center">+</Button>
    </DialogTrigger>
    <DialogScrollContent>
      <DialogHeader>
        <DialogTitle>Add a template</DialogTitle>
        <DialogDescription>
          Click to select a template to use for the new profile
        </DialogDescription>
      </DialogHeader>
      <LanguageList
        :isDialog="true"
        existingSelection=""
        @langSelected="$emit('langSelected', $event)"
      />
    </DialogScrollContent>
  </Dialog>
</template>
