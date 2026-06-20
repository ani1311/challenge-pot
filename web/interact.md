# Loading leaderboard data in a Leptos component

`fetch_leaderboard` sends an HTTP request and waits for its response, so it is
an `async` function:

```rust
pub async fn fetch_leaderboard() -> Result<LeaderboardResponse, gloo_net::Error>
```

Its result has two layers:

- `Result` represents whether the HTTP request and JSON decoding succeeded.
- On success, `LeaderboardResponse` contains the rows in its `entries` field.

```text
fetch_leaderboard()
    -> Result<LeaderboardResponse, gloo_net::Error>
    -> Result<{ entries: Vec<LeaderboardEntry> }, error>
```

## Why the component cannot use `.await` directly

A normal Leptos `#[component]` function builds the initial UI synchronously and
returns a view. Rust only permits `.await` inside an `async` function, and a
browser request completes later, after the component has already rendered.

`LocalResource` connects that asynchronous work to Leptos reactivity. It starts
the future in the browser and exposes its current value to the view. Use
`LocalResource` here because the web crate is configured for client-side
rendering (`leptos` has the `csr` feature) and the browser HTTP future does not
need to be `Send`.

## Component implementation

```rust
use leptos::prelude::*;

use crate::api;

#[component]
pub fn Leaderboard() -> impl IntoView {
    let leaderboard = LocalResource::new(|| async {
        api::leaderboard::fetch_leaderboard().await
    });

    view! {
        {move || leaderboard.map(|result| match result {
            Ok(response) => view! {
                <table>
                    <thead>
                        <tr>
                            <th>"Username"</th>
                            <th>"Points"</th>
                        </tr>
                    </thead>
                    <tbody>
                        {response.entries.iter().map(|entry| {
                            let username = entry.user.username.clone();
                            let points = entry.points;

                            view! {
                                <tr>
                                    <td>{username}</td>
                                    <td>{points}</td>
                                </tr>
                            }
                        }).collect_view()}
                    </tbody>
                </table>
            }.into_any(),

            Err(error) => view! {
                <p>"Failed to load leaderboard: " {error.to_string()}</p>
            }.into_any(),
        }).unwrap_or_else(|| view! {
            <p>"Loading leaderboard..."</p>
        }.into_any())}
    }
}
```

## What each part does

1. `LocalResource::new` receives a closure that creates the request future. The
   closure is rerun if it reads reactive values that later change; this closure
   currently reads none, so it loads once for the component instance.
2. Do not call `.expect(...)` inside the resource. Keeping the `Result` lets the
   UI show a useful failure state instead of panicking the application.
3. `leaderboard.map(...)` reactively borrows the loaded value and transforms it
   only when it is available. Unlike `get()`, it does not require the resource
   value to implement `Clone`.
4. The `result` passed to `map` is a borrowed
   `Result<LeaderboardResponse, gloo_net::Error>`, so the match handles either
   `Ok(response)` or `Err(error)` without taking ownership from the resource.
5. `response.entries` is the list to render. The response itself is not an
   iterator.
6. `iter()` borrows the entries. Cloning `username` gives each row an owned
   value that can safely be retained by the rendered view.
7. `unwrap_or_else` handles `None`, which is the loading state. Calling `map`
   inside a reactive view closure causes Leptos to rerender this section when
   the request completes, so no `Suspense` boundary is needed.

The imports `LeaderboardEntry` and `LeaderboardUser` are not required by this
component: their types are inferred from `response.entries`. An unused
`use leaderboard::fetch_leaderboard;` in `web/src/api.rs` can also be removed;
the component calls the public module path directly.
