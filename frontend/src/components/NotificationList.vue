<template>
  <div class="pt-5 pr-3 absolute top-0 right-0 w-1/5 flex flex-col space-y-4">
    <Notification
      v-for="notification in notifications"
      :key="notification.id"
      :notification="notification"
      @finished="removeNotification(notification)"
    />
  </div>
</template>

<script setup lang="ts">
import { toRefs } from "vue";
import store from "@/store";
import { removeNotification } from "@/store/notifications";
import Notification from "./Notification.vue";

const { notifications } = toRefs(store);
</script>
