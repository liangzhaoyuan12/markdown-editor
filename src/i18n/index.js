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

function getSavedLanguage() {
  if (typeof localStorage !== 'undefined') {
    return localStorage.getItem('app-language') || defaultLang
  }
  return defaultLang
}

function setLanguage(lang) {
  if (messages[lang]) {
    if (typeof localStorage !== 'undefined') {
      localStorage.setItem('app-language', lang)
    }
    return lang
  }
  return defaultLang
}

function getCurrentMessages() {
  const lang = getSavedLanguage()
  return messages[lang] || messages[defaultLang]
}

function t(key, defaultValue = '') {
  const messages = getCurrentMessages()
  const keys = key.split('.')
  let result = messages
  
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
  getSavedLanguage,
  setLanguage,
  getCurrentMessages,
  t,
  availableLanguages
}