# Ch 11 领域建模与项目骨架

> 状态：占位（待撰写）

## 本章目标

建立可长期演进的工程结构。

## 小节规划

- 11.1 workspace 分层：`api` / `application` / `domain` / `infra` / `shared`（轻量 DDD，不过度设计）
- 11.2 领域模型：`Tenant` / `User` / `Project` / `Task` / `Comment` 实体与值对象
- 11.3 用例（use case）层：应用服务与 Repository trait 边界
- 11.4 依赖注入：手写组合根（不引 DI 框架）
- 11.5 错误类型分层：`domain::Error` vs `api::AppError` 映射
- 11.6 数据库 schema 设计：约束、索引、软删除、`updated_at` 触发器

## 配套代码

- `realworld/` 全部 crate 骨架 + migrations v1

## 写作清单

- [ ] 撰写正文
- [ ] 搭建 `realworld/` workspace 骨架并通过 `cargo check`
- [ ] 编写 migrations v1
- [ ] 绘制分层依赖图（禁止跨层依赖）
- [ ] 添加章末习题
