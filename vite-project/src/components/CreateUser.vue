<script setup lang="ts">
import {ref} from "vue";
import axios, {type AxiosResponse} from "axios";

const username = ref('');
const password = ref('');

const submit = (): void => {
    const info = {
        username: username.value,
        password: password.value
    };

    console.log(info)

    axios.post('/api/user/create', info).then((res: AxiosResponse<{ success: boolean }>): void => {
        console.log(res.data.success);
    });
}
</script>

<template lang="pug">
.create_user_form
    h1 CREATE USER
    form(@submit.prevent="submit")
        table
            colgroup
                col(style="width: 200px;")
                col(style="width: 200px;")
            tr
                th
                    label(for="username")
                        span ID
                td
                    input#username(type="text" v-model="username")
            tr
                th
                    label(for="password")
                        span Password
                td
                    input#password(type="password" v-model="password")
            tr
                td
                td
                    input(type="submit" value="Create")

</template>

<style scoped lang="less">

table {
    table-layout: fixed;
    border-collapse: collapse;
}

th, td {
    border: 1px solid grey;
}

input[type="text"], input[type="password"] {
    width: 190px;
}


</style>