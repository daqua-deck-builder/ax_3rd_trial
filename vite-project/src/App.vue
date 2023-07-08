<script setup lang="ts">
import CreateUser from "./components/CreateUser.vue";
import axios, {type AxiosResponse} from "axios"
import {sock} from "./websocket.ts";
import {onBeforeMount, ref} from "vue";

const messages = ref<string[]>([]);

const message = ref<string>('');

const handler = (e: any): void => {
    const newMessage = JSON.parse(e.data);

    switch (newMessage.message_type) {
        case 'connected':
            console.log(newMessage.id);
            break;
        default:
            console.log(newMessage)
            break;
    }
}
sock.addEventListener("message", handler);

sock.addEventListener('open', (): void => {
    emit("SetUserId", {user_id: "100"});
});

type User = {
    username: string,
    id: number
};

const users = ref<User[]>([]);

onBeforeMount(() => {
    axios.get('/api/user').then((res: AxiosResponse<{ users: User[] }>): void => {
        users.value = res.data.users;
    });
});

const emit = (message_type: string, payload: Object): Promise<void> => {
    return new Promise<void>((resolve) => {
        sock.send(JSON.stringify(
            [
                {
                    ...payload,
                    message_type
                }
            ]
        ));
        resolve();
    });
}

const submit = () => {
    emit('SendMessage', {text: message.value}).then(() => {
        message.value = '';
    });
}

</script>

<template lang="pug">
CreateUser
h1 USERS
ul.users
    li(v-for="u in users" v-text="u.username")
h1 TODOS
ul
    li(v-for="m in messages") {{ m }}
form(@submit.prevent="submit")
    input(type="text" v-model="message")
</template>

<style scoped>
ul {
    list-style: none;
}

li {
    text-decoration: underline;
}
</style>
