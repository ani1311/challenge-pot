# Workspace Refactor Plan

Goal: split this project into separate `web` and `server` crates, with a third shared crate for data types used by both.

## Proposed Layout

```text
challenge-pot/
  Cargo.toml
  Cargo.lock
  interact.md

  common/
    Cargo.toml
    src/
      lib.rs

  web/
    Cargo.toml
    index.html
    main.css
    src/
      main.rs
      components.rs
      components/
        bottom_bar.rs
      pages.rs
      pages/
        about.rs
        leaderboard.rs
        track.rs

  server/
    Cargo.toml
    src/
      main.rs
```

## Where Shared Structs Should Go

Use a third crate named `common`.

That crate should contain structs/enums that both the web frontend and server API need to agree on.

Examples:

```rust
pub struct FoodEntry {
    pub name: String,
    pub sugar_grams: f32,
}
```

Good things to keep in `common`:

- request structs
- response structs
- shared domain structs
- enums used by both client and server
- simple validation constants

Avoid putting these in `common`:

- database code
- HTTP handlers
- Leptos components
- server-only auth/session logic
- browser-only UI state

The rule of thumb: if both the REST API and the Leptos app need to know the same shape of data, it belongs in `common`.

## Cargo Workspace

The root `Cargo.toml` should become a workspace file:

```toml
[workspace]
members = ["common", "web", "server"]
resolver = "3"
```

Then each crate gets its own `Cargo.toml`.

## Web Crate

Move the current Leptos app into `web/`.

Current files to move:

```text
Cargo.toml      -> web/Cargo.toml
index.html      -> web/index.html
main.css        -> web/main.css
src/            -> web/src/
```

The web crate should depend on `common`:

```toml
[dependencies]
common = { path = "../common" }
console_error_panic_hook = "0.1.7"
leptos = { version = "0.8.19", features = ["csr"] }
```

Run the web app from `web/`:

```sh
cd web
trunk serve --open
```

## Server Crate

Create a minimal `server` crate with only a main function for now.

The server crate should also depend on `common`:

```toml
[dependencies]
common = { path = "../common" }
```

Later, when the REST API is ready, add Axum or another web framework here.

## Refactor Steps

1. Create `web/`, `server/`, and `common/`.
2. Move the existing Leptos app into `web/`.
3. Replace the root `Cargo.toml` with a workspace `Cargo.toml`.
4. Create `common/Cargo.toml` and `common/src/lib.rs`.
5. Create `server/Cargo.toml` and `server/src/main.rs`.
6. Add `common = { path = "../common" }` to both `web` and `server`.
7. Run `cargo check` from the workspace root.
8. Run `cargo check -p challenge-pot-web`.
9. Run `cargo check -p challenge-pot-server`.
10. Run `trunk serve --open` from `web/`.

## Notes

Do not move code yet. This is the proposed structure. After approval, make the refactor in small steps and verify after each major step.
