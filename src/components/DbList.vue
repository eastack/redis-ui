<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {Database} from "../entity/Database";
import {onMounted, ref} from "vue";

let dbFormDisplay = ref(false)

async function showDbForm() {
  dbFormDisplay.value = true
}

async function hiddenDbForm() {
  dbFormDisplay.value = false
}

let dbs = ref<Database[]>([]);

async function loadDbList() {
  console.log("List db")
  return await invoke<Database[]>('list_db')
      .then(response => dbs.value = response)
      .catch(error => console.log(error));
}

async function removeDb(id: string) {
  return await invoke('remove_db', {id: id})
      .then(response => {
        console.log(response)
        loadDbList()
      }).catch(error => console.log(error))
}

let db = ref<Database>({} as Database)

async function editDb(id: string) {
  return await invoke<Database>('get_db', {id: id})
      .then(response => {
        db.value = response
        console.log(response)
        showDbForm()
        db.value = {} as Database
      })
      .catch(error => console.log(error));
}

async function createDb() {
  if (db.value.id == undefined) {
    console.log("新建数据库...")
    if (db.value.color == undefined) db.value.color = '#cfe8fd'
    if (db.value.auth == undefined) db.value.auth = 'None'
    if (db.value.port == undefined) db.value.port = 6379
    if (db.value.db == undefined) db.value.db = 0

    await invoke('add_db', {db: db.value})
        .then(response => {
          console.log(`新建数据库成功: ${response}`)
          hiddenDbForm()
          loadDbList()
          db.value = {} as Database
        }).catch(error => console.log(error));
  } else {
    console.log(db.value)
    await invoke('update_db', {id: db.value.id, db: db.value})
        .then(response => {
          console.log(`修改数据库成功: ${response}`)
          hiddenDbForm()
          loadDbList()
          db.value = {} as Database
        }).catch(error => console.log(error));
  }
}

async function dbDetail(id: string) {
  console.log("dbDetail", id)
}

onMounted(() => {
  loadDbList()
})
</script>

<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity" v-show="dbFormDisplay"></div>
  <!-- 新增数据库弹窗 -->
  <div class="fixed z-10 inset-0 overflow-y-auto" v-show="dbFormDisplay">
    <div class="flex items-center justify-center min-h-full">
      <form @submit.prevent="createDb" class="bg-white rounded-lg overflow-hidden p-6">
        <div class="pb-4">新增数据库</div>
        <div class="bg-white pb-4">
          <input hidden v-model="db.id">
          <input placeholder="名称" class="block shadow border my-3 py-2 px-3 rounded" v-model="db.name">
          <input placeholder="备注" class="block shadow border my-3 py-2 px-3 rounded" v-model="db.comment">
          <input placeholder="主机" class="block shadow border my-3 py-2 px-3 rounded" v-model="db.host">
          <input placeholder="端口" class="block shadow border my-3 py-2 px-3 rounded" v-model.number="db.port">
          <input placeholder="数据库" class="block shadow border my-3 py-2 px-3 rounded" v-model.number="db.db">
        </div>
        <div class="bg-white pt-4">
          <button type="button"
                  @click="createDb"
                  class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-600 text-base font-medium text-white hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm">
            确定
          </button>
          <button type="button" @click="hiddenDbForm"
                  class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">
            取消
          </button>
        </div>
      </form>
    </div>
  </div>

  <div id="db-nav" class="flex gap-4 flex-row px-8 py-5">
    <input id="search-input" type="text" placeholder="Search"
           class="basis-4/5 items-center text-slate-400 rounded-md ring-1 ring-slate-900/10 shadow-sm py-1.5 pl-2 pr-3 hover:ring-slate-300 dark:bg-slate-800 dark:highlight-white/5 dark:hover:bg-slate-700">
    <button @click="showDbForm"
            class="basis-1/5 bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">
      新增数据库
    </button>
  </div>
  <div id="db-list">
    <ol class="divide-y divide-gray-100 px-8 ">
      <li v-for="db in dbs" key="{{db.id}}" @click="dbDetail(db.id)" class="flex p-3 hover:bg-slate-100">
        <div class="information basis-4/5 flex justify-around text-center">
          <span class="basis-1/3">{{ db.name }}</span>
          <span class="basis-1/3">{{ db.comment }}</span>
          <span class="basis-1/3">{{ db.host }}</span>
          <span class="basis-1/3">{{ db.port }}</span>
          <span class="basis-1/3">{{ db.db }}</span>
        </div>
        <span class="operator flex basis-1/5 px-5 justify-around">
          <button @click="editDb(db.id)"
                  class="basis-1/4 bg-indigo-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded">编辑</button>
          <button @click="removeDb(db.id)"
                  class="basis-1/4 bg-red-500 hover:bg-red-700 text-white font-bold py-2 px-4 rounded">删除</button>
        </span>
      </li>
    </ol>
  </div>
</template>

<style scoped>
#db-nav {
  /*position: sticky;*/
}

#search-input {
  /*box-shadow: rgb(255, 255, 255) 0px 0px 0px 0px, rgba(15, 23, 42, 0.1) 0px 0px 0px 1px, rgba(0, 0, 0, 0.05) 0px 1px 2px 0px*/
}
</style>
