# Ch 18 安全加固

> 状态：占位（待撰写）

## 本章目标

对照 OWASP Top 10 逐项设防。

## 小节规划

- 18.1 输入校验：validator 库、路径/查询参数白名单
- 18.2 请求体大小限制：`DefaultBodyLimit` + multipart 限制
- 18.3 限流：tower_governor（IP + 用户维度）、熔断
- 18.4 注入防护：SQL（sqlx 参数化天然免疫）、XSS（输出编码）、CSRF（API 场景分析）
- 18.5 传输安全：TLS 终结（反向代理/rustls）、安全响应头（HSTS、CSP）、CORS 白名单
- 18.6 依赖安全：cargo-audit、cargo-deny 进 CI
- 18.7 安全审计日志与告警

## 配套代码

- `realworld/api/security`

## 写作清单

- [ ] 撰写正文
- [ ] 实现代码并通过 `cargo test` 验证
- [ ] 输出 OWASP Top 10 对照表
- [ ] 添加章末习题
