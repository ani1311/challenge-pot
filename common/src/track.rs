use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum TrackEntryKind {
    SugarGrams,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TrackRequest {
    pub kind: TrackEntryKind,
    pub grams: f32,
}
