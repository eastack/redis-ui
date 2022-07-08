import {createApp} from 'vue'
import App from './App.vue'
import NewDb from './components/NewDb.vue'
import DbList from './components/DbList.vue'
import EditDb from './components/EditDb.vue'
import Test from './components/Test.vue'
import './index.css'
import {createRouter, createWebHistory} from "vue-router";

const routes: any[] = [
    {path: '/', component: DbList},
    {path: '/new-db', component: NewDb},
    {path: '/db-list', component: DbList},
    {path: '/edit-db/:id', component: EditDb},
    {path: '/test', component: Test},
]

let router = createRouter({
    history: createWebHistory(),
    routes: routes
})

createApp(App)
    .use(router)
    .mount('#app')
