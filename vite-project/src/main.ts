import {createApp} from 'vue'
import './style.css'
import App from './App.vue'
import axios, {type AxiosResponse} from "axios";

axios.get('/api/item1.json').then((res: AxiosResponse<{ id: number }>): void => {
    console.log(res.data);
    createApp(App).mount('#app');
});
