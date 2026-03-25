# DocTranslate 项目说明

## 项目概述

本地文档翻译 + 智能问答桌面应用，基于 Tauri 2.0 + Vue 3 + Element Plus 构建。

## 核心功能

1. **设置模块** — 配置 AI 模型提供商（DeepSeek、Moonshot、智谱、百炼）
2. **文档管理** — 上传、列表、删除 Word/PDF 文档
3. **toPdf 任务** — 中文文档翻译为双语 PDF（段落对照格式）
4. **聊天问答** — 多文档选择，基于向量检索的 RAG 问答

## 技术栈

| 层级 | 技术 |
|------|------|
| 前端 | Vue 3 + TypeScript + Element Plus |
| 后端 | Tauri 2.0 (Rust) |
| 向量存储 | LanceDB |
| 元数据存储 | SQLite |
| AI 服务 | 国内提供商 API |

## 设计文档

- **规格说明**: `docs/superpowers/specs/2026-03-25-doctranslate-design.md`
- **计划 A**: `docs/superpowers/plans/2026-03-25-doctranslate-plan-a.md`（基础架构 + 设置 + 文档管理）
- **计划 B**: `docs/superpowers/plans/2026-03-25-doctranslate-plan-b.md`（toPdf 任务 + 聊天模块）

## 实现状态

- [ ] 计划 A：基础架构 + 设置 + 文档管理
- [ ] 计划 B：toPdf 任务 + 聊天模块

## 下一步

使用子代理驱动方式执行计划 A：

```
使用 subagent-driven-development 技能，执行 docs/superpowers/plans/2026-03-25-doctranslate-plan-a.md
```

## 关键配置

- Git 用户：王钢旗 <1571805755@qq.com>
- 默认分支：main
- 当前分支：master