[English](README.md) | **简体中文** | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | [한국어](README.ko.md) | [Русский](README.ru.md)

# Markdown Editor

一款跨平台的本地Markdown编辑器，支持 macOS、Windows 和 Linux。

## 功能特性

- 📝 所见即所得的 Markdown 编辑体验，支持实时预览和同步滚动
- 🔍 内置搜索功能，支持高亮和快速跳转
- ↩️ 撤销/重做（Ctrl+Z / Ctrl+Y）
- ⌨️ 丰富的快捷键支持
  - `Ctrl+N` 新建、`Ctrl+O` 打开、`Ctrl+S` 保存、`Ctrl+F` 搜索
  - `Ctrl+Enter` 在下方插入新行
  - 工具栏支持加粗、斜体、删除线、引用、标题、列表、链接、图片、代码块、表格、Emoji 等
- 💾 完整的文件管理（新建、打开、保存、另存为），支持自动保存
- 🎨 亮色/暗色主题切换，跟随系统主题
- 🌍 多语言支持：简体中文、繁體中文、English、日本語、한국어、Русский
- 🖥️ 跨平台支持（macOS、Windows、Linux）

## 技术栈

- **前端框架**: Vue 3 + Vite
- **桌面应用**: Tauri 2.x
- **Markdown 渲染**: Rust (pulldown-cmark)，通过 Tauri IPC 调用
- **后端语言**: Rust

## 开发环境要求

- Node.js 16+
- Rust 1.70+
- 系统依赖（参考 [Tauri 文档](https://tauri.app/start/prerequisites/)）

## 安装依赖

```bash
npm install
```

## 开发模式

```bash
npm run tauri dev
```

## 构建应用

```bash
npm run tauri build
```

构建完成后，可执行文件将位于 `src-tauri/target/release/bundle/` 目录下。

## 项目结构

```
markdown-editor/
├── src/                    # Vue 前端代码
│   ├── components/         # Vue 组件
│   │   ├── MarkdownEditor.vue  # Markdown编辑器组件
│   │   ├── Toolbar.vue     # 工具栏组件
│   │   └── emojiData.js    # Emoji 数据
│   ├── i18n/               # 多语言翻译文件
│   │   ├── index.js
│   │   ├── en.js           # English
│   │   ├── zh-CN.js        # 简体中文
│   │   ├── zh-TW.js        # 繁體中文
│   │   ├── ja.js           # 日本語
│   │   ├── ko.js           # 한국어
│   │   └── ru.js           # Русский
│   ├── App.vue             # 主应用组件
│   └── main.js             # 入口文件
├── src-tauri/              # Tauri Rust 后端
│   ├── src/
│   │   ├── lib.rs          # Rust 后端逻辑（文件I/O、Markdown渲染）
│   │   └── main.rs
│   ├── capabilities/       # 权限配置
│   └── tauri.conf.json     # Tauri 配置
└── index.html              # HTML 模板
```

## 使用说明

1. **新建文件**: 点击工具栏"新建"按钮或按 Ctrl+N
2. **打开文件**: 点击"打开"按钮或按 Ctrl+O，选择本地的 .md 文件
3. **编辑内容**: 在左侧编辑区编写Markdown内容，右侧实时预览
4. **保存文件**: 点击"保存"按钮或按 Ctrl+S
5. **另存为**: 点击"另存为"按钮保存到新位置

## 许可证

MIT
