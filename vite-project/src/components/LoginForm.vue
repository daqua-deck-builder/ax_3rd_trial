<template lang="pug">
form.login_form(@submit.prevent="submit")
    h1 ログインテスト
    label
        span ログインID:
        input(type="text" v-model="username")
    br
    label
        span パスワード:
        input(type="password" v-model="password")
    br
    input(type="submit" value="Login")


</template>

<script setup lang="ts">

import {ref} from "vue";
import axios, {type AxiosResponse} from "axios";

const username = ref("");
const password = ref("");

const prevent_submit = ref(false);
const submit = () => {
    if (prevent_submit.value) {
        return;
    }
    prevent_submit.value = true;
    axios.post('/api/user/login', {username: username.value, password: password.value}).then((res: AxiosResponse<{
        success: boolean
    }>) => {
        if (res.data.success) {
            alert('認証成功');
            username.value = '';
            password.value = '';
        } else {
            alert('認証失敗');
        }
        prevent_submit.value = false;
    });
}
</script>

<style scoped lang="less">
.login_form {
    background-color: pink;
}
</style>