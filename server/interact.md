# Axum JWT Auth Middleware

Goal: add auth at the HTTP/presentation layer first, using a dummy JWT check for now.

## Where It Should Live

- `presentation/http/auth.rs`
  - Axum middleware code.
  - Reads `Authorization` header.
  - Validates dummy JWT.
  - Adds authenticated user info into request extensions.

- `presentation/http.rs`
  - Builds the router.
  - Applies middleware to protected routes.

- `application`
  - Should not know about JWT.
  - Use cases can receive a `UserId` later if they need the authenticated user.

- `domain`
  - No JWT, no Axum, no headers.

## Dummy JWT Rule

For now, keep it simple:

```text
Authorization: Bearer dummy-token
```

If the header is missing or not exactly `Bearer dummy-token`, return `401 Unauthorized`.

## Add Dependencies

You likely already have `axum`. If not:

```bash
cargo add axum -p challenge-pot-server
```

If you want middleware helpers from Tower later:

```bash
cargo add tower -p challenge-pot-server
```

For real JWT later:

```bash
cargo add jsonwebtoken -p challenge-pot-server
```

Do not add `jsonwebtoken` yet unless you are ready to parse/sign real tokens.

## Auth Middleware Shape

In `presentation/http/auth.rs`, write a function like:

```rust
pub async fn auth_middleware(
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // 1. Read Authorization header
    // 2. Check it equals "Bearer dummy-token"
    // 3. If bad, return Err(StatusCode::UNAUTHORIZED)
    // 4. If good, insert authenticated user into request.extensions_mut()
    // 5. Call next.run(request).await
}
```

You will need these imports:

```rust
use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
```

## Auth User Type

For now, define a small HTTP-layer type:

```rust
#[derive(Clone, Debug)]
pub struct AuthUser {
    pub user_id: String,
}
```

On successful dummy auth, insert:

```rust
request.extensions_mut().insert(AuthUser {
    user_id: "user_1".to_string(),
});
```

Later, real JWT parsing should fill this from token claims.

## Applying Middleware

In `presentation/http.rs`, protect only API routes that need login:

```rust
use axum::middleware;
```

Then wrap protected routes:

```rust
let protected_routes = Router::new()
    .route("/leaderboard", get(get_leaderborrd))
    .route_layer(middleware::from_fn(auth::auth_middleware));
```

Then merge into the app:

```rust
Router::new()
    .route("/health", get(health))
    .merge(protected_routes)
    .with_state(state)
```

Keep `/health` public.

## Reading AuthUser In A Handler

In a protected handler, read the authenticated user with:

```rust
use axum::Extension;
```

Handler shape:

```rust
pub async fn some_handler(
    Extension(auth_user): Extension<AuthUser>,
) {
    // auth_user.user_id is available here
}
```

If your handler also uses state:

```rust
pub async fn get_leaderboard(
    State(state): State<AppState>,
    Extension(auth_user): Extension<AuthUser>,
) -> Json<LeaderboardResponse> {
    // call use case here
}
```

## Important Design Rule

JWT validation is presentation/infrastructure concern.

Use cases should receive normal Rust values like:

```rust
user_id: UserId
```

They should not receive:

```rust
Authorization
JWT
Request
HeaderMap
```

## Real JWT Later

Later replace the dummy check with:

1. Parse `Authorization: Bearer <token>`.
2. Decode token using `jsonwebtoken`.
3. Validate signature and expiry.
4. Convert claims into `AuthUser`.
5. Insert `AuthUser` into request extensions.

## Login Error Handling

Use a typed endpoint result, rather than a generic `{ result, error }` response
envelope.

The HTTP handler should call an application use case. The use case depends on
the `UserRepository` port; `FsPersistence` implements that port. The handler
does not read files or contain repository logic.

Its handler should return `Result<Json<LoginResponse>, ApiError>`:

```rust
pub async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, ApiError> {
    let user = application::get_user(
        request.username,
        state.persistence.as_ref(),
    )
    .map_err(ApiError::from)?;

    let claims = Claims {
        sub: user.user_id,
        exp: expiration(),
    };

    let jwt = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(b"some-secret"),
    )
    .map_err(|err| ApiError::internal(err))?;

    Ok(Json(LoginResponse { jwt }))
}
```

Define `ApiError` in the server presentation adapter, for example
`server/src/presentation/http/error.rs`. It should contain an HTTP status, a
stable machine-readable code, and a safe client-facing message, then implement
`IntoResponse` once for it. Export the module from `presentation/http.rs`.

```rust
pub struct ApiError {
    status: StatusCode,
    code: &'static str,
    message: String,
}
```

For example, error responses can have this shared shape:

```json
{
  "code": "invalid_credentials",
  "message": "Invalid username or password"
}
```

Do **not** move `ApiError` into `common`. `ApiError` depends on Axum's
`StatusCode` and `IntoResponse`, which are HTTP-adapter concerns. `common` may
hold a transport-only `ErrorResponse` DTO if both the Rust web client and
server need to deserialize the same JSON shape, but it must not contain status
mapping or Axum types.

### Layer Responsibilities

- The domain/application layer returns typed failures such as `UserNotFound`,
  `InvalidCredentials`, and `RepositoryError`; it does not return `ApiError`.
- Infrastructure maps filesystem/IO failures into repository-level failures.
- The HTTP layer maps those failures to `ApiError` and the appropriate HTTP
  status code.
- Successful responses remain endpoint-specific, such as `LoginResponse`.
- Failed responses use the shared `ApiError` JSON shape.

Do not wrap every success in a generic response object: the HTTP status code
and Rust `Result` already distinguish success from failure.

For real username/password authentication, an unknown user and an incorrect
password should both return `401 Unauthorized` with `invalid_credentials`; do
not disclose whether a username exists. If login is intentionally only a
development user selector, returning `404 Not Found` for an unknown user is
reasonable, but it is not authentication.

### Current Dependency Direction

```text
HTTP handler -> application::get_user -> UserRepository port <- FsPersistence
                      |                                      ^
                      +---- domain User / application errors --+
```

`main.rs` is the composition root: it constructs `FsPersistence` and injects
it into `AppState`. This is the one place where infrastructure is wired to the
application. For a stricter boundary, make `AppState` store an application
port trait object instead of `Arc<FsPersistence>`; handlers then have no
concrete infrastructure type at all. That is an incremental refactor, not a
requirement for handlers to correctly use application use cases today.

## Shared Error Contract for the Web Client

Yes: the web client needs to deserialize the JSON error *body*. It does not
need, and should not receive, the server's `ApiError` implementation.

Use two types with distinct responsibilities:

```text
common::ApiErrorResponse       shared HTTP JSON contract
presentation::http::ApiError  server-only HTTP adapter and status mapping
```

`common/src/api_error.rs` should contain a serializable DTO only. Rename the
current `ApiError` to make that distinction explicit:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}
```

It must not contain `StatusCode`, `IntoResponse`, or any Axum dependency.
Export it from `common/src/lib.rs`.

The server-only adapter uses that DTO when producing a response:

```rust
pub struct ApiError {
    status: StatusCode,
    body: ApiErrorResponse,
}
```

`IntoResponse` converts it to `(status, Json(body))`. The HTTP status stays in
the HTTP transport, while `code` and `message` are the portable payload that
the client can display or branch on.

### Client API Rule

The current `web/src/api/login.rs` calls `resp.json::<LoginResponse>()` for
every response. A `401` JSON error body will therefore be reported as a JSON
parsing failure, rather than the actual API error. Apply the same fix to each
web API function:

```rust
let response = Request::post(&api_url("/login"))
    .json(&LoginRequest { username })?
    .send()
    .await?;

if response.ok() {
    return response.json::<LoginResponse>().await;
}

let error = response.json::<ApiErrorResponse>().await?;
Err(error)
```

The client function should return a typed client error, for example
`Result<LoginResponse, ClientError>`, rather than `Result<LoginResponse,
String>`. `ClientError` can distinguish `Transport`, `Decode`, and
`Api(ApiErrorResponse)` failures. The page can show `ApiErrorResponse.message`
for expected API failures, while logging or showing a generic message for
transport and decoding failures.

### Server `ApiError` Implementation

Keep this server-only type at `server/src/presentation/http/error.rs`:

```rust
use std::io;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use common::api_error::ApiErrorResponse;

pub struct ApiError {
    status: StatusCode,
    body: ApiErrorResponse,
}

impl ApiError {
    pub fn user_lookup(error: io::Error) -> Self {
        match error.kind() {
            io::ErrorKind::NotFound => Self {
                status: StatusCode::NOT_FOUND,
                body: ApiErrorResponse {
                    code: "user_not_found".to_owned(),
                    message: "User not found".to_owned(),
                },
            },
            _ => Self::internal(error),
        }
    }

    pub fn internal(error: impl std::fmt::Display) -> Self {
        tracing::error!(%error, "unexpected API error");

        Self {
            status: StatusCode::INTERNAL_SERVER_ERROR,
            body: ApiErrorResponse {
                code: "internal_error".to_owned(),
                message: "An unexpected error occurred".to_owned(),
            },
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (self.status, Json(self.body)).into_response()
    }
}
```

The shared error body at `common/src/api_error.rs` is:

```rust
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ApiErrorResponse {
    pub code: String,
    pub message: String,
}
```

Export the server module from `presentation/http.rs` with `pub mod error;`.
`tracing::error!` requires the `tracing` server dependency; omit that line
until structured logging is added.
