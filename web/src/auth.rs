use leptos::reactive::{
    signal::RwSignal,
    traits::{Get, Set},
};
use web_sys::window;

const TOKEN_STORAGE_KEY: &str = "challenge-pot.jwt";

#[derive(Clone, Copy)]
pub struct AuthState {
    token: RwSignal<Option<String>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            token: RwSignal::new(read_stored_token()),
        }
    }

    pub fn is_authenticated(self) -> bool {
        self.token.get().is_some()
    }

    pub fn login(self, token: String) {
        store_token(&token);
        self.token.set(Some(token));
    }

    pub fn token(self) -> Option<String> {
        self.token.get()
    }
}

fn read_stored_token() -> Option<String> {
    window()?
        .local_storage()
        .ok()??
        .get_item(TOKEN_STORAGE_KEY)
        .ok()?
}

fn store_token(token: &str) {
    if let Some(storage) = window().and_then(|window| window.local_storage().ok().flatten()) {
        let _ = storage.set_item(TOKEN_STORAGE_KEY, token);
    }
}
