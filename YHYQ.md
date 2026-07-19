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
- 已检查实机 Codex `26.707.9981.0` 的主进程更新器实现，确认 `CODEX_SPARKLE_ENABLED=false` 会在更新器初始化前同时关闭 macOS Sparkle 与 Windows Store/MSIX 更新器。
- 已确认仅依赖渲染层 `disableSparkleAutodownload` 存在启动时序风险；实现将从 Codex 进程启动环境阻断更新器，并覆盖 Windows 打包版、Windows 便携版和 macOS App 启动链路。
- 已确认新增开关不会修改 Codex++ 自身的 GitHub Release 检查、下载和安装功能。
- 已新增 `codexAppDisableAutoUpdate` 设置，默认关闭；旧版配置缺少该字段时继续允许 Codex 更新。
- 已新增跨平台 Codex 更新策略：Windows 当前用户环境写入或移除 `CODEX_SPARKLE_ENABLED=false` 并广播环境变化；macOS 使用 `launchctl` 设置或移除同一变量；便携版和直接启动进程会显式注入或移除该变量。
- 已在 Codex 启动前、管理器保存设置、页面桥接设置更新、完整配置导入和设置重置时同步应用策略。
- 已在 Codex增强页“界面与启动”分组新增“关闭 Codex 自动更新”开关，切换后立即保存，并明确说明不影响 Codex++ 自身 GitHub Release 更新、需重启 Codex 完整生效。
- 验证完成：专项测试 5 项、启动器测试 69 项、桥接测试 25 项、强制中文和粘贴测试 10 项、前端测试 11 项、TypeScript 检查、Vite 生产构建及 Windows 管理器 `cargo check` 均通过。
- `tools/i18n-verify.mjs` 仍报告仓库既有的缺失和陈旧翻译键；本次新增的两个英文词条已确认完整覆盖，没有出现在缺失列表中。
- 已将发布版本提升到 `1.2.49`，同步更新 Rust workspace、Cargo.lock、前端 package、package-lock、Tauri 配置和更新日志。
- 发布前完整验证通过：`cargo test --workspace -- --test-threads=1`、前端 TypeScript 检查、11 项前端测试、Vite 生产构建、Rust 格式检查、差异检查和本地品牌保护全部通过。
- 已使用 Cargo metadata 与 Node 重新核对所有发布版本均为 `1.2.49`，并确认 GitHub 远端尚不存在 `v1.2.49` Release。
