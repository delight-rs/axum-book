# Ch 7 应用状态与配置管理

> 状态：占位（待撰写）

## 本章目标

状态注入模式与 12-factor 配置。

## 小节规划

- 7.1 `State<T>` 注入：共享不可变状态（连接池、配置）
- 7.2 状态内部可变性：`tokio::sync::RwLock` / `ArcSwap` 选型对比
- 7.3 分层配置：figment/config + 环境变量覆盖 + 敏感信息处理
- 7.4 多环境：dev / test / prod 配置切换
- 7.5 可替换抽象：用 trait 解耦存储层（为 Ch 11 DDD 铺路）
- 7.6 优雅启动流程：配置加载 → 连接池 → 中间件 → 监听

## 配套代码

- `examples/ch07-state-config`

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码并通过 `cargo test` 验证
- [ ] 确定配置库选型（figment 或 config）并固化为全书约定
- [ ] 添加章末习题
