<template>
  <div class="editor-wrapper">
    <div class="editor-pane">
      <textarea
        ref="textareaRef"
        v-model="content"
        class="editor-textarea"
        @input="onInput"
        @scroll="onEditorScroll"
      ></textarea>
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
import { ref, computed, watch } from 'vue'
import { marked } from 'marked'
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
const content = ref(props.modelValue)
let isSyncing = false

const previewHtml = computed(() => {
  return marked(content.value || '')
})

function onInput() {
  emit('update:modelValue', content.value)
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

function handleToolbarAction(action, emoji = null) {
  const textarea = textareaRef.value
  if (!textarea) {
    textareaRef.value?.focus()
    return
  }
  textarea.focus()
  
  switch (action) {
    case 'undo':
      document.execCommand('undo')
      break
    case 'redo':
      document.execCommand('redo')
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
    case 'table':
      insertText('\n| Column 1 | Column 2 | Column 3 |\n|----------|----------|----------|\n| Cell 1   | Cell 2   | Cell 3   |\n| Cell 4   | Cell 5   | Cell 6   |\n')
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
      textarea.select()
      break
  }
}

defineExpose({
  handleToolbarAction,
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
  
  setTimeout(() => { isSyncing = false }, 50)
}

function onPreviewScroll(e) {
  if (isSyncing || !textareaRef.value) return
  isSyncing = true
  
  const preview = e.target
  const textarea = textareaRef.value
  const scrollRatio = preview.scrollTop / (preview.scrollHeight - preview.clientHeight)
  textarea.scrollTop = scrollRatio * (textarea.scrollHeight - textarea.clientHeight)
  
  setTimeout(() => { isSyncing = false }, 50)
}

watch(() => props.modelValue, (newValue) => {
  if (content.value !== newValue) {
    content.value = newValue
  }
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