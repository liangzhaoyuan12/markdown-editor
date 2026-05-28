[English](README.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | **日本語** | [한국어](README.ko.md) | [Русский](README.ru.md)

# Markdown Editor

macOS、Windows、Linux に対応したクロスプラットフォームの Markdown エディタです。

## 機能

- 📝 ライブプレビューと同期スクロールによる Markdown 編集
- 🔍 ハイライトとクイックナビゲーションを備えた検索機能
- ↩️ 元に戻す/やり直し（Ctrl+Z / Ctrl+Y）
- ⌨️ 豊富なキーボードショートカット
  - `Ctrl+N` 新規作成、`Ctrl+O` 開く、`Ctrl+S` 保存、`Ctrl+F` 検索
  - `Ctrl+Enter` 現在行の下に新しい行を挿入
  - ツールバー: 太字、斜体、取り消し線、引用、見出し、リスト、リンク、画像、コードブロック、テーブル、絵文字など
- 💾 ファイル管理（新規作成、開く、保存、名前を付けて保存）、自動保存対応
- 🖨️ システム印刷ダイアログ — Markdown 文書を直接印刷、または印刷ダイアログで「PDF として保存」を選択して PDF に書き出し
- 🎨 ライト/ダークテーマ切替、システムテーマに追従
- 🌍 多言語対応：English、简体中文、繁體中文、日本語、한국어、Русский
- 🖥️ クロスプラットフォーム（macOS、Windows、Linux）

## 技術スタック

- **フロントエンド**: Vue 3 + Vite
- **デスクトップ**: Tauri 2.x
- **Markdown レンダリング**: Rust (pulldown-cmark)、Tauri IPC 経由
- **バックエンド**: Rust

## 開発環境

- Node.js 16+
- Rust 1.70+
- システム依存関係（[Tauri ドキュメント](https://tauri.app/start/prerequisites/) を参照）

## 依存関係のインストール

```bash
npm install
```

## 開発モード

```bash
npm run tauri dev
```

## ビルド

```bash
npm run tauri build
```

ビルド後、実行ファイルは `src-tauri/target/release/bundle/` に生成されます。

## プロジェクト構成

```
markdown-editor/
├── src/                    # Vue フロントエンド
│   ├── components/         # Vue コンポーネント
│   │   ├── MarkdownEditor.vue  # Markdown エディタコンポーネント
│   │   ├── Toolbar.vue     # ツールバーコンポーネント
│   │   └── emojiData.js    # 絵文字データ
│   ├── i18n/               # 多言語翻訳ファイル
│   │   ├── index.js
│   │   ├── en.js           # English
│   │   ├── zh-CN.js        # 简体中文
│   │   ├── zh-TW.js        # 繁體中文
│   │   ├── ja.js           # 日本語
│   │   ├── ko.js           # 한국어
│   │   └── ru.js           # Русский
│   ├── App.vue             # メインアプリケーション
│   └── main.js             # エントリーポイント
├── src-tauri/              # Tauri Rust バックエンド
│   ├── src/
│   │   ├── lib.rs          # Rust バックエンドロジック（ファイルI/O、Markdown レンダリング）
│   │   └── main.rs
│   ├── capabilities/       # パーミッション設定
│   └── tauri.conf.json     # Tauri 設定
└── index.html              # HTML テンプレート
```

## 使い方

1. **新規作成**: 「新規作成」ボタンをクリック、または Ctrl+N
2. **ファイルを開く**: 「開く」ボタンをクリック、または Ctrl+O で .md ファイルを選択
3. **編集**: 左側のペインで Markdown を記述、右側でリアルタイムプレビュー
4. **保存**: 「保存」ボタンをクリック、または Ctrl+S
5. **名前を付けて保存**: 「名前を付けて保存」で新しい場所に保存
6. **印刷**: ツールバーの印刷ボタンをクリックしてシステム印刷ダイアログを開き、プリンターで印刷するか「PDF として保存」を選択して PDF に書き出し

## ライセンス

MIT
