import type Api from "@/types/api";
import type Notification from "@/types/notification";

type Store = {
  apiOrganization: Api | null;
  notifications: Notification[];
};

export default Store;
