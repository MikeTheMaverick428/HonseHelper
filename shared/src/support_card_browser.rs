use crate::SupportCardEffectRow;
use crate::SupportCardUniqueEffectDetail;
use serde::{Deserialize, Serialize};

pub const BROWSER_TYPE: &str = "support_card";

pub fn card_type_label(ct: i64) -> &'static str {
    match ct {
        1 => "Speed",
        2 => "Stamina",
        3 => "Power",
        4 => "Guts",
        5 => "Wisdom",
        6 => "Friend",
        7 => "Group",
        _ => "Unknown",
    }
}

pub fn card_type_color(ct: i64) -> &'static str {
    match ct {
        1 => "#3b82f6",
        2 => "#f97316",
        3 => "#ef4444",
        4 => "#eab308",
        5 => "#22c55e",
        6 => "#a855f7",
        7 => "#6b7280",
        _ => "#6b7280",
    }
}

pub fn rarity_label(r: i64) -> &'static str {
    match r {
        1 => "R",
        2 => "SR",
        3 => "SSR",
        _ => "?",
    }
}

pub fn rarity_color(r: i64) -> &'static str {
    match r {
        1 => "#9ca3af",
        2 => "#60a5fa",
        3 => "#fbbf24",
        _ => "#6b7280",
    }
}

// ── Filter ───────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OwnershipStatus {
    /// Owned by the user or available as a borrow from a followed trainer.
    Accessible,
    /// Not owned, but available as a borrow from a followed trainer.
    BorrowOnly,
    /// Not owned by the user.
    Unowned,
    /// Not owned and no followed trainer has it as their borrow.
    Inaccessible,
}

impl OwnershipStatus {
    pub fn label(&self) -> &'static str {
        match self {
            Self::Accessible => "Accessible (owned or borrow)",
            Self::BorrowOnly => "Borrow only",
            Self::Unowned => "Unowned",
            Self::Inaccessible => "Inaccessible (not owned, no borrow)",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value")]
pub enum SupportCardFilter {
    Ownership { status: OwnershipStatus },
    NameSearch { search_text: String },
    Rarity { rarity: i64 },
    CardType { card_type: i64 },
    LimitBreak { min: i64, max: i64 },
    HasEffect { effect_type: i64 },
    Character { character_id: i64 },
    HasSkill {
        group_id: i64,
        exact_skill_id: Option<i64>,
        sources: SupportCardSkillSources,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCardSkillSources {
    pub hint: bool,
    pub chain_event: bool,
    pub random_event: bool,
}

impl Default for SupportCardSkillSources {
    fn default() -> Self {
        Self {
            hint: true,
            chain_event: true,
            random_event: true,
        }
    }
}

// ── Sort ─────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCardSortConfig {
    pub key: String,
    pub direction: String,
}

impl Default for SupportCardSortConfig {
    fn default() -> Self {
        Self {
            key: "Name".to_string(),
            direction: "Asc".to_string(),
        }
    }
}

// ── Query ────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SupportCardBrowserQuery {
    pub filters: Vec<SupportCardFilter>,
    pub sort: SupportCardSortConfig,
    pub page: u32,
    pub page_size: u32,
}

// ── Page Item ────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardPageItem {
    pub support_card_id: i64,
    pub name: String,
    pub rarity: i64,
    pub card_type: i64,
    pub level: i64,
    pub max_level: i64,
    pub limit_break_count: i64,
    pub exp: i64,
    pub favorite_flag: bool,
    pub stock: i64,
    pub character_id: i64,
    pub owned: bool,
    /// The card can be borrowed from at least one currently followed trainer.
    #[serde(default)]
    pub borrow_available: bool,
    /// Highest level available among followed trainers' borrows.
    #[serde(default)]
    pub borrow_level: i64,
    /// Highest limit-break count available among followed trainers' borrows.
    #[serde(default)]
    pub borrow_limit_break_count: i64,
}

// ── Filter Options ───────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardFilterOptions {
    pub rarities: Vec<(i64, String)>,
    pub card_types: Vec<(i64, String)>,
    pub effect_types: Vec<(i64, String)>,
    pub characters: Vec<(i64, String)>,
    pub skills: Vec<(i64, String)>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardEventDetail {
    pub story_id: i64,
    pub event_name: String,
    pub category: String,
    pub choices: Vec<SupportCardEventChoiceDetail>,
    pub conditions: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardEventBranch {
    pub probability: Option<String>,
    pub rewards: Vec<SupportCardEventRewardDetail>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardEventChoiceDetail {
    pub choice_index: i64,
    pub branches: Vec<SupportCardEventBranch>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardEventRewardDetail {
    pub reward_type: i64,
    pub reward_label: String,
    pub size: Option<i64>,
    pub skill_id: Option<i64>,
    pub skill_name: Option<String>,
    pub negative: bool,
    pub alternatives: Option<Vec<i64>>,
    pub effect_label: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardSkillDetail {
    pub skill_id: i64,
    pub skill_name: String,
    pub skill_level: i64,
    pub source: String,
    pub source_name: String,
    #[serde(default)]
    pub skill_type: String,
    #[serde(default)]
    pub rarity: i64,
}

/// A single followed trainer currently borrowing the support card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardBorrowRow {
    pub trainer_id: i64,
    pub name: String,
    pub level: i64,
    pub limit_break_count: i64,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SupportCardDetail {
    pub effects: Vec<SupportCardEffectRow>,
    pub unique_effect: Option<SupportCardUniqueEffectDetail>,
    pub skill_hints: Vec<SupportCardSkillDetail>,
    pub events: Vec<SupportCardEventDetail>,
}
