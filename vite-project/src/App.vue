<script setup lang="ts">
import HelloWorld from './components/HelloWorld.vue'

import {sock} from "./websocket.ts";
import {ref} from "vue";

const messages = ref<string[]>([]);

const message = ref<string>('');

const handler = (e: any): void => {
    const newMessage = JSON.parse(e.data);

    switch(newMessage.message_type) {
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
form(@submit.prevent="submit")
    input(type="text" v-model="message")
ul
    li(v-for="m in messages") {{ m }}
HelloWorld(msg="Axum + Vite + Vue")
</template>

<style scoped>
ul {
    list-style: none;
}

li {
    text-decoration: underline;
}
</style>
