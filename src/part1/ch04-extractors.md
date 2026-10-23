# Ch 4 提取器（Extractor）

> 状态：占位（待撰写）

## 本章目标

掌握 Axum 最核心的类型系统。

## 小节规划

- 4.1 提取器模型：`FromRequest` / `FromRequestParts` 与 handler 参数顺序
- 4.2 内置提取器：`Path`、`Query`、`Json`、`State`、`HeaderMap`、`OriginalUri`
- 4.3 `Option<T>` / `Result<T, E>` 与 `OptionalFromRequestParts`（0.8 变化）
- 4.4 提取器组合：元组上限与自定义组合类型
- 4.5 自定义提取器实战：`AuthUser`（从 header 解析 JWT）、`Tenant`（多租户上下文）
- 4.6 请求体限制与 `Bytes` / multipart（预告 Ch 14）

## 配套代码

- `examples/ch04-extractors`

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码并通过 `cargo test` 验证
- [ ] 补齐 `AuthUser` / `Tenant` 提取器雏形
- [ ] 添加章末习题
