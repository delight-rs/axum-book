# Ch 5 响应与错误处理

> 状态：占位（待撰写）

## 本章目标

建立全书统一的响应与错误体系。

## 小节规划

- 5.1 `IntoResponse` 与响应构造（`Json`、`Html`、状态码、自定义 header）
- 5.2 统一响应规范：`{code, message, data}` 信封 + 业务错误码表
- 5.3 设计 `AppError`：thiserror + 错误分类（参数/业务/认证/上游/内部）
- 5.4 `IntoResponse` for `AppError`：错误码 → HTTP 状态码映射表
- 5.5 错误链与日志：`#[derive(thiserror::Error)]` + tracing 记录
- 5.6 常见坑：handler 返回 `Result` 的 E 必须实现 `IntoResponse`

## 配套代码

- `examples/ch05-error-handling`（此后全书复用该错误体系）

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码并通过 `cargo test` 验证
- [ ] 固化错误码分段规范（`1xxxx`/`2xxxx`/`3xxxx`/`5xxxx`）
- [ ] 添加章末习题
