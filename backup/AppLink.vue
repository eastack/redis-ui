<script setup lang="ts">
import {AppRouteNames} from "../src/main";
import {RouteParams} from "vue-router";

export interface AppLinkProps {
  name: AppRouteNames,
  params?: Partial<RouteParams>
}

const props = withDefaults(defineProps<AppLinkProps>(), {
  params: () => ({}),
})
</script>

<template>
  <RouterLink
      :aria-label="props.name"
      :to="props">
    <slot/>
  </RouterLink>
</template>

<style scoped>
</style>
