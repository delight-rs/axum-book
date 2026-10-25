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

### 2.1.2 开发工具

推荐两款 Rust 开发工具，

- [Zed](https://zed.dev/): • 下一代专为高性能与 AI 协同而生的轻量级代码编辑器。由 Rust 语言构建，支持 GPU 硬件加速，拥有极致的启动速度与打字响应率。同时，它原生集成了多款尖端 AI 结对编程助手。本书采用 `Zed` 作为核心开发工具。
- [Vs code](https://code.visualstudio.com/): • 目前全球最成熟、最流行的通用代码编辑器。搭配官方首选的 `rust-analyzer` 插件，即可获得无与伦比的强类型补全、代码重构及宏展开跳转体验，是目前绝大多数 Rust 工程师的生产力标配。

### 2.1.3 可选但推荐的工具

#### bacon

`bacon` 用于监听源码变化，一旦有改动就自动重新编译，重启服务，实现 "改完即生效" 的热重载。

1. 使用 `--locked` 确保依赖树完全锁定，安装最稳定的生产级版本。

   ```sh
     cargo install --local bacon
   ```

2. 安装完成后，我们需要到项目根目录钟初始化 bacon 配置

   ```sh
     bacon --init
   ```

   这会在项目根目录下自动生成一个 `bacon.toml` 配置文件。bacon 的配置文件钟内置了 `check`,`clippy`, `test` 等默认任务(jobs), 我们也可以添加自定义的任务如 `jobs.run`

   ```toml
   # 1. 定义专属的开发热重载运行任务
   [jobs.run]
   command = ["cargo", "run"]
   need_stdout = true
   allow_warnings = true

   # 2. 核心优化：配置文件监控的过滤规则
   [bacon]
   ignore = []
   ```

   然后通过 `bacon run`, 即可在后台拉起 Axum 服务。一旦按下 `Ctrl + S` 保存新修改的内容 bacon 会自动重新执行相关的任务：重新编译，原地重启 Axum 进程等。

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

```

```
