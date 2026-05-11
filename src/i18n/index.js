import { ref } from 'vue'
import zhCN from './zh-CN.js'
import zhTW from './zh-TW.js'
import en from './en.js'
import ru from './ru.js'
import ja from './ja.js'
import ko from './ko.js'

const messages = {
  'zh-CN': zhCN,
  'zh-TW': zhTW,
  'en': en,
  'ru': ru,
  'ja': ja,
  'ko': ko
}

const defaultLang = 'zh-CN'

function resolveLanguage(lang) {
  if (lang === 'system') {
    const sysLang = (typeof navigator !== 'undefined' && navigator.language) || ''
    const base = sysLang.split('-').slice(0, 2).join('-')
    if (messages[base]) return base
    const primary = sysLang.split('-')[0]
    const match = Object.keys(messages).find(k => k.startsWith(primary))
    if (match) return match
    if (messages['en']) return 'en'
    return defaultLang
  }
  return lang
}

const currentLang = ref(initLang())

function initLang() {
  const saved = getSavedLanguage()
  return resolveLanguage(saved)
}

function getSavedLanguage() {
  if (typeof localStorage !== 'undefined') {
    return localStorage.getItem('app-language') || 'system'
  }
  return 'system'
}

function setLanguage(lang) {
  if (lang === 'system' || messages[lang]) {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('app-language', lang)
    }
    currentLang.value = resolveLanguage(lang)
    return lang
  }
  return 'system'
}

function getCurrentMessages() {
  return messages[currentLang.value] || messages[defaultLang]
}

function t(key, defaultValue = '') {
  const msgs = getCurrentMessages()
  const keys = key.split('.')
  let result = msgs
  
  for (const k of keys) {
    if (result && typeof result === 'object' && k in result) {
      result = result[k]
    } else {
      return defaultValue || key
    }
  }
  
  return result || defaultValue || key
}

const availableLanguages = Object.keys(messages)

export {
  messages,
  defaultLang,
  resolveLanguage,
  getSavedLanguage,
  setLanguage,
  getCurrentMessages,
  t,
  availableLanguages
}