<template>
  <Dialog v-model="isVisible" class="max-w-md px-5 py-5 sm:px-6">
    <!-- Title -->
    <DialogTitle v-if="title" class="mb-3 text-lg font-semibold">{{
      title
    }}</DialogTitle>
    <!-- Message -->
    <div class="mb-6">{{ message }}</div>
    <!-- Buttons -->
    <div
      class="flex flex-col-reverse gap-2 sm:flex-row sm:flex-wrap sm:justify-end"
    >
      <Button
        variant="outline"
        class="w-full sm:w-auto"
        @click="emitClose('cancel')"
        >{{ cancelButtonText }}</Button
      >
      <Button
        v-if="rejectButtonText"
        :variant="rejectButtonStyle === 'danger' ? 'destructive' : 'outline'"
        class="w-full sm:w-auto"
        @click="emitClose('reject')"
        >{{ rejectButtonText }}</Button
      >
      <Button
        v-focus
        :variant="buttonVariant"
        class="w-full sm:w-auto"
        @click="emitClose('confirm')"
        >{{ confirmButtonText }}</Button
      >
    </div>
  </Dialog>
</template>

<script setup lang="ts">
import { computed } from "vue";
import { DialogTitle } from "reka-ui";

import Button from "../ui/Button.vue";
import Dialog from "../ui/Dialog.vue";

const props = defineProps({
  title: { type: String, default: "Confirmation" },
  message: String,
  confirmButtonStyle: { type: String, default: "cta" },
  confirmButtonText: { type: String, default: "Confirm" },
  cancelButtonStyle: { type: String, default: "subtle" },
  cancelButtonText: { type: String, default: "Cancel" },
  rejectButtonStyle: { type: String, default: "danger" },
  rejectButtonText: { type: String },
});
const emit = defineEmits<{
  (event: "confirm" | "reject" | "cancel"): void;
}>();
const isVisible = defineModel({ type: Boolean });
const buttonVariant = computed(() => {
  if (props.confirmButtonStyle === "danger") return "destructive";
  if (props.confirmButtonStyle === "success") return "success";
  return "default";
});

function emitClose(closeEvent: "confirm" | "reject" | "cancel" = "cancel") {
  isVisible.value = false;
  emit(closeEvent);
}
</script>
