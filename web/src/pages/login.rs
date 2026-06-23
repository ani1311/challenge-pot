use leptos::{ev::SubmitEvent, prelude::*, reactive::spawn_local};
use leptos_router::hooks::use_navigate;

use crate::{api, auth::AuthState};

#[component]
pub fn Login() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let navigate = use_navigate();

    let (username, set_username) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (submitting, set_submitting) = signal(false);

    let submit = move |event: SubmitEvent| {
        event.prevent_default();

        let username = username.get().trim().to_owned();

        if username.is_empty() {
            set_error.set(Some("Enter a username".to_owned()));
            return;
        }

        set_error.set(None);
        set_submitting.set(true);

        let navigate = navigate.clone();

        spawn_local(async move {
            match api::login::login(username).await {
                Ok(response) => {
                    auth.login(response.jwt);
                    navigate("/leaderboard", Default::default())
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
                <p class="page-subtitle">"Choose username to continue."</p>
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

                {move || error.get().map(|message| view!{
                    <p class="form-error" role="alert">{message}</p>
                })}

                <button
                    class="primary-button"
                    type="submit"
                    disabled=move || submitting.get()
                >
                    {move|| if submitting.get() { "Logging in..." } else { "Login" }}
                </button>
            </form>
        </section>
    }
}
