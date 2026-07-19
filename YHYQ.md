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
- 已将 `main` 和 `v1.2.49` 标签推送到 `https://github.com/ygzzfyh123/CodexPPP`。
- 标签推送后 GitHub 未自动创建运行记录，已使用同一个 `release-assets.yml` 工作流手动触发 `v1.2.49`，运行编号 `29684352403`。
- GitHub Actions 已成功完成版本与品牌校验、Windows x64、macOS x64、macOS ARM64 构建、安装包结构校验、六个资产上传和 GitHub Release 发布。
- 已将 `v1.2.49` Release 正文更新为完整中文说明，明确只关闭官方 Codex 自动下载和安装更新，不影响 Codex++ 自身更新，并记录跨平台实现范围和验证结果。
- 发布地址：`https://github.com/ygzzfyh123/CodexPPP/releases/tag/v1.2.49`。
- 用户请求：研究并实现 Codex++ 在电脑端使用 API 调用模式时，仍可在设置中额外登录 ChatGPT/Codex 账户，从而与同账户手机 ChatGPT 应用建立远程调用链接。
- 已读取项目根目录、工作记录、Git 状态和文件清单；当前位于干净分支 `远程调用`，仓库包含独立的 `apps/codex-plus-mobile-relay` 应用。
- 已创建修改前检查点提交 `ef642fd`，后续将重点分析 Codex 登录态、API 鉴权、移动端远程调用协议和现有注入桥接的可复用边界。
- 已刷新并核对 2026-07-19 的官方 Codex 手册：设备远控要求主机桌面应用登录与手机相同的 ChatGPT 账号和 workspace；退出 ChatGPT 会关闭 Remote Control；官方 CLI 同时提供实验性的 `codex remote-control start/stop/pair --json`。
- 已确认本机 Codex 桌面版本为 `26.707.9981.0`、CLI 为 `0.144.2`，当前纯 API 登录只在 `auth.json` 中保存 `auth_mode=apikey` 与 API Key，因此没有可供官方远控使用的 ChatGPT token。
- 已确认 Codex++ 现有“官方登录混入 API Key”模式正好具备所需双边界：ChatGPT token 保留在 `auth.json`，自定义 API Key 写入当前 provider 的 `experimental_bearer_token`。
- 已审计旧提交 `bd8a5ef` 的自建手机中继方案；该方案后来已从正式管理器和设置模型移除，只留下未纳入 workspace 的实验应用和部分样式，不应作为本次官方 ChatGPT 手机远控的实现基础。
- 已确定实现方向：新增官方手机远控面板，支持检测账号、发起 ChatGPT 登录、把当前纯 API 供应商迁移为官方登录混入 API、启动或停止官方 Remote Control，并生成短时手动配对码。
- 用户追加任务：官方账号登录不能使用 Codex 专属登录网页，应尽量采用直接登录 ChatGPT 官网的体验，并将官方登录结果安全交给本地 Codex；在此基础上继续实现手机控制 Codex。
- 已调整安全边界：不直接读取浏览器 Cookie 数据库或抓取任意网页令牌，优先定位并复用 OpenAI 官方桌面端或 app-server 的 ChatGPT 登录与本地 token 交换流程。
- 用户进一步明确希望先在 `chatgpt.com` 完成普通 ChatGPT 登录，并提出手工粘贴 Netscape Cookie 文件作为备选。
- 已确认用户提供的示例包含可直接代表网页会话的敏感凭据；不会将其写入代码、日志或工作记录，也不会实现解析、保存或转换 ChatGPT 会话 Cookie 的登录方式。
- 实现方案调整为：先打开 `chatgpt.com` 让用户完成普通网页登录，再复用同一浏览器会话发起官方本地 OAuth 令牌交换；该流程不读取浏览器 Cookie，也不要求用户向 Codex++ 粘贴账号会话密钥。
- 已新增 `official_remote` 核心模块，通过长期存活的 Codex app-server stdio JSON-RPC 会话实现账号登录与官方 Remote Control 管理。
- ChatGPT 登录使用 `appBrand = "chatgpt"`，关闭托管成功页并使用本地回调；管理器提供“打开 ChatGPT 官网”和“连接本机 Codex”两个明确步骤，不读取或导入浏览器 Cookie。
- 已实现登录发起、完成状态轮询和取消操作；登录前备份 `config.toml`、`auth.json` 和供应商设置，失败、取消或迁移异常时自动恢复。
- 已实现纯 API 单供应商到官方混合模式的事务迁移：保留 ChatGPT token 于 `auth.json`，保留自定义 API Key 于当前 provider 的 `experimental_bearer_token`，并将 profile 更新为 `Official + official_mix_api_key`。
- 已拒绝聚合供应商和自定义多模型供应商的自动迁移，避免无法可靠恢复复杂路由时覆盖现有配置。
- 已实现官方手机远控状态读取、启用、关闭、短时手动配对码、配对状态轮询、设备列表和设备撤销。
- 已新增管理器“手机远控”页面，展示 ChatGPT 账号、套餐、远控主机状态、安装/环境标识、配对码和已连接设备，并补齐中英文文案与窄屏布局。
- 已为 app-server 错误增加 URL 查询参数、token 关键词和超长片段脱敏，管理器响应和诊断日志不返回访问令牌、刷新令牌或 Cookie。
- 已验证本机 Codex app-server 的 ChatGPT 品牌登录地址为 OpenAI 官方 OAuth，关闭托管成功页后回调地址为本机环回地址。
- 专项验证通过：3 项 `official_remote` Rust 测试、管理器 `cargo check`、前端 TypeScript 检查、11 项前端测试和 Vite 生产构建。
- 已完成页面视觉检查：桌面端 `1280x720` 无横向溢出，窄屏 `390x844` 的 DOM 尺寸检查无文字裁切或控件重叠；普通浏览器缺少 Tauri bridge 时仅会出现预期的 invoke 测试提示。
- 已扫描仓库，确认用户提供的 Cookie 值和会话凭据未写入代码、日志或工作记录。
