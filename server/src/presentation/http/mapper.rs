use common::TrackEntryKind;

use crate::domain::Activity;

impl From<Activity> for TrackEntryKind {
    fn from(activity: Activity) -> Self {
        match activity {
            Activity::EatSugar { grams } => TrackEntryKind::EatSugar { grams },

            Activity::Walk { kilometers } => TrackEntryKind::Walk { kilometers },
            Activity::Hike { kilometers } => TrackEntryKind::Hike { kilometers },
            Activity::Run { kilometers } => TrackEntryKind::Run { kilometers },
            Activity::Swimming { kilometers } => TrackEntryKind::Swimming { kilometers },
            Activity::Bike { kilometers } => TrackEntryKind::Bike { kilometers },
            Activity::MountainBike { kilometers } => TrackEntryKind::MountainBike { kilometers },

            Activity::RaquetSport { hours } => TrackEntryKind::RaquetSport { hours },
        }
    }
}

impl From<TrackEntryKind> for Activity {
    fn from(stored_activity: TrackEntryKind) -> Self {
        match stored_activity {
            TrackEntryKind::EatSugar { grams } => Activity::EatSugar { grams },

            TrackEntryKind::Walk { kilometers } => Activity::Walk { kilometers },
            TrackEntryKind::Hike { kilometers } => Activity::Hike { kilometers },
            TrackEntryKind::Run { kilometers } => Activity::Run { kilometers },
            TrackEntryKind::Swimming { kilometers } => Activity::Swimming { kilometers },
            TrackEntryKind::Bike { kilometers } => Activity::Bike { kilometers },
            TrackEntryKind::MountainBike { kilometers } => Activity::MountainBike { kilometers },

            TrackEntryKind::RaquetSport { hours } => Activity::RaquetSport { hours },
        }
    }
}
