<template>
  <span class="text-3xl">Hello</span>
</template>

<script setup lang="ts">
import { watch } from "@vue/runtime-core";
import { useQuery } from "vue-query";
import { getOrganizations } from "@/services/organization";
import { notifyError } from "@/store/notifications";

const { isError, data, suspense } = useQuery(
  "organizations",
  getOrganizations,
  {
    retry: 0,
  }
);

watch(isError, () => {
  if (isError) {
    notifyError();
  }
});

await suspense();
</script>
