<template>
  <div class="editor-wrapper">
    <div class="editor-pane" :class="{ 'search-visible': showSearch }">
      <div v-if="showSearch" class="search-bar">
        <input 
          ref="searchInputRef"
          v-model="searchQuery" 
          @input="performSearch"
          @keydown="handleSearchKeydown"
          :placeholder="t('search.placeholder')" 
          class="search-input"
        />
        <span class="search-count">{{ searchCountText }}</span>
        <button @click="prevMatch" :disabled="searchResults.length === 0" :title="t('search.prev')" class="search-btn">▲</button>
        <button @click="nextMatch" :disabled="searchResults.length === 0" :title="t('search.next')" class="search-btn">▼</button>
        <button @click="toggleSearch" :title="t('search.close')" class="search-btn search-close">✕</button>
      </div>
      <div class="editor-content">
        <pre
          v-if="showSearch && searchQuery"
          ref="highlightRef"
          class="highlight-layer"
          v-html="highlightedHtml"
          aria-hidden="true"
        ></pre>
        <textarea
          ref="textareaRef"
          v-model="content"
          class="editor-textarea"
          :class="{ 'searching': showSearch && searchQuery && searchResults.length > 0 }"
          @input="onInput"
          @scroll="onEditorScroll"
        ></textarea>
      </div>
    </div>
    <div class="preview-pane">
      <div
        ref="previewRef"
        class="preview-content markdown-body"
        v-html="previewHtml"
        @scroll="onPreviewScroll"
      ></div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch, onMounted, onUnmounted, nextTick, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { t } from '../i18n/index.js'

const props = defineProps({
  modelValue: {
    type: String,
    default: ''
  }
})

const emit = defineEmits(['update:modelValue'])

const textareaRef = ref(null)
const previewRef = ref(null)
const searchInputRef = ref(null)
const highlightRef = ref(null)
const content = ref(props.modelValue)
let isSyncing = false

// 搜索状态
const showSearch = ref(false)
const searchQuery = ref('')
const searchResults = ref([])
const currentResultIndex = ref(-1)

// 历史记录管理
const history = ref([props.modelValue])
const historyIndex = ref(0)
const maxHistorySize = 50

const previewHtml = ref('')
let renderTimer = null
const RENDER_DEBOUNCE_MS = 120

async function renderMarkdown() {
  const md = content.value
  if (!md) {
    previewHtml.value = ''
    return
  }
  try {
    previewHtml.value = await invoke('markdown_to_html', { markdown: md })
  } catch (e) {
    console.error('Markdown render failed:', e)
  }
}

function scheduleRender() {
  if (renderTimer) clearTimeout(renderTimer)
  renderTimer = setTimeout(renderMarkdown, RENDER_DEBOUNCE_MS)
}

watch(content, () => {
  scheduleRender()
  if (showSearch.value && searchQuery.value) {
    performSearch()
  }
})

function onInput() {
  emit('update:modelValue', content.value)
  
  // 添加到历史记录
  addToHistory(content.value)
}

function addToHistory(newContent) {
  // 如果当前不在历史记录的末尾，删除后面的记录
  if (historyIndex.value < history.value.length - 1) {
    history.value = history.value.slice(0, historyIndex.value + 1)
  }
  
  // 添加新内容
  history.value.push(newContent)
  historyIndex.value = history.value.length - 1
  
  // 限制历史记录大小
  if (history.value.length > maxHistorySize) {
    history.value.shift()
    historyIndex.value--
  }
}

function undo() {
  if (historyIndex.value > 0) {
    historyIndex.value--
    content.value = history.value[historyIndex.value]
    emit('update:modelValue', content.value)
  }
}

function redo() {
  if (historyIndex.value < history.value.length - 1) {
    historyIndex.value++
    content.value = history.value[historyIndex.value]
    emit('update:modelValue', content.value)
  }
}

function insertText(before, after = '') {
  const textarea = textareaRef.value
  if (!textarea) return
  
  const start = textarea.selectionStart
  const end = textarea.selectionEnd
  const selectedText = content.value.substring(start, end)
  
  const newText = before + selectedText + after
  content.value = content.value.substring(0, start) + newText + content.value.substring(end)
  
  emit('update:modelValue', content.value)
  
  setTimeout(() => {
    textarea.focus()
    if (selectedText) {
      textarea.setSelectionRange(start + before.length, start + before.length + selectedText.length)
    } else {
      textarea.setSelectionRange(start + before.length, start + before.length)
    }
  }, 0)
}

function insertLinePrefix(prefix) {
  const textarea = textareaRef.value
  if (!textarea) return
  
  const start = textarea.selectionStart
  const text = content.value
  const lineStart = text.lastIndexOf('\n', start - 1) + 1
  
  content.value = text.substring(0, lineStart) + prefix + text.substring(lineStart)
  emit('update:modelValue', content.value)
  
  setTimeout(() => {
    textarea.focus()
    textarea.setSelectionRange(start + prefix.length, start + prefix.length)
  }, 0)
}

function generateTable(rows, cols) {
  let table = '\n'
  
  // 生成表头
  for (let i = 0; i < cols; i++) {
    table += `| Column ${i + 1} `
  }
  table += '|\n'
  
  // 生成分隔线
  for (let i = 0; i < cols; i++) {
    table += '|---------- '
  }
  table += '|\n'
  
  // 生成数据行
  for (let row = 0; row < rows; row++) {
    for (let col = 0; col < cols; col++) {
      table += `| Cell ${row * cols + col + 1}   `
    }
    table += '|\n'
  }
  
  return table
}

function handleToolbarAction(action, emoji = null) {
  const textarea = textareaRef.value
  if (!textarea) {
    textareaRef.value?.focus()
    return
  }
  textarea.focus()
  
  switch (action) {
    case 'undo':
      undo()
      break
    case 'redo':
      redo()
      break
    case 'bold':
      insertText('**', '**')
      break
    case 'italic':
      insertText('*', '*')
      break
    case 'strikethrough':
      insertText('~~', '~~')
      break
    case 'quote':
      insertLinePrefix('> ')
      break
    case 'h1':
      insertLinePrefix('# ')
      break
    case 'h2':
      insertLinePrefix('## ')
      break
    case 'h3':
      insertLinePrefix('### ')
      break
    case 'h4':
      insertLinePrefix('#### ')
      break
    case 'h5':
      insertLinePrefix('##### ')
      break
    case 'h6':
      insertLinePrefix('###### ')
      break
    case 'ul':
      insertLinePrefix('- ')
      break
    case 'ol':
      insertLinePrefix('1. ')
      break
    case 'hr':
      insertText('\n\n---\n\n')
      break
    case 'link':
      insertText('[', '](url)')
      break
    case 'image':
      insertText('![', '](url)')
      break
    case 'code':
      insertText('`', '`')
      break
    case 'codeblock':
      insertText('\n```\n', '\n```\n')
      break
    case 'table-insert':
      // 从弹窗接收行数和列数
      if (emoji && emoji.rows && emoji.cols) {
        const tableText = generateTable(emoji.rows, emoji.cols)
        insertText(tableText)
      }
      break
    case 'datetime':
      insertText(new Date().toLocaleString())
      break
    case 'emoji':
      insertText('😀')
      break
    case 'emoji-insert':
      if (emoji) {
        insertText(emoji)
      }
      break
    case 'clear':
      if (confirm(t('common.confirmClear'))) {
        content.value = ''
        emit('update:modelValue', '')
      }
      break
    case 'search':
      toggleSearch()
      break
  }
}

const searchCountText = computed(() => {
  if (!searchQuery.value) return ''
  if (searchResults.value.length === 0) return t('search.noResults')
  if (currentResultIndex.value < 0) return `${searchResults.value.length}`
  return `${currentResultIndex.value + 1}/${searchResults.value.length}`
})

function escapeHtml(text) {
  return text
    .replace(/&/g, '&amp;')
    .replace(/</g, '&lt;')
    .replace(/>/g, '&gt;')
    .replace(/"/g, '&quot;')
}

const highlightedHtml = computed(() => {
  const text = content.value
  if (!text) return ''
  const results = searchResults.value
  if (!searchQuery.value || results.length === 0) return escapeHtml(text)

  let html = ''
  let lastEnd = 0
  const currentIdx = currentResultIndex.value
  for (let i = 0; i < results.length; i++) {
    const match = results[i]
    html += escapeHtml(text.slice(lastEnd, match.start))
    html += '<mark' + (i === currentIdx ? ' class="current"' : '') + '>' + escapeHtml(text.slice(match.start, match.end)) + '</mark>'
    lastEnd = match.end
  }
  html += escapeHtml(text.slice(lastEnd))
  return html
})

function toggleSearch() {
  showSearch.value = !showSearch.value
  if (showSearch.value) {
    nextTick(() => {
      searchInputRef.value?.focus()
    })
  } else {
    searchQuery.value = ''
    searchResults.value = []
    currentResultIndex.value = -1
  }
}

function performSearch() {
  const query = searchQuery.value
  const text = content.value
  searchResults.value = []
  currentResultIndex.value = -1

  if (!query || !text) return

  let start = 0
  while ((start = text.indexOf(query, start)) !== -1) {
    searchResults.value.push({ start, end: start + query.length })
    start += query.length
  }
}

function selectMatch(index) {
  const textarea = textareaRef.value
  if (!textarea || index < 0 || index >= searchResults.value.length) return

  const match = searchResults.value[index]
  currentResultIndex.value = index
  textarea.focus()
  textarea.setSelectionRange(match.start, match.end)
}

function nextMatch() {
  if (searchResults.value.length === 0) return
  const next = currentResultIndex.value < 0 ? 0 : (currentResultIndex.value + 1) % searchResults.value.length
  selectMatch(next)
}

function prevMatch() {
  if (searchResults.value.length === 0) return
  const prev = currentResultIndex.value < 0 ? searchResults.value.length - 1
    : (currentResultIndex.value - 1 + searchResults.value.length) % searchResults.value.length
  selectMatch(prev)
}

function handleSearchKeydown(event) {
  if (event.key === 'Enter') {
    event.preventDefault()
    if (event.shiftKey) {
      prevMatch()
    } else {
      nextMatch()
    }
  } else if (event.key === 'Escape') {
    event.preventDefault()
    toggleSearch()
  }
}

defineExpose({
  handleToolbarAction,
  toggleSearch,
  getMarkdown: () => content.value,
  setMarkdown: (text) => { content.value = text }
})

function onEditorScroll(e) {
  if (isSyncing || !previewRef.value) return
  isSyncing = true
  
  const textarea = e.target
  const preview = previewRef.value
  const scrollRatio = textarea.scrollTop / (textarea.scrollHeight - textarea.clientHeight)
  preview.scrollTop = scrollRatio * (preview.scrollHeight - preview.clientHeight)
  
  if (highlightRef.value) {
    highlightRef.value.scrollTop = textarea.scrollTop
  }
  
  setTimeout(() => { isSyncing = false }, 50)
}

function onPreviewScroll(e) {
  if (isSyncing || !textareaRef.value) return
  isSyncing = true
  
  const preview = e.target
  const textarea = textareaRef.value
  const scrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight)
  textarea.scrollTop = scrollRatio * (textarea.scrollHeight - textarea.clientHeight)
  
  if (highlightRef.value) {
    highlightRef.value.scrollTop = textarea.scrollTop
  }
  
  setTimeout(() => { isSyncing = false }, 50)
}

watch(() => props.modelValue, (newValue) => {
  if (content.value !== newValue) {
    content.value = newValue
    // 外部内容变更（如打开文件）时重置历史记录
    history.value = [newValue]
    historyIndex.value = 0
  }
})

// 键盘事件处理
function handleKeydown(event) {
  // 检查是否在 textarea 中
  if (event.target !== textareaRef.value) return
  
  // Ctrl/Cmd + Z: 撤销
  if ((event.ctrlKey || event.metaKey) && event.key === 'z' && !event.shiftKey) {
    event.preventDefault()
    undo()
  }
  // Ctrl/Cmd + Y 或 Ctrl/Cmd + Shift + Z: 重做
  else if ((event.ctrlKey || event.metaKey) && (event.key === 'y' || (event.key === 'z' && event.shiftKey))) {
    event.preventDefault()
    redo()
  }
}

onMounted(() => {
  history.value = [content.value]
  historyIndex.value = 0
  document.addEventListener('keydown', handleKeydown)
  renderMarkdown()
})

onUnmounted(() => {
  document.removeEventListener('keydown', handleKeydown)
  if (renderTimer) clearTimeout(renderTimer)
})
</script>

<style scoped>
.editor-wrapper {
  display: flex;
  height: calc(100vh - 92px);
  width: 100%;
}

.editor-pane,
.preview-pane {
  flex: 1;
  overflow: auto;
  height: 100%;
}

.editor-pane {
  border-right: 1px solid var(--border-color);
  position: relative;
}

.editor-content {
  position: relative;
  width: 100%;
  height: 100%;
}

.editor-textarea {
  width: 100%;
  height: 100%;
  padding: 20px;
  border: none;
  outline: none;
  resize: none;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  background: var(--bg-primary);
  color: var(--text-primary);
  position: relative;
  z-index: 2;
}

.editor-textarea.searching {
  color: transparent;
  -webkit-text-fill-color: transparent;
  caret-color: var(--text-primary);
  background: transparent;
}

.editor-textarea.searching::selection {
  background: transparent;
}

.highlight-layer {
  position: absolute;
  top: 0;
  left: 0;
  width: 100%;
  height: 100%;
  padding: 20px;
  margin: 0;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
  overflow-wrap: break-word;
  overflow: hidden;
  background: var(--bg-primary);
  color: var(--text-primary);
  border: none;
  pointer-events: none;
  z-index: 1;
}

.highlight-layer :deep(mark) {
  background: #b8d4f0;
  color: #1a1a1a;
  border-radius: 2px;
}

.highlight-layer :deep(mark.current) {
  background: #1a73e8;
  color: #ffffff;
}

.dark-theme .highlight-layer :deep(mark) {
  background: #2a4a6b;
  color: #d4d4d4;
}

.dark-theme .highlight-layer :deep(mark.current) {
  background: #4a9eff;
  color: #ffffff;
}

.editor-pane.search-visible .editor-textarea {
  padding-top: 44px;
}

.editor-pane.search-visible .highlight-layer {
  padding-top: 44px;
}

.search-bar {
  position: absolute;
  top: 0;
  right: 0;
  display: flex;
  align-items: center;
  gap: 4px;
  padding: 6px 10px;
  background: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-top: none;
  border-right: none;
  border-bottom-left-radius: 6px;
  box-shadow: 0 2px 8px rgba(0, 0, 0, 0.12);
  z-index: 10;
}

.search-input {
  width: 180px;
  padding: 4px 8px;
  border: 1px solid var(--border-color);
  border-radius: 3px;
  background: var(--bg-secondary);
  color: var(--text-primary);
  font-size: 13px;
  outline: none;
  transition: border-color 0.15s;
}

.search-input:focus {
  border-color: var(--accent-color);
}

.search-input::placeholder {
  color: var(--text-muted);
}

.search-count {
  font-size: 12px;
  color: var(--text-muted);
  min-width: 40px;
  text-align: center;
  user-select: none;
}

.search-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border: 1px solid var(--border-color);
  background: var(--bg-secondary);
  color: var(--text-primary);
  border-radius: 3px;
  cursor: pointer;
  font-size: 11px;
  padding: 0;
  transition: background-color 0.15s;
}

.search-btn:hover {
  background: var(--hover-bg);
}

.search-btn:disabled {
  opacity: 0.35;
  cursor: not-allowed;
}

.search-btn:disabled:hover {
  background: var(--bg-secondary);
}

.search-close {
  border: none;
  background: transparent;
  color: var(--text-muted);
  font-size: 14px;
  font-weight: bold;
}

.search-close:hover {
  background: var(--hover-bg);
  color: var(--text-primary);
}

.editor-pane.search-visible .editor-textarea {
  padding-top: 44px;
}

.preview-content {
  padding: 20px;
  min-height: 100%;
  background: var(--bg-tertiary);
  color: var(--text-primary);
}

.markdown-body {
  font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Helvetica, Arial, sans-serif;
  line-height: 1.6;
  color: var(--text-primary);
}

.markdown-body h1,
.markdown-body h2,
.markdown-body h3,
.markdown-body h4,
.markdown-body h5,
.markdown-body h6 {
  margin-top: 24px;
  margin-bottom: 16px;
  font-weight: 600;
  line-height: 1.25;
}

.markdown-body h1 {
  font-size: 2em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.markdown-body h2 {
  font-size: 1.5em;
  border-bottom: 1px solid var(--border-color);
  padding-bottom: 0.3em;
}

.markdown-body h3 {
  font-size: 1.25em;
}

.markdown-body p {
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-body code {
  padding: 0.2em 0.4em;
  margin: 0;
  font-size: 85%;
  background-color: rgba(128, 128, 128, 0.15);
  border-radius: 3px;
  font-family: 'Consolas', 'Monaco', 'Courier New', monospace;
}

.markdown-body pre {
  padding: 16px;
  overflow: auto;
  font-size: 85%;
  line-height: 1.45;
  background-color: var(--bg-secondary);
  border-radius: 3px;
}

.markdown-body pre code {
  padding: 0;
  margin: 0;
  font-size: 100%;
  background: transparent;
  border: 0;
}

.markdown-body blockquote {
  padding: 0 1em;
  color: var(--text-muted);
  border-left: 0.25em solid var(--border-color);
  margin: 0 0 16px 0;
}

.markdown-body ul,
.markdown-body ol {
  padding-left: 2em;
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-body li {
  margin-top: 0.25em;
}

.markdown-body a {
  color: #0366d6;
  text-decoration: none;
}

.dark-theme .markdown-body a {
  color: #58a6ff;
}

.markdown-body a:hover {
  text-decoration: underline;
}

.markdown-body img {
  max-width: 100%;
  box-sizing: content-box;
}

.markdown-body table {
  display: block;
  width: 100%;
  overflow: auto;
  border-spacing: 0;
  border-collapse: collapse;
  margin-top: 0;
  margin-bottom: 16px;
}

.markdown-body table th,
.markdown-body table td {
  padding: 6px 13px;
  border: 1px solid var(--border-color);
}

.markdown-body table tr {
  background-color: var(--bg-primary);
  border-top: 1px solid var(--border-color);
}

.markdown-body table tr:nth-child(2n) {
  background-color: var(--bg-secondary);
}

.markdown-body hr {
  height: 0.25em;
  padding: 0;
  margin: 24px 0;
  background-color: var(--border-color);
  border: 0;
}
</style>