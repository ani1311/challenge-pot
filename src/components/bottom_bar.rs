use leptos::prelude::*;

#[component]
pub fn BottomBar() -> impl IntoView {
    view! {
        <footer>
            <button type="button">"leaderboard"</button>
            <button type="button">"Track"</button>
            <button type="button">"About"</button>
        </footer>
    }
}
