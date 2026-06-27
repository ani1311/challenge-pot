use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;

mod api;
mod auth;
mod components;
mod pages;

use components::bottom_bar::BottomBar;

use crate::auth::AuthState;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    let auth = AuthState::new();
    provide_context(auth);

    view! {
        <Router>
            <BottomBar/>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=|| view! {<Redirect path="/login"/>}/>
                    <Route path=path!("/challenge-pot") view=|| view! {<Redirect path="/login"/>}/>
                    <Route path=path!("/about") view=pages::About/>
                    <Route path=path!("/login") view=pages::Login/>

                    <ProtectedRoute
                        path=path!("/leaderboard")
                        view=pages::Leaderboard
                        condition=move || Some(auth.is_authenticated())
                        redirect_path=|| "/login"
                    />

                    <ProtectedRoute
                        path=path!("/track")
                        view=pages::Track
                        condition=move || Some(auth.is_authenticated())
                        redirect_path=|| "/login"
                    />
                </Routes>
            </main>
        </Router>
    }
}
