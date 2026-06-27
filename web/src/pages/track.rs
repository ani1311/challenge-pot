use common::{TrackEntryKind, TrackRequest};
use leptos::{ev::SubmitEvent, prelude::*, reactive::spawn_local};

use crate::{api, auth::AuthState};

#[derive(Clone, Copy, PartialEq, Eq)]
enum TrackFormKind {
    EatSugar,
    Walk,
    Hike,
    Run,
    Swimming,
    Bike,
    MountainBike,
    RaquetSport,
}

impl TrackFormKind {
    fn value(self) -> &'static str {
        match self {
            Self::EatSugar => "eat-sugar",
            Self::Walk => "walk",
            Self::Hike => "hike",
            Self::Run => "run",
            Self::Swimming => "swimming",
            Self::Bike => "bike",
            Self::MountainBike => "mountain-bike",
            Self::RaquetSport => "raquet-sport",
        }
    }

    fn from_value(value: &str) -> Option<Self> {
        match value {
            "eat-sugar" => Some(Self::EatSugar),
            "walk" => Some(Self::Walk),
            "hike" => Some(Self::Hike),
            "run" => Some(Self::Run),
            "swimming" => Some(Self::Swimming),
            "bike" => Some(Self::Bike),
            "mountain-bike" => Some(Self::MountainBike),
            "raquet-sport" => Some(Self::RaquetSport),
            _ => None,
        }
    }

    fn label(self) -> &'static str {
        match self {
            Self::EatSugar => "Sugar",
            Self::Walk => "Walk",
            Self::Hike => "Hike",
            Self::Run => "Run",
            Self::Swimming => "Swimming",
            Self::Bike => "Bike",
            Self::MountainBike => "Mountain bike",
            Self::RaquetSport => "Racquet sport",
        }
    }

    fn unit(self) -> &'static str {
        match self {
            Self::EatSugar => "grams",
            Self::Walk
            | Self::Hike
            | Self::Run
            | Self::Swimming
            | Self::Bike
            | Self::MountainBike => "kilometers",
            Self::RaquetSport => "hours",
        }
    }

    fn placeholder(self) -> &'static str {
        match self.unit() {
            "grams" => "12.5",
            "kilometers" => "3.0",
            "hours" => "0.5",
            _ => "",
        }
    }

    fn to_track_entry_kind(self, value: f32) -> TrackEntryKind {
        match self {
            TrackFormKind::EatSugar => TrackEntryKind::EatSugar { grams: value },
            TrackFormKind::Walk => TrackEntryKind::Walk { kilometers: value },
            TrackFormKind::Hike => TrackEntryKind::Hike { kilometers: value },
            TrackFormKind::Run => TrackEntryKind::Run { kilometers: value },
            TrackFormKind::Swimming => TrackEntryKind::Swimming { kilometers: value },
            TrackFormKind::Bike => TrackEntryKind::Bike { kilometers: value },
            TrackFormKind::MountainBike => TrackEntryKind::MountainBike { kilometers: value },
            TrackFormKind::RaquetSport => TrackEntryKind::RaquetSport { hours: value },
        }
    }
}

#[component]
pub fn Track() -> impl IntoView {
    let auth = expect_context::<AuthState>();
    let (kind, set_kind) = signal(TrackFormKind::EatSugar);
    let (value, set_value) = signal(String::new());
    let (error, set_error) = signal(None::<String>);
    let (submitting, set_submitting) = signal(false);

    let submit = move |event: SubmitEvent| {
        event.prevent_default();

        let value = match value.get().trim().parse::<f32>() {
            Ok(value) if value >= 0.0 => value,
            _ => {
                set_error.set(Some(format!(
                    "Enter a non-negative number of {}.",
                    kind.get().unit()
                )));
                return;
            }
        };

        let track_entry_kind = kind.get().to_track_entry_kind(value);

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
                    kind: track_entry_kind,
                },
            )
            .await
            {
                Ok(()) => {
                    set_value.set(String::new());
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
                    <span class="field-label">"Activity"</span>
                    <select
                        class="input-field"
                        prop:value=move || kind.get().value()
                        on:change=move |event| {
                            if let Some(selected_kind) = TrackFormKind::from_value(&event_target_value(&event)) {
                                set_kind.set(selected_kind);
                                set_error.set(None);
                            }
                        }
                    >
                        <option value=TrackFormKind::EatSugar.value()>"Sugar"</option>
                        <option value=TrackFormKind::Walk.value()>"Walk"</option>
                        <option value=TrackFormKind::Hike.value()>"Hike"</option>
                        <option value=TrackFormKind::Run.value()>"Run"</option>
                        <option value=TrackFormKind::Swimming.value()>"Swimming"</option>
                        <option value=TrackFormKind::Bike.value()>"Bike"</option>
                        <option value=TrackFormKind::MountainBike.value()>"Mountain bike"</option>
                        <option value=TrackFormKind::RaquetSport.value()>"Racquet sport"</option>
                    </select>
                </label>

                <label class="field">
                    <span class="field-label">
                        {move || format!("{} ({})", kind.get().label(), kind.get().unit())}
                    </span>
                    <input
                        class="input-field"
                        type="number"
                        min="0"
                        step="any"
                        inputmode="decimal"
                        placeholder=move || kind.get().placeholder()
                        prop:value=move || value.get()
                        on:input=move |event| set_value.set(event_target_value(&event))
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
