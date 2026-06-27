use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TrackEntryKind {
    EatSugar { grams: f32 },
    Walk { kilometers: f32 },
    Hike { kilometers: f32 },
    Run { kilometers: f32 },
    Swimming { kilometers: f32 },
    Bike { kilometers: f32 },
    MountainBike { kilometers: f32 },
    RaquetSport { hours: f32 },
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackRequest {
    pub kind: TrackEntryKind,
}
