<script setup lang="ts">
import {computed, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";
import {Database} from "../entity/Database";
import {routerPush} from "../main";

async function submit() {
  console.log(`Create new database: ${JSON.stringify(database.value)}`)
  return await invoke('add_db', {db: database.value})
      .then(response => {
        console.log(`Create database success: ${response}`)
        routerPush("index")
      }).catch(error => console.log(error));
}

const host = ref<string>('127.0.0.1')
const port = ref<number>(0)
const alias = ref<string>('测试')
const username = ref<string>('12345')
const password = ref<string>('12345')
const db = ref<number>(0)

const useTls = ref(false);

const database = computed(() => {
  return {
    host: host.value,
    port: port.value,
    alias: alias.value,
    db: db.value,
    username: username.value,
    password: password.value,
    tls: useTls.value,
  } as Database
})

console.log(database)
</script>

<template>
  <h1>添加数据库</h1>
  <form class="w-full mx-auto my-10" @submit.prevent="submit">
    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="id">Host*</label>
        <input id="host" placeholder="Enter Hostname / IP address" v-model="host">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="port">Port*</label>
        <input id="port" placeholder="Enter Port" v-model.number="port">
      </div>
    </div>

    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="alias">Database Alias*</label>
        <input id="alias" placeholder="Enter Database Alias" v-model="alias">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="alias">Database Index</label>
        <input id="alias" placeholder="Enter Database Index" v-model.number="db">
      </div>
    </div>

    <div class="flex flex-wrap mb-4">
      <div class="w-1/2 px-3 text-item">
        <label for="username">Username</label>
        <input id="username" placeholder="Enter Username" v-model="username">
      </div>
      <div class="w-1/2 px-3 text-item">
        <label for="password">Password</label>
        <input id="password" type="password" placeholder="Enter Password" v-model="password">
      </div>
    </div>

    <div class="mb-6">
      <div class="px-3 flex items-center mb-4">
        <input class="w-4 h-4 focus:ring-4" type="checkbox" id="use-tls" v-model="useTls">
        <label class="ml-2 text-sm font-medium text-gray-900" for="use-tls">Use TLS</label>
      </div>
      <template v-if="useTls">
        <div class="w-1/2 px-3 text-item mb-4">
          <label for="password">Password</label>
          <input id="password" type="password" placeholder="Enter Password" v-model="password">
        </div>
      </template>
    </div>
    <div class="flex justify-around">
      <button class="btn btn-white" @click="routerPush('index')">取消</button>
      <button class="btn btn-blue">提交</button>
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
  @apply font-bold py-2 px-4 rounded;
}

.btn-blue {
  @apply bg-blue-400 text-white;
}

.btn-blue:hover {
  @apply bg-blue-500;
}

.btn-white {
  @apply bg-gray-400 text-white;
}

.btn-white:hover {
  @apply bg-gray-500;
}
</style>
