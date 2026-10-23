# Ch 6 中间件与 tower 服务模型

> 状态：占位（待撰写）

## 本章目标

理解"为什么 Axum 的一切都是 tower Service"。

## 小节规划

- 6.1 tower `Service` / `Layer` 模型图解（poll-ready 模型简述）
- 6.2 tower-http 全家桶：`TraceLayer`、`CorsLayer`、`CompressionLayer`、`RequestIdLayer`、`TimeoutLayer`、`CatchPanicLayer`
- 6.3 `ServiceBuilder` 组合与**中间件顺序陷阱**（谁先执行、如何验证）
- 6.4 自定义中间件：`from_fn` 与手写 `Layer`（限流中间件雏形）
- 6.5 作用域：全局 / 路由级 / handler 级（`layer` vs `route_layer`）
- 6.6 `middleware::from_fn_with_state` 与带状态中间件

## 配套代码

- `examples/ch06-middleware`

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码并通过 `cargo test` 验证
- [ ] 绘制 tower 分层执行顺序图解
- [ ] 添加章末习题
