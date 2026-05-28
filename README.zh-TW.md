[English](README.md) | [简体中文](README.zh-CN.md) | **繁體中文** | [日本語](README.ja.md) | [한국어](README.ko.md) | [Русский](README.ru.md)

# Markdown Editor

一款跨平台的本地 Markdown 編輯器，支援 macOS、Windows 和 Linux。

## 功能特性

- 📝 所見即所得的 Markdown 編輯體驗，支援即時預覽和同步捲動
- 🔍 內建搜尋功能，支援螢光標示和快速跳轉
- ↩️ 復原/重做（Ctrl+Z / Ctrl+Y）
- ⌨️ 豐富的快速鍵支援
  - `Ctrl+N` 新建、`Ctrl+O` 開啟、`Ctrl+S` 儲存、`Ctrl+F` 搜尋
  - `Ctrl+Enter` 在下方插入新行
  - 工具列支援粗體、斜體、刪除線、引用、標題、列表、連結、圖片、程式碼區塊、表格、Emoji 等
- 💾 完整的檔案管理（新建、開啟、儲存、另存新檔），支援自動儲存
- 🖨️ 系統列印對話框 — 可直接列印 Markdown 文件，或在列印對話框中選擇"另存為 PDF"匯出為 PDF
- 🎨 亮色/暗色主題切換，跟隨系統主題
- 🌍 多語言支援：English、简体中文、繁體中文、日本語、한국어、Русский
- 🖥️ 跨平台支援（macOS、Windows、Linux）

## 技術棧

- **前端框架**: Vue 3 + Vite
- **桌面應用**: Tauri 2.x
- **Markdown 渲染**: Rust (pulldown-cmark)，透過 Tauri IPC 呼叫
- **後端語言**: Rust

## 開發環境要求

- Node.js 16+
- Rust 1.70+
- 系統依賴（參考 [Tauri 文件](https://tauri.app/start/prerequisites/)）

## 安裝依賴

```bash
npm install
```

## 開發模式

```bash
npm run tauri dev
```

## 構建應用

```bash
npm run tauri build
```

構建完成後，可執行檔將位於 `src-tauri/target/release/bundle/` 目錄下。

## 專案結構

```
markdown-editor/
├── src/                    # Vue 前端程式碼
│   ├── components/         # Vue 元件
│   │   ├── MarkdownEditor.vue  # Markdown 編輯器元件
│   │   ├── Toolbar.vue     # 工具列元件
│   │   └── emojiData.js    # Emoji 資料
│   ├── i18n/               # 多語言翻譯檔案
│   │   ├── index.js
│   │   ├── en.js           # English
│   │   ├── zh-CN.js        # 简体中文
│   │   ├── zh-TW.js        # 繁體中文
│   │   ├── ja.js           # 日本語
│   │   ├── ko.js           # 한국어
│   │   └── ru.js           # Русский
│   ├── App.vue             # 主應用元件
│   └── main.js             # 進入點
├── src-tauri/              # Tauri Rust 後端
│   ├── src/
│   │   ├── lib.rs          # Rust 後端邏輯（檔案 I/O、Markdown 渲染）
│   │   └── main.rs
│   ├── capabilities/       # 權限配置
│   └── tauri.conf.json     # Tauri 配置
└── index.html              # HTML 模板
```

## 使用說明

1. **新建檔案**: 點擊工具列「新建」按鈕或按 Ctrl+N
2. **開啟檔案**: 點擊「開啟」按鈕或按 Ctrl+O，選擇本地的 .md 檔案
3. **編輯內容**: 在左側編輯區編寫 Markdown 內容，右側即時預覽
4. **儲存檔案**: 點擊「儲存」按鈕或按 Ctrl+S
5. **另存新檔**: 點擊「另存新檔」按鈕儲存到新位置
6. **列印**: 點擊工具列上的列印按鈕，呼叫系統列印對話框，可選擇印表機列印或選擇"另存為 PDF"匯出為 PDF 檔案

## 授權條款

MIT
