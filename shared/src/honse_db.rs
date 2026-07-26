use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MasterDbStatus {
    pub found: bool,
    pub source: String,
    pub path: Option<String>,
    pub message: String,
    pub last_checked: Option<String>,
    pub checked_candidates: Vec<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppDbTableSyncState {
    pub table_name: String,
    pub source_table: String,
    pub row_count: i64,
    pub app_version: String,
    pub source_db_path: String,
    pub synced_at: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AppDbSyncReport {
    pub synced: bool,
    pub up_to_date: bool,
    pub app_version: String,
    pub source_db_path: Option<String>,
    pub refreshed_tables: Vec<String>,
    pub table_states: Vec<AppDbTableSyncState>,
    pub message: String,
    pub checked_at: String,
    pub db_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct CharacterDataRow {
    pub id: i64,
    pub name: String,
    pub birth_day: Option<i64>,
    pub birth_month: Option<i64>,
    pub birth_year: Option<i64>,
    pub trainee: bool,
    pub support: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SkillDataRow {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub precondition1: Option<String>,
    pub condition1: Option<String>,
    pub precondition2: Option<String>,
    pub condition2: Option<String>,
    pub skill_category: Option<i64>,
    pub group_id: Option<i64>,
    pub rarity: Option<i64>,
    pub icon_id: Option<i64>,
    pub ability_type: Option<i64>,
    pub target_type: Option<i64>,
    pub ability_type_2: Option<i64>,
    pub ability_type_3: Option<i64>,
    pub target_type_2: Option<i64>,
    pub target_type_3: Option<i64>,
    pub effect_value_1: Option<i64>,
    pub effect_value_2: Option<i64>,
    pub effect_value_3: Option<i64>,
    pub target_value_1: Option<i64>,
    pub target_value_2: Option<i64>,
    pub target_value_3: Option<i64>,
    pub effect_duration: Option<i64>,
    pub effect_cooldown: Option<i64>,
    pub activate_lot: Option<i64>,
    pub skill_cost: Option<i64>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SkillType {
    Passive,
    Velocity,
    Acceleration,
    Recovery,
    Navigation,
    Visibility,
    Debuff,
    SelfDebuff,
    Special,
    Unknown,
}

impl SkillType {
    pub fn label(&self) -> &'static str {
        match self {
            SkillType::Passive => "Passive",
            SkillType::Velocity => "Velocity",
            SkillType::Acceleration => "Acceleration",
            SkillType::Recovery => "Recovery",
            SkillType::Navigation => "Navigation",
            SkillType::Visibility => "Visibility",
            SkillType::Debuff => "Debuff",
            SkillType::SelfDebuff => "Self Debuff",
            SkillType::Special => "Special",
            SkillType::Unknown => "Unknown",
        }
    }
}

impl From<&SkillDataRow> for SkillType {
    fn from(row: &SkillDataRow) -> Self {
        let icon = row.icon_id.unwrap_or(0);
        let ability = row.ability_type.unwrap_or(0);

        let is_red = (30000..=39999).contains(&icon);
        let is_yellow = (20000..=29999).contains(&icon);

        if is_red {
            return SkillType::Debuff;
        }

        if ability == 21 && is_yellow {
            return SkillType::SelfDebuff;
        }

        let is_green = (10000..=19999).contains(&icon);

        match ability {
            1..=5 if is_green || icon == 0 => SkillType::Passive,
            _ if is_green => SkillType::Passive,
            27 => SkillType::Velocity,
            31 => SkillType::Acceleration,
            9 => SkillType::Recovery,
            28 => SkillType::Navigation,
            8 => SkillType::Visibility,
            6 | 14 | 22 | 501 | 502 => SkillType::Special,
            _ => SkillType::Unknown,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TraineeDataRow {
    pub id: i64,
    pub name: String,
    pub character_id: i64,
    pub growth_rate_spe: Option<i64>,
    pub growth_rate_sta: Option<i64>,
    pub growth_rate_str: Option<i64>,
    pub growth_rate_gut: Option<i64>,
    pub growth_rate_wit: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TraineeStatsDataRow {
    pub trainee_id: i64,
    pub rarity: i64,
    pub spe: i64,
    pub sta: i64,
    pub pwr: i64,
    pub gut: i64,
    pub wit: i64,
    pub aptitude_dist_sprint: i64,
    pub aptitude_dist_mile: i64,
    pub aptitude_dist_medium: i64,
    pub aptitude_dist_long: i64,
    pub aptitude_ground_turf: i64,
    pub aptitude_ground_dirt: i64,
    pub aptitude_style_front: i64,
    pub aptitude_style_pace_chaser: i64,
    pub aptitude_style_late_surger: i64,
    pub aptitude_style_end_closer: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SparkDataRow {
    pub id: i64,
    pub group_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub stars_count: Option<i64>,
    pub spark_type: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct RaceDataRow {
    pub program_id: i64,
    pub race_id: i64,
    pub race_instance_id: i64,
    pub course_set_id: Option<i64>,
    pub race_name: Option<String>,
    pub track_name: Option<String>,
    pub race_grade: Option<i64>,
    pub race_group: Option<i64>,
    pub distance: Option<i64>,
    pub ground: Option<i64>,
    pub program_grade: Option<i64>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AffinityMemberRow {
    pub id: i64,
    pub affinity_group: i64,
    pub chara_id: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct AffinityGroupRow {
    pub affinity_group: i64,
    pub affinity_point: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MajorWinsDataRow {
    pub id: i64,
    pub name: Option<String>,
    pub priority: Option<i64>,
    pub group_id: Option<i64>,
    pub condition: Option<i64>,
    pub win_saddle_type: Option<i64>,
    pub race_instance_ids: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScheduleRaceRow {
    pub race_instance_id: u32,
    pub race_id: u32,
    pub thumbnail_id: u32,
    pub course_set: u32,
    pub program_id: u32,
    pub race_name: String,
    pub track_name: String,
    pub distance: u32,
    pub ground: u32,
    pub inout: u32,
    pub race_group: u32,
    pub grade: u32,
    pub entry_num: u32,
    pub race_permission: u32,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct TrophyRaceRow {
    pub trophy_id: u32,
    pub trophy_type: u32,
    pub trophy_name: String,
    pub race_grade: u32,
    pub race_instance_ids: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct ScenarioDataRow {
    pub id: i64,
    pub name: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportCardRow {
    pub id: i64,
    pub character_id: i64,
    pub name: Option<String>,
    pub rarity: i64,
    pub card_type: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct GatherVeteransResult {
    pub added: usize,
    pub removed: usize,
    pub total: usize,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportEventRow {
    pub story_id: i64,
    pub support_card_id: i64,
    pub event_name: String,
    pub category: String,
    pub conditions: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportEventChoiceRow {
    pub id: i64,
    pub story_id: i64,
    pub choice_index: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportEventRewardRow {
    pub id: i64,
    pub choice_id: i64,
    pub reward_type: i64,
    pub size: Option<i64>,
    pub skill_id: Option<i64>,
    pub negative: bool,
    pub alternatives: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupplementaryDataSyncReport {
    pub synced: bool,
    pub up_to_date: bool,
    pub event_count: i64,
    pub choice_count: i64,
    pub reward_count: i64,
    pub datasets: Vec<DatasetSyncStatus>,
    pub synced_at: String,
    pub message: String,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DatasetSyncStatus {
    pub id: String,
    pub version: i64,
    pub event_count: i64,
    pub choice_count: i64,
    pub reward_count: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupplementaryDataCheckReport {
    pub datasets: Vec<DatasetCheckEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DatasetCheckEntry {
    pub id: String,
    pub available_version: i64,
    pub local_version: Option<i64>,
    pub needs_update: bool,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportCardEffectRow {
    pub support_card_id: i64,
    pub effect_type: i64,
    pub init_value: i64,
    pub lv5: i64,
    pub lv10: i64,
    pub lv15: i64,
    pub lv20: i64,
    pub lv25: i64,
    pub lv30: i64,
    pub lv35: i64,
    pub lv40: i64,
    pub lv45: i64,
    pub lv50: i64,
}

impl SupportCardEffectRow {
    pub fn value_at_level(&self, level: i64) -> i64 {
        const LEVELS: [i64; 11] = [1, 5, 10, 15, 20, 25, 30, 35, 40, 45, 50];

        let get_value = |lvl: i64| -> i64 {
            match lvl {
                1 => self.init_value,
                5 => self.lv5,
                10 => self.lv10,
                15 => self.lv15,
                20 => self.lv20,
                25 => self.lv25,
                30 => self.lv30,
                35 => self.lv35,
                40 => self.lv40,
                45 => self.lv45,
                50 => self.lv50,
                _ => -1,
            }
        };

        let known: Vec<(i64, i64)> = LEVELS
            .iter()
            .map(|lvl| (*lvl, get_value(*lvl)))
            .filter(|(_, v)| *v >= 0)
            .collect();

        if known.is_empty() {
            return -1;
        }

        if level < known[0].0 {
            return -1;
        }

        let last = known[known.len() - 1];
        if level >= last.0 {
            return last.1;
        }

        for i in 0..known.len() - 1 {
            let (x0, y0) = known[i];
            let (x1, y1) = known[i + 1];
            if level >= x0 && level < x1 {
                if y0 == y1 {
                    return y0;
                }
                return y0 + (y1 - y0) * (level - x0) / (x1 - x0);
            }
        }

        last.1
    }
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportCardUniqueEffectRow {
    pub support_card_id: i64,
    pub name: String,
    pub limit_break_level: i64,
    pub effect_type_0: i64,
    pub effect_value_0: i64,
    pub sub_0_1: i64,
    pub sub_0_2: i64,
    pub sub_0_3: i64,
    pub sub_0_4: i64,
    pub effect_type_1: i64,
    pub effect_value_1: i64,
    pub sub_1_1: i64,
    pub sub_1_2: i64,
    pub sub_1_3: i64,
    pub sub_1_4: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportCardUniqueEffectDetail {
    pub name: String,
    pub limit_break_level: i64,
    pub entries: Vec<UniqueEffectEntry>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct UniqueEffectEntry {
    pub effect_label: String,
    pub effect_value: i64,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct SupportCardSkillHintRow {
    pub support_card_id: i64,
    pub skill_id: i64,
    pub skill_level: i64,
    pub alt_level: Option<i64>,
}
