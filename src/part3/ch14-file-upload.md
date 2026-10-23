# Ch 14 文件上传与静态资源

> 状态：占位（待撰写）

## 本章目标

处理文件的大流量路径。

## 小节规划

- 14.1 multipart 上传：`Multipart` 提取、大小限制、流式落盘（`tokio::fs`）
- 14.2 对象存储抽象：本地磁盘实现 + S3 兼容实现（trait 切换）
- 14.3 静态资源：`tower_http::services::ServeDir` + 缓存头 + SPA fallback
- 14.4 文件下载：流式响应（`Body::from_stream`）、断点续传（Range）
- 14.5 安全：类型校验（白名单）、病毒扫描（外部服务集成）、签名 URL

## 配套代码

- `realworld/application/file`

## 写作清单

- [ ] 撰写正文
- [ ] 实现代码并通过 `cargo test` 验证
- [ ] 与 Ch 4 请求体限制内容交叉引用
- [ ] 添加章末习题
