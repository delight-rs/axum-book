//! Ch 4 提取器（Extractor）示例。
//!
//! 本模块把第 4 章讲到的提取器模型、内置提取器、可选提取器与自定义提取器
//! 集中在一个可运行的服务里。每个行为都有对应的测试断言（见文末 `tests` 模块），
//! 书稿中的结论以这些测试的通过为准。
//!
//! 文中 `// ANCHOR:` / `// ANCHOR_END:` 注释是 mdBook 的引用锚点，
//! 书稿通过 `{{#include ...}}` 直接引用本文件，保证书与代码不漂移。

use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};

use axum::{
    Json, Router,
    body::Bytes,
    extract::{
        DefaultBodyLimit, FromRequestParts, OptionalFromRequestParts, OriginalUri, Path, Query,
        State, rejection::QueryRejection,
    },
    http::{
        HeaderMap, StatusCode, Uri,
        header::{self, AUTHORIZATION},
        request::Parts,
    },
    response::{IntoResponse, Response},
    routing::{get, post},
};
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode, errors::ErrorKind,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

// ANCHOR: app
/// 组装演示用路由表，也是 `main.rs` 启动服务时使用的路由表。
pub fn app(state: AppState) -> Router {
    // 4.2 内置提取器：Path / Query / Json / HeaderMap / Bytes / DefaultBodyLimit。
    let extract_routes = Router::new()
        .route("/path/{id}", get(path_param))
        .route("/query", get(query_params))
        .route("/query/or-default", get(query_or_default))
        .route("/json", post(json_body))
        .route("/json/optional", post(json_optional))
        .route("/headers", get(headers))
        .route("/bytes", post(bytes_body))
        .route(
            "/limited",
            post(limited_body).layer(DefaultBodyLimit::max(64)),
        );

    // 4.3 / 4.4 / 4.5：State、OriginalUri、可选提取器与自定义提取器。
    let api_v1 = Router::new()
        .route("/state", get(show_state))
        .route("/original-uri", get(original_uri))
        .route("/dev-token", post(dev_token))
        .route("/me", get(me))
        .route("/me/optional", get(me_optional))
        .route("/tenant/optional", get(tenant_optional))
        .route("/context", get(context))
        .route("/caller", get(caller))
        .route("/tasks", post(create_task));

    Router::new()
        .nest("/extract", extract_routes)
        .nest("/api/v1", api_v1)
        .with_state(state)
}
// ANCHOR_END: app

// ---- 应用状态：自定义提取器从这里读取 JWT 密钥 ----
// ANCHOR: state
/// 全局共享状态。Ch 7 会展开配置、连接池等真实状态，
/// 这里只保留 AuthUser 验签所需的密钥。
#[derive(Clone)]
pub struct AppState {
    jwt_secret: Arc<str>,
}

impl AppState {
    pub fn new(secret: impl Into<Arc<str>>) -> Self {
        Self {
            jwt_secret: secret.into(),
        }
    }
}
// ANCHOR_END: state

// ---- 4.2 Path 与 Query ----
// ANCHOR: path

async fn path_param(Path(id): Path<u64>) -> String {
    format!("id = {id}")
}

// ANCHOR_END: path

// ANCHOR: query

#[derive(Debug, Deserialize)]
struct PageQuery {
    #[serde(default = "default_page")]
    page: u32,
    #[serde(default = "default_page_size")]
    page_size: u32,
    status: Option<String>,
}

fn default_page() -> u32 {
    1
}

fn default_page_size() -> u32 {
    20
}

async fn query_params(Query(query): Query<PageQuery>) -> String {
    format!(
        "page={}, page_size={}, status={}",
        query.page,
        query.page_size,
        query.status.as_deref().unwrap_or("none")
    )
}
// ANCHOR_END: query

// ---- 4.3 Result 提取器：在 handler 内部处理 rejection ----
// ANCHOR: result_extractor

async fn query_or_default(params: Result<Query<PageQuery>, QueryRejection>) -> String {
    match params {
        Ok(Query(query)) => format!("page={}", query.page),
        Err(_) => "invalid query, using defaults".to_owned(),
    }
}
// ANCHOR_END: result_extractor

// ---- 4.2 Json 与 4.3 Option<Json<T>> ----
// ANCHOR: json

#[derive(Debug, Deserialize, Serialize)]
struct CreateTask {
    title: String,
    #[serde(default)]
    done: bool,
}

async fn json_body(Json(task): Json<CreateTask>) -> (StatusCode, Json<CreateTask>) {
    (StatusCode::CREATED, Json(task))
}
// ANCHOR_END: json

// ANCHOR: json_optional

async fn json_optional(task: Option<Json<CreateTask>>) -> String {
    match task {
        Some(Json(task)) => format!("title = {}", task.title),
        None => "no json body".to_owned(),
    }
}
// ANCHOR_END: json_optional

// ---- 4.2 HeaderMap 与 OriginalUri ----
// ANCHOR: headers

async fn headers(headers: HeaderMap) -> String {
    let user_agent = headers
        .get(header::USER_AGENT)
        .and_then(|value| value.to_str().ok())
        .unwrap_or("unknown");
    let request_id = headers
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .unwrap_or("none");
    format!("user-agent={user_agent}, x-request-id={request_id}")
}
// ANCHOR_END: headers

// ANCHOR: original_uri

async fn original_uri(OriginalUri(original): OriginalUri, uri: Uri) -> String {
    format!("uri={uri}, original={original}")
}
// ANCHOR_END: original_uri

// ---- 4.2 State ----
// ANCHOR: state_extractor

async fn show_state(State(state): State<AppState>) -> String {
    format!("secret length={}", state.jwt_secret.len())
}
// ANCHOR_END: state_extractor

// ---- 4.5 自定义提取器：统一 rejection ----
// ANCHOR: rejections

/// 统一响应信封。Ch 5 会把这里的临时版本替换为全书通用的响应类型。
#[derive(Debug, Serialize)]
struct Envelope<T> {
    code: u32,
    message: String,
    data: T,
}

impl<T> Envelope<T> {
    fn ok(data: T) -> Self {
        Self {
            code: 0,
            message: "ok".to_owned(),
            data,
        }
    }
}

/// 提取器失败时返回的错误。
///
/// 这里手写 HTTP 状态码与响应信封；Ch 5 会把它替换为全书的 `AppError`。
#[derive(Debug)]
pub struct ExtractError {
    status: StatusCode,
    code: u32,
    message: String,
}

impl ExtractError {
    fn new(status: StatusCode, code: u32, message: impl Into<String>) -> Self {
        Self {
            status,
            code,
            message: message.into(),
        }
    }

    fn unauthorized(message: impl Into<String>) -> Self {
        Self::new(StatusCode::UNAUTHORIZED, 30001, message)
    }

    fn forbidden(message: impl Into<String>) -> Self {
        Self::new(StatusCode::FORBIDDEN, 30003, message)
    }

    fn bad_request(code: u32, message: impl Into<String>) -> Self {
        Self::new(StatusCode::BAD_REQUEST, code, message)
    }
}

impl IntoResponse for ExtractError {
    fn into_response(self) -> Response {
        let body = Json(Envelope {
            code: self.code,
            message: self.message,
            data: Value::Null,
        });
        (self.status, body).into_response()
    }
}
// ANCHOR_END: rejections

// ---- 4.5 自定义提取器：AuthUser ----
// ANCHOR: auth_user_struct

/// 通过 JWT 认证后得到的当前用户。
#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: u64,
    pub tenant_id: u64,
}

/// JWT 载荷。字段名遵循 RFC 7519 的注册声明：`sub` 主题、`exp` 过期时间。
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: u64,
    tid: u64,
    exp: usize,
}
// ANCHOR_END: auth_user_struct

// ANCHOR: auth_user_required

impl FromRequestParts<AppState> for AuthUser {
    type Rejection = ExtractError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let token = bearer_token(parts)?;
        decode_token(state, token)
    }
}
// ANCHOR_END: auth_user_required

// ANCHOR: auth_user_optional

impl OptionalFromRequestParts<AppState> for AuthUser {
    type Rejection = ExtractError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        match optional_bearer_token(parts)? {
            Some(token) => decode_token(state, token).map(Some),
            None => Ok(None),
        }
    }
}
// ANCHOR_END: auth_user_optional

// ANCHOR: auth_user_helpers

/// 读取 `Authorization: Bearer <token>`，header 缺失时返回 `None`。
fn optional_bearer_token(parts: &Parts) -> Result<Option<&str>, ExtractError> {
    if parts.headers.get(AUTHORIZATION).is_none() {
        return Ok(None);
    }
    bearer_token(parts).map(Some)
}

/// 读取并校验 `Authorization: Bearer <token>`，header 缺失时返回错误。
fn bearer_token(parts: &Parts) -> Result<&str, ExtractError> {
    let value = parts
        .headers
        .get(AUTHORIZATION)
        .ok_or_else(|| ExtractError::unauthorized("missing Authorization header"))?
        .to_str()
        .map_err(|_| ExtractError::unauthorized("Authorization header is not valid ASCII"))?;

    let (scheme, token) = value
        .split_once(' ')
        .ok_or_else(|| ExtractError::unauthorized("expected `Bearer <token>`"))?;

    if !scheme.eq_ignore_ascii_case("Bearer") {
        return Err(ExtractError::unauthorized("expected `Bearer <token>`"));
    }
    if token.is_empty() {
        return Err(ExtractError::unauthorized("token is empty"));
    }

    Ok(token)
}

fn decode_token(state: &AppState, token: &str) -> Result<AuthUser, ExtractError> {
    let key = DecodingKey::from_secret(state.jwt_secret.as_bytes());
    let validation = Validation::new(Algorithm::HS256);

    let data = decode::<Claims>(token, &key, &validation).map_err(|err| match err.kind() {
        ErrorKind::ExpiredSignature => ExtractError::unauthorized("token expired"),
        _ => ExtractError::unauthorized("invalid token"),
    })?;

    Ok(AuthUser {
        user_id: data.claims.sub,
        tenant_id: data.claims.tid,
    })
}
// ANCHOR_END: auth_user_helpers

// ANCHOR: token_signing

/// 签发一个 1 小时后过期的 HS256 token，仅供示例与测试使用。
pub fn sign_token(state: &AppState, user_id: u64, tenant_id: u64) -> String {
    sign_with_exp(state, user_id, tenant_id, unix_now() + 3600)
}

fn sign_with_exp(state: &AppState, user_id: u64, tenant_id: u64, exp: usize) -> String {
    let claims = Claims {
        sub: user_id,
        tid: tenant_id,
        exp,
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .expect("HS256 signing cannot fail")
}

fn unix_now() -> usize {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock is before the Unix epoch")
        .as_secs() as usize
}
// ANCHOR_END: token_signing

#[derive(Debug, Deserialize)]
struct DevTokenRequest {
    user_id: u64,
    tenant_id: u64,
}

#[derive(Debug, Serialize)]
struct DevTokenData {
    token: String,
    expires_in: u64,
}

/// 开发期辅助接口：直接签发 token，方便用 curl 演示。
/// 生产环境绝不能保留这样的端点，Ch 10 会给出完整的登录流程。
async fn dev_token(
    State(state): State<AppState>,
    Json(request): Json<DevTokenRequest>,
) -> Json<Envelope<DevTokenData>> {
    let token = sign_token(&state, request.user_id, request.tenant_id);
    Json(Envelope::ok(DevTokenData {
        token,
        expires_in: 3600,
    }))
}

#[derive(Debug, Serialize)]
struct UserData {
    user_id: u64,
    tenant_id: u64,
}

async fn me(user: AuthUser) -> Json<Envelope<UserData>> {
    Json(Envelope::ok(UserData {
        user_id: user.user_id,
        tenant_id: user.tenant_id,
    }))
}

async fn me_optional(user: Option<AuthUser>) -> String {
    match user {
        Some(user) => format!("user={}", user.user_id),
        None => "anonymous".to_owned(),
    }
}

// ---- 4.5 自定义提取器：Tenant ----
// ANCHOR: tenant

/// 从 `X-Tenant-Id` 头解析出的租户上下文。
#[derive(Debug, Clone, Copy)]
pub struct Tenant(pub u64);

impl FromRequestParts<AppState> for Tenant {
    type Rejection = ExtractError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        <Self as OptionalFromRequestParts<AppState>>::from_request_parts(parts, state)
            .await?
            .ok_or_else(|| ExtractError::bad_request(10002, "missing X-Tenant-Id header"))
    }
}

impl OptionalFromRequestParts<AppState> for Tenant {
    type Rejection = ExtractError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &AppState,
    ) -> Result<Option<Self>, Self::Rejection> {
        let Some(value) = parts.headers.get("x-tenant-id") else {
            return Ok(None);
        };

        let value = value
            .to_str()
            .map_err(|_| ExtractError::bad_request(10001, "X-Tenant-Id is not valid ASCII"))?;
        let id = value
            .parse::<u64>()
            .map_err(|_| ExtractError::bad_request(10001, "X-Tenant-Id must be a number"))?;

        Ok(Some(Tenant(id)))
    }
}

async fn tenant_optional(tenant: Option<Tenant>) -> String {
    match tenant {
        Some(Tenant(id)) => format!("tenant={id}"),
        None => "tenant=none".to_owned(),
    }
}
// ANCHOR_END: tenant

// ---- 4.4 提取器组合：元组 ----
// ANCHOR: tuple_extractor

async fn context((user, tenant): (AuthUser, Tenant)) -> Json<Envelope<UserData>> {
    Json(Envelope::ok(UserData {
        user_id: user.user_id,
        tenant_id: tenant.0,
    }))
}
// ANCHOR_END: tuple_extractor

// ---- 4.4 提取器组合：自定义组合类型 ----
// ANCHOR: caller

/// 把 AuthUser 与 Tenant 组合成一个上下文类型，并校验两者属于同一租户。
#[derive(Debug)]
pub struct Caller {
    pub user: AuthUser,
    pub tenant: Tenant,
}

impl FromRequestParts<AppState> for Caller {
    type Rejection = ExtractError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &AppState,
    ) -> Result<Self, Self::Rejection> {
        let user =
            <AuthUser as FromRequestParts<AppState>>::from_request_parts(parts, state).await?;
        let tenant =
            <Tenant as FromRequestParts<AppState>>::from_request_parts(parts, state).await?;

        if user.tenant_id != tenant.0 {
            return Err(ExtractError::forbidden(
                "X-Tenant-Id does not match the token",
            ));
        }

        Ok(Self { user, tenant })
    }
}

async fn caller(caller: Caller) -> Json<Envelope<UserData>> {
    Json(Envelope::ok(UserData {
        user_id: caller.user.user_id,
        tenant_id: caller.tenant.0,
    }))
}
// ANCHOR_END: caller

// ANCHOR: handler_order

#[derive(Debug, Serialize)]
struct TaskData {
    title: String,
    done: bool,
    user_id: u64,
    tenant_id: u64,
}

async fn create_task(
    caller: Caller,
    Json(task): Json<CreateTask>,
) -> (StatusCode, Json<Envelope<TaskData>>) {
    (
        StatusCode::CREATED,
        Json(Envelope::ok(TaskData {
            title: task.title,
            done: task.done,
            user_id: caller.user.user_id,
            tenant_id: caller.tenant.0,
        })),
    )
}
// ANCHOR_END: handler_order

// ---- 4.6 请求体限制：Bytes ----
// ANCHOR: body_limit

async fn bytes_body(body: Bytes) -> String {
    format!("received {} bytes", body.len())
}

async fn limited_body(body: Bytes) -> String {
    format!("received {} bytes", body.len())
}
// ANCHOR_END: body_limit

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{body::Body, extract::Request, http::Method};
    use http_body_util::BodyExt;
    use tower::ServiceExt;

    fn test_state() -> AppState {
        AppState::new("test-secret")
    }

    fn bearer(state: &AppState, user_id: u64, tenant_id: u64) -> String {
        format!("Bearer {}", sign_token(state, user_id, tenant_id))
    }

    /// 向路由表发送一个内存请求，返回 (状态码, 响应头, 响应体)。
    async fn send(
        state: AppState,
        method: Method,
        uri: &str,
        headers: &[(&str, &str)],
        body: &str,
    ) -> (StatusCode, HeaderMap, String) {
        let mut builder = Request::builder().method(method).uri(uri);
        for (name, value) in headers {
            builder = builder.header(*name, *value);
        }

        let response = app(state)
            .oneshot(
                builder
                    .body(Body::from(body.to_owned()))
                    .expect("valid request"),
            )
            .await
            .expect("router is infallible");

        let status = response.status();
        let headers = response.headers().clone();
        let body = response.into_body().collect().await.unwrap().to_bytes();

        (
            status,
            headers,
            String::from_utf8(body.to_vec()).expect("utf-8 response"),
        )
    }

    // ---- 4.2 Path / Query ----

    #[tokio::test]
    async fn path_extracts_typed_segment() {
        let state = test_state();

        let (status, _, body) = send(state.clone(), Method::GET, "/extract/path/42", &[], "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "id = 42");

        // Path<u64> 解析失败时，rejection 自动转成 400。
        let (status, _, _) = send(state, Method::GET, "/extract/path/abc", &[], "").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn path_tuple_arity_must_match() {
        // 0.8 要求元组长度与捕获段数量一致；只给一段却要两个值 → 400。
        let router = Router::new().route(
            "/pair/{id}",
            get(|Path((a, b)): Path<(u64, u64)>| async move { format!("{a}-{b}") }),
        );
        let response = router
            .oneshot(
                Request::builder()
                    .uri("/pair/1")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn query_extracts_and_rejects_invalid() {
        let state = test_state();

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/extract/query?page=2&page_size=5&status=todo",
            &[],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "page=2, page_size=5, status=todo");

        // 缺省值来自 #[serde(default = "...")]。
        let (status, _, body) = send(state.clone(), Method::GET, "/extract/query", &[], "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "page=1, page_size=20, status=none");

        let (status, _, _) = send(state, Method::GET, "/extract/query?page=abc", &[], "").await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
    }

    #[tokio::test]
    async fn result_extractor_handles_rejection() {
        let state = test_state();

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/extract/query/or-default?page=abc",
            &[],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "invalid query, using defaults");

        let (status, _, body) = send(
            state,
            Method::GET,
            "/extract/query/or-default?page=3",
            &[],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "page=3");
    }

    // ---- 4.2 Json / 4.3 Option<Json<T>> ----

    #[tokio::test]
    async fn json_body_and_rejections() {
        let state = test_state();
        let json_header = [("content-type", "application/json")];

        let (status, _, body) = send(
            state.clone(),
            Method::POST,
            "/extract/json",
            &json_header,
            r#"{"title":"write ch04"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let task: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(task["title"], "write ch04");
        assert_eq!(task["done"], false);

        // 没有 Content-Type: application/json → 415。
        let (status, _, _) = send(
            state.clone(),
            Method::POST,
            "/extract/json",
            &[],
            r#"{"title":"x"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);

        // JSON 语法错误 → 400。
        let (status, _, _) = send(
            state.clone(),
            Method::POST,
            "/extract/json",
            &json_header,
            r#"{"title":"#,
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);

        // 语法正确但字段类型不匹配 → 422。
        let (status, _, _) = send(
            state,
            Method::POST,
            "/extract/json",
            &json_header,
            r#"{"title":42}"#,
        )
        .await;
        assert_eq!(status, StatusCode::UNPROCESSABLE_ENTITY);
    }

    #[tokio::test]
    async fn optional_json_distinguishes_missing_and_invalid() {
        let state = test_state();

        // 没有 Content-Type → None，handler 正常执行。
        let (status, _, body) = send(
            state.clone(),
            Method::POST,
            "/extract/json/optional",
            &[],
            r#"{"title":"x"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "no json body");

        let (status, _, body) = send(
            state.clone(),
            Method::POST,
            "/extract/json/optional",
            &[("content-type", "application/json")],
            r#"{"title":"x"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "title = x");

        // Content-Type 存在但不是 JSON → 仍然拒绝，而不是 None。
        let (status, _, _) = send(
            state,
            Method::POST,
            "/extract/json/optional",
            &[("content-type", "text/plain")],
            r#"{"title":"x"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::UNSUPPORTED_MEDIA_TYPE);
    }

    // ---- 4.2 HeaderMap / OriginalUri / State ----

    #[tokio::test]
    async fn headers_extractor_reads_metadata() {
        let (status, _, body) = send(
            test_state(),
            Method::GET,
            "/extract/headers",
            &[("user-agent", "ch04-test"), ("x-request-id", "req-1")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "user-agent=ch04-test, x-request-id=req-1");
    }

    #[tokio::test]
    async fn original_uri_keeps_full_path_under_nest() {
        let (status, _, body) =
            send(test_state(), Method::GET, "/api/v1/original-uri", &[], "").await;
        assert_eq!(status, StatusCode::OK);
        // Uri 已被 nest 剥掉前缀，OriginalUri 保留完整路径。
        assert_eq!(body, "uri=/original-uri, original=/api/v1/original-uri");
    }

    #[tokio::test]
    async fn state_is_available_to_handlers() {
        let (status, _, body) = send(test_state(), Method::GET, "/api/v1/state", &[], "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "secret length=11");
    }

    // ---- 4.5 AuthUser ----

    #[tokio::test]
    async fn auth_requires_valid_bearer_token() {
        let state = test_state();

        let (status, _, body) = send(state.clone(), Method::GET, "/api/v1/me", &[], "").await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
        let error: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(error["code"], 30001);

        let token = bearer(&state, 42, 7);
        let (status, _, body) = send(
            state,
            Method::GET,
            "/api/v1/me",
            &[("authorization", token.as_str())],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let user: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(user["data"]["user_id"], 42);
        assert_eq!(user["data"]["tenant_id"], 7);
    }

    #[tokio::test]
    async fn expired_and_malformed_tokens_are_rejected() {
        let state = test_state();

        let expired = format!("Bearer {}", sign_with_exp(&state, 42, 7, 1));
        let (status, _, _) = send(
            state.clone(),
            Method::GET,
            "/api/v1/me",
            &[("authorization", expired.as_str())],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let (status, _, _) = send(
            state,
            Method::GET,
            "/api/v1/me",
            &[("authorization", "Bearer not-a-jwt")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn optional_auth_returns_none_only_when_header_is_missing() {
        let state = test_state();

        let (status, _, body) =
            send(state.clone(), Method::GET, "/api/v1/me/optional", &[], "").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "anonymous");

        // header 存在但 token 非法：请求被拒绝，不会变成 None。
        let (status, _, _) = send(
            state.clone(),
            Method::GET,
            "/api/v1/me/optional",
            &[("authorization", "Bearer not-a-jwt")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);

        let token = bearer(&state, 42, 7);
        let (status, _, body) = send(
            state,
            Method::GET,
            "/api/v1/me/optional",
            &[("authorization", token.as_str())],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "user=42");
    }

    // ---- 4.5 Tenant ----

    #[tokio::test]
    async fn tenant_is_optional_but_validates_present_values() {
        let state = test_state();

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/tenant/optional",
            &[],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "tenant=none");

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/tenant/optional",
            &[("x-tenant-id", "7")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "tenant=7");

        let (status, _, body) = send(
            state,
            Method::GET,
            "/api/v1/tenant/optional",
            &[("x-tenant-id", "abc")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let error: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(error["code"], 10001);
    }

    // ---- 4.4 组合提取器 ----

    #[tokio::test]
    async fn tuple_extractor_runs_multiple_parts_extractors() {
        let state = test_state();
        let token = bearer(&state, 42, 7);

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/context",
            &[("authorization", token.as_str()), ("x-tenant-id", "7")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let context: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(context["data"]["user_id"], 42);
        assert_eq!(context["data"]["tenant_id"], 7);

        let (status, _, _) = send(
            state,
            Method::GET,
            "/api/v1/context",
            &[("x-tenant-id", "7")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn caller_composes_extractors_and_checks_tenant() {
        let state = test_state();
        let token = bearer(&state, 42, 7);

        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/caller",
            &[("authorization", token.as_str()), ("x-tenant-id", "7")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        let caller: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(caller["data"]["user_id"], 42);
        assert_eq!(caller["data"]["tenant_id"], 7);

        // token 属于租户 7，header 却声明租户 8 → 403。
        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/caller",
            &[("authorization", token.as_str()), ("x-tenant-id", "8")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::FORBIDDEN);
        let error: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(error["code"], 30003);

        // 缺少租户头 → 400。
        let (status, _, body) = send(
            state.clone(),
            Method::GET,
            "/api/v1/caller",
            &[("authorization", token.as_str())],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::BAD_REQUEST);
        let error: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(error["code"], 10002);

        // 缺少 token → 401。
        let (status, _, _) = send(
            state,
            Method::GET,
            "/api/v1/caller",
            &[("x-tenant-id", "7")],
            "",
        )
        .await;
        assert_eq!(status, StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn create_task_combines_parts_and_json() {
        let state = test_state();
        let token = bearer(&state, 42, 7);

        let (status, _, body) = send(
            state,
            Method::POST,
            "/api/v1/tasks",
            &[
                ("authorization", token.as_str()),
                ("x-tenant-id", "7"),
                ("content-type", "application/json"),
            ],
            r#"{"title":"write ch04"}"#,
        )
        .await;
        assert_eq!(status, StatusCode::CREATED);
        let task: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(task["data"]["title"], "write ch04");
        assert_eq!(task["data"]["done"], false);
        assert_eq!(task["data"]["user_id"], 42);
        assert_eq!(task["data"]["tenant_id"], 7);
    }

    // ---- 4.6 Bytes 与请求体限制 ----

    #[tokio::test]
    async fn bytes_and_body_limit() {
        let state = test_state();

        let (status, _, body) =
            send(state.clone(), Method::POST, "/extract/bytes", &[], "hello").await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "received 5 bytes");

        let ok_body = "a".repeat(64);
        let (status, _, body) = send(
            state.clone(),
            Method::POST,
            "/extract/limited",
            &[],
            &ok_body,
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        assert_eq!(body, "received 64 bytes");

        let too_large = "a".repeat(65);
        let (status, _, _) = send(state, Method::POST, "/extract/limited", &[], &too_large).await;
        assert_eq!(status, StatusCode::PAYLOAD_TOO_LARGE);
    }
}
