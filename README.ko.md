[English](README.md) | [简体中文](README.zh-CN.md) | [繁體中文](README.zh-TW.md) | [日本語](README.ja.md) | **한국어** | [Русский](README.ru.md)

# Markdown Editor

macOS, Windows, Linux를 지원하는 크로스 플랫폼 Markdown 편집기입니다.

## 기능

- 📝 실시간 미리보기 및 동기화된 스크롤을 통한 Markdown 편집
- 🔍 하이라이트 및 빠른 탐색이 가능한 검색 기능
- ↩️ 실행 취소/다시 실행 (Ctrl+Z / Ctrl+Y)
- ⌨️ 다양한 키보드 단축키
  - `Ctrl+N` 새로 만들기, `Ctrl+O` 열기, `Ctrl+S` 저장, `Ctrl+F` 검색
  - `Ctrl+Enter` 현재 줄 아래에 새 줄 삽입
  - 도구 모음: 굵게, 기울임, 취소선, 인용, 제목, 목록, 링크, 이미지, 코드 블록, 표, 이모지 등
- 💾 파일 관리 (새로 만들기, 열기, 저장, 다른 이름으로 저장), 자동 저장 지원
- 🖨️ 시스템 인쇄 대화상자 — Markdown 문서를 직접 인쇄하거나 인쇄 대화상자에서 "PDF로 저장"을 선택하여 PDF로 내보내기
- 🎨 라이트/다크 테마 전환, 시스템 테마 연동
- 🌍 다국어 지원: English, 简体中文, 繁體中文, 日本語, 한국어, Русский
- 🖥️ 크로스 플랫폼 (macOS, Windows, Linux)

## 기술 스택

- **프론트엔드**: Vue 3 + Vite
- **데스크톱**: Tauri 2.x
- **Markdown 렌더링**: Rust (pulldown-cmark), Tauri IPC를 통해 호출
- **백엔드**: Rust

## 개발 환경

- Node.js 16+
- Rust 1.70+
- 시스템 종속성 ([Tauri 문서](https://tauri.app/start/prerequisites/) 참조)

## 의존성 설치

```bash
npm install
```

## 개발 모드

```bash
npm run tauri dev
```

## 빌드

```bash
npm run tauri build
```

빌드 후 실행 파일은 `src-tauri/target/release/bundle/` 디렉터리에 생성됩니다.

## 프로젝트 구조

```
markdown-editor/
├── src/                    # Vue 프론트엔드
│   ├── components/         # Vue 컴포넌트
│   │   ├── MarkdownEditor.vue  # Markdown 편집기 컴포넌트
│   │   ├── Toolbar.vue     # 도구 모음 컴포넌트
│   │   └── emojiData.js    # 이모지 데이터
│   ├── i18n/               # 다국어 번역 파일
│   │   ├── index.js
│   │   ├── en.js           # English
│   │   ├── zh-CN.js        # 简体中文
│   │   ├── zh-TW.js        # 繁體中文
│   │   ├── ja.js           # 日本語
│   │   ├── ko.js           # 한국어
│   │   └── ru.js           # Русский
│   ├── App.vue             # 메인 앱 컴포넌트
│   └── main.js             # 진입점
├── src-tauri/              # Tauri Rust 백엔드
│   ├── src/
│   │   ├── lib.rs          # Rust 백엔드 로직 (파일 I/O, Markdown 렌더링)
│   │   └── main.rs
│   ├── capabilities/       # 권한 설정
│   └── tauri.conf.json     # Tauri 구성
└── index.html              # HTML 템플릿
```

## 사용 방법

1. **새 파일**: "새로 만들기" 버튼 클릭 또는 Ctrl+N
2. **파일 열기**: "열기" 버튼 클릭 또는 Ctrl+O, .md 파일 선택
3. **편집**: 왼쪽 창에서 Markdown 작성, 오른쪽에서 실시간 미리보기
4. **저장**: "저장" 버튼 클릭 또는 Ctrl+S
5. **다른 이름으로 저장**: "다른 이름으로 저장" 버튼으로 새 위치에 저장
6. **인쇄**: 도구 모음의 인쇄 버튼을 클릭하여 시스템 인쇄 대화상자를 열고, 프린터로 인쇄하거나 "PDF로 저장"을 선택하여 PDF로 내보내기

## 라이선스

MIT
