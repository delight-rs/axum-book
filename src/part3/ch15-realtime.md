# Ch 15 实时通信：WebSocket 与 SSE

> 状态：占位（待撰写）

## 本章目标

任务通知/评论实时推送。

## 小节规划

- 15.1 `WebSocketUpgrade`：握手、消息收发、关闭语义
- 15.2 广播模型：`tokio::sync::broadcast` 房间管理、心跳与掉线清理
- 15.3 认证与 WS：握手阶段鉴权（query/header token）
- 15.4 SSE 对比：事件流、自动重连、何时选 SSE 而非 WS
- 15.5 背压与资源上限：连接数限制、写超时

## 配套代码

- `realworld/application/realtime`

## 写作清单

- [ ] 撰写正文
- [ ] 实现代码并通过 `cargo test` 验证
- [ ] 与 Ch 8 背压/取消安全内容交叉引用
- [ ] 添加章末习题
