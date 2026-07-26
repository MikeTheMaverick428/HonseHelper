use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackTrainedChara {
    pub viewer_id: i64,
    pub trained_chara_id: i64,
    #[serde(default)]
    pub owner_viewer_id: i64,
    #[serde(default)]
    pub owner_trained_chara_id: i64,
    #[serde(default)]
    pub single_mode_chara_id: i64,
    #[serde(default)]
    pub chara_seed: i64,
    pub card_id: i64,
    #[serde(default)]
    pub succession_trained_chara_id_1: i64,
    #[serde(default)]
    pub succession_trained_chara_id_2: i64,
    #[serde(default)]
    pub use_type: i64,
    #[serde(default)]
    pub speed: i32,
    #[serde(default)]
    pub stamina: i32,
    #[serde(default)]
    pub power: i32,
    #[serde(default)]
    pub wiz: i32,
    #[serde(default)]
    pub guts: i32,
    #[serde(default)]
    pub fans: i64,
    pub rank_score: i64,
    pub rank: i64,
    #[serde(default)]
    pub scenario_id: i64,
    #[serde(default)]
    pub route_id: i64,
    #[serde(default)]
    pub arrive_route_race_id: i64,
    pub proper_ground_turf: i64,
    pub proper_ground_dirt: i64,
    pub proper_running_style_nige: i64,
    pub proper_running_style_senko: i64,
    pub proper_running_style_sashi: i64,
    pub proper_running_style_oikomi: i64,
    pub proper_distance_short: i64,
    pub proper_distance_mile: i64,
    pub proper_distance_middle: i64,
    pub proper_distance_long: i64,
    #[serde(default)]
    pub succession_num: i64,
    pub rarity: i64,
    #[serde(default)]
    pub is_saved: i64,
    #[serde(default)]
    pub is_locked: i64,
    pub talent_level: i64,
    #[serde(default)]
    pub race_cloth_id: i64,
    #[serde(default)]
    pub chara_grade: i64,
    #[serde(default)]
    pub running_style: i64,
    #[serde(default)]
    pub nickname_id: i64,
    #[serde(default)]
    pub wins: i64,
    pub register_time: String,
    #[serde(default)]
    pub create_time: String,
    #[serde(default)]
    pub skill_array: Vec<MssgPackSkill>,
    #[serde(default)]
    pub support_card_list: Vec<MssgPackSupportCard>,
    #[serde(default)]
    pub race_result_list: Vec<MssgPackRaceResult>,
    #[serde(default)]
    pub win_saddle_id_array: Vec<i64>,
    #[serde(default)]
    pub nickname_id_array: Vec<i64>,
    pub factor_id_array: Vec<i64>,
    pub factor_info_array: Vec<MssgPackFactorInfo>,
    #[serde(default)]
    pub succession_chara_array: Vec<MssgPackSuccessionChara>,
    #[serde(default)]
    pub succession_history_array: Vec<MssgPackSuccessionHistory>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackSkill {
    pub skill_id: i64,
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackSupportCard {
    pub position: i64,
    pub support_card_id: i64,
    pub exp: i64,
    pub limit_break_count: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackRaceResult {
    pub turn: i64,
    pub program_id: i64,
    pub weather: i64,
    pub ground_condition: i64,
    pub running_style: i64,
    pub popularity: i64,
    pub result_rank: i64,
    pub result_time: i64,
    pub prize_money: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackFactorInfo {
    pub factor_id: i64,
    pub level: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackSuccessionChara {
    pub position_id: i64,
    pub card_id: i64,
    pub rank: i64,
    pub rarity: i64,
    pub talent_level: i64,
    pub factor_id_array: Vec<i64>,
    pub factor_info_array: Vec<MssgPackFactorInfo>,
    pub win_saddle_id_array: Vec<i64>,
    pub owner_viewer_id: i64,
    #[serde(default)]
    pub race_result_list: Vec<MssgPackRaceResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackSuccessionHistory {
    #[serde(default)]
    pub id: i64,
    #[serde(default)]
    pub viewer_id: i64,
    #[serde(default)]
    pub trained_chara_id: i64,
    #[serde(default)]
    pub history_type: i64,
    #[serde(default)]
    pub succession_card_id: i64,
    #[serde(default)]
    pub date: i64,
    #[serde(default)]
    pub rental_viewer_id: i64,
    #[serde(default)]
    pub user_name: String,
    #[serde(default)]
    pub circle_name: String,
}

// Minimal root struct that only captures the data.trained_chara field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackDataOnly {
    #[serde(alias = "Data")]
    pub data: MssgPackDataContainer,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackDataContainer {
    #[serde(
        alias = "trainedChara",
        alias = "trained_chara_list",
        alias = "trainedCharaList",
        alias = "trained_chara_array"
    )]
    pub trained_chara: Vec<MssgPackTrainedChara>,
    #[serde(default, alias = "trainedCharaFavoriteArray")]
    pub trained_chara_favorite_array: Vec<MssgPackFavouriteCharaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MssgPackFavouriteCharaItem {
    pub icon_type: Option<i16>,
    pub memo: Option<String>,
    pub trained_chara_id: i64,
}

#[derive(Debug)]
pub struct MssgPackReadError {
    pub message: String,
}

impl std::fmt::Display for MssgPackReadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::error::Error for MssgPackReadError {}
