use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::filters::Filter;
use crate::legacy_planner::lookup_dtos::AffinityResult;
use crate::legacy_planner::LegacyPlannerSlot;

pub fn serialize_i64_as_string<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn deserialize_i64_from_string<'de, D>(deserializer: D) -> Result<i64, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    struct I64Visitor;
    impl<'de> de::Visitor<'de> for I64Visitor {
        type Value = i64;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an integer or string representation of an integer")
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<i64, E> {
            Ok(v)
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<i64, E> {
            Ok(v as i64)
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<i64, E> {
            v.parse::<i64>().map_err(de::Error::custom)
        }
    }
    deserializer.deserialize_any(I64Visitor)
}

pub fn serialize_option_i64_as_string<S>(
    value: &Option<i64>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match value {
        Some(v) => serializer.serialize_str(&v.to_string()),
        None => serializer.serialize_none(),
    }
}

pub fn deserialize_option_i64_from_string<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de;
    struct OptI64Visitor;
    impl<'de> de::Visitor<'de> for OptI64Visitor {
        type Value = Option<i64>;
        fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            f.write_str("an optional integer or string")
        }
        fn visit_none<E: de::Error>(self) -> Result<Option<i64>, E> {
            Ok(None)
        }
        fn visit_unit<E: de::Error>(self) -> Result<Option<i64>, E> {
            Ok(None)
        }
        fn visit_i64<E: de::Error>(self, v: i64) -> Result<Option<i64>, E> {
            Ok(Some(v))
        }
        fn visit_u64<E: de::Error>(self, v: u64) -> Result<Option<i64>, E> {
            Ok(Some(v as i64))
        }
        fn visit_str<E: de::Error>(self, v: &str) -> Result<Option<i64>, E> {
            if v.is_empty() {
                Ok(None)
            } else {
                v.parse::<i64>().map(Some).map_err(de::Error::custom)
            }
        }
    }
    deserializer.deserialize_any(OptI64Visitor)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum VeteranFilter {
    ByHash(u64),
    ByCharacter(i64),
    ByScenario(String),
    ByTrainee(i64),
    ByRanking {
        min: Option<i64>,
        max: Option<i64>,
    },
    BySparkBlue {
        group_ids: Vec<i64>,
        lvl: i8,
        min_uma_count: Option<i64>,
        max_uma_count: Option<i64>,
    },
    BySparkPink {
        group_ids: Vec<i64>,
        lvl: i8,
        min_uma_count: Option<i64>,
        max_uma_count: Option<i64>,
    },
    BySparkGreen {
        group_ids: Vec<i64>,
        lvl: i8,
        min_uma_count: Option<i64>,
        max_uma_count: Option<i64>,
    },
    BySparkWhite {
        group_ids: Vec<i64>,
        lvl: i8,
        min_uma_count: Option<i64>,
        max_uma_count: Option<i64>,
    },
    HasSparkOnMainVeteran {
        group_ids: Vec<i64>,
    },
    ByWhiteSparkCount {
        min: Option<i64>,
        max: Option<i64>,
    },
    ByMajorWinsCount {
        min: Option<i64>,
        max: Option<i64>,
        both_parents: Option<bool>,
    },
    BySpecificMajorWin {
        win_id: i64,
        shared: bool,
    },
    ByAptitude {
        field: String,
        min_level: String,
    },
    HasFavouriteMemo {
        text: Option<String>,
    },
    HasFavouriteIcon {
        icon: Option<String>,
    },
    BorrowStatus(String),
    IsIndependentTrainer(String),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SortConfig {
    pub key: String,
    pub direction: String,
}

impl Default for SortConfig {
    fn default() -> Self {
        Self {
            key: String::new(),
            direction: String::new(),
        }
    }
}

fn default_true() -> bool {
    true
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranRow {
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub hash: i64,
    pub trainee_id: i64,
    pub scenario: Option<i64>,
    pub favorite_icon_type: Option<i64>,
    pub favorite_memo: Option<String>,
    pub created_at: String,
    pub rank: i64,
    pub rank_score: i64,
    pub stat_speed: Option<i64>,
    pub stat_stamina: Option<i64>,
    pub stat_power: Option<i64>,
    pub stat_guts: Option<i64>,
    pub stat_wit: Option<i64>,
    pub aptitude_turf: Option<i64>,
    pub aptitude_dirt: Option<i64>,
    pub aptitude_sprint: Option<i64>,
    pub aptitude_mile: Option<i64>,
    pub aptitude_medium: Option<i64>,
    pub aptitude_long: Option<i64>,
    pub aptitude_front: Option<i64>,
    pub aptitude_pace_chaser: Option<i64>,
    pub aptitude_late_surger: Option<i64>,
    pub aptitude_end_closer: Option<i64>,
    #[serde(
        serialize_with = "serialize_option_i64_as_string",
        deserialize_with = "deserialize_option_i64_from_string"
    )]
    pub owner_id: Option<i64>,
    pub owned: bool,
    #[serde(default = "default_true")]
    pub active: bool,
    pub rarity: Option<i64>,
    pub talent_level: Option<i64>,
    pub trainee_name: Option<String>,
    pub major_wins_count: i64,
    pub major_wins_on_veteran_count: i64,
    pub white_spark_count: i64,
    pub white_spark_on_veteran_count: i64,
    #[serde(default)]
    pub spark_groups: Vec<SparkGroupRow>,
    #[serde(
        serialize_with = "serialize_option_i64_as_string",
        deserialize_with = "deserialize_option_i64_from_string"
    )]
    pub min_hash: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<i32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nickname_id: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranPageItem {
    pub veteran: VeteranRow,
    pub affinity: Option<AffinityResult>,
    #[serde(default)]
    pub tags: Vec<TagRow>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranBrowserQuery {
    pub filters: Vec<Filter>,
    pub sort: SortConfig,
    pub page: u32,
    pub page_size: u32,
    pub legacy_planner_slot: Option<LegacyPlannerSlot>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QueryVeteransResult {
    pub veterans: Vec<VeteranRow>,
    pub total: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SparkGroupRow {
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub veteran_hash: i64,
    pub spark_group_id: i64,
    pub uma_count: i64,
    pub level_sum: i64,
    pub veteran_level_sum: i64,
    pub name: String,
    pub spark_type: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MajorWinRow {
    pub id: i64,
    pub name: Option<String>,
    pub group_id: Option<i64>,
    pub shared_count: Option<i64>,
    pub on_veteran: bool,
    #[serde(default)]
    pub priority: Option<i64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentRow {
    #[serde(
        serialize_with = "serialize_i64_as_string",
        deserialize_with = "deserialize_i64_from_string"
    )]
    pub hash: i64,
    pub trainee_id: i64,
    pub rank: i64,
    pub rarity: i64,
    pub talent_level: Option<i64>,
    pub trainee_name: Option<String>,
    pub major_wins_count: i64,
    pub spark_count: i64,
    #[serde(default)]
    pub blue_sparks: Vec<SparkGroupRow>,
    pub owner_id: Option<i64>,
    pub owned: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FilterOptions {
    pub characters: Vec<(i64, String)>,
    pub trainees: Vec<(i64, String)>,
    pub blue_spark_groups: Vec<(i64, String)>,
    pub pink_spark_groups: Vec<(i64, String)>,
    pub green_spark_groups: Vec<(i64, String)>,
    pub white_spark_groups: Vec<(i64, String)>,
    pub scenarios: Vec<(i64, String)>,
    pub tags: Vec<String>,
    pub trainers: Vec<(i64, String)>,
}

impl FilterOptions {
    pub fn spark_group_name(&self, group_id: i64) -> Option<&str> {
        self.blue_spark_groups
            .iter()
            .chain(self.pink_spark_groups.iter())
            .chain(self.green_spark_groups.iter())
            .chain(self.white_spark_groups.iter())
            .find(|(i, _)| *i == group_id)
            .map(|(_, name)| name.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PresetData {
    pub name: String,
    pub filters: Option<String>,
    pub sort: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagRow {
    pub id: i64,
    pub tag_value: String,
    pub create_date: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranSkillRow {
    pub skill_id: i64,
    pub name: String,
    pub level: i64,
    pub category: Option<i64>,
    #[serde(default)]
    pub skill_type: String,
    #[serde(default)]
    pub rarity: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranSupportCardRow {
    pub position: i64,
    pub support_card_id: i64,
    pub name: String,
    pub rarity: i64,
    pub card_type: i64,
    pub exp: i64,
    pub limit_break_count: i64,
}
