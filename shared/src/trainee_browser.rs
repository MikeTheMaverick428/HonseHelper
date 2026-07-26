use serde::{Deserialize, Serialize};
use crate::filters::{AptitudeType, StatType};
use crate::legacy_planner::{LegacyPlannerSlot, lookup_dtos::AffinityResult};

pub const BROWSER_TYPE: &str = "trainee";

pub fn piece_needed(owned_rarity: i64, base_rarity: i64) -> i64 {
    if owned_rarity > 0 {
        match owned_rarity {
            1 => 50,
            2 => 100,
            3 => 200,
            4 => 300,
            _ => 0,
        }
    } else {
        match base_rarity {
            2 => 50,
            _ => 150,
        }
    }
}

// ── Filter ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TraineeFilter {
    Owned {
        owned: bool,
    },
    GrowthBonus {
        stat: StatType,
        min_value: Option<i64>,
    },
    MinAptitude {
        category: AptitudeType,
        min_level: i64,
    },
    MaxAAptitudes {
        max_count: i64,
    },
    Character {
        character_id: i64,
    },
    HasSkill {
        group_id: i64,
        exact_skill_id: Option<i64>,
        sources: TraineeSkillSources,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraineeSkillSources {
    pub innate: bool,
    pub event: bool,
    pub secret: bool,
}

impl Default for TraineeSkillSources {
    fn default() -> Self {
        Self {
            innate: true,
            event: true,
            secret: true,
        }
    }
}

fn stat_col(stat: &StatType) -> &'static str {
    stat.growth_column_name()
}

fn aptitude_col(apt: &AptitudeType) -> &'static str {
    apt.column_name()
}

pub fn filter_sql(filters: &[TraineeFilter]) -> (String, bool) {
    let mut clauses: Vec<String> = Vec::new();
    let mut needs_stats_join = false;

    for f in filters {
        match f {
            TraineeFilter::Owned { owned } => {
                if *owned {
                    clauses.push("tor.trainee_id IS NOT NULL".into());
                } else {
                    clauses.push("tor.trainee_id IS NULL".into());
                }
            }
            TraineeFilter::GrowthBonus { stat, min_value } => {
                let col = stat_col(stat);
                if let Some(min) = min_value {
                    clauses.push(format!("COALESCE(td.{col}, 0) >= {min}"));
                } else {
                    clauses.push(format!("COALESCE(td.{col}, 0) > 0"));
                }
            }
            TraineeFilter::MinAptitude {
                category,
                min_level,
            } => {
                needs_stats_join = true;
                let col = aptitude_col(category);
                clauses.push(format!("COALESCE(tsdf.{col}, 0) >= {min_level}"));
            }
            TraineeFilter::MaxAAptitudes { max_count } => {
                needs_stats_join = true;
                clauses.push(format!(
                    "(CASE WHEN COALESCE(tsdf.aptitude_ground_turf,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_ground_dirt,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_sprint,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_mile,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_medium,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_long,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_front,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_pace_chaser,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_late_surger,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_end_closer,0) >= 7 THEN 1 ELSE 0 END) <= {max_count}"
                ));
            }
            TraineeFilter::Character { character_id } => {
                clauses.push(format!("td.character_id = {}", character_id));
            }
            TraineeFilter::HasSkill { .. } => {
                // handled in storage::trainee_browser::build_filter_where
            }
        }
    }

    if clauses.is_empty() {
        ("1=1".into(), needs_stats_join)
    } else {
        (clauses.join(" AND "), needs_stats_join)
    }
}

// ── Sort ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraineeSortConfig {
    pub key: String,
    pub direction: String,
}

impl Default for TraineeSortConfig {
    fn default() -> Self {
        Self {
            key: "Name".to_string(),
            direction: "Asc".to_string(),
        }
    }
}

// ── Query ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TraineeBrowserQuery {
    pub filters: Vec<TraineeFilter>,
    pub sort: TraineeSortConfig,
    pub page: u32,
    pub page_size: u32,
    #[serde(default)]
    pub legacy_planner_slot: Option<LegacyPlannerSlot>,
    #[serde(default)]
    pub planner_context: bool,
}

// ── Page Item ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineePageItem {
    pub id: i64,
    pub name: String,
    pub character_id: i64,
    pub character_name: String,
    pub owned_rarity: i64,
    pub base_rarity: i64,
    pub piece_count: i64,
    pub piece_needed: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity: Option<AffinityResult>,
}

// ── Filter Options ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeFilterOptions {
    pub characters: Vec<(i64, String)>,
    pub skills: Vec<(i64, String)>,
}

// ── Event detail types ────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeEventDetail {
    pub story_id: i64,
    pub event_name: String,
    pub category: String,
    pub choices: Vec<TraineeEventChoiceDetail>,
    pub conditions: Option<String>,
    pub conditions_display: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeEventBranch {
    pub probability: Option<String>,
    pub rewards: Vec<TraineeEventRewardDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeEventChoiceDetail {
    pub choice_index: i64,
    pub branches: Vec<TraineeEventBranch>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeEventRewardDetail {
    pub reward_type: i64,
    pub reward_label: String,
    pub size: Option<i64>,
    pub skill_id: Option<i64>,
    pub skill_name: Option<String>,
    pub negative: bool,
    pub alternatives: Option<Vec<i64>>,
    pub effect_label: Option<String>,
}

// ── Skill detail ─────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeSkillDetail {
    pub skill_id: i64,
    pub name: String,
    pub skill_type: String,
    pub rarity: i64,
    pub level: i64,
    pub need_rank: i64,
    pub source: String,
    pub source_name: String,
    pub unlocked: bool,
}

// ── Detail ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TraineeDetail {
    pub id: i64,
    pub name: String,
    pub character_name: String,
    pub rarity: i64,
    pub talent_level: i64,
    pub growth_spe: i64,
    pub growth_sta: i64,
    pub growth_str: i64,
    pub growth_gut: i64,
    pub growth_wit: i64,
    pub stat_spe: i64,
    pub stat_sta: i64,
    pub stat_pwr: i64,
    pub stat_gut: i64,
    pub stat_wit: i64,
    pub aptitude_sprint: i64,
    pub aptitude_mile: i64,
    pub aptitude_medium: i64,
    pub aptitude_long: i64,
    pub aptitude_turf: i64,
    pub aptitude_dirt: i64,
    pub aptitude_front: i64,
    pub aptitude_pace_chaser: i64,
    pub aptitude_late_surger: i64,
    pub aptitude_end_closer: i64,
    pub events: Vec<TraineeEventDetail>,
    pub skills: Vec<TraineeSkillDetail>,
}
