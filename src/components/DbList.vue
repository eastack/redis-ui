<script setup lang="ts">
import {invoke} from "@tauri-apps/api/tauri";
import {Database} from "../entity/Database";
import {onMounted, ref} from "vue";
import DbListNavigation from "./DbListNavigation.vue";
import {PencilAltIcon, TrashIcon} from '@heroicons/vue/outline'
import {routerPush} from "../main";
import ConfirmModal from './ConfirmModal.vue'

let dbs = ref<Database[]>([]);

async function listDb() {
  console.log("List db")
  return await invoke<Database[]>('list_db')
      .then(response => dbs.value = response)
      .catch(error => console.log(error));
}

const showDelConfirm = ref(false);
const toDelDbId = ref('');
const delConfirmMessage = "确定要删除此数据库吗?"

async function confirmDel(id: string) {
  showDelConfirm.value = true
  toDelDbId.value = id
}

async function removeDb(id: string) {
  return await invoke('remove_db', {id: id})
      .then(response => {
        console.log(response)
        showDelConfirm.value = false
        toDelDbId.value = ''
        listDb()
      }).catch(error => console.log(error))
}

let db = ref<Database>({} as Database)

async function dbDetail(id: string) {
  console.log("dbDetail", id)
  return routerPush("dashboard", {id: id})
}

onMounted(() => {
  listDb()
})

async function updateData(databases: Database[]) {
  console.log("search ok:", databases)
  dbs.value = databases
}

const show = ref(false)
</script>

<template>
  <ConfirmModal :show="showDelConfirm" :message="delConfirmMessage">
    <button type="button" class="btn btn-white mx-5" @click="showDelConfirm = false">取消</button>
    <button type="button" class="btn btn-blue mx-5" @click="removeDb(toDelDbId)">确定</button>
  </ConfirmModal>
  <DbListNavigation @searched="updateData"/>
  <div id="db-list">
    <!--    <div class="divide-y divide-gray-100 px-8 basis-4/5">-->
    <!--      <span>Database Alias</span>-->
    <!--      <span>Host:Port</span>-->
    <!--      <span>Logical Database</span>-->
    <!--    </div>-->
    <ol class="divide-y divide-gray-100 px-8">
      <li v-for="(db, index) in dbs" key="{{db.id}}" class="flex p-3 hover:bg-slate-100 px-8"
          :class="{'bg-gray-50': index % 2 === 0}">
        <div class="basis-4/5 flex justify-around text-left">
          <span @click="dbDetail(db.id)" class="basis-1/3 hover:underline cursor-pointer">{{ db.alias }}</span>
          <span class="basis-1/3">{{ db.host }}:{{ db.port }}</span>
          <span class="basis-1/3">{{ db.db }}</span>
        </div>
        <span class="basis-1/5 flex justify-around">
          <button @click="routerPush('edit-db', {id: db.id})">
            <PencilAltIcon class="edit-button"></PencilAltIcon>
          </button>
          <button @click="confirmDel(db.id)"><TrashIcon class="del-button"></TrashIcon></button>
        </span>
      </li>
    </ol>
  </div>
</template>

<style scoped>
.edit-button {
  @apply w-7 stroke-slate-300 hover:stroke-blue-400
}

.del-button {
  @apply w-7 stroke-slate-300 hover:stroke-red-400
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
