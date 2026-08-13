use serde::{Deserialize, Serialize};

use crate::db_models::UmaHash;
use crate::legacy_planner::lookup_dtos::AffinityResult;
use crate::veteran_browser::SparkGroupRow;

pub const BROWSER_TYPE: &str = "trainer";

/// Placeholder helper: derives a support card level from rarity + limit break count.
///
/// Borrowed cards do not expose a direct level in memory; the game computes the
/// displayed level from the accumulated experience. Until the exact level curve is
/// reverse-engineered, this returns the maximum attainable level for a card of the
/// given rarity at the given limit break count. Implement the real formula manually later.
pub fn max_level_for_limit_break(rarity: i64, limit_break: i64) -> i64 {
    let base = match rarity {
        1 => 20, // R
        2 => 25, // SR
        3 => 30, // SSR
        _ => 20,
    };
    (base + 5 * limit_break).min(50)
}

// ── Filter ───────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum TrainerFilter {
    NameSearch { query: String },
    Following { is_following: bool },
    VeteranTrainee { ids: Vec<i64>, negate: bool },
    VeteranRank { min: i64 },
    ScType { card_types: Vec<i64> },
    ScRarity { rarities: Vec<i64> },
    ScLimitBreak { min: i64, max: i64 },
    ScCharacter { character_ids: Vec<i64> },
}

// ── Sort ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainerSortConfig {
    pub key: String,
    pub direction: String,
}

impl Default for TrainerSortConfig {
    fn default() -> Self {
        Self {
            key: "Name".to_string(),
            direction: "Asc".to_string(),
        }
    }
}

// ── Query ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TrainerBrowserQuery {
    pub filters: Vec<TrainerFilter>,
    pub sort: TrainerSortConfig,
    pub page: u32,
    pub page_size: u32,
}

// ── Page Item ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowVeteranInfo {
    pub hash: UmaHash,
    pub character_id: i64,
    pub rank: i64,
    pub rank_score: i64,
    #[serde(default)]
    pub blue_sparks: Vec<SparkGroupRow>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_hash: Option<UmaHash>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub affinity: Option<AffinityResult>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BorrowSupportCardInfo {
    pub id: i64,
    pub name: String,
    pub character_id: i64,
    pub card_type: i64,
    pub card_rarity: i64,
    pub level: i64,
    pub limit_break_count: i64,
    /// Whether the current user owns this card (as opposed to it being borrow-only).
    #[serde(default)]
    pub owned: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainerPageItem {
    pub trainer_id: i64,
    pub name: String,
    pub friend_state: i64,
    pub last_login: Option<String>,
    pub comment: String,
    pub fan: i64,
    pub honor_id: i64,
    pub circle_id: i64,
    pub circle_name: String,
    pub is_following: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follower_num: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_recheck_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_source: Option<String>,
    pub borrow_veteran: Option<BorrowVeteranInfo>,
    pub borrow_support_card: Option<BorrowSupportCardInfo>,
    pub borrow_uma_trainee_name: String,
}

// ── Filter Options ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TrainerFilterOptions {
    pub characters: Vec<(i64, String)>,
    pub card_types: Vec<(i64, String)>,
    pub rarities: Vec<(i64, String)>,
}
