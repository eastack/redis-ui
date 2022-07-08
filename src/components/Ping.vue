<script setup lang="ts">
import {ref} from 'vue'
import {invoke} from "@tauri-apps/api/tauri";

defineProps<{ action: string }>()

async function ping() {
  return await invoke('ping')
      .then(response => console.log(response))
      .catch(error => console.log(error));
}

async function listKeys() {
  console.log(keyPattern.value)
  return await invoke<[string]>('keys', {'keyPattern': keyPattern.value})
      .then(response => {
        keys.value = response;
        console.log(response)
        console.log(keys.value)
      })
      .catch(error => console.log(error));
}


async function listDb() {
  console.log(keyPattern.value)
  return await invoke('list_db')
      .then(response => {
        // keys.value = response;
        console.log(response)
        console.log(keys.value)
      })
      .catch(error => console.log(error));
}

async function newDatabase() {
  console.log('create new database...')

  return await invoke<[string]>('create_window')
      .then(response => console.log(response))
      .catch(error => console.log(error));
}

const keyPattern = ref('');
const keys = ref(['']);
</script>

<template>
  <h6>{{ action }}</h6>
  <button type="button" @click="ping">Click me to: {{ action }}</button>
  <input type="text" v-model="keyPattern">
  <button type="button" @click="listKeys">Keys</button>
  <ul>
    <li v-for="key in keys">
      {{ key }}
    </li>
  </ul>

  <button type="button" @click="newDatabase">New DB</button>
  <br/>
  <button type="button" @click="listDb">Config</button>
</template>

<style scoped>
li {
  text-align: center;
  list-style: none;
}
</style>
