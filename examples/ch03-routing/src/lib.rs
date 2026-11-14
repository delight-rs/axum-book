//! Ch 3 路由系统示例。
//!
//! 本模块把第 3 章讲到的路由组织与匹配规则集中在一个路由表里，
//! 每一条行为都有对应的测试断言（见文末 `tests` 模块），
//! 书稿中的结论以这些测试的通过为准。
//!
//! 文中 `// ANCHOR:` / `// ANCHOR_END:` 注释是 mdBook 的引用锚点，
//! 书稿通过 `{{#include ...}}` 直接引用本文件，保证书与代码不漂移。

use axum::{
    Router,
    extract::{Path, Request},
    http::{StatusCode, Uri},
    middleware::{self, Next},
    response::Response,
    routing::{MethodFilter, any, get, on},
};

// ANCHOR: app
/// 组装演示用路由表，也是 `main.rs` 启动服务时使用的路由表。
pub fn app() -> Router {
    // 3.1 merge：先把不同业务的路由分开组织，再合并成一个总路由。
    let user_routes = Router::new()
        .route("/users", get(list_users).post(create_user))
        .route("/users/me", get(get_me))
        .route(
            "/users/{id}",
            get(get_user).put(update_user).delete(delete_user),
        )
        .route("/users/{id}/tasks", get(list_user_tasks));

    let asset_routes = Router::new()
        .route("/assets", get(list_assets))
        .route("/assets/{*path}", get(get_asset));

    // 3.3 any：所有方法落到同一个 handler。
    // 3.4 nest + route_layer：中间件只包裹注册在它之前的路由。
    let api_v1 = Router::new()
        .route("/ping", any(ping))
        .route("/health", get(health))
        .route_layer(middleware::from_fn(mark_api_middleware));

    Router::new()
        .route("/", get(root))
        .route("/search/{keyword}", get(search))
        .route(
            "/report",
            on(MethodFilter::GET.or(MethodFilter::HEAD), report),
        )
        .merge(user_routes)
        .merge(asset_routes)
        .nest("/api/v1", api_v1)
        .method_not_allowed_fallback(handle_405)
        .fallback(handle_404)
}
// ANCHOR_END: app

// ---- 静态段与捕获段 ----
// ANCHOR: user_handlers

async fn root() -> &'static str {
    "hello from ch03"
}

async fn search(Path(keyword): Path<String>) -> String {
    format!("search: {keyword}")
}

async fn report() -> &'static str {
    "report"
}

async fn list_users() -> &'static str {
    "list users"
}

async fn create_user() -> (StatusCode, &'static str) {
    (StatusCode::CREATED, "user created")
}

async fn get_me() -> &'static str {
    "me"
}

async fn get_user(Path(id): Path<u64>) -> String {
    format!("user {id}")
}

async fn update_user(Path(id): Path<u64>) -> String {
    format!("user {id} updated")
}

async fn delete_user(Path(id): Path<u64>) -> String {
    format!("user {id} deleted")
}

async fn list_user_tasks(Path(id): Path<u64>) -> String {
    format!("tasks of user {id}")
}
// ANCHOR_END: user_handlers

// ---- 通配段 ----
// ANCHOR: asset_handlers

async fn list_assets() -> &'static str {
    "assets index"
}

async fn get_asset(Path(path): Path<String>) -> String {
    format!("asset: {path}")
}
// ANCHOR_END: asset_handlers

// ---- nest 之下的 handler：看到的 Uri 已被剥掉前缀 ----
// ANCHOR: api_handlers

async fn ping(uri: Uri) -> String {
    format!("pong, uri = {uri}")
}

async fn health() -> &'static str {
    "ok"
}
// ANCHOR_END: api_handlers

// ---- 中间件与兜底 ----
// ANCHOR: middleware_and_fallbacks

/// 演示中间件的作用范围：给所有经过它的响应打一个标记头。
async fn mark_api_middleware(req: Request, next: Next) -> Response {
    let mut res = next.run(req).await;
    res.headers_mut()
        .insert("x-api-middleware", "hit".parse().unwrap());
    res
}

async fn handle_405() -> (StatusCode, &'static str) {
    (StatusCode::METHOD_NOT_ALLOWED, "method not allowed")
}

async fn handle_404(uri: Uri) -> (StatusCode, String) {
    (StatusCode::NOT_FOUND, format!("no route for {uri}"))
}
// ANCHOR_END: middleware_and_fallbacks

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{
        body::Body,
        extract::Request,
        http::{HeaderMap, Method},
    };
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    /// 向路由表发送一个内存请求，返回 (状态码, 响应头, 响应体)。
    // ANCHOR: test_send
    async fn send(method: Method, uri: &str) -> (StatusCode, HeaderMap, String) {
        let response = app()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(uri)
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        let status = response.status();
        let headers = response.headers().clone();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        (status, headers, String::from_utf8(body.to_vec()).unwrap())
    }
    // ANCHOR_END: test_send

    // ---- 3.2 路径语法 ----

    // ANCHOR: test_static_and_capture
    #[tokio::test]
    async fn static_and_capture_routes() {
        let (status, _, body) = send(Method::GET, "/").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "hello from ch03");

        let (status, _, body) = send(Method::GET, "/users/42").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "user 42");

        let (status, _, body) = send(Method::GET, "/search/axum").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "search: axum");

        // 捕获段强类型化：非数字 id 解析 u64 失败，自动返回 400。
        let (status, _, _) = send(Method::GET, "/users/abc").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }
    // ANCHOR_END: test_static_and_capture

    // ANCHOR: test_static_beats_capture
    #[tokio::test]
    async fn static_segment_beats_capture() {
        // /users/me 是静态段，优先于 /users/{id} 捕获段。
        let (status, _, body) = send(Method::GET, "/users/me").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "me");
    }
    // ANCHOR_END: test_static_beats_capture

    // ANCHOR: test_wildcard
    #[tokio::test]
    async fn multiple_captures_in_one_path() {
        // 不同段可以各带一个捕获。
        let (status, _, body) = send(Method::GET, "/users/42/tasks").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "tasks of user 42");
    }

    #[tokio::test]
    async fn wildcard_captures_remaining_segments() {
        let (status, _, body) = send(Method::GET, "/assets/js/app.js").await;
        assert_eq!(status, StatusCode::OK);
        // 捕获值不含前导斜杠。
        assert_eq!(body, "asset: js/app.js");
    }

    #[tokio::test]
    async fn wildcard_does_not_match_empty() {
        // /assets/{*path} 不匹配 /assets/，落入 fallback。
        let (status, _, _) = send(Method::GET, "/assets/").await;
        assert_eq!(status, StatusCode::NOT_FOUND);

        // /assets 由静态段精确匹配。
        let (status, _, body) = send(Method::GET, "/assets").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "assets index");
    }
    // ANCHOR_END: test_wildcard

    // ---- 3.3 多方法处理 ----

    #[tokio::test]
    async fn multiple_methods_on_one_path() {
        let (status, _, body) = send(Method::GET, "/users").await;
        assert_eq!((status, body.as_str()), (StatusCode::OK, "list users"));

        let (status, _, body) = send(Method::POST, "/users").await;
        assert_eq!(status, StatusCode::CREATED);
        assert_eq!(body, "user created");
    }

    // ANCHOR: test_405
    #[tokio::test]
    async fn wrong_method_returns_405_with_allow_header() {
        // PATCH /users/{id} 没有注册 handler：路径匹配、方法不匹配。
        // 走 method_not_allowed_fallback，而不是 fallback。
        let (status, headers, body) = send(Method::PATCH, "/users/42").await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(body, "method not allowed");

        // 405 响应自动携带 Allow 头，列出该路径支持的方法。
        let allow = headers.get("allow").unwrap().to_str().unwrap();
        assert!(allow.contains("GET"));
        assert!(allow.contains("PUT"));
        assert!(allow.contains("DELETE"));
    }
    // ANCHOR_END: test_405

    // ANCHOR: test_head
    #[tokio::test]
    async fn get_route_also_serves_head() {
        // get 路由兼收 HEAD 请求（响应体在传输层被剥离）。
        let (status, _, _) = send(Method::HEAD, "/users/me").await;
        assert_eq!(status, StatusCode::OK);
    }
    // ANCHOR_END: test_head

    // ANCHOR: test_on_filter
    #[tokio::test]
    async fn on_method_filter_combines_methods() {
        // on(MethodFilter::GET.or(MethodFilter::HEAD), ...) 与 get() 等价，
        // 也可以组合任意方法位。
        let (status, _, body) = send(Method::GET, "/report").await;
        assert_eq!((status, body.as_str()), (StatusCode::OK, "report"));

        // OPTIONS 没在过滤位里 → 405。
        let (status, _, _) = send(Method::OPTIONS, "/report").await;
        assert_eq!(status, StatusCode::METHOD_NOT_ALLOWED);
    }
    // ANCHOR_END: test_on_filter

    // ANCHOR: test_any
    #[tokio::test]
    async fn any_accepts_all_methods() {
        for method in [Method::GET, Method::POST, Method::PATCH, Method::OPTIONS] {
            let (status, _, body) = send(method.clone(), "/api/v1/ping").await;
            assert_eq!(status, StatusCode::OK, "method = {method}");
            assert_eq!(body, "pong, uri = /ping");
        }
    }
    // ANCHOR_END: test_any

    // ---- 3.1 fallback 语义 ----

    // ANCHOR: test_fallback
    #[tokio::test]
    async fn fallback_only_fires_when_no_route_matches() {
        // 路径不存在 → fallback。
        let (status, _, body) = send(Method::GET, "/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert_eq!(body, "no route for /nope");

        // 路径存在但方法不匹配 → 405（见 wrong_method_returns_405_with_allow_header），
        // 绝不会落入 fallback。
    }
    // ANCHOR_END: test_fallback

    // ---- 3.4 nest 与作用域中间件 ----

    // ANCHOR: test_nest_prefix
    #[tokio::test]
    async fn nest_strips_prefix_from_uri() {
        // ping 被 nest 在 /api/v1 之下，它看到的 Uri 已经被剥掉前缀。
        let (status, _, body) = send(Method::GET, "/api/v1/ping").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "pong, uri = /ping");
    }
    // ANCHOR_END: test_nest_prefix

    // ANCHOR: test_middleware_scope
    #[tokio::test]
    async fn nested_middleware_only_runs_on_matched_routes() {
        // 命中 api_v1 内部路由：中间件生效。
        let (status, headers, _) = send(Method::GET, "/api/v1/health").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(headers.get("x-api-middleware").unwrap(), "hit");

        // 根路由不属于 api_v1：中间件不生效。
        let (_, headers, _) = send(Method::GET, "/").await;
        assert!(headers.get("x-api-middleware").is_none());

        // 路径进了 nest 前缀但没有命中内部路由：route_layer 不执行，
        // 请求落入外层 fallback。
        let (status, headers, _) = send(Method::GET, "/api/v1/nope").await;
        assert_eq!(status, StatusCode::NOT_FOUND);
        assert!(headers.get("x-api-middleware").is_none());
    }
    // ANCHOR_END: test_middleware_scope

    // ---- 常见的 panic 场景（都是开发期配置错误） ----

    // ANCHOR: test_old_syntax
    #[test]
    #[should_panic]
    fn old_capture_syntax_panics() {
        // axum 0.7 及以前的 `:id` 语法在 0.8 会直接 panic。
        let _: Router = Router::new().route("/users/:id", get(get_user));
    }

    #[test]
    #[should_panic]
    fn old_wildcard_syntax_panics() {
        let _: Router = Router::new().route("/assets/*rest", get(get_asset));
    }
    // ANCHOR_END: test_old_syntax

    // ANCHOR: test_panics
    #[test]
    #[should_panic(expected = "Cannot merge two `Router`s")]
    fn merge_two_fallbacks_panics() {
        let a = Router::new().route("/a", get(root)).fallback(handle_404);
        let b = Router::new().route("/b", get(root)).fallback(handle_404);
        let _: Router = a.merge(b);
    }

    #[test]
    #[should_panic(expected = "Nesting at the root")]
    fn nest_at_root_panics() {
        let inner = Router::new().route("/x", get(root));
        let _: Router = Router::new().nest("/", inner);
    }

    #[test]
    #[should_panic]
    fn duplicate_route_panics() {
        let _: Router = Router::new()
            .route("/users", get(list_users))
            .route("/users", get(list_users));
    }

    #[test]
    #[should_panic]
    fn two_different_captures_at_same_level_conflict() {
        // 同一层两个不同名字的捕获段（/users/{id} 与 /users/{name}）
        // 无法共存，注册时 panic。
        let _: Router = Router::new()
            .route("/users/{id}", get(get_user))
            .route("/users/{name}", get(get_user));
    }
    // ANCHOR_END: test_panics
}
