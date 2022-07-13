<template>
  <div class="fixed inset-0 bg-gray-500 bg-opacity-75 transition-opacity"></div>
  <!-- 新增数据库弹窗 -->
  <div class="fixed z-10 inset-0 overflow-y-auto">
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
                  @click="submit"
                  class="w-full inline-flex justify-center rounded-md border border-transparent shadow-sm px-4 py-2 bg-blue-200 text-base font-medium text-white hover:bg-blue-300 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500 sm:ml-3 sm:w-auto sm:text-sm">
            确定
          </button>
          <button type="button" @click="cancel"
                  class="mt-3 w-full inline-flex justify-center rounded-md border border-gray-300 shadow-sm px-4 py-2 bg-white text-base font-medium text-gray-700 hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-indigo-500 sm:mt-0 sm:ml-3 sm:w-auto sm:text-sm">
            取消
          </button>
        </div>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import {Database} from "../entity/Database";
import {onMounted, withDefaults} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

interface Props {
  db: Database
  enabled?: boolean
}

const props = withDefaults(defineProps<Props>(), {
  enabled: false
})

const emit = defineEmits<{
  (event: "cancel"): void
  (event: "success"): void
}>()

async function cancel() {
  emit("cancel")
}

async function submit() {
  if (props.db.id == undefined) {
    console.log("新建数据库...")
    if (props.db.port == undefined) props.db.port = 6379
    if (props.db.db == undefined) props.db.db = 0

    await invoke('add_db', {db: props.db})
        .then(response => {
          console.log(`新建数据库成功: ${response}`)
          emit("success")
        })
        .catch(error => console.log(error));
  } else {
    await invoke('update_db', {id: props.db.id, db: props.db})
        .then(response => {
          console.log(`修改数据库成功: ${response}`)
          emit("success")
        })
        .catch(error => console.log(error));
  }
}

onMounted(() => {
  console.log("Props: ", props.db)
  console.log(props.enabled)
  console.log("DbDetail Mounted")
})
</script>

<style scoped>

</style>
