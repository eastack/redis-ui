import {createApp} from 'vue'
import App from './App.vue'
import CreateDb from './components/CreateDb.vue'
import DbList from './components/DbList.vue'
import EditDb from './components/EditDb.vue'
import Dashboard from './components/Dashboard.vue'
import './index.css'
import {createRouter, createWebHashHistory, RouteParams, RouteRecordRaw} from "vue-router";

export type AppRouteNames =
    | 'index'
    | 'create-db'
    | 'edit-db'
    | 'dashboard'

export const routes: RouteRecordRaw[] = [
    {
        name: 'index',
        path: '/',
        component: DbList
    },
    {
        name: 'create-db',
        path: '/database/create',
        component: CreateDb
    },
    {
        name: 'edit-db',
        path: '/database/:id/edit',
        component: EditDb,
    },
    {
        name: 'dashboard',
        path: '/database/:id/dashboard',
        component: Dashboard,
    },
];

let router = createRouter({
    history: createWebHashHistory(),
    routes: routes
})

interface Todo {
    title: string;
    description: string;
    completed: boolean;
}

type TodoPreview = Pick<Todo, "title" | "description">;

const todo: TodoPreview = {
    title: "Clean room",
    description: "false",
};


export function routerPush(name: AppRouteNames, params?: RouteParams): ReturnType<typeof router.push> {
    if (params !== undefined) {
        return router.push({name, params})
    } else {
        return router.push({name})
    }
}

createApp(App)
    .use(router)
    .mount('#app')
