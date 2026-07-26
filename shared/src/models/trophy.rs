use nohash_hasher::IntSet;

use crate::{models::RaceGrade, TrophyRaceRow};

pub struct TrophyModel {
    pub trophy_id: u32,
    pub trophy_type: TrophyType,
    pub trophy_name: String,
    pub race_instance_ids: IntSet<u32>,
    pub race_grade: RaceGrade,
}

impl From<TrophyRaceRow> for TrophyModel {
    fn from(value: TrophyRaceRow) -> Self {
        Self {
            trophy_id: value.trophy_id,
            trophy_type: value.trophy_type.into(),
            trophy_name: value.trophy_name.clone(),
            race_instance_ids: {
                let vec: Vec<u32> = serde_json::from_str(&value.race_instance_ids).unwrap();
                IntSet::from_iter(vec.into_iter())
            },
            race_grade: RaceGrade::from_raw(value.race_grade as i32),
        }
    }
}

pub enum TrophyType {
    Regular,
    Scenario,
    Legend,
}

impl From<u32> for TrophyType {
    fn from(value: u32) -> Self {
        match value {
            1 => Self::Regular,
            2 => Self::Scenario,
            3 => Self::Legend,
            _ => Self::Regular,
        }
    }
}
