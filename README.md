**English** | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Русский](README.ru.md)

# Markdown Editor

A cross-platform local Markdown editor supporting macOS, Windows, and Linux.

## Features

- 📝 WYSIWYG Markdown editing with live preview and synchronized scrolling
- 🔍 Built-in search with highlighting and quick navigation
- ↩️ Undo/Redo (Ctrl+Z / Ctrl+Y)
- ⌨️ Rich keyboard shortcuts
  - `Ctrl+N` New, `Ctrl+O` Open, `Ctrl+S` Save, `Ctrl+F` Search
  - `Ctrl+Enter` Insert a new line below
  - Toolbar supports bold, italic, strikethrough, blockquote, headings, lists, links, images, code blocks, tables, emoji, and more
- 💾 Full file management (New, Open, Save, Save As) with auto-save support
- 🖨️ System print dialog — print Markdown documents directly or save as PDF
- 🎨 Light/Dark theme toggle with system theme follow
- 🌍 Multi-language support: English, 简体中文, 繁體中文, 日本語, 한국어, Русский
- 🖥️ Cross-platform (macOS, Windows, Linux)

## Tech Stack

- **Frontend**: Vue 3 + Vite
- **Desktop**: Tauri 2.x
- **Markdown Rendering**: Rust (pulldown-cmark) via Tauri IPC
- **Backend**: Rust

## Prerequisites

- Node.js 16+
- Rust 1.70+
- System dependencies (see [Tauri docs](https://tauri.app/start/prerequisites/))

## Install Dependencies

```bash
npm install
```

## Development

```bash
npm run tauri dev
```

## Build

```bash
npm run tauri build
```

After building, the executable will be located in `src-tauri/target/release/bundle/`.

## Project Structure

```
markdown-editor/
├── src/                    # Vue frontend
│   ├── components/         # Vue components
│   │   ├── MarkdownEditor.vue  # Markdown editor component
│   │   ├── Toolbar.vue     # Toolbar component
│   │   └── emojiData.js    # Emoji data
│   ├── i18n/               # i18n translation files
│   │   ├── index.js
│   │   ├── en.js           # English
│   │   ├── zh-CN.js        # 简体中文
│   │   ├── zh-TW.js        # 繁體中文
│   │   ├── ja.js           # 日本語
│   │   ├── ko.js           # 한국어
│   │   └── ru.js           # Русский
│   ├── App.vue             # Main app component
│   └── main.js             # Entry point
├── src-tauri/              # Tauri Rust backend
│   ├── src/
│   │   ├── lib.rs          # Rust backend logic (file I/O, Markdown rendering)
│   │   └── main.rs
│   ├── capabilities/       # Permission config
│   └── tauri.conf.json     # Tauri configuration
└── index.html              # HTML template
```

## Usage

1. **New File**: Click "New" button or press Ctrl+N
2. **Open File**: Click "Open" button or press Ctrl+O, select a local .md file
3. **Edit**: Write Markdown in the left pane, preview updates in real-time on the right
4. **Save**: Click "Save" button or press Ctrl+S
5. **Save As**: Click "Save As" button to save to a new location
6. **Print**: Click the Print button on the toolbar to open the system print dialog; select a printer to print, or choose "Save as PDF" to export as PDF

## License

MIT
