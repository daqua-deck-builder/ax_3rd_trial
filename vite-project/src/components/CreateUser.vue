<script setup lang="ts">
import {ref} from "vue";
import axios, {AxiosError, type AxiosResponse} from "axios";

const username = ref('');
const password = ref('');

const ERRORS: Record<string, string> = {
    'unknown error': "不明なエラーです。",
    'invalid letter': "ユーザー名に不正な文字が含まれています。"
}

const submit = (): void => {
    const info = {
        username: username.value,
        password: password.value
    };

    axios.post('/api/user/create', info).then((res: AxiosResponse<{ success: boolean, reason?: string[] }>): void => {
        if (res.data.success) {
            alert('作成しました');
        } else {
            alert((res.data.reason || ['unknown error']).map(r => ERRORS[r] || '不明なエラーです'));
        }
    }).catch((error: AxiosError<{ success: boolean, reason?: string[] }>) => {
        if (error.response && error.response.status === 500) {
            alert((error.response.data.reason || ['unknown error']).map(r => ERRORS[r] || '不明なエラーです'));
        } else {
            console.error(error);
        }
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