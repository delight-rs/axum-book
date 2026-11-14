# Ch N 章节标题

> **本章目标**：一句话说清读完本章能做什么。
>
> 上一章……（与上一章衔接：回顾关键概念，引出本章要解决的问题）

## N.1 第一个大主题

### N.1.1 心智模型：……

用一个生活化类比建立直觉，配一张 mermaid 流程图：

```mermaid
flowchart LR
  A[请求] --> B[Router]
  B --> C[handler]
```

- 类比要点 1：……
- 类比要点 2：……

### N.1.2 代码：……

从 `examples/chXX-主题` 引入代码（不贴不可编译的片段）：

```rust
{{#include ../../examples/chXX-主题/src/main.rs:anchor}}
```

运行方式：

```sh
cd examples/chXX-主题/
cargo run
```

用 curl 演示并给出真实输出：

```sh
curl http://127.0.0.1:3000/xxx
# 期望输出
```

## N.2 第二个大主题

……

## 常见坑

- 坑 1：现象 → 原因 → 规避方式
- 坑 2：……

## 小结

- 本章要点 1
- 本章要点 2
- 本章要点 3

## 习题

1. 基础题：……
2. 进阶题：……
3. 思考题：……

## 本章对应代码路径

- `examples/chXX-主题/`：本章示例工程
- `src/partN/chXX-主题.md`：本章书稿
