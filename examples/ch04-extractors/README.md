# ch04-extractors

《Axum 工业级实战全书》第 4 章「提取器（Extractor）」配套示例。

一个服务把本章的提取器模型、内置提取器、可选提取器、自定义提取器与请求体限制集中在一起。
所有行为断言见 `src/lib.rs` 的 `tests` 模块。

## 运行

```sh
cargo run
# Listening on http://127.0.0.1:3000
```

默认 JWT 密钥是 `ch04-dev-secret`，可以用环境变量覆盖：

```sh
JWT_SECRET=change-me cargo run
```

```sh
cargo test
```

## 路由表

| 方法 | 路径                        | 说明                                        |
| ---- | --------------------------- | ------------------------------------------- |
| GET  | `/extract/path/{id}`        | `Path<u64>`，解析失败返回 400               |
| GET  | `/extract/query`            | `Query<PageQuery>`，带默认值与非法值拒绝    |
| GET  | `/extract/query/or-default` | `Result<Query<_>, QueryRejection>`          |
| POST | `/extract/json`             | `Json<CreateTask>`，415 / 400 / 422         |
| POST | `/extract/json/optional`    | `Option<Json<CreateTask>>`                  |
| GET  | `/extract/headers`          | `HeaderMap`                                 |
| POST | `/extract/bytes`            | `Bytes`，读取完整请求体                     |
| POST | `/extract/limited`          | `DefaultBodyLimit::max(64)`，超限返回 413   |
| GET  | `/api/v1/state`             | `State<AppState>`                           |
| GET  | `/api/v1/original-uri`      | `Uri` 与 `OriginalUri` 在 `nest` 下的差异   |
| POST | `/api/v1/dev-token`         | 开发期签发 JWT，生产环境禁止保留            |
| GET  | `/api/v1/me`                | `AuthUser`，缺失或非法 token 返回 401       |
| GET  | `/api/v1/me/optional`       | `Option<AuthUser>`，仅缺失 header 时为 None |
| GET  | `/api/v1/tenant/optional`   | `Option<Tenant>`，仅缺失 header 时为 None   |
| GET  | `/api/v1/context`           | `(AuthUser, Tenant)` 元组提取器             |
| GET  | `/api/v1/caller`            | `Caller`，组合 AuthUser + Tenant 并校验租户 |
| POST | `/api/v1/tasks`             | `Caller + Json<T>`，请求体提取器必须在最后  |

## 取一个开发用 token

```sh
TOKEN=$(curl -s -X POST http://127.0.0.1:3000/api/v1/dev-token \
  -H 'content-type: application/json' \
  -d '{"user_id":42,"tenant_id":7}' | jq -r '.data.token')
```

没有 `jq` 时：

```sh
TOKEN=$(curl -s -X POST http://127.0.0.1:3000/api/v1/dev-token \
  -H 'content-type: application/json' \
  -d '{"user_id":42,"tenant_id":7}' \
  | python3 -c 'import sys,json; print(json.load(sys.stdin)["data"]["token"])')
```

## 期望行为（示例）

```text
$ curl http://127.0.0.1:3000/extract/path/42
id = 42

$ curl -i http://127.0.0.1:3000/extract/path/abc
HTTP/1.1 400 Bad Request

$ curl 'http://127.0.0.1:3000/extract/query?page=2&page_size=5&status=todo'
page=2, page_size=5, status=todo

$ curl -X POST http://127.0.0.1:3000/extract/json \
    -H 'content-type: application/json' -d '{"title":"write ch04"}'
{"title":"write ch04","done":false}

$ curl http://127.0.0.1:3000/api/v1/original-uri
uri=/original-uri, original=/api/v1/original-uri

$ curl http://127.0.0.1:3000/api/v1/me -H "Authorization: Bearer $TOKEN"
{"code":0,"message":"ok","data":{"user_id":42,"tenant_id":7}}

$ curl http://127.0.0.1:3000/api/v1/caller \
    -H "Authorization: Bearer $TOKEN" -H 'X-Tenant-Id: 8'
{"code":30003,"message":"X-Tenant-Id does not match the token","data":null}

$ curl -i -X POST http://127.0.0.1:3000/extract/limited --data-binary @body65.txt
HTTP/1.1 413 Payload Too Large
```

`/extract/json/optional` 的“没有 Content-Type”分支需要注意：`curl -d` 会自动补上
`Content-Type: application/x-www-form-urlencoded`，演示时要显式移除：

```sh
curl -X POST http://127.0.0.1:3000/extract/json/optional \
  -H 'Content-Type:' -d '{"title":"x"}'
# no json body
```
