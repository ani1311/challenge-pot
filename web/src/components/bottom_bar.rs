use leptos::prelude::*;
use leptos_router::components::A;

#[component]
pub fn BottomBar() -> impl IntoView {
    view! {
        <footer>
            <A href="/leaderboard">"Leaderboard"</A>
            <A href="/track">"Track"</A>
            <A href="/about">"About"</A>
        </footer>
    }
}
