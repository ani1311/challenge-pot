use leptos::prelude::*;

mod components;

use components::bottom_bar::BottomBar;

fn main() {
    console_error_panic_hook::set_once();

    mount_to_body(App);
}

#[component]
fn App() -> impl IntoView {
    view! {
        <BottomBar/>
    }
}
