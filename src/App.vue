<script setup>
import { ref, shallowRef, watch, onMounted, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import MarkdownEditor from './components/MarkdownEditor.vue'
import Toolbar from './components/Toolbar.vue'

const content = ref('# Welcome to Markdown Editor\n\nStart writing your markdown here...')
const currentFilePath = ref(null)
const isModified = ref(false)
const autoSaveEnabled = ref(false)
const editorRef = shallowRef(null)
let autoSaveTimer = null

// Theme management
const themeMode = ref('system') // 'light', 'dark', 'system'
const isDarkMode = ref(false)

// Load theme preference from localStorage
onMounted(() => {
  const savedTheme = localStorage.getItem('theme-mode')
  if (savedTheme && ['light', 'dark', 'system'].includes(savedTheme)) {
    themeMode.value = savedTheme
  }
  updateTheme()
})

// Watch for theme mode changes
watch(themeMode, () => {
  localStorage.setItem('theme-mode', themeMode.value)
  updateTheme()
})

// Update theme based on mode and system preference
function updateTheme() {
  if (themeMode.value === 'system') {
    const prefersDark = window.matchMedia('(prefers-color-scheme: dark)').matches
    isDarkMode.value = prefersDark
  } else {
    isDarkMode.value = themeMode.value === 'dark'
  }
  
  // Apply theme class to document
  if (isDarkMode.value) {
    document.documentElement.classList.add('dark-theme')
    document.documentElement.classList.remove('light-theme')
  } else {
    document.documentElement.classList.add('light-theme')
    document.documentElement.classList.remove('dark-theme')
  }
}

// Listen for system theme changes
if (typeof window !== 'undefined') {
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    if (themeMode.value === 'system') {
      isDarkMode.value = e.matches
      updateTheme()
    }
  })
}

// Theme toggle handler
function handleThemeChange() {
  const modes = ['light', 'dark', 'system']
  const currentIndex = modes.indexOf(themeMode.value)
  themeMode.value = modes[(currentIndex + 1) % modes.length]
}

// Get theme display text
const themeDisplayText = computed(() => {
  const texts = {
    light: '浅色',
    dark: '深色',
    system: '系统'
  }
  return texts[themeMode.value] || '系统'
})

// File operations
async function handleNew() {
  if (isModified.value) {
    const confirm = window.confirm('当前文件未保存，是否继续？')
    if (!confirm) return
  }
  content.value = '# New Document\n\n'
  currentFilePath.value = null
  isModified.value = false
  autoSaveEnabled.value = false
}

async function handleOpen() {
  try {
    if (isModified.value) {
      const confirm = window.confirm('当前文件未保存，是否继续？')
      if (!confirm) return
    }

    const filePath = await invoke('open_file_dialog')
    if (filePath) {
      const fileContent = await invoke('read_file', { path: filePath })
      content.value = fileContent
      currentFilePath.value = filePath
      isModified.value = false
      // Auto save stays disabled until user explicitly enables it
    }
  } catch (error) {
    alert('打开文件失败: ' + error)
  }
}

async function handleSave() {
  try {
    let filePath = currentFilePath.value
    
    if (!filePath) {
      filePath = await invoke('save_file_dialog', { defaultName: 'document.md' })
      if (!filePath) return
    }

    await invoke('write_file', { 
      path: filePath, 
      content: content.value 
    })
    
    currentFilePath.value = filePath
    isModified.value = false
    showNotification('文件已保存')
    
    // If auto save is enabled and this was a new file, trigger initial auto save
    if (autoSaveEnabled.value) {
      performAutoSave()
    }
  } catch (error) {
    alert('保存文件失败: ' + error)
  }
}

async function handleSaveAs() {
  try {
    const filePath = await invoke('save_file_dialog', { 
      defaultName: getFileName() || 'document.md' 
    })
    
    if (filePath) {
      await invoke('write_file', { 
        path: filePath, 
        content: content.value 
      })
      
      currentFilePath.value = filePath
      isModified.value = false
      showNotification('文件已保存')
    }
  } catch (error) {
    alert('保存文件失败: ' + error)
  }
}

// Helper functions
function getFileName() {
  if (!currentFilePath.value) return null
  const parts = currentFilePath.value.split('/')
  return parts[parts.length - 1]
}

function showNotification(message) {
  const notif = document.createElement('div')
  notif.className = 'notification'
  notif.textContent = message
  document.body.appendChild(notif)
  setTimeout(() => notif.remove(), 2000)
}

// Track content changes
function onContentChange(newContent) {
  content.value = newContent
  isModified.value = true
  
  // Auto save if enabled and file path exists (with debounce)
  if (autoSaveEnabled.value && currentFilePath.value) {
    // Clear previous timer
    if (autoSaveTimer) {
      clearTimeout(autoSaveTimer)
    }
    // Set new timer - save after 500ms of inactivity
    autoSaveTimer = setTimeout(() => {
      performAutoSave()
    }, 500)
  }
}

// Auto save function
async function performAutoSave() {
  if (!currentFilePath.value) return
  
  try {
    await invoke('write_file', { 
      path: currentFilePath.value, 
      content: content.value 
    })
    isModified.value = false
  } catch (error) {
    console.error('自动保存失败:', error)
  }
}

// Handle auto save toggle
function handleAutoSaveToggle() {
  if (!currentFilePath.value) {
    // If no file path, disable auto save and show message
    autoSaveEnabled.value = false
    alert('请先保存文件以启用自动保存功能。\n\n点击"保存"按钮选择保存位置后，即可使用自动保存。')
    return
  }
  
  // If enabling auto save, save immediately
  if (autoSaveEnabled.value) {
    performAutoSave()
  }
}

// Watch autoSaveEnabled changes
watch(autoSaveEnabled, (newValue) => {
  // If enabling auto save and file exists, save immediately
  if (newValue && currentFilePath.value) {
    performAutoSave()
  }
})

// Handle toolbar actions
function handleToolbarAction(action, emoji = null) {
  if (editorRef.value) {
    editorRef.value.handleToolbarAction(action, emoji)
  }
}

// Keyboard shortcuts
function handleKeydown(event) {
  if ((event.ctrlKey || event.metaKey) && event.key === 's') {
    event.preventDefault()
    handleSave()
  } else if ((event.ctrlKey || event.metaKey) && event.key === 'o') {
    event.preventDefault()
    handleOpen()
  } else if ((event.ctrlKey || event.metaKey) && event.key === 'n') {
    event.preventDefault()
    handleNew()
  }
}
</script>

<template>
  <div class="app" @keydown="handleKeydown" tabindex="0">
    <!-- File menu bar -->
    <header class="menu-bar">
      <div class="menu-left">
        <button @click="handleNew" title="新建 (Ctrl+N)" class="menu-btn">
          📄 新建
        </button>
        <button @click="handleOpen" title="打开 (Ctrl+O)" class="menu-btn">
          📂 打开
        </button>
        <button 
          @click="handleSave" 
          :disabled="autoSaveEnabled" 
          :title="autoSaveEnabled ? '自动保存已启用' : '保存 (Ctrl+S)'" 
          class="menu-btn"
          :class="{ 'menu-btn-disabled': autoSaveEnabled }"
        >
          💾 保存
        </button>
        <button @click="handleSaveAs" title="另存为" class="menu-btn">
          另存为
        </button>
      </div>
      <div class="menu-right">
        <label 
          class="autosave-toggle" 
          :class="{ 'autosave-disabled': !currentFilePath }"
          :title="currentFilePath ? '自动保存' : '请先保存文件以启用自动保存'"
        >
          <input 
            type="checkbox" 
            v-model="autoSaveEnabled"
            :disabled="!currentFilePath"
            @change="handleAutoSaveToggle"
          />
          <span class="toggle-slider"></span>
          <span class="toggle-label">自动保存</span>
        </label>
        <button 
          @click="handleThemeChange" 
          class="theme-toggle-btn"
          :title="'切换主题 (当前: ' + themeDisplayText + ')'"
        >
          <svg v-if="themeMode === 'light'" viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 7c-2.76 0-5 2.24-5 5s2.24 5 5 5 5-2.24 5-5-2.24-5-5-5zM2 13h2c.55 0 1-.45 1-1s-.45-1-1-1H2c-.55 0-1 .45-1 1s.45 1 1 1zm18 0h2c.55 0 1-.45 1-1s-.45-1-1-1h-2c-.55 0-1 .45-1 1s.45 1 1 1zM11 2v2c0 .55.45 1 1 1s1-.45 1-1V2c0-.55-.45-1-1-1s-1 .45-1 1zm0 18v2c0 .55.45 1 1 1s1-.45 1-1v-2c0-.55-.45-1-1-1s-1 .45-1 1zM5.99 4.58a.996.996 0 00-1.41 0 .996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0s.39-1.03 0-1.41L5.99 4.58zm12.37 12.37a.996.996 0 00-1.41 0 .996.996 0 000 1.41l1.06 1.06c.39.39 1.03.39 1.41 0a.996.996 0 000-1.41l-1.06-1.06zm1.06-10.96a.996.996 0 000-1.41.996.996 0 00-1.41 0l-1.06 1.06c-.39.39-.39 1.03 0 1.41s1.03.39 1.41 0l1.06-1.06zM7.05 18.36a.996.996 0 000 1.41.996.996 0 001.41 0l1.06-1.06c.39-.39.39-1.03 0-1.41s-1.03-.39-1.41 0l-1.06 1.06z"/></svg>
          <svg v-else-if="themeMode === 'dark'" viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 3a9 9 0 109 9c0-.46-.04-.92-.1-1.36a5.389 5.389 0 01-4.4 2.26 5.403 5.403 0 01-3.14-9.8c-.44-.06-.9-.1-1.36-.1z"/></svg>
          <svg v-else viewBox="0 0 24 24" width="18" height="18"><path fill="currentColor" d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-2 15l-5-5 1.41-1.41L10 14.17l7.59-7.59L19 8l-9 9z"/></svg>
          <span class="theme-text">{{ themeDisplayText }}</span>
        </button>
        <span v-if="currentFilePath" class="file-path">{{ getFileName() }}</span>
        <span v-else class="file-path">未命名文档</span>
        <span v-if="isModified" class="modified-indicator">*</span>
      </div>
    </header>

    <!-- Formatting toolbar -->
    <Toolbar @action="handleToolbarAction" />

    <!-- Editor -->
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
/* CSS Variables for theming */
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

/* Light theme (explicit) */
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

/* Dark theme */
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

/* Menu bar styles */
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

/* Theme toggle button */
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

/* Auto save toggle switch */
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

/* Editor wrapper */
.editor-wrapper {
  flex: 1;
  overflow: hidden;
}

/* Notification */
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

