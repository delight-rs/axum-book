# Ch 2 快速上手：最小可用服务

> **本章目标**：跑起第一个 Axum 服务，并搭建好热更新的开发模式

## 2.1 环境准备

### 2.1.1 安装 Rust 工具链

Rust 工具链由 `rustup` 统一管理，包含编译器 `rustc` 和包管理器 `cargo`。 先确认它们可用:

```sh
rustc --version
cargo --version
```

如果在 terminal 中看到类似下面的输出，说明环境已就绪：

```text
rustc 1.85.0 (...)
cargo 1.85.0 (...)
```

本书统一使用 **Rust 2024 edition**, 要求 rustc 1.85 或更高的版本。如果版本过旧，请先升级：

```sh
rustup update
```

如果还没有安装 Rust, 请到 [Installing Rust](https://rust-lang.org/learn/get-started/) 按官网说明安装对应平台的 Rust。

## 本章目标

15 分钟内跑起第一个服务，跑通开发闭环。

## 小节规划

- 2.1 环境准备（rustup、cargo-watch、just）
- 2.2 Hello, Axum：`axum::serve` + `Router` + 路由处理器
- 2.3 工程结构规范（bin/lib 分层、模块划分）
- 2.4 开发工作流：热重载、`RUST_LOG`、快速迭代

## 配套代码

- `examples/ch02-hello-world`（含 `README.md`：运行方式 + 期望输出）

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码并通过 `cargo run` 验证
- [ ] 补齐运行演示与常见坑
- [ ] 添加章末习题
