# Ch 9 数据库实战（sqlx + PostgreSQL）

> 状态：占位（待撰写）

## 本章目标

生产级数据访问全链路。

## 小节规划

- 9.1 选型论证：sqlx vs sea-orm vs diesel（迁移成本、编译期校验、运行时行为）
- 9.2 连接池配置与 `PgPool` 注入
- 9.3 Migrations：`sqlx migrate` + 版本化迁移文件规范
- 9.4 离线宏 `query!`：编译期 SQL 校验、`#[derive(FromRow)]`
- 9.5 事务与一致性：`PgTransaction`、嵌套事务、`FOR UPDATE` 乐观/悲观锁
- 9.6 分页与过滤：keyset 分页 vs offset 分页、动态查询构造（QueryBuilder）
- 9.7 慢查询排查：`EXPLAIN ANALYZE`、索引设计、N+1 识别
- 9.8 测试策略：testcontainers 起真实 PG，不 mock 数据库

## 配套代码

- `realworld/infra` + `examples/ch09-db`

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码与迁移文件并通过 `cargo test` 验证
- [ ] 附录 D（sea-orm 对照）与本章呼应
- [ ] 添加章末习题
