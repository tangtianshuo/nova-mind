# OpenClaw Tauri 沙箱自动部署方案

## 快速开始

1. 安装依赖
```bash
cd src-tauri
cargo build
cd ..
npm install
```

2. 开发模式
```bash
npm run tauri dev
```

3. 生产构建
```bash
npm run tauri build
```

## 特性

- ✅ 自动下载 OpenClaw 二进制文件
- ✅ 多平台支持 (macOS/Linux/Windows)
- ✅ 断点续传下载
- ✅ 自动版本更新
- ✅ 进程生命周期管理
- ✅ 实时日志流
- ✅ 完整的 TypeScript API

## 文档

详见 `openclaw-tauri-sandbox-guide.md`
