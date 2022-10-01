import appStore from "@/store";
import type Organization from "@/types/organization";

export const getOrganizations = async (): Promise<Organization[]> => {
  const url = new URL("/organizations/", appStore.apiOrganization?.baseUrl);
  return await fetch(url)
    .then((r) => r.json())
    .then((res) => res["result"]);
};

export const getOrganization = async (id: string) => {
  const url = new URL(
    `/organizations/${id}/`,
    appStore.apiOrganization?.baseUrl
  );
  return await fetch(url);
};

export const createOrganization = async () => {
  const url = new URL("/organizations/", appStore.apiOrganization?.baseUrl);
  return await fetch(url);
};
