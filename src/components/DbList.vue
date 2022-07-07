<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {Database} from "../entity/Database";
import {onMounted, ref} from "vue";
import {useRouter} from "vue-router";

const router = useRouter();

async function goToNewDb() {
  console.log("GoTo new-db")
  router.push("/new-db")
}

async function goToList() {
  console.log("GoTo new-db")
  router.push("/db-list")
}

let dbs = ref<Database[]>([]);

async function dbList() {
  console.log("List db")
  return await invoke<Database[]>('list_db')
      .then(response => dbs.value = response)
      .catch(error => console.log(error));
}

async function removeDb(id: string) {
  return await invoke('remove_db', {
    'id': id
  }).then(response => {
    console.log(response)
    dbList()
  }).catch(error => console.log(error))
}

onMounted(() => {
  dbList()
})

</script>

<template>
  <div id="db-nav">
    <button @click="goToNewDb">新增数据库</button>
    <input type="text" placeholder="Search">
  </div>
  <div id="db-list">
    <ol>
      <li v-for="db in dbs">
        {{ db.name }} - {{ db.host }}: {{ db.port }}
        <button @click="removeDb(db.id)">删除</button>
      </li>
    </ol>
  </div>
</template>

<style scoped>
#db-list {
  display: flex;
  justify-content: space-around;
}

#db-nav {
  display: flex;
  justify-content: space-around;
  position: sticky;
}
</style>
