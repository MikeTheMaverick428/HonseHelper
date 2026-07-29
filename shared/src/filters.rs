use serde::{Deserialize, Serialize};

use crate::{db_models::UmaHash, legacy_planner::SparkGroupInfo};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct CharacterFilter {
    pub ids: Vec<i64>,
    pub negate: bool,
    pub on_parent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct TraineeFilter {
    pub ids: Vec<i64>,
    pub negate: bool,
    pub on_parent: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct SparkFilter {
    pub group_id: i32,
    pub min_stars: Option<i32>,
    pub max_stars: Option<i32>,
    pub on_trainee: bool,
    pub shared_count: Option<i8>,
    pub spark_type: Option<i64>,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct WhiteSparkFilter {
    pub group_ids: Vec<i64>,
    pub min_stars: Option<i32>,
    pub max_stars: Option<i32>,
    pub on_trainee: bool,
    pub shared_count: Option<i8>,
}

impl WhiteSparkFilter {
    pub fn matches(&self, group: &SparkGroupInfo, skip_group_id: bool) -> bool {
        if !skip_group_id && !self.group_ids.contains(&(group.spark_group_id as i64)) {
            return false;
        }

        if self.on_trainee && group.trainee_stars_veteran <= 0 {
            return false;
        }

        if let Some(shared_count) = self.shared_count {
            if group.uma_count < shared_count {
                return false;
            }
        }

        let total_stars = if self.on_trainee {
            group.trainee_stars_veteran as i32
        } else {
            group.total_stars as i32
        };

        match (self.min_stars, self.max_stars) {
            (Some(min), Some(max)) => total_stars >= min && total_stars <= max,
            (Some(min), None) => total_stars >= min,
            (None, Some(max)) => total_stars <= max,
            (None, None) => true,
        }
    }
}

impl SparkFilter {
    pub fn matches(&self, group: &SparkGroupInfo, skip_group_id: bool) -> bool {
        if !skip_group_id && group.spark_group_id != self.group_id as i64 {
            return false;
        }

        if self.on_trainee && group.trainee_stars_veteran <= 0 {
            return false;
        }

        if let Some(shared_count) = self.shared_count {
            if group.uma_count < shared_count {
                return false;
            }
        }

        let total_stars = if self.on_trainee {
            group.trainee_stars_veteran as i32
        } else {
            group.total_stars as i32
        };

        match (self.min_stars, self.max_stars) {
            (Some(min), Some(max)) => total_stars >= min && total_stars <= max,
            (Some(min), None) => total_stars >= min,
            (None, Some(max)) => total_stars <= max,
            (None, None) => true,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AptitudeType {
    #[serde(alias = "Turf")]
    Turf,
    #[serde(alias = "Dirt")]
    Dirt,
    #[serde(alias = "Sprint")]
    Sprint,
    #[serde(alias = "Mile")]
    Mile,
    #[serde(alias = "Medium")]
    Medium,
    #[serde(alias = "Long")]
    Long,
    #[serde(alias = "Front")]
    Front,
    #[serde(alias = "PaceChaser")]
    PaceChaser,
    #[serde(alias = "LateSurger")]
    LateSurger,
    #[serde(alias = "EndCloser")]
    EndCloser,
}

impl AptitudeType {
    pub fn column_name(&self) -> &'static str {
        match self {
            AptitudeType::Turf => "aptitude_ground_turf",
            AptitudeType::Dirt => "aptitude_ground_dirt",
            AptitudeType::Sprint => "aptitude_dist_sprint",
            AptitudeType::Mile => "aptitude_dist_mile",
            AptitudeType::Medium => "aptitude_dist_medium",
            AptitudeType::Long => "aptitude_dist_long",
            AptitudeType::Front => "aptitude_style_front",
            AptitudeType::PaceChaser => "aptitude_style_pace_chaser",
            AptitudeType::LateSurger => "aptitude_style_late_surger",
            AptitudeType::EndCloser => "aptitude_style_end_closer",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum StatType {
    Speed,
    Stamina,
    Power,
    Guts,
    Wisdom,
}

impl StatType {
    pub fn all() -> &'static [StatType] {
        &[
            StatType::Speed,
            StatType::Stamina,
            StatType::Power,
            StatType::Guts,
            StatType::Wisdom,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            StatType::Speed => "Speed",
            StatType::Stamina => "Stamina",
            StatType::Power => "Power",
            StatType::Guts => "Guts",
            StatType::Wisdom => "Wisdom",
        }
    }

    pub fn value(&self) -> &'static str {
        match self {
            StatType::Speed => "speed",
            StatType::Stamina => "stamina",
            StatType::Power => "power",
            StatType::Guts => "guts",
            StatType::Wisdom => "wisdom",
        }
    }

    pub fn from_str(s: &str) -> Option<StatType> {
        match s {
            "speed" => Some(StatType::Speed),
            "stamina" => Some(StatType::Stamina),
            "power" => Some(StatType::Power),
            "guts" => Some(StatType::Guts),
            "wisdom" => Some(StatType::Wisdom),
            _ => None,
        }
    }

    pub fn growth_column_name(&self) -> &'static str {
        match self {
            StatType::Speed => "growth_rate_spe",
            StatType::Stamina => "growth_rate_sta",
            StatType::Power => "growth_rate_str",
            StatType::Guts => "growth_rate_gut",
            StatType::Wisdom => "growth_rate_wit",
        }
    }
}

impl AptitudeType {
    pub fn all() -> &'static [AptitudeType] {
        &[
            AptitudeType::Turf,
            AptitudeType::Dirt,
            AptitudeType::Sprint,
            AptitudeType::Mile,
            AptitudeType::Medium,
            AptitudeType::Long,
            AptitudeType::Front,
            AptitudeType::PaceChaser,
            AptitudeType::LateSurger,
            AptitudeType::EndCloser,
        ]
    }

    pub fn label(&self) -> &'static str {
        match self {
            AptitudeType::Turf => "Turf",
            AptitudeType::Dirt => "Dirt",
            AptitudeType::Sprint => "Sprint",
            AptitudeType::Mile => "Mile",
            AptitudeType::Medium => "Medium",
            AptitudeType::Long => "Long",
            AptitudeType::Front => "Front Runner",
            AptitudeType::PaceChaser => "Pace Chaser",
            AptitudeType::LateSurger => "Late Surger",
            AptitudeType::EndCloser => "End Closer",
        }
    }

    pub fn value(&self) -> &'static str {
        match self {
            AptitudeType::Turf => "turf",
            AptitudeType::Dirt => "dirt",
            AptitudeType::Sprint => "sprint",
            AptitudeType::Mile => "mile",
            AptitudeType::Medium => "medium",
            AptitudeType::Long => "long",
            AptitudeType::Front => "front",
            AptitudeType::PaceChaser => "pace_chaser",
            AptitudeType::LateSurger => "late_surger",
            AptitudeType::EndCloser => "end_closer",
        }
    }

    pub fn from_str(s: &str) -> Option<AptitudeType> {
        match s {
            "turf" => Some(AptitudeType::Turf),
            "dirt" => Some(AptitudeType::Dirt),
            "sprint" => Some(AptitudeType::Sprint),
            "mile" => Some(AptitudeType::Mile),
            "medium" => Some(AptitudeType::Medium),
            "long" => Some(AptitudeType::Long),
            "front" => Some(AptitudeType::Front),
            "pace_chaser" => Some(AptitudeType::PaceChaser),
            "late_surger" => Some(AptitudeType::LateSurger),
            "end_closer" => Some(AptitudeType::EndCloser),
            _ => None,
        }
    }
}

impl Into<u8> for AptitudeType {
    fn into(self) -> u8 {
        match self {
            AptitudeType::Turf => 11,
            AptitudeType::Dirt => 12,
            AptitudeType::Front => 21,
            AptitudeType::PaceChaser => 22,
            AptitudeType::LateSurger => 23,
            AptitudeType::EndCloser => 24,
            AptitudeType::Sprint => 31,
            AptitudeType::Mile => 32,
            AptitudeType::Medium => 33,
            AptitudeType::Long => 34,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum Filter {
    TraineeHash(Vec<UmaHash>),
    ParentHash(Vec<UmaHash>),
    HasParent(Vec<UmaHash>),
    Ranking {
        min: Option<i64>,
        max: Option<i64>,
    },
    Trainee(TraineeFilter),
    Character(CharacterFilter),
    Scenario(u16),
    Spark(SparkFilter),
    WhiteSparkCount {
        min: Option<i32>,
        max: Option<i32>,
    },
    Aptitude {
        aptitude_type: AptitudeType,
        min_level: i8,
    },
    MajorWinsCount {
        min: Option<i32>,
        both: bool,
    },
    SpecificMajorWin {
        major_win_names: Vec<String>,
        shared_with_parent: Option<bool>,
    },
    HasFavouriteMemo {
        search_text: Option<String>,
    },
    HasFavouriteIcon {
        icon_type: Option<i16>,
    },
    HasTag {
        tag_value: String,
    },
    Affinity {
        min: u32,
    },
    BorrowStatus {
        is_borrowed: bool,
    },
    IsIndependentTrainer {
        is_independent: bool,
    },
    TrainerId(Vec<i64>),
    WhiteSpark(WhiteSparkFilter),
}

impl Filter {
    pub fn is_rust_side(&self) -> bool {
        match self {
            Filter::Affinity { .. } => true,
            _ => false,
        }
    }
}
