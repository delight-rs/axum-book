# Ch 16 测试体系

> 状态：占位（待撰写）

## 本章目标

建立可复制到任何 Axum 项目的测试方法论。

## 小节规划

- 16.1 分层策略：单元（领域）/ 用例（fake repo）/ 集成（真实 PG）/ API（oneshot）
- 16.2 `tower::ServiceExt::oneshot` 直测 Router：无需真实端口
- 16.3 testcontainers：测试隔离、加速技巧（模板库、并行）
- 16.4 测试替身：fake / mockall 选择；**优先 fake，少用 mock**
- 16.5 快照测试（insta）与 golden 响应
- 16.6 覆盖率：cargo-llvm-cov 基线（目标：领域层 ≥80%）

## 配套代码

- 全书测试基建 `shared/testing`

## 写作清单

- [ ] 撰写正文
- [ ] 搭建测试基建并通过 `cargo test` 验证
- [ ] 为 `realworld` 各模块补齐测试至基线覆盖率
- [ ] 添加章末习题
