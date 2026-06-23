use common::{TrackEntryKind, TrackRequest};
use leptos::{ev::SubmitEvent, prelude::*, reactive::spawn_local};

use crate::{api, auth::AuthState};

#[component]
pub fn Track() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let (grams, set_grams) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (submitting, set_submitting) = signal(false);

    let submit = move |event: SubmitEvent| {
        event.prevent_default();

        let grams = match grams.get().trim().parse::<f32>() {
            Ok(grams) if grams >= 0.0 => grams,
            _ => {
                set_error.set(Some("Enter a non-negative number of grams.".to_owned()));
                return;
            }
        };
        let Some(token) = auth.token() else {
            set_error.set(Some("Your session has expired. Log in again.".to_owned()));
            return;
        };

        set_error.set(None);
        set_submitting.set(true);

        spawn_local(async move {
            match api::track::track(
                token,
                TrackRequest {
                    kind: TrackEntryKind::SugarGrams,
                    grams,
                },
            )
            .await
            {
                Ok(()) => {
                    set_grams.set(String::new());
                    set_submitting.set(false);
                }
                Err(message) => {
                    set_error.set(Some(message));
                    set_submitting.set(false);
                }
            }
        });
    };

    view! {
        <section class="page page--track">
            <header class="page-header">
                <h1 class="page-title">"Track"</h1>
                <p class="page-subtitle">"Log sugar from something you just ate."</p>
            </header>

            <form class="track-form" on:submit=submit>
                <label class="field">
                    <span class="field-label">"Sugar grams"</span>
                    <input
                        class="input-field"
                        type="number"
                        min="0"
                        step="any"
                        inputmode="decimal"
                        placeholder="12.5"
                        prop:value=move || grams.get()
                        on:input=move |event| set_grams.set(event_target_value(&event))
                    />
                </label>

                {move || error.get().map(|message| view! {
                    <p class="form-error" role="alert">{message}</p>
                })}

                <button
                    class="primary-button"
                    type="submit"
                    disabled=move || submitting.get()
                >
                    {move || if submitting.get() { "Saving..." } else { "Track" }}
                </button>
            </form>
        </section>
    }
}
