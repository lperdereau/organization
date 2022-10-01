import store from "@/store";
import { type Notification, NotificationType } from "@/types/notification";
import { ref, toRef } from "vue";

const id = ref(0);
const DURATION = 2000;

const notify = (notification: Notification) => {
  store.notifications.push(notification);
};

export const removeNotification = (notification: Notification) => {
  store.notifications = store.notifications.filter((n) => {
    return n.id != notification.id;
  });
};

export const notifyError = (msg = "An error occured.", duration = DURATION) => {
  id.value = id.value + 1;
  notify({ id: id.value, msg, type: NotificationType.ERROR, duration });
};

export const notifyWarn = (msg: string, duration = DURATION) => {
  id.value = id.value + 1;
  notify({ id: id.value, msg, type: NotificationType.WARN, duration });
};

export const notifySuccess = (
  msg = "Operation succeeded",
  duration = DURATION
) => {
  id.value = id.value + 1;
  notify({ id: id.value, msg, type: NotificationType.SUCCESS, duration });
};
