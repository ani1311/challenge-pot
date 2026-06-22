# Implementing the login form

The router and `AuthState` are already in place. The remaining work is:

1. Add an API client for `POST /login`.
2. Render a username form in `pages/login.rs`.
3. On a successful response, store the JWT and navigate to `/leaderboard`.

Keep the HTTP code out of the component. The component should only manage form
state and call the API function.

## 1. Export a login API module

Update `web/src/api.rs`:

```rust
pub mod client;
pub mod leaderboard;
pub mod login;
```

## 2. Add `web/src/api/login.rs`

`LoginRequest`, `LoginResponse`, and `ApiErrorResponse` exist in the `common`
crate. Deserialize a successful response as `LoginResponse`; for a non-2xx
response, deserialize `ApiErrorResponse` and map its typed `ErrorCode` to a
display string. Keep the API function's existing `Result<LoginResponse,
String>` return type so the page does not need to change.

```rust
use common::{
    api_error::{ApiErrorResponse, ErrorCode},
    login::{LoginRequest, LoginResponse},
};
use gloo_net::http::Request;

use crate::api::client::api_url;

pub async fn login(username: String) -> Result<LoginResponse, String> {
    let response = Request::post(&api_url("/login"))
        .json(&LoginRequest { username })
        .map_err(|error| error.to_string())?
        .send()
        .await
        .map_err(|error| error.to_string())?;

    if response.ok() {
        return response
            .json::<LoginResponse>()
            .await
            .map_err(|error| error.to_string());
    }

    let error = response
        .json::<ApiErrorResponse>()
        .await
        .map_err(|_| "Could not log in. Try again.".to_owned())?;

    match error.code {
        ErrorCode::UserNotFound => Err("That username does not exist.".to_owned()),
        ErrorCode::InternalError => Err("Could not log in. Try again.".to_owned()),
    }
}
```

## 3. Replace `web/src/pages/login.rs`

```rust
use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_navigate;

use crate::{api, auth::AuthState};

#[component]
pub fn Login() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (submitting, set_submitting) = signal(false);

    let submit = move |event: leptos::ev::SubmitEvent| {
        event.prevent_default();

        let username = username.get().trim().to_owned();

        if username.is_empty() {
            set_error.set(Some("Enter a username.".to_owned()));
            return;
        }

        set_error.set(None);
        set_submitting.set(true);

        spawn_local(async move {
            match api::login::login(username).await {
                Ok(response) => {
                    auth.login(response.jwt);
                    navigate("/leaderboard", Default::default());
                }
                Err(message) => {
                    set_error.set(Some(message));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <section class="page page--login">
            <header class="page-header">
                <h1 class="page-title">"Login"</h1>
                <p class="page-subtitle">"Choose a username to continue."</p>
            </header>

            <form class="login-form" on:submit=submit>
                <label class="field">
                    <span class="field-label">"Username"</span>
                    <input
                        class="field-input"
                        type="text"
                        required
                        autocomplete="username"
                        prop:value=move || username.get()
                        on:input=move |event| set_username.set(event_target_value(&event))
                    />
                </label>

                {move || error.get().map(|message| view! {
                    <p class="form-error" role="alert">{message}</p>
                })}

                <button
                    class="primary-button"
                    type="submit"
                    disabled=move || submitting.get()
                >
                    {move || if submitting.get() { "Logging in..." } else { "Login" }}
                </button>
            </form>
        </section>
    }
}
```

## Flow

Submitting the form prevents the browser's normal form navigation, validates
the username, and starts an asynchronous request. On success, the response JWT
is stored in `AuthState`, so `ProtectedRoute` permits protected pages, then
`navigate` redirects to `/leaderboard`. On failure, the error is rendered and
the button is re-enabled.
