# Nova Mind - 高优先级规划建议

> 版本: v1.1.0  
> 更新日期: 2026-03-26  
> 状态: 基于已完成的核心功能

---

## 📊 项目现状分析

### ✅ 已完成 (P0)
1. **思维导图核心交互**
   - ✅ 节点增删改查
   - ✅ 拖拽移动
   - ✅ 缩放平移
   - ✅ 展开折叠
   - ✅ 撤销重做
   - ✅ 自动保存
   - ✅ Markdown 导出

2. **AI 对话基础**
   - ✅ 流式响应 UI
   - ✅ Markdown 渲染
   - ✅ 消息发送接收
   - ✅ 草稿保存

3. **文档导出**
   - ✅ Markdown 导出
   - ✅ Word 导出

### ❌ 待完成 (P0 高优先级)

#### 1. OpenClaw 沙箱集成 🔴 **最优先**
**现状**: 当前 AI 对话为模拟响应，未连接真实 AI  
**影响**: 无法实现真正的 AI 能力，核心价值无法体现

**建议实现方案**:

```rust
// src-tauri/src/sandbox.rs
pub async fn chat_completion(
    &self,
    messages: Vec<Message>,
    skill: Option<&str>,
) -> Result<String, SandboxError> {
    // 1. 构建 OpenClaw 请求
    let request = OpenClawRequest {
        model: "nova-model".to_string(),
        messages: messages.into_iter().map(|m| m.into()).collect(),
        stream: true,
        skill: skill.map(|s| s.to_string()),
    };
    
    // 2. 发送到 OpenClaw 沙箱
    let response = self.client.post("/v1/chat/completions")
        .json(&request)
        .send()
        .await?;
    
    // 3. 解析流式响应
    Ok(response.text().await?)
}
```

**关键任务**:
- [ ] 实现 OpenClaw HTTP 客户端
- [ ] 添加消息格式转换
- [ ] 支持 SSE 流式解析
- [ ] 错误处理和重试机制
- [ ] 超时和限流控制

#### 2. 思维导图 ↔ AI 对话 双向关联 🔴 **核心价值**

**现状**: 思维导图和 AI 对话是两个独立模块  
**价值**: 打通后可实现"选中节点 → AI 分析 → 结果导入"的完整工作流

**建议实现方案**:

```typescript
// src/stores/mindmapChatStore.ts
export const useMindmapChatStore = defineStore('mindmapChat', () => {
  const linkedConversationId = ref<number | null>(null);
  const linkedMindmapId = ref<number | null>(null);
  
  // 获取思维导图内容作为上下文
  function getMindmapContext(): string {
    const mindmap = mindmapStore.currentMindmap;
    return mindmapToMarkdown(mindmap);
  }
  
  // 发送消息时自动附加上下文
  async function sendWithContext(content: string) {
    const context = getMindmapContext();
    const fullPrompt = `【相关思维导图内容】\n${context}\n\n【用户问题】\n${content}`;
    return chatStore.sendMessage(fullPrompt);
  }
  
  // 将 AI 回复解析为思维导图节点
  function parseToMindmapNodes(response: string): MindmapNode[] {
    // 实现 Markdown → 节点结构转换
  }
});
```

**交互流程**:
```
用户选中思维导图节点
    ↓
点击"AI 分析"
    ↓
自动携带上下文发起对话
    ↓
AI 生成建议/分析
    ↓
用户一键将结果导入为子节点
```

**关键任务**:
- [ ] 建立思维导图与会话的关联关系
- [ ] 实现思维导图 → Markdown 转换
- [ ] 实现 AI 回复 → 节点结构解析
- [ ] 添加"导入为子节点"按钮
- [ ] 双向同步更新

#### 3. Skill 系统完善 🔴 **差异化竞争力**

**现状**: 基础 Skill CRUD 已完成，变量系统未实现  
**价值**: 差异化 Prompt 模板，提高 AI 输出质量

**建议实现方案**:

```typescript
// src/components/SkillVariableForm.vue
interface SkillVariable {
  name: string;
  type: 'text' | 'textarea' | 'select' | 'number' | 'date' | 'checkbox';
  label: string;
  required?: boolean;
  placeholder?: string;
  default?: string | number | boolean;
  options?: Array<{ label: string; value: string }>;
  validation?: {
    minLength?: number;
    maxLength?: number;
    pattern?: string;
  };
}

// Skill 模板示例
const PRD_SKILL: Skill = {
  name: 'PRD 生成器',
  category: '产品文档',
  content: `作为产品经理，请根据以下信息生成 PRD：

## 功能名称
{{function_name}}

## 功能描述
{{function_description}}

## 用户故事
{{user_story}}

## 验收标准
{{acceptance_criteria}}

请生成完整的 PRD 文档，包括：
1. 背景和目标
2. 功能详情
3. 交互设计
4. 非功能性需求
5. 风险评估`,
  variables: [
    { name: 'function_name', type: 'text', label: '功能名称', required: true },
    { name: 'function_description', type: 'textarea', label: '功能描述', required: true },
    { name: 'user_story', type: 'textarea', label: '用户故事' },
    { 
      name: 'acceptance_criteria', 
      type: 'textarea', 
      label: '验收标准',
      placeholder: '格式：\n- 场景1：预期结果1\n- 场景2：预期结果2' 
    }
  ]
};
```

**预设 Skill 模板**:

| Skill 名称 | 分类 | 核心变量 | 使用场景 |
|-----------|------|----------|---------|
| PRD 生成器 | 产品文档 | 功能名、描述、用户故事、验收标准 | 快速生成产品需求文档 |
| 竞品分析 | 市场分析 | 产品名称、竞品列表、分析维度 | 生成竞品对比报告 |
| 用户故事地图 | 敏捷开发 | 目标用户、核心需求、功能列表 | 生成用户故事地图 |
| 需求评审 | 质量管理 | 需求描述、技术约束、风险点 | 生成评审检查清单 |
| 会议纪要 | 团队协作 | 会议主题、参与者、讨论要点 | 生成结构化会议纪要 |
| 周报生成 | 效率工具 | 本周完成、下周计划、阻碍因素 | 生成格式化周报 |

**关键任务**:
- [ ] 实现 SkillVariableForm 组件
- [ ] 添加变量验证逻辑
- [ ] 创建 5+ 预设 Skill 模板
- [ ] Skill 调用界面优化
- [ ] Skill 使用统计

---

## 🎯 高优先级任务清单

### Phase 1: AI 能力集成 (预计 2 周)

#### Week 1: OpenClaw 集成
- [ ] **Day 1-2**: OpenClaw HTTP 客户端实现
  - 连接管理
  - 请求/响应序列化
  - 错误处理

- [ ] **Day 3-4**: SSE 流式响应处理
  - 流式解析器
  - 增量渲染
  - 中断支持

- [ ] **Day 5**: 集成测试
  - 基础对话测试
  - 长对话测试
  - 错误场景测试

#### Week 2: 上下文关联
- [ ] **Day 1-2**: 思维导图上下文提取
  - 节点结构转文本
  - 上下文组装
  - Token 限制处理

- [ ] **Day 3-4**: 智能解析
  - Markdown 解析
  - 节点结构生成
  - 冲突处理

- [ ] **Day 5**: 用户交互优化
  - 预览确认
  - 批量导入
  - 撤销支持

### Phase 2: Skill 系统 (预计 1.5 周)

#### Week 3: 变量表单
- [ ] **Day 1-2**: SkillVariableForm 组件
  - 动态表单生成
  - 验证规则
  - 样式适配

- [ ] **Day 3**: 预设模板
  - PRD 生成器
  - 竞品分析
  - 用户故事地图

- [ ] **Day 4-5**: 集成调用
  - Skill 选择器
  - 变量填充
  - 模板渲染

#### Week 4: 模板完善
- [ ] **Day 1-2**: 更多预设模板
  - 需求评审
  - 会议纪要
  - 周报生成

- [ ] **Day 3**: 模板市场（可选）
  - 模板导入/导出
  - 模板市场 UI

- [ ] **Day 4-5**: 优化完善
  - 性能优化
  - Bug 修复
  - 文档完善

---

## 📈 预期成果

### 功能完整性
```
当前状态: ████████░░ 80%
Phase 1 后: ██████████ 95%
Phase 2 后: ██████████ 98%
```

### 核心价值
1. **AI 驱动**: 从"能用"到"好用"
2. **上下文关联**: 从"工具"到"助手"
3. **差异化**: 从"通用"到"垂直"

### 用户体验
- **学习成本**: 降低 50%（预设模板）
- **工作效率**: 提升 3-5 倍（AI 辅助）
- **协作体验**: 提升 2 倍（上下文同步）

---

## ⚠️ 风险与建议

### 技术风险
1. **OpenClaw 可用性**
   - 建议: 准备备选方案（直接 API 调用）
   - 影响: 中
   - 概率: 低

2. **Token 限制**
   - 建议: 实现智能截断和分片
   - 影响: 低
   - 概率: 高

3. **解析准确性**
   - 建议: 提供人工确认环节
   - 影响: 中
   - 概率: 中

### 资源需求
1. **开发时间**: 约 3.5 周
2. **测试资源**: 约 1 周
3. **文档编写**: 约 3 天

---

## 🚀 推荐实施路径

### Option A: 激进路线 (推荐)
```
Week 1-2: OpenClaw 集成
    ↓
Week 3-4: Skill 系统
    ↓
Week 5: 测试和优化
    ↓
Week 6: 发布准备
```

**优点**: 快速验证核心价值  
**缺点**: 可能需要快速迭代

### Option B: 稳健路线
```
Week 1: OpenClaw 基础集成
Week 2: 上下文关联
Week 3: Skill 变量表单
Week 4: 预设模板
Week 5: 测试和优化
Week 6: 发布准备
```

**优点**: 逐步验证，降低风险  
**缺点**: 时间较长

---

## 📝 下一步行动

### 立即执行 (本周)
1. ✅ OpenClaw 环境准备
2. ✅ API 文档确认
3. ⏳ 开始实现 OpenClaw 客户端

### 短期目标 (2 周)
1. 完成 OpenClaw 集成
2. 完成上下文关联
3. 完成基础 Skill 系统

### 中期目标 (1 个月)
1. 5+ 预设 Skill 模板
2. 完整的测试覆盖
3. Beta 版本发布

---

**文档状态**: 📝 待审核  
**下一步**: 确认技术方案后开始实现  
**联系**: 请通过 GitHub Issues 反馈意见
