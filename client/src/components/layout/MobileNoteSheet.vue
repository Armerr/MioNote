<template>
  <Sheet
    v-model="open"
    class="inset-0 w-full border-0 bg-theme-canvas shadow-none lg:hidden"
  >
    <MobileNoteList
      ref="mobileNoteList"
      show-close
      @close="open = false"
      @navigate="open = false"
    />
  </Sheet>
</template>

<script setup lang="ts">
import { ref } from "vue";

import MobileNoteList from "./MobileNoteList.vue";
import Sheet from "../ui/Sheet.vue";

const open = defineModel({ type: Boolean, default: false });
const mobileNoteList = ref();

defineExpose({
  focusSearch: () => mobileNoteList.value?.focusSearch(),
});
</script>
