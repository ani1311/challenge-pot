use leptos::prelude::*;


#[component]
pub fn SugarGramsForm() -> impl IntoView {
    let (grams, set_grams) = signal(String::new());

    view! {
        <form class="track-form">
            <label class="field">
                <span class="field-label">"Sugar grams"</span>
                <input
                    class="input-field"
                    type="number"
                    min="0"
                    step="any"
                    inputmode="decimal"
                    placeholder="12.5"
                    on:input=move |event| {
                        set_grams.set(event_target_value(&event));
                    }
                />
            </label>
        </form>
    }
}
