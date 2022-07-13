<script setup lang="ts">
import {ArrowLeftIcon} from "@heroicons/vue/solid";
import {routerPush} from "../main";
import {useRoute} from "vue-router";

interface Props {
  id: string
}

const props = defineProps<Props>()
const route = useRoute()
</script>

<template>
  <div>
    <nav>
      <ArrowLeftIcon @click="routerPush('index')"/>
    </nav>
    <div>ID: {{ route.params.id }}</div>
  </div>
</template>

<style scoped>
</style>
