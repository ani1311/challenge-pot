use common::TrackEntryKind;
use leptos::prelude::*;

mod sugar_grams_form;
use sugar_grams_form::SugarGramsForm;

#[component]
pub fn Track() -> impl IntoView {
    let (kind, set_kind) = signal(TrackEntryKind::SugarGrams);
    view! {
        <section class="page page--track">
            <header class="page-header">
                <h1 class="page-title">"Track"</h1>
                <p class="page-subtitle">"Log sugar from something you just ate."</p>
            </header>

            <label class="field">
                <span class="field-label">"Type"</span>
                <select
                    class="field-input"
                    on:change=move |event| {
                        let selected = match event_target_value(&event).as_str() {
                            "sugar-grams" => TrackEntryKind::SugarGrams,
                            _ => TrackEntryKind::SugarGrams,
                        };

                        set_kind.set(selected);
                    }
                >
                    <option value="sugar-grams">"Sugar grams"</option>
                </select>
            </label>


            {move || match kind.get() {
                TrackEntryKind::SugarGrams => {
                    view! {
                        <SugarGramsForm/>
                    }.into_any()
                }
            }}

            <button class="primary-button" type="button">"Track"</button>
        </section>
    }
}
