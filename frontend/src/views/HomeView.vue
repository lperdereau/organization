<template>
  <span>Hello</span>
</template>

<script setup lang="ts">
import { useQuery } from "vue-query";

import { getOrganizations } from "@/services/organization";

const { isLoading, isError, isFetching, data, error, refetch } = useQuery(
  "organizations",
  getOrganizations
);
</script>
