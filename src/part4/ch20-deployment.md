# Ch 20 部署与运维

> 状态：占位（待撰写）

## 本章目标

一键交付到生产。

## 小节规划

- 20.1 Docker 多阶段构建：musl/glibc、镜像瘦身（distroless）、`USER` 权限
- 20.2 docker-compose 全栈编排：app + PG + Redis + 可观测组件
- 20.3 Kubernetes：Deployment / Service / Ingress / 探针 / HPA 示例清单
- 20.4 CI/CD：GitHub Actions（fmt → clippy → test → 镜像 → 部署）
- 20.5 发布策略：滚动更新、优雅停机验证、数据库迁移与回滚策略
- 20.6 运维手册：监控指标阈值、故障排查 SOP

## 配套代码

- `deploy/`（Dockerfile、compose、k8s、workflows）

## 写作清单

- [ ] 撰写正文
- [ ] 完成 `deploy/` 全套文件并本地验证 compose 一键起服务
- [ ] 与 Ch 8 优雅停机内容交叉引用
- [ ] 添加章末习题
