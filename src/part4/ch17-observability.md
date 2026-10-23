# Ch 17 可观测性

> 状态：占位（待撰写）

## 本章目标

服务上线后"看得见、查得到"。

## 小节规划

- 17.1 tracing 体系：spans 设计规范（request → usecase → sql）、字段约定
- 17.2 结构化日志：tracing-subscriber JSON formatter、日志级别管理
- 17.3 request-id 全链路贯穿：`RequestIdLayer` + 响应头 + 日志关联
- 17.4 Metrics：`axum-prometheus` / 自定义指标（请求耗时、QPS、业务指标）
- 17.5 OpenTelemetry：trace 导出到 Jaeger/Tempo（含 docker-compose 演示）
- 17.6 健康检查：`/healthz`（liveness）/ `/readyz`（readiness，含 DB/Redis 探测）

## 配套代码

- `realworld/api/observability`

## 写作清单

- [ ] 撰写正文
- [ ] 实现代码并本地起 Jaeger 演示 trace 链路
- [ ] 与 Ch 6 `TraceLayer`、Ch 8 内容交叉引用
- [ ] 添加章末习题
