<script setup lang="ts">
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  DialogClose,
} from "@/components/ui/dialog";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";

const props = defineProps({
  submitText: String,
  description: String,
  source: String,
  title: {
    type: String,
    required: true,
  },
  buttonName: {
    type: String,
    required: true,
  },
});

defineEmits<{
  submitted: [];
}>();
</script>

<template>
  <Card>
    <CardHeader>
      <CardTitle>{{ props.title }}</CardTitle>
      <CardDescription>{{ props.source ?? "builtin" }}</CardDescription>
    </CardHeader>
    <CardContent>
      <Dialog>
        <DialogTrigger
          ><Button>{{ props.buttonName }}</Button></DialogTrigger
        >
        <DialogContent>
          <DialogHeader>
            <DialogTitle>{{ props.title }}</DialogTitle>
            <DialogDescription>
              {{ props.description }}
            </DialogDescription>
          </DialogHeader>
          <slot />
          <DialogFooter>
            <DialogClose as-child>
              <Button @click="$emit('submitted')">
                {{ props.submitText ?? "Submit" }}
              </Button>
            </DialogClose>
          </DialogFooter>
        </DialogContent>
      </Dialog>
    </CardContent>
  </Card>
</template>
