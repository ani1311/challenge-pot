use leptos::reactive::{signal::RwSignal, traits::{Get, Set}};

#[derive(Clone, Copy)]
pub struct AuthState {
    token: RwSignal<Option<String>>,
}

impl AuthState {
    pub fn new() -> Self {
        Self {
            token: RwSignal::new(None),
        }
    }

    pub fn is_authenticated(self) -> bool {
        self.token.get().is_some()
    }

    pub fn login(self, token: String) {
        self.token.set(Some(token));
    }

    pub fn token(self) -> Option<String> {
        self.token.get()
    }
}
