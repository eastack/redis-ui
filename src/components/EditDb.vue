<script setup lang="ts">
import {computed, onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Database} from "../entity/Database";
import {routerPush} from "../main";
import {useRoute} from "vue-router";

const route = useRoute()

const id = computed<string>(() => route.params.id as string)

onMounted(() => {
  loadDb(id.value)
})

async function loadDb(id: string) {
  console.log(`id: ${id}`);
  return await invoke<Database>('get_db', {id: id})
      .then(response => database.value = response)
      .catch(error => console.log(error));
}

const database = ref<Database>({} as Database);

async function submit() {
  console.log(`Update database: ${JSON.stringify(database.value)}`)
  return await invoke('update_db', {id: id.value, db: database.value})
      .then(response => {
        console.log(`Update database success: ${response}`)
        routerPush("index")
      }).catch(error => console.log(error));
}
</script>

<template>
  <h1>添加数据库</h1>
  <form class="w-full mx-auto my-10" @submit.prevent="submit">
    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="id">Host*</label>
        <input id="host" placeholder="Enter Hostname / IP address" v-model="database.host">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="port">Port*</label>
        <input id="port" placeholder="Enter Port" v-model.number="database.port">
      </div>
    </div>

    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="alias">Database Alias*</label>
        <input id="alias" placeholder="Enter Database Alias" v-model="database.alias">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="alias">Database Index</label>
        <input id="alias" placeholder="Enter Database Index" v-model.number="database.db">
      </div>
    </div>

    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="username">Username</label>
        <input id="username" placeholder="Enter Username" v-model="database.username">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="password">Password</label>
        <input id="password" type="password" placeholder="Enter Password" v-model="database.password">
      </div>
    </div>

    <div class="mb-6">
      <div class="px-3 flex items-center mb-4">
        <input class="w-4 h-4 focus:ring-4" type="checkbox" id="use-tls" v-model="database.tls">
        <label class="ml-2 text-sm font-medium text-gray-900" for="use-tls">Use TLS</label>
      </div>
      <template v-if="database.tls">
        <div class="w-1/2 px-3 text-item mb-4">
          <label for="password">Password</label>
          <input id="password" type="password" placeholder="Enter Password">
        </div>
      </template>
    </div>
    <div class="flex justify-around">
      <button class="btn btn-white" @click="routerPush('index')">取消</button>
      <button class="btn btn-blue">保存</button>
    </div>
  </form>
</template>

<style scoped>
.text-item label {
  @apply block tracking-wide text-gray-700 text-xs font-bold mb-2
}

.text-item input {
  @apply block w-full bg-gray-200 text-gray-700 border rounded py-3 px-4 mb-3
}

.btn {
  @apply font-bold py-2 px-4 rounded
}

.btn-blue {
  @apply bg-blue-400 text-white
}

.btn-blue:hover {
  @apply bg-blue-500
}

.btn-white {
  @apply bg-gray-400 text-white
}

.btn-white:hover {
  @apply bg-gray-500
}
</style>
