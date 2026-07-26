use serde::{Deserialize, Serialize};

use crate::models::{Ground, RaceGrade};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScheduleRaceModel {
    pub race_instance_id: u32,
    pub race_id: u32,
    pub race_name: String,
    pub track_name: String,
    pub distance: u16,
    pub distance_grade: RaceDistance,
    pub ground: Ground,
    pub race_group: i64,
    pub grade: RaceGrade,
    // TODO: ADD MONTH AND HALF LOL
    pub race_permission: i64,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum RaceDistance {
    Sprint,
    Mile,
    Medium,
    Long,
}

impl From<u16> for RaceDistance {
    fn from(value: u16) -> Self {
        if value <= 1200 {
            Self::Sprint
        } else if value <= 2000 {
            Self::Mile
        } else if value <= 2500 {
            Self::Medium
        } else {
            Self::Long
        }
    }
}
