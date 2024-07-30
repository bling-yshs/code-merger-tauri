import './assets/main.css'

import { createApp } from 'vue'
import ElementPlus from 'element-plus'
import 'element-plus/dist/index.css'
// 黑暗模式
import 'element-plus/theme-chalk/dark/css-vars.css'
import App from './App.vue'
import router from './router'
import 'virtual:uno.css'
const app = createApp(App)
app.use(router)
app.use(ElementPlus)
app.mount('#app')
