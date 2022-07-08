<script setup lang="ts">
import {useRoute, useRouter} from "vue-router";
import {onMounted, ref} from "vue";
import {invoke} from "@tauri-apps/api/tauri";

async function onSubmit() {
  console.log("New DB")
  return await invoke('add_db', {
    'db': {
      name: name.value,
      port: port.value,
      host: host.value,
      password: password.value,
      comment: comment.value,
      color: '',
      auth: 'None',
      db: 0,
    }
  })
      .then(response => {
        console.log("new ok")
        gotoDbList()
      })
      .catch(error => console.log(error));
}

const name = ref<string>()
const port = ref<number>()
const host = ref<string>()
const password = ref<string>()
const comment = ref<string>()

const route = useRoute();
const router = useRouter()

function gotoDbList() {
  router.push("/db-list")
}

onMounted(() => {
  console.log(route.params.id)
})
</script>

<template>
  <form @submit.prevent="onSubmit">
    <input type="text" name="name" placeholder="Name" v-model="name"><br>
    <input type="text" name="post" placeholder="Port" v-model.number="port"><br>
    <input type="text" name="host" placeholder="Host" v-model="host"><br>
    <input type="text" name="post" placeholder="Port" v-model.number="password"><br>
    <input type="text" name="comment" placeholder="Comment" v-model="comment"><br>
    <input type="password" name="password" placeholder="Password"><br>
    <button type="submit">确定</button>
    <button @click="gotoDbList">取消</button>
  </form>
</template>


<style scoped>

</style>