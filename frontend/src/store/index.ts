import { reactive } from "vue";
import type Store from "@/types/store";

const store: Store = reactive({
  apiOrganization: null,
});

export default store;
