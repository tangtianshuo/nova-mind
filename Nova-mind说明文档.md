<br />

***

## 1. 产品命名方案

### **Nova Mind / 诺瓦智源**

- **项目代号：** `nova-mind`
- **命名释意：**
  - **Mind (精神/头脑)：** 产品经理的核心价值在于思考、逻辑与决策。
  - **智源：** 寓意 AI 能力的源头，通过侧向加载（Sidecar）的 OpenClaw 提供源源不断的智力支持。
- **取名逻辑：** 延续 Nova 系列的简洁风格。如果 Core 是心脏，那么 Mind 就是大脑。它强调了“Skill 提示词”和“AI 赋能”的特性，直击 PM 需要进行大量文档撰写、需求分析的痛点。

***

## 2. 技术实现路径文档

这份文档旨在定义如何利用 **Tauri** 的轻量化特性与 **OpenClaw** 的原生能力，构建一个高性能的桌面端工作台。

### **一、 总体架构设计**

产品采用 **前后端分离 + Sidecar 模式**。

- **前端 (Frontend):** 采用 Vue3 或 React + Tailwind CSS，负责 UI 交互、Markdown 预览、Skill 库管理。
- **后端 (Trust/Rust):** Tauri 核心，负责系统级权限、窗口管理及与 Sidecar 的通信。
- **侧向进程 (Sidecar):** 封装 OpenClaw 编译后的二进制文件，通过标准输入输出 (STDIO) 或 Localhost HTTP 进行内部通信。

### **二、 核心实现模块**

#### **1. Sidecar 封装策略**

Tauri 允许运行外部二进制文件。我们将 OpenClaw 放置在 `src-tauri/binaries` 目录下。

- **配置：** 在 `tauri.conf.json` 中定义 `bundle > externalBin`。
- **启动流：** 在 Rust 入口函数内通过 `tauri::api::process::Command` 启动 OpenClaw，并保持进程常驻，确保 AI 响应的低延迟。

#### **2. Skill 提示词引擎 (Preset System)**

为了让 PM 快速上手，需要设计一套结构化的 Prompt 管理机制：

- **配置化存储：** 预制 Skill 采用 YAML 或 JSON 格式存储，包含：`Role`, `Task`, `Constraint`, `Output Format`。
- **变量替换：** 允许 PM 在 UI 输入核心变量（如：功能描述），前端自动填充进模板，并发送给 OpenClaw 进行推理。

#### **3. 数据持久化**

- **SQLite/Lowdb：** 用于存储 PM 的历史对话记录、自定义 Skill 和 PRD 草稿。
- **文件系统访问：** 利用 Tauri 的 `fs` 模块，支持 PM 直接导出 `.md` 或 `.docx` 格式的文档到本地磁盘。

### **三、 关键技术路径**

| 阶段       | 核心任务           | 技术要点                                       |
| :------- | :------------- | :----------------------------------------- |
| **环境集成** | Sidecar 通信调试   | 解决 Rust 与 OpenClaw 进程间的信号传递与异常捕获。          |
| **交互开发** | 沉浸式 UI 设计      | 采用侧边栏切换 Skill，支持流式渲染 (Streaming) 输出 AI 内容。 |
| **能力预制** | PM 场景 Skill 固化 | 编写竞品分析、PRD 生成、用户故事拆解等高频 Prompt。            |
| **性能优化** | 内存管理           | 利用 Tauri 的轻量特性，严格控制 Sidecar 进程的资源占用。       |

### **四、 预期的 PM 高频场景**

1. **一键 PRD 草拟：** 输入功能点，调用预制 Skill 快速生成标准模板。
2. **代码/逻辑审查：** 借助 OpenClaw 强大的模型能力，辅助分析业务逻辑漏洞。
3. **多格式输出：** 转换 AI 结果为流程图代码（Mermaid）或标准文档。

***

**下一步建议：**
如果你需要，我可以为你起草一份 `tauri.conf.json` 中关于 Sidecar 的具体配置代码，或者为你编写几组针对产品经理高频场景的 **Structured Prompt (结构化提示词)** 模板。你希望先看哪一部分？
