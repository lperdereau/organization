<script setup lang="ts">
import { RouterLink, RouterView } from "vue-router";
import NotificationList from "./components/NotificationList.vue";
</script>

<template class="relative h-full w-full">
  <NotificationList />
  <Suspense>
    <template #default>
      <RouterView />
    </template>
    <template #fallback>
      <span>Loading ...</span>
    </template>
  </Suspense>
</template>
