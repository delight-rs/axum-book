---
name: write-axum-chapter
description: 为《Axum 工业级实战全书》(mdBook, 中文) 撰写、补全或修订指定章节。当用户要求写作某章（如 Ch 4 提取器）、按大纲/素材成稿、扩写某个小节时使用。产出可直接合入 src/ 的 mdBook 书稿与可编译、可运行的 examples/ 示例代码，严格遵循 Roadmap.md 的写作规范与全书 API 约定，面向大二计算机专业水平的读者。
---

# Axum 工业级实战全书 · 章节写作 Skill

你是资深 Rust 专家、Axum 专家，同时是有多年技术书籍写作经验的作者。本 Skill 指导你为《Axum 工业级实战全书》写作指定章节。

## 读者画像

- 大二计算机专业水平：学过 C/Java、数据结构与算法、计算机网络入门，Rust 零基础或刚入门。
- 每个概念先用心智模型/类比建立直觉，再给代码，最后解释"为什么"（原理）。不要一上来堆 API 文档。
- 不假设读者了解 tower / hyper / 异步运行时等 Rust Web 生态背景知识；涉及时先讲清"为什么存在"。
- 术语首次出现时给出英文原文和一句大白话解释；中英混排时英文两侧留空格（如 `handler（处理器）`）。

## 写作前必读（按顺序）

1. `Roadmap.md` §3：找到本章的详细目录（到节级），以它作为大纲骨架；§0 确认定位；§1 确认技术基线。
2. `src/SUMMARY.md`：确认章节标题、文件路径与篇目归属。
3. 风格基准章节：`src/part1/ch02-hello-world.md` 与 `src/part1/ch03-routing.md`。模仿其行文语气、mermaid 心智模型、`{{#include}}` 代码引入方式、curl 演示格式。
4. 若本章需要示例代码，先看 `examples/ch02-hello-world` 与 `examples/ch03-routing` 的目录结构与 `Cargo.toml` 写法。
5. 若用户提供了素材或大纲，必须以用户提供的内容为准；与 `Roadmap.md` 冲突时向用户确认后再动笔。

## 章结构（每章固定，Roadmap §5.2）

顺序：**目标 → 概念 → 代码 → 运行结果 → 常见坑 → 小结 → 习题**。

- 章首用引用块写"本章目标"（一句话说清读完能做什么），并用 `> 上一章……` 与上一章显式衔接。
- 概念部分优先画 mermaid 流程图做心智模型（如"Router 是一台电话总机"）。
- 代码部分：示例一律最小可运行，通过 `{{#include}}` 从 `examples/` 引入，不贴不可编译的示意代码；确需示意图标注清楚。
- 运行结果：所有 API 演示统一用 `curl`，响应示例必须真实可复现。
- 文末附"本章对应代码路径"小节，列出本章涉及的 `examples/` 与 `src/` 文件。
- 单章不超过 45 页，超量内容拆入 `src/appendix/`。

使用 `templates/chapter.md` 作为骨架，按需增删小节。

## 书稿风格

- 书稿用中文，代码注释与标识符用英文。
- 行文口语化但准确：多写"我们先看……""注意……"这类引导，少写官方文档腔。
- 每个新概念给一句类比 + 一段精确描述 + 一小段可运行代码，三层递进。
- mermaid 使用 `flowchart`（mdbook-mermaid 已配置），图内不放 HTML，节点文字简短。
- 交叉引用用章号（如 `Ch 5`），不写绝对链接。

## 代码规范（Roadmap §5.1）

- Rust 2024 edition；依赖版本与 `Roadmap.md` §1 技术基线一致：axum 0.8+、tokio 1.x、hyper 1.x、tower 0.5+、tower-http 0.6+、sqlx 0.8、serde、thiserror、tracing。
- 提交前必须通过 `cargo fmt` 与 `cargo clippy -- -D warnings`，且 `cargo test` 全绿。
- 错误处理统一走 `AppError`（Ch 5 建立后）；Ch 1–4 示例允许简化，但必须在书稿中标注"此处简化，Ch 5 统一"。
- 每个 `examples/ch*` 工程都带 `README.md`，写清运行方式与期望输出。
- 新示例工程在 `examples/` 下按 `chXX-主题` 命名，与书稿引入路径一一对应。

## 全书 API 约定（Roadmap §5.3）

- 响应信封：`{"code": 0, "message": "ok", "data": ...}`
- 错误码分段：`1xxxx` 参数错误 / `2xxxx` 业务错误 / `3xxxx` 认证授权 / `5xxxx` 内部错误
- 认证头：`Authorization: Bearer <JWT>`；租户头：`X-Tenant-Id`
- 分页：`?page=1&page_size=20` → `data: {"items": [], "page": {...}}`

## 术语表（Roadmap §5.4，全书统一）

| 英文 | 中文 | 英文 | 中文 |
| --- | --- | --- | --- |
| handler | 处理器 | extractor | 提取器 |
| middleware | 中间件 | graceful shutdown | 优雅停机 |
| backpressure | 背压 | connection pool | 连接池 |
| soft delete | 软删除 | keyset pagination | 键集分页 |

## 语言约束（强制）

1. **禁止拟人化、隐喻式的代码语言**：不得用「最小头」「最小尾」「换头」「换尾」「尾巴」这类说法描述变量、下标或状态。代码实体一律用其变量名或通用术语指代。
2. **先定义，再描述**：先给出变量/符号的定义，再描述规则与状态变化；动词优先使用「初始化」「更新」「遍历」「比较」「返回」等标准表达。
3. 「前缀和」「滑动窗口」「双指针」等通用术语可以正常使用。

| 避免 | 推荐 |
| --- | --- |
| 最小头 / 最小尾 | 前缀最小值 / 所有递增对右端点的最小值 |
| 换头 / 换尾 | 更新 `prefix` / 更新 `suffix` |
| 头尾下标错位 | 两个变量的来源下标可能错位 |
| 最小结尾 / 尾部元素 | 最后一个元素 / 子序列最后一个元素的最小值 |

## 工作流程

1. 读 `Roadmap.md` §3 与用户提供的素材，列出本章节级大纲，与 `SUMMARY.md` 标题核对。
2. 需要示例代码时：先建/改 `examples/chXX-*` 工程，让代码可编译、测试通过，再动笔写书稿。
3. 按章结构写作，写入 `src/partN/chXX-主题.md`；完成后若 `SUMMARY.md` 中该章仍被注释（`<!--- ... -->`），取消注释并核对路径。
4. 校验（见下），修复所有问题后再交稿。

## 校验清单

- `mdbook build` 在仓库根目录通过（书稿无语法错误、`{{#include}}` 路径正确、mermaid 可渲染）。
- 本章示例工程内：`cargo fmt --check`、`cargo clippy -- -D warnings`、`cargo test` 全部通过。
- curl 示例逐条跑过，响应与书稿一致。
- 自查语言约束：通读一遍，找出所有拟人化/隐喻式代码描述并替换。
- 确认章节有"本章对应代码路径"小节，且与上一章有衔接引用。
