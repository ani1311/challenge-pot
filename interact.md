# Track Page Plan

The Track page should keep the selector/switch architecture, even though there is only one tracking type right now.

Reason: later each selected type can render a different form with different fields.

Current intended structure:

```text
web/src/pages/
  track.rs
  track/
    sugar_grams_form.rs
```

Current shared type:

```rust
pub enum TrackEntryKind {
    SugarGrams,
}
```

# Parent Page

`web/src/pages/track.rs` should own:

- the selected `TrackEntryKind` signal
- the selector
- the reactive `match`
- choosing which child form to render

This is the right shape:

```rust
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
                view! { <SugarGramsForm/> }.into_any()
            }
        }}
    </section>
}
```

This follows the Leptos control-flow pattern:

```rust
{move || match signal.get() {
    Variant => view! { <SomeComponent/> }.into_any(),
}}
```

Why this is good:

- `kind.get()` makes the closure reactive.
- changing the selector updates `kind`.
- the `match` reruns and renders the correct form.
- `.into_any()` keeps this working later when match branches return different component types.

# Child Form

`web/src/pages/track/sugar_grams_form.rs` should own only the fields for sugar grams.

It should not know about the selector.

It should own:

- grams input state
- submit behavior for sugar grams

Current issue in the code:

```rust
let (calories, set_calories) = signal(0.0);
```

This should be grams, not calories.

Also, the current view says:

```rust
<h1>"About"</h1>
```

That is placeholder text and should become the sugar grams form.

Suggested shape:

```rust
use leptos::prelude::*;

#[component]
pub fn SugarGramsForm() -> impl IntoView {
    let (grams, set_grams) = signal(String::new());

    view! {
        <form class="track-form">
            <label class="field">
                <span class="field-label">"Sugar grams"</span>
                <input
                    class="field-input"
                    type="number"
                    min="0"
                    step="any"
                    inputmode="decimal"
                    placeholder="12.5"
                    prop:value=grams
                    on:input=move |event| {
                        set_grams.set(event_target_value(&event));
                    }
                />
            </label>

            <button class="primary-button" type="submit">"Track"</button>
        </form>
    }
}
```

Use `String` for form state because browser input values are strings.

Parse on submit:

```rust
let grams = grams.get().parse::<f32>();
```

Then validate:

```rust
if grams < 0.0 {
    // reject input
}
```

# Common Types

`common/src/track.rs` currently only has:

```rust
pub enum TrackEntryKind {
    SugarGrams,
}
```

That is enough for rendering the selector/switch.

When the form starts submitting to the server, add:

```rust
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackRequest {
    pub kind: TrackEntryKind,
    pub grams: f32,
}
```

Then re-export it from `common/src/lib.rs`:

```rust
pub use track::{TrackEntryKind, TrackRequest};
```

For now, if there is no network submit yet, `TrackRequest` can wait.

# CSS Classes Needed

Use these classes for the Track page:

```text
page
page--track
page-header
page-title
page-subtitle
field
field-label
field-input
track-form
primary-button
```

Recommended ownership:

- `page page--track`: outer Track page section
- `page-header`: title/subtitle wrapper
- `field`: label + input/select group
- `field-label`: label text
- `field-input`: select/input styling
- `track-form`: sugar grams form container
- `primary-button`: submit button

# Current Code Fix List

In `web/src/pages/track.rs`:

- keep the selector
- fill in the `on:change` handler
- keep the reactive `match`
- keep rendering `SugarGramsForm`

In `web/src/pages/track/sugar_grams_form.rs`:

- rename `calories` state to `grams`
- use `String::new()` instead of `0.0` for input state
- replace the `"About"` placeholder with a form
- use `type="number"`, `step="any"`, and `inputmode="decimal"`

Do not remove the selector just because there is only one option right now. The selector is part of the intended expandable design.

# Input Cursor Jump Debug

Current input:

```rust
<input
    class="input-field"
    type="number"
    min="0"
    step="any"
    inputmode="decimal"
    placeholder="12.5"
    prop:value=grams
    on:input=move |event| {
        set_grams.set(event_target_value(&event));
    }
/>
```

The glitch is likely caused by:

```rust
prop:value=grams
```

That makes the input controlled. On every `input` event:

1. browser updates the input
2. Leptos reads the value
3. signal updates
4. Leptos writes `prop:value` back into the input

For `type="number"`, values like `.` or `12.` are awkward intermediate states. The browser may normalize them, and the controlled write can reset the cursor position.

## Recommended Fix

For this form, use an uncontrolled input:

```rust
<input
    class="field-input"
    type="number"
    min="0"
    step="any"
    inputmode="decimal"
    placeholder="12.5"
    on:input=move |event| {
        set_grams.set(event_target_value(&event));
    }
/>
```

Remove:

```rust
prop:value=grams
```

The signal will still update as the user types, but Leptos will not rewrite the input value on every keystroke.

## Even Smoother Option

If `type="number"` still behaves badly with decimal typing, use `type="text"` with decimal keyboard hints:

```rust
<input
    class="field-input"
    type="text"
    inputmode="decimal"
    placeholder="12.5"
    on:input=move |event| {
        set_grams.set(event_target_value(&event));
    }
/>
```

Then parse and validate on submit:

```rust
let grams = grams.get().parse::<f32>();
```

This usually gives the best typing experience because the browser does not try to enforce numeric formatting while the user is still editing.

## Small Class Name Issue

The current input uses:

```rust
class="input-field"
```

The notes/CSS plan use:

```rust
class="field-input"
```

Pick one class name and use it consistently. Prefer `field-input` because it matches the `field` / `field-label` naming.
