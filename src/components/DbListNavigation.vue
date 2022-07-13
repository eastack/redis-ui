<template>
  <div id="db-nav" class="flex gap-4 flex-row justify-center px-8 py-5">
    <button @click="routerPush('create-db')"
            class="basis-1/5 bg-gray-200 hover:bg-gray-300 text-gray-600 hover:text-gray-700 font-bold py-2 px-4 rounded">
      新增数据库
    </button>
    <input id="search-input" type="text" placeholder="Search" v-model="q" @input="search"
           class="basis-4/5 items-center text-slate-700 rounded hover:ring-2 ring-1 pl-2 my-1">
  </div>
</template>

<script setup lang="ts">
import {routerPush} from "../main";
import {ref} from "vue";
import {useDebounceFn} from "@vueuse/core";
import {Database} from "../entity/Database";
import {invoke} from "@tauri-apps/api/tauri";

const emit = defineEmits<{
  (event: "searched", databases: Database[]): void
}>()

const q = ref('')

const debouncedFn = useDebounceFn(() => {
  listDb(q.value)
}, 300)

function search() {
  debouncedFn()
}

async function listDb(q: string) {
  return await invoke<Database[]>('list_db', {q: q})
      .then(response => {
        console.log(response)
        emit("searched", response)
      })
      .catch(error => console.log(error));
}
</script>

<style scoped>

</style>
