use leptos::prelude::*;
use leptos_router::components::*;
use leptos_router::path;


mod components;
mod pages;
mod api;

use components::bottom_bar::BottomBar;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <Router>
            <BottomBar/>
            <main>
                <Routes fallback=|| "Not found.">
                    <Route path=path!("/") view=||"home"/>
                    <Route path=path!("/leaderboard") view=pages::Leaderboard/>
                    <Route path=path!("/track") view=pages::Track/>
                    <Route path=path!("/about") view=pages::About/>
                </Routes>
            </main>
        </Router>
    }
}
