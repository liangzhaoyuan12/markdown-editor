import { createApp } from "vue";
import App from "./App.vue";
import { getSavedLanguage, setLanguage } from './i18n/index.js'

const savedLang = getSavedLanguage()
setLanguage(savedLang)

createApp(App).mount("#app");