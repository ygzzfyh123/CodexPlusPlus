# 工作记录

## 2026-07-17

- 用户反馈：Codex++ 在 macOS 中会让模型错误地认为没有终端或文件读写工具；原版 Codex 与 Windows 版 Codex++ 不会复现。
- 已根据 `日志` 目录中的桌面端日志初步定位到本地协议代理：异常请求到达上游前缺少工具定义，而不是工具调用被权限拒绝。
- 已确认 `turn/start` 的服务档位注入仅追加字段，不会删除工具定义。
- 已修复 Responses 到 Chat Completions 的工具转换兼容性：保留新版具名工具，支持 `input_schema` / `inputSchema`，并兼容命名空间内的结构化、自定义和内置工具。
- 已为代理诊断日志增加不含提示词、参数内容或密钥的工具形态摘要，便于 macOS 复测定位。
- 验证完成：`cargo test -p codex-plus-core --test protocol_proxy` 通过 53 项；`cargo test -p codex-plus-core -- --test-threads=1` 全部通过。
- 用户要求通过 GitHub Actions 构建并发布本次修复；计划发布补丁版本 `v1.2.48`，发行说明将明确说明 macOS 新版 Codex 工具声明兼容修复。
- 已发布 `v1.2.48`：GitHub Actions 运行 `29584792734` 已成功构建 Windows x64、macOS x64 和 macOS ARM64，并校验后上传六个安装包。
- 已更新 GitHub Release 正文，明确记录 macOS 新版 Codex 工具定义被代理过滤的根因、兼容修复内容和测试结果。

## 2026-07-19

- 用户请求：为 Codex++ 增加“关闭 Codex 自动更新”选项，关闭 Codex 桌面应用的自动下载和自动安装更新。
- 已读取现有工作记录并开始定位 Codex 桌面应用更新链路、设置持久化和注入层实现。
- 用户再次明确：目标是关闭官方 Codex 桌面应用更新，不是关闭 Codex++ 自身更新；Codex++ 的 GitHub Release 更新功能必须保持不变。
