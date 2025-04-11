<template>
  <component
    :is="as"
    :class="cn(buttonVariants({ variant, size }), $attrs.class ?? '')"
  >
    <slot />
  </component>
</template>

<script setup lang="ts">
import { cn } from "../utils";
import { cva } from "class-variance-authority";

const buttonVariants = cva(
  "inline-flex items-center justify-center rounded-md text-sm font-medium ring-offset-background transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 disabled:pointer-events-none disabled:opacity-50",
  {
    variants: {
      variant: {
        default:
          "bg-shadcn-primary text-shadcn-primary-foreground! hover:bg-shadcn-primary/90",
        destructive:
          "bg-shadcn-destructive text-shadcn-destructive-foreground! hover:bg-shadcn-destructive/90",
        outline:
          "border border-shadcn-input bg-shadcn-background hover:bg-shadcn-accent hover:text-shadcn-accent-foreground",
        secondary:
          "bg-shadcn-secondary text-shadcn-secondary-foreground hover:bg-shadcn-secondary/80",
        ghost: "hover:bg-shadcn-accent hover:text-shadcn-accent-foreground",
        link: "text-shadcn-primary underline-offset-4 hover:underline",
      },
      size: {
        default: "h-10 px-4 py-2",
        sm: "h-9 rounded-md px-3",
        lg: "h-11 rounded-md px-8",
        icon: "h-10 w-10",
      },
    },
    defaultVariants: {
      variant: "default",
      size: "default",
    },
  }
);

interface Props {
  variant?: NonNullable<Parameters<typeof buttonVariants>[0]>["variant"];
  size?: NonNullable<Parameters<typeof buttonVariants>[0]>["size"];
  as?: string;
}

// eslint-disable-next-line vue/define-macros-order
withDefaults(defineProps<Props>(), {
  as: "button",
});
</script>
