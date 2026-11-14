# ch03-routing

《Axum 工业级实战全书》第 3 章「路由系统」配套示例。

一个迷你服务，把本章的匹配规则与组织方式集中在同一个路由表里。
所有行为断言见 `src/lib.rs` 的 `tests` 模块。

## 运行

```sh
cargo run
# Listening on http://127.0.0.1:3000
```

```sh
cargo test
```

## 路由表一览

| 方法           | 路径                | 说明                                       |
| -------------- | ------------------- | ------------------------------------------ |
| GET            | `/`                 | 静态段                                     |
| GET            | `/search/{keyword}` | 单段捕获                                   |
| GET / POST     | `/users`            | 多方法同路径                               |
| GET            | `/users/me`         | 静态段优先于 `/users/{id}`                 |
| GET/PUT/DELETE | `/users/{id}`       | 捕获段强类型化（`Path<u64>`）              |
| GET            | `/users/{id}/tasks` | 多捕获段                                   |
| GET            | `/assets`           | 静态段精确匹配                             |
| GET            | `/assets/{*path}`   | 通配段，不匹配空段                         |
| 任意方法       | `/api/v1/ping`      | `any`；`nest` 前缀已剥离                   |
| GET            | `/api/v1/health`    | `nest` + 作用域中间件                      |
| 其他           | 任何路径            | `method_not_allowed_fallback` / `fallback` |

## 期望行为（示例）

```text
$ curl http://127.0.0.1:3000/users/42
user 42

$ curl http://127.0.0.1:3000/assets/js/app.js
asset: js/app.js

$ curl -X PATCH http://127.0.0.1:3000/users/42 -i
HTTP/1.1 405 Method Not Allowed
allow: GET,HEAD,PUT,DELETE
method not allowed

$ curl http://127.0.0.1:3000/api/v1/ping
pong, uri = /ping

$ curl http://127.0.0.1:3000/nope -i
HTTP/1.1 404 Not Found
no route for /nope
```
