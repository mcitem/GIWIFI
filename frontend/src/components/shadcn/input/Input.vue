<script setup lang="ts">
import type { HTMLAttributes } from "vue";
import { cn } from "../utils";
import { useVModel } from "@vueuse/core";

const props = defineProps<{
  defaultValue?: string | number;
  modelValue?: string | number;
  class?: HTMLAttributes["class"];
}>();

const emits = defineEmits<{
  (e: "update:modelValue", payload: string | number): void;
}>();

const modelValue = useVModel(props, "modelValue", emits, {
  passive: true,
  defaultValue: props.defaultValue,
});
</script>

<template>
  <input
    v-model="modelValue"
    :class="
      cn(
        'flex h-10 w-full rounded-md border border-shadcn-input bg-shadcn-input-background px-3 py-2 text-sm ring-offset-background file:border-0 file:bg-transparent file:text-shadcn-foreground file:text-sm file:font-medium placeholder:text-shadcn-muted-foreground focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-shadcn-ring disabled:cursor-not-allowed disabled:opacity-50',
        props.class
      )
    "
  />
</template>
<style></style>
