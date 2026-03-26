# Nova Mind v1.0.0 Release Notes

> 发布日期: 2026-04-30

## 📦 概述

Nova Mind 是一个 AI 驱动的产品管理思维导图桌面应用，帮助 PM 团队更高效地管理和可视化产品需求。

## ✨ 新功能

### 核心功能

- 🧠 **思维导图编辑** - 可视化产品需求和想法
- 💬 **AI 对话** - 基于 OpenClaw 沙箱的智能助手
- 📋 **技能管理** - 可定制的 Prompt 模板库
- 💾 **会话管理** - 支持多会话，历史消息持久化

### 用户体验

- 🌙 **主题切换** - 支持浅色/深色/自动主题
- 📝 **Markdown 渲染** - 代码高亮、表格、列表支持
- 💾 **草稿自动保存** - 防止数据丢失
- 📤 **文档导出** - 支持 Markdown 和 Word 格式

## 🔧 技术特性

- 🏗️ **Tauri 2.0** - 基于 Rust 的高性能桌面框架
- ⚡ **Vue 3 + TypeScript** - 现代前端技术栈
- 🎨 **Tailwind CSS** - 响应式设计
- 🗄️ **SQLite** - 轻量级本地数据库

## 🐛 修复

- 修复了主题切换时闪烁的问题
- 修复了长消息滚动性能问题
- 修复了草稿保存逻辑

## 🔒 安全

- API Key 安全存储
- Tauri CSP 安全策略
- 沙箱隔离执行环境

## 📋 系统要求

### Windows
- Windows 10 (64-bit) 或更高版本
- 至少 4GB RAM
- 至少 200MB 可用磁盘空间

### macOS
- macOS 10.15 (Catalina) 或更高版本
- 至少 4GB RAM
- 至少 200MB 可用磁盘空间

### Linux
- Ubuntu 18.04 或更高版本
- 至少 4GB RAM
- 至少 200MB 可用磁盘空间

## 📥 下载

| 平台 | 安装包 | 大小 |
|------|--------|------|
| Windows | `Nova-Mind_1.0.0_x64-setup.exe` | ~50MB |
| macOS | `Nova-Mind_1.0.0.dmg` | ~80MB |
| Linux | `Nova-Mind_1.0.0_amd64.deb` | ~60MB |

## 🙏 致谢

感谢所有参与 Nova Mind 开发的团队成员和贡献者！

## 📄 许可证

Nova Mind 采用 MIT 许可证。详见 [LICENSE](LICENSE) 文件。

---

## 版本历史

### v1.0.0 (2026-04-30)
- 首次正式发布
- MVP 核心功能完成
- 支持 Windows/macOS/Linux
