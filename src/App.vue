<script setup>import { ref, shallowRef, watch, onMounted, computed } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import MarkdownEditor from './components/MarkdownEditor.vue';
import Toolbar from './components/Toolbar.vue';
import { t, getSavedLanguage, setLanguage, getCurrentMessages, availableLanguages } from './i18n/index.js';
const content = ref('# Welcome to Markdown Editor\n\nStart writing your markdown here...');
const currentFilePath = ref(null);
const isModified = ref(false);
const autoSaveEnabled = ref(false);
const editorRef = shallowRef(null);
let autoSaveTimer = null;
const themeMode = ref('system');
const isDarkMode = ref(false);
const currentLanguage = ref(getSavedLanguage());
onMounted(() => {
 const savedTheme = localStorage.getItem('theme-mode');
 if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
 themeMode.value = savedTheme;
 }
 updateTheme();
 
 // 监听从文件管理器打开文件的事件
 listen('open-file', (event) => {
   const filePath = event.payload;
   console.log('Opening file:', filePath);
   openFileFromPath(filePath);
 });
});
watch(themeMode, () => {
 localStorage.setItem('theme-mode', themeMode.value);
 updateTheme();
});
function updateTheme() {
 if (themeMode.value === 'system') {
 const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches;
 isDarkMode.value = prefersDark;
 }
 else {
 isDarkMode.value = themeMode.value === 'dark';
 }
 if (isDarkMode.value) {
 document.documentElement.classList.add('dark-theme');
 document.documentElement.classList.remove('light-theme');
 }
 else {
 document.documentElement.classList.add('light-theme');
 document.documentElement.classList.remove('dark-theme');
 }
}
if (typeof window !== 'undefined') {
 window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
 if (themeMode.value === 'system') {
 isDarkMode.value = e.matches;
 updateTheme();
 }
 });
}
function handleThemeChange() {
 const modes = ['light', 'dark', 'system'];
 const currentIndex = modes.indexOf(themeMode.value);
 themeMode.value = modes[(currentIndex + 1) % modes.length];
}
const themeDisplayText = computed(() => {
 const texts = {
 light: t('common.light'),
 dark: t('common.dark'),
 system: t('common.system')
 };
 return texts[themeMode.value] || 'System';
});
function handleLanguageChange(lang) {
 currentLanguage.value = lang;
 setLanguage(lang);
 location.reload();
}
const currentLangText = computed(() => {
 const messages = getCurrentMessages();
 return messages.languages[currentLanguage.value] || currentLanguage.value;
});
async function handleNew() {
 if (isModified.value) {
 const confirm = window.confirm(t('common.confirmClose'));
 if (!confirm)
 return;
 }
 content.value = '# New Document\n\n';
 currentFilePath.value = null;
 isModified.value = false;
 autoSaveEnabled.value = false;
}
async function handleOpen() {
 try {
 if (isModified.value) {
 const confirm = window.confirm(t('common.confirmClose'));
 if (!confirm)
 return;
 }
 const filePath = await invoke('open_file_dialog');
 if (filePath) {
 await openFileFromPath(filePath);
 }
 }
 catch (error) {
 alert('打开文件失败: ' + error);
 }
}

async function openFileFromPath(filePath) {
 try {
 if (isModified.value) {
 const confirm = window.confirm(t('common.confirmClose'));
 if (!confirm)
 return;
 }
 const fileContent = await invoke('read_file', { path: filePath });
 content.value = fileContent;
 currentFilePath.value = filePath;
 isModified.value = false;
 autoSaveEnabled.value = false; // 重置自动保存状态
 }
 catch (error) {
 alert('打开文件失败: ' + error);
 }
}
async function handleSave() {
 try {
 let filePath = currentFilePath.value;
 if (!filePath) {
 filePath = await invoke('save_file_dialog', { defaultName: 'document.md' });
 if (!filePath)
 return;
 }
 await invoke('write_file', {
 path: filePath,
 content: content.value
 });
 currentFilePath.value = filePath;
 isModified.value = false;
 showNotification(t('common.fileSaved'));
 if (autoSaveEnabled.value) {
 performAutoSave();
 }
 }
 catch (error) {
 alert('保存文件失败: ' + error);
 }
}
async function handleSaveAs() {
 try {
 const filePath = await invoke('save_file_dialog', {
 defaultName: getFileName() || 'document.md'
 });
 if (filePath) {
 await invoke('write_file', {
 path: filePath,
 content: content.value
 });
 currentFilePath.value = filePath;
 isModified.value = false;
 showNotification(t('common.fileSaved'));
 }
 }
 catch (error) {
 alert('保存文件失败: ' + error);
 }
}
function getFileName() {
 if (!currentFilePath.value)
 return null;
 const parts = currentFilePath.value.split('/');
 return parts[parts.length - 1];
}
function showNotification(message) {
 const notif = document.createElement('div');
 notif.className = 'notification';
 notif.textContent = message;
 document.body.appendChild(notif);
 setTimeout(() => notif.remove(), 2000);
}
function onContentChange(newContent) {
 content.value = newContent;
 isModified.value = true;
 if (autoSaveEnabled.value && currentFilePath.value) {
 if (autoSaveTimer) {
 clearTimeout(autoSaveTimer);
 }
 autoSaveTimer = setTimeout(() => {
 performAutoSave();
 }, 500);
 }
}
async function performAutoSave() {
 if (!currentFilePath.value)
 return;
 try {
 await invoke('write_file', {
 path: currentFilePath.value,
 content: content.value
 });
 isModified.value = false;
 }
 catch (error) {
 console.error('自动保存失败:', error);
 }
}
function handleAutoSaveToggle() {
 if (!currentFilePath.value) {
 autoSaveEnabled.value = false;
 alert(t('common.pleaseSaveFirst'));
 return;
 }
 if (autoSaveEnabled.value) {
 performAutoSave();
 }
}
watch(autoSaveEnabled, (newValue) => {
 if (newValue && currentFilePath.value) {
 performAutoSave();
 }
});
function handleToolbarAction(action, emoji = null) {
 if (editorRef.value) {
 editorRef.value.handleToolbarAction(action, emoji);
 }
}
function handleKeydown(event) {
 if ((event.ctrlKey || event.metaKey) && event.key === 's') {
 event.preventDefault();
 handleSave();
 }
 else if ((event.ctrlKey || event.metaKey) && event.key === 'o') {
 event.preventDefault();
 handleOpen();
 }
 else if ((event.ctrlKey || event.metaKey) && event.key === 'n') {
 event.preventDefault();
 handleNew();
 }
}
</script>

<template>
  <div class="app" @keydown="handleKeydown" tabindex="0">
    <header class="menu-bar">
      <div class="menu-left">
        <button @click="handleNew" :title="t('common.new') + ' (Ctrl+N)'" class="menu-btn">
          📄 {{ t('common.new') }}
        </button>
        <button @click="handleOpen" :title="t('common.open') + ' (Ctrl+O)'" class="menu-btn">
          📂 {{ t('common.open') }}
        </button>
        <button 
          @click="handleSave" 
          :disabled="autoSaveEnabled" 
          :title="autoSaveEnabled ? t('common.autoSave') + ' ' + t('common.enabled') : t('common.save') + ' (Ctrl+S)'" 
          class="menu-btn"
          :class="{ 'menu-btn-disabled': autoSaveEnabled }"
        >
          💾 {{ t('common.save') }}
        </button>
        <button @click="handleSaveAs" :title="t('common.saveAs')" class="menu-btn">
          {{ t('common.saveAs') }}
        </button>
      </div>
      <div class="menu-right">
        <label 
          class="autosave-toggle" 
          :class="{ 'autosave-disabled': !currentFilePath }"
          :title="currentFilePath ? t('common.autoSave') : t('common.pleaseSaveFirst')"
        >
          <input 
            type="checkbox" 
            v-model="autoSaveEnabled"
            :disabled="!currentFilePath"
            @change="handleAutoSaveToggle"
          />
          <span class="toggle-slider"></span>
          <span class="toggle-label">{{ t('common.autoSave') }}</span>
        </label>
        <button 
          @click="handleThemeChange" 
          class="theme-toggle-btn"
          :title="t('common.theme') + ' (' + themeDisplayText + ')'"
        >
          <svg v-if="themeMode === 'light'" viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58a.996.996 0 00-1.41 0 .996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37a.996.996 0 00-1.41 0 .996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0a.996.996 0 000-1.41l-1.06-1.06zm1.06-10.96a.996.996 0 000-1.41.996.996 0 00-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06zM7.05 18.36a.996.996 0 000 1.41.996.996 0 001.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41s-1.03-.39-1.41 0l-1.06 1.06z"/></svg>
          <svg v-else-if="themeMode === 'dark'" viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 3a9 9 0 109 9c0-.46-.04-.92-.1-1.36a5.389 5.389 0 01-4.4 2.26 5.403 5.403 0 01-3.14-9.8c-.44-.06-.9-.1-1.36-.1z"/></svg>
          <svg v-else viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
          <span class="theme-text">{{ themeDisplayText }}</span>
        </button>
        <select 
          v-model="currentLanguage" 
          @change="(e) => handleLanguageChange(e.target.value)"
          class="language-select"
          :title="t('common.language')"
        >
          <option v-for="lang in availableLanguages" :key="lang" :value="lang">
            {{ getCurrentMessages().languages[lang] }}
          </option>
        </select>
        <span v-if="currentFilePath" class="file-path">{{ getFileName() }}</span>
        <span v-else class="file-path">{{ t('common.untitled') }}</span>
        <span v-if="isModified" class="modified-indicator">*</span>
      </div>
    </header>
    <Toolbar @action="handleToolbarAction" />
    <main class="editor-wrapper">
      <MarkdownEditor 
        ref="editorRef"
        v-model="content" 
        @update:modelValue="onContentChange"
      />
    </main>
  </div>
</template>

<style>
:root {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --bg-tertiary: #fafafa;
  --border-color: #ddd;
  --text-primary: #333;
  --text-secondary: #666;
  --text-muted: #999;
  --hover-bg: #e8eaed;
  --active-bg: #d0d3d8;
  --accent-color: #4caf50;
  --modified-color: #ff6b6b;
  --notification-bg: #4caf50;
  --notification-text: #ffffff;
}

.light-theme {
  --bg-primary: #ffffff;
  --bg-secondary: #f5f5f5;
  --bg-tertiary: #fafafa;
  --border-color: #ddd;
  --text-primary: #333;
  --text-secondary: #666;
  --text-muted: #999;
  --hover-bg: #e8eaed;
  --active-bg: #d0d3d8;
  --accent-color: #4caf50;
  --modified-color: #ff6b6b;
  --notification-bg: #4caf50;
  --notification-text: #ffffff;
}

.dark-theme {
  --bg-primary: #1e1e1e;
  --bg-secondary: #252525;
  --bg-tertiary: #2d2d2d;
  --border-color: #444;
  --text-primary: #d4d4d4;
  --text-secondary: #aaa;
  --text-muted: #777;
  --hover-bg: #3d3d3d;
  --active-bg: #4d4d4d;
  --accent-color: #4caf50;
  --modified-color: #ff6b6b;
  --notification-bg: #4caf50;
  --notification-text: #ffffff;
}

* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
  overflow: hidden;
}

.app {
  width: 100vw;
  height: 100vh;
  display: flex;
  flex-direction: column;
}

.menu-bar {
  height: 44px;
  background: linear-gradient(to bottom, var(--bg-primary), var(--bg-secondary));
  border-bottom: 1px solid var(--border-color);
  display: flex;
  align-items: center;
  justify-content: space-between;
  padding: 0 12px;
}

.menu-left {
  display: flex;
  gap: 4px;
}

.menu-btn {
  padding: 6px 12px;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
  transition: background-color 0.15s;
  color: var(--text-primary);
}

.menu-btn:hover {
  background: var(--hover-bg);
}

.menu-btn:active {
  background: var(--active-bg);
}

.menu-btn-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.menu-btn-disabled:hover {
  background: transparent;
}

.menu-right {
  display: flex;
  align-items: center;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 13px;
}

.language-select {
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  background: var(--bg-primary);
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-primary);
  font-size: 12px;
  transition: background-color 0.15s;
}

.language-select:hover {
  background: var(--hover-bg);
}

.language-select:focus {
  outline: none;
  border-color: var(--accent-color);
}

.theme-toggle-btn {
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 4px 8px;
  border: none;
  background: transparent;
  border-radius: 4px;
  cursor: pointer;
  color: var(--text-secondary);
  font-size: 12px;
  transition: background-color 0.15s;
}

.theme-toggle-btn:hover {
  background: var(--hover-bg);
}

.theme-toggle-btn:active {
  background: var(--active-bg);
}

.theme-text {
  margin-left: 2px;
}

.autosave-toggle {
  display: flex;
  align-items: center;
  gap: 6px;
  cursor: pointer;
  user-select: none;
  margin-right: 8px;
}

.autosave-toggle.autosave-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.autosave-toggle input[type="checkbox"] {
  display: none;
}

.autosave-toggle input[type="checkbox"]:disabled + .toggle-slider {
  background-color: var(--border-color);
  cursor: not-allowed;
}

.toggle-slider {
  position: relative;
  width: 36px;
  height: 18px;
  background-color: var(--border-color);
  border-radius: 9px;
  transition: background-color 0.2s;
}

.toggle-slider::before {
  content: '';
  position: absolute;
  width: 14px;
  height: 14px;
  left: 2px;
  top: 2px;
  background-color: white;
  border-radius: 50%;
  transition: transform 0.2s;
}

.autosave-toggle input[type="checkbox"]:checked + .toggle-slider {
  background-color: var(--accent-color);
}

.autosave-toggle input[type="checkbox"]:checked + .toggle-slider::before {
  transform: translateX(18px);
}

.toggle-label {
  font-size: 13px;
  color: var(--text-secondary);
}

.file-path {
  max-width: 300px;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.modified-indicator {
  color: var(--modified-color);
  font-weight: bold;
  font-size: 16px;
}

.editor-wrapper {
  flex: 1;
  overflow: hidden;
}

.notification {
  position: fixed;
  top: 80px;
  right: 20px;
  background: var(--notification-bg);
  color: var(--notification-text);
  padding: 12px 24px;
  border-radius: 4px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.2);
  animation: slideIn 0.3s ease-out;
  z-index: 9999;
}

@keyframes slideIn {
  from {
    transform: translateX(400px);
    opacity: 0;
  }
  to {
    transform: translateX(0);
    opacity: 1;
  }
}
</style>