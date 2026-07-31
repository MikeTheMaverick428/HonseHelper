use std::collections::BTreeMap;

use nohash_hasher::IntSet;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

use crate::{
    legacy_planner::lookup_dtos::SlimUma,
    models::{CharacterOption, SparkType},
};

pub mod lookup_dtos;
pub use lookup_dtos::AffinityResult;

mod u64_as_str {
    use super::*;
    pub fn serialize<S: Serializer>(v: &u64, s: S) -> Result<S::Ok, S::Error> {
        s.serialize_str(&v.to_string())
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<u64, D::Error> {
        let s = String::deserialize(d)?;
        s.parse::<u64>().map_err(serde::de::Error::custom)
    }
}

mod opt_u64_as_str {
    use super::*;
    pub fn serialize<S: Serializer>(v: &Option<u64>, s: S) -> Result<S::Ok, S::Error> {
        match v {
            Some(n) => s.serialize_some(&n.to_string()),
            None => s.serialize_none(),
        }
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<u64>, D::Error> {
        let opt = Option::<String>::deserialize(d)?;
        match opt {
            Some(s) => s.parse::<u64>().map(Some).map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelectedTrainee {
    pub trainee_id: i64,
    pub character_id: i64,
    pub trainee_name: String,
    pub character_name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SparkGroupInfo {
    pub spark_group_id: i64,
    pub name: String,
    pub spark_type: SparkType,
    pub total_stars: i8,
    pub trainee_stars_veteran: i8,
    pub uma_count: i8,
}

impl SparkGroupInfo {
    pub fn sort_key(&self) -> (i8, i8, i8) {
        let type_order = match self.spark_type {
            SparkType::Stat => 0,
            SparkType::Aptitude => 1,
            SparkType::Unique => 2,
            _ => 3,
        };

        let max_trainee = std::cmp::max(self.trainee_stars_veteran, 0) as i8;

        (type_order, -max_trainee, -self.total_stars)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct LegacyUma {
    pub name: String,
    pub character_id: i64,
    #[serde(with = "u64_as_str")]
    pub hash: u64,
    pub spark_groups: Vec<SparkGroupInfo>,
    pub major_wins: Vec<i64>,
    #[serde(with = "opt_u64_as_str")]
    pub parent1_hash: Option<u64>,
    #[serde(with = "opt_u64_as_str")]
    pub parent2_hash: Option<u64>,
    pub is_borrowed: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ParentUma {
    pub name: String,
    pub character_id: i64,
    #[serde(with = "u64_as_str")]
    pub hash: u64,
    pub spark_groups: Vec<SparkGroupInfo>,
    pub major_wins: Vec<i64>,
    #[serde(default)]
    pub api_mode: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum LegacySlotValue {
    LegacyUma(LegacyUma),
    ParentUma(ParentUma),
    Character(CharacterOption),
}

impl From<LegacySlotValue> for SlimUma {
    fn from(value: LegacySlotValue) -> Self {
        match value {
            LegacySlotValue::Character(u) => SlimUma {
                hash: 0,
                character_id: u.character_id,
                wins: IntSet::default(),
            },
            LegacySlotValue::ParentUma(u) => SlimUma {
                hash: u.hash as u64,
                character_id: u.character_id,
                wins: u
                    .major_wins
                    .iter()
                    .map(|id| (*id) as u32)
                    .collect::<IntSet<_>>(),
            },
            LegacySlotValue::LegacyUma(u) => SlimUma {
                hash: u.hash as u64,
                character_id: u.character_id,
                wins: u
                    .major_wins
                    .iter()
                    .map(|id| (*id) as u32)
                    .collect::<IntSet<_>>(),
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct LegacyPlannerState {
    pub chosen_character: Option<SelectedTrainee>,
    pub parent_a: Option<LegacySlotValue>,
    pub parent_b: Option<LegacySlotValue>,
    pub grandparent_aa: Option<LegacySlotValue>,
    pub grandparent_ab: Option<LegacySlotValue>,
    pub grandparent_ba: Option<LegacySlotValue>,
    pub grandparent_bb: Option<LegacySlotValue>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VeteranAffinity {
    pub hash: i64,
    pub affinity: Option<i32>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LegacyPlannerSlot {
    ParentA,
    ParentB,
    GrandparentAA,
    GrandparentAB,
    GrandparentBA,
    GrandparentBB,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlannerAffinities {
    pub trainee_parent_a: Option<AffinityResult>,
    pub trainee_parent_b: Option<AffinityResult>,
    pub parent_a_parent_b: Option<AffinityResult>,
    pub parent_a_grandparent_aa: Option<AffinityResult>,
    pub parent_a_grandparent_ab: Option<AffinityResult>,
    pub parent_b_grandparent_ba: Option<AffinityResult>,
    pub parent_b_grandparent_bb: Option<AffinityResult>,
}

impl PlannerAffinities {
    pub fn total(&self) -> AffinityResult {
        let mut base = 0u32;
        let mut bonus = 0u32;
        for aff in [
            &self.trainee_parent_a,
            &self.trainee_parent_b,
            &self.parent_a_parent_b,
            &self.parent_a_grandparent_aa,
            &self.parent_a_grandparent_ab,
            &self.parent_b_grandparent_ba,
            &self.parent_b_grandparent_bb,
        ] {
            if let Some(a) = aff {
                base += a.base;
                bonus += a.bonus;
            }
        }
        AffinityResult { base, bonus }
    }
}

// ── Affinity Summary (extended, with per-pair breakdown) ────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AffinityPairInfo {
    pub label: String,
    pub base_affinity: Option<i32>,
    pub bonus_affinity: Option<i32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct PlannerAffinitySummary {
    pub pairs: Vec<AffinityPairInfo>,
    pub total: i32,
    pub base: i32,
    pub bonus: i32,
    pub total_affinity_by_hash: BTreeMap<u64, i32>,
}

// ── Spark Summary (white spark generating chance) ───────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SparkSummaryRow {
    pub spark_group_id: i64,
    pub spark_name: String,
    pub spark_type: SparkType,
    pub total_stars: i32,
    pub legacy_uma_count: usize,
    pub white_probability_pct: Option<f64>,
    pub maru_skill_probability_pct: Option<f64>,
    pub gold_skill_probability_pct: Option<f64>,
}

// ── Inspiration Summary ─────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InspirationSummaryRow {
    pub spark_group_id: i64,
    pub spark_name: String,
    pub spark_type: SparkType,
    pub sparking_chance: f64,
    pub career_chance: f64,
}

impl LegacyPlannerSlot {
    pub fn from_label(label: &str) -> Option<Self> {
        match label {
            "Parent A" => Some(Self::ParentA),
            "Parent B" => Some(Self::ParentB),
            "Grandparent AA" => Some(Self::GrandparentAA),
            "Grandparent AB" => Some(Self::GrandparentAB),
            "Grandparent BA" => Some(Self::GrandparentBA),
            "Grandparent BB" => Some(Self::GrandparentBB),
            _ => None,
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::ParentA => "Parent A",
            Self::ParentB => "Parent B",
            Self::GrandparentAA => "Grandparent AA",
            Self::GrandparentAB => "Grandparent AB",
            Self::GrandparentBA => "Grandparent BA",
            Self::GrandparentBB => "Grandparent BB",
        }
    }
}
