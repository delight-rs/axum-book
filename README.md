# Axum Book

> 基于最新 **axum 0.8+** 的工业级 Web 后端实战教程：从零到生产。

[![mdBook](https://img.shields.io/badge/built%20with-mdBook-blue)](https://rust-lang.github.io/mdBook/)

**当前状态**：完成简介， 详见 [Roadmap.md](./Roadmap.md)。
**在线阅读**: 在线阅读

## 关于本书

| 项目     | 内容                                                                               |
| -------- | ---------------------------------------------------------------------------------- |
| 定位     | 面向中级 Rust 开发者的生产级 Web 后端实战教程                                      |
| 目标读者 | 掌握 Rust 基础（所有权、trait、异步入门）、写过 Web 服务、想用 Axum 做生产项目的人 |
| 读完产出 | 能独立设计并交付一个可观测、可测试、可部署的 Axum 生产级服务                       |
| 技术基线 | Rust 2024 edition · axum 0.8+ · tokio 1.x · hyper 1.x · tower 0.5+                 |
| 主线项目 | 贯穿全书的 `realworld` 项目：多租户任务/内容管理 API                               |

## 全书目录

- **第一篇 · 基础篇**（Ch 1–5）：生态地图 → 最小服务 → 路由 → 提取器 → 响应与错误处理
- **第二篇 · 进阶篇**（Ch 6–10）：中间件与 tower → 状态与配置 → 异步并发 → sqlx 数据库 → 认证
- **第三篇 · 实战篇**（Ch 11–16）：领域建模 → 用户体系 → CRUD 与查询 → 文件上传 → WebSocket/SSE → 测试
- **第四篇 · 生产篇**（Ch 17–20）：可观测性 → 安全加固 → 性能压测 → 部署运维
- **附录**（A–F）：版本速查 · 错误码全表 · 术语表 · sea-orm 对照 · 习题答案 · 延伸阅读

完整大纲见 [Roadmap.md](./Roadmap.md) §3。

## 仓库结构

```text
axum-book/
├── README.md              # 本文件
├── Roadmap.md             # 写作路线图（活文档）
├── book.toml              # mdBook 配置
├── src/                   # 书稿（mdBook 源文件）
│   ├── SUMMARY.md         # 目录
│   ├── part1/ ... part4/  # 四篇正文
│   └── appendix/          # 附录
├── examples/              # （规划中）每章独立小示例
├── realworld/             # （规划中）贯穿全书的主线项目（workspace）
├── benchmarks/            # （规划中）Ch 19 压测脚本与基准
├── deploy/                # （规划中）Docker / compose / k8s / CI-CD
└── book/                  # mdbook build 输出目录
```

## 本地构建与预览

- 需要 [mdBook](https://rust-lang.github.io/mdBook/)：
- 需要 `mdbook-mermaid`：

```sh
cargo install mdbook --version 0.5.4
cargo install mdbook-mermaid # 渲染 mermaid 流程图

# 本地预览（热重载）
mdbook serve --open

# 构建静态站点（输出到 book/）
mdbook build
```

## 写作规范

写作启动前，请先阅读 [Roadmap.md](./Roadmap.md)，核心约定如下：

- **代码优先**：所有概念落到可编译、可运行、有测试的代码上；示例默认带日志、错误处理、配置、测试。
- **统一错误体系**：`AppError`（thiserror）+ 响应信封 `{code, message, data}`，错误码分段 `1xxxx`/`2xxxx`/`3xxxx`/`5xxxx`。
- **每章固定结构**：目标 → 概念 → 代码 → 运行结果 → 常见坑 → 小结 → 习题。
- **版本一致**：全书依赖版本由 CI 统一编译锁定，章节示例与 `examples/`、`realworld/` 真实文件一一对应。

## 许可证

MIT License
