// WARNING: THIS CODE IS AUTOGENERATED.
// DO NOT EDIT!!!

use crate::types::ProximityAlertTriggered;
use crate::types::User;

impl ProximityAlertTriggered {
    /// This function creates an empty struct for the object ProximityAlertTriggered.
    pub fn new(traveler: User, watcher: User, distance: i64) -> Self {
        Self {
            traveler,
            watcher,
            distance,
        }
    }
}
