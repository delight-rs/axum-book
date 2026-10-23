# Ch 8 异步运行时与并发模型

> 状态：占位（待撰写）

## 本章目标

写出不阻塞、可压测的异步代码。

## 小节规划

- 8.1 tokio 运行时：worker 线程、任务调度速览
- 8.2 阻塞陷阱：CPU 密集/同步 IO → `spawn_blocking` / rayon（配演示代码与对比基准）
- 8.3 背压与超时：`TimeoutLayer`、`tower::limit`、信号量限流
- 8.4 优雅停机：`with_graceful_shutdown` + `CancellationToken` 传导（连池排空、在途请求完成）
- 8.5 连接池原理与调优（sqlx pool 参数：max/min/idle_timeout）
- 8.6 常见坑：`Mutex` vs `RwLock` 跨 await、`select!` 取消安全

## 配套代码

- `examples/ch08-concurrency`（含压测脚本）

## 写作清单

- [ ] 撰写正文
- [ ] 编写示例代码与对比基准并记录数据
- [ ] 添加章末习题
