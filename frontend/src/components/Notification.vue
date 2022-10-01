<template>
  <div :class="['rounded-b', 'shadow-md', cssClass]" role="alert">
    <div class="flex px-4 py-3 items-center">
      <div class="py-1">
        <svg
          xmlns="http://www.w3.org/2000/svg"
          fill="none"
          viewBox="0 0 24 24"
          stroke-width="1.5"
          stroke="currentColor"
          class="w-6 h-6"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            d="M9.75 9.75l4.5 4.5m0-4.5l-4.5 4.5M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      </div>
      <div>
        <p class="font-bold">{{ notification.msg }}</p>
      </div>
    </div>
    <div class="w-full bg-gray-200 h-2.5 dark:bg-gray-700">
      <div class="h-2.5" :style="{ width: `${width}%` }"></div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { defineProps, watch, ref, computed } from "vue";
import {type Notification, NotificationType} from "@/types/notification";

type Props = {
  notification: Notification;
};

const { notification } = defineProps<Props>();

const COLOR_MAP = {
  [NotificationType.ERROR]: 'error',
  [NotificationType.WARN]: 'warn',
  [NotificationType.SUCCESS]: 'success',
}

const cssClass = computed(() => {
  return COLOR_MAP[notification.type]
})

const emit = defineEmits(["finished"]);

let time = ref(notification.duration);

const interval = setInterval(function () {
  time.value -= 1;
}, 1);

const width = computed(() => {
  return (time.value / notification.duration) * 100;
});

watch(time, () => {
  if (!time.value) {
    clearInterval(interval);
    emit("finished");
  }
});
</script>

<style scoped lang="scss">
.error {
  @apply bg-red-100 text-red-900;

  div:nth-child(2) > div {
    @apply bg-red-600;
  }
}

.warn {
  @apply bg-yellow-100 text-yellow-900;

  div:nth-child(2) > div {
    @apply bg-yellow-600;
  }
}

.success {
  @apply bg-green-100 text-green-900;

  div:nth-child(2) > div {
    @apply bg-green-600;
  }
}
</style>
