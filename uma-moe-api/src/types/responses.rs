use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReleaseNumInfo {
    pub act_num: i32,
    pub card_num: i32,
    pub chara_event_num: i32,
    pub chara_story_num: i32,
    pub good_end_num: i32,
    pub home_event_num: i32,
    pub main_story_num: i32,
    pub music_num: i32,
    pub scenario_event_num: i32,
    pub support_card_num: i32,
    pub support_event_num: i32,
    pub voice_num: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrophyNumInfo {
    pub grade_1: i32,
    pub grade_2: i32,
    pub grade_3: i32,
    pub grade_ex: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStadiumUser {
    pub best_point: i64,
    pub best_team_class: i32,
    pub team_class: i32,
    pub team_class_state: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub error: String,
    pub status: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionResponse {
    pub app_version: String,
    pub resource_version: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionHistoryResponse {
    pub current: VersionResponse,
    pub history: Vec<VersionResponse>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchResponse {
    pub items: Vec<UnifiedAccountRecord>,
    pub total: String,
    pub page: i32,
    pub limit: i32,
    pub total_pages: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnifiedAccountRecord {
    pub account_id: String,
    pub trainer_name: String,
    pub borrow_copy_count: i32,
    pub borrow_view_count: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    pub inheritance: Inheritance,
    pub support_card: SupportCard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inheritance {
    pub inheritance_id: i32,
    pub account_id: String,
    pub main_parent_id: i32,
    pub parent_left_id: i32,
    pub parent_right_id: i32,
    pub parent_rank: i32,
    pub parent_rarity: i32,
    pub blue_sparks: Vec<i32>,
    pub pink_sparks: Vec<i32>,
    pub green_sparks: Vec<i32>,
    pub white_sparks: Vec<i32>,
    pub win_count: i32,
    pub white_count: i32,
    pub main_blue_factors: i32,
    pub main_pink_factors: i32,
    pub main_green_factors: i32,
    pub main_white_factors: Vec<i32>,
    pub main_white_count: i32,
    pub left_blue_factors: i32,
    pub left_pink_factors: i32,
    pub left_green_factors: i32,
    pub left_white_factors: Vec<i32>,
    pub left_white_count: i32,
    pub right_blue_factors: i32,
    pub right_pink_factors: i32,
    pub right_green_factors: i32,
    pub right_white_factors: Vec<i32>,
    pub right_white_count: i32,
    pub main_win_saddles: Vec<i32>,
    pub left_win_saddles: Vec<i32>,
    pub right_win_saddles: Vec<i32>,
    pub race_results: Vec<i32>,
    pub blue_stars_sum: i32,
    pub pink_stars_sum: i32,
    pub green_stars_sum: i32,
    pub white_stars_sum: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affinity_score: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportCard {
    pub account_id: String,
    pub support_card_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit_break_count: Option<i32>,
    pub experience: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeteranSkill {
    pub level: i32,
    pub skill_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeteranSupportCard {
    pub position: i32,
    pub support_card_id: i32,
    pub exp: i32,
    pub limit_break_count: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VeteranFactor {
    pub factor_id: i32,
    pub level: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessionChara {
    pub card_id: i32,
    pub factor_info_array: Vec<VeteranFactor>,
    pub owner_viewer_id: i64,
    pub position_id: i32,
    pub rank: i32,
    pub rarity: i32,
    pub talent_level: i32,
    pub win_saddle_id_array: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Veteran {
    pub id: String,
    pub account_id: String,
    pub trained_chara_id: i32,
    pub card_id: i32,
    pub scenario_id: i32,
    pub route_id: i32,
    pub rarity: i32,
    pub succession_trained_chara_id_1: i32,
    pub succession_trained_chara_id_2: i32,
    pub succession_num: i32,
    pub speed: i32,
    pub stamina: i32,
    pub power: i32,
    pub wiz: i32,
    pub guts: i32,
    pub fans: i32,
    pub rank_score: i32,
    pub rank: i32,
    pub chara_grade: i32,
    pub talent_level: i32,
    pub running_style: i32,
    pub race_cloth_id: i32,
    pub nickname_id: i32,
    pub wins: i32,
    pub proper_ground_turf: i32,
    pub proper_ground_dirt: i32,
    pub proper_running_style_nige: i32,
    pub proper_running_style_senko: i32,
    pub proper_running_style_sashi: i32,
    pub proper_running_style_oikomi: i32,
    pub proper_distance_short: i32,
    pub proper_distance_mile: i32,
    pub proper_distance_middle: i32,
    pub proper_distance_long: i32,
    pub skill_array: Vec<VeteranSkill>,
    pub support_card_list: Vec<VeteranSupportCard>,
    pub factor_info_array: Vec<VeteranFactor>,
    pub win_saddle_id_array: Vec<i32>,
    pub succession_chara_array: Vec<SuccessionChara>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub register_time: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    pub ingested_at: String,
    pub updated_at: String,
    pub is_pinned: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamStadiumChara {
    pub id: i64,
    pub trainer_id: String,
    pub distance_type: i32,
    pub member_id: i32,
    pub trained_chara_id: i32,
    pub running_style: i32,
    pub card_id: i32,
    pub speed: i32,
    pub power: i32,
    pub stamina: i32,
    pub wiz: i32,
    pub guts: i32,
    pub fans: i32,
    pub rank_score: i32,
    pub skills: Vec<i32>,
    pub creation_time: String,
    pub scenario_id: i32,
    pub factors: Vec<i32>,
    pub support_cards: Vec<i32>,
    pub proper_ground_turf: i32,
    pub proper_ground_dirt: i32,
    pub proper_running_style_nige: i32,
    pub proper_running_style_senko: i32,
    pub proper_running_style_sashi: i32,
    pub proper_running_style_oikomi: i32,
    pub proper_distance_short: i32,
    pub proper_distance_mile: i32,
    pub proper_distance_middle: i32,
    pub proper_distance_long: i32,
    pub rarity: i32,
    pub talent_level: i32,
    pub team_rating: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainerInfo {
    pub account_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub best_team_class: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_circle_scout: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub follower_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_chara_dress_id: Option<i32>,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub own_follow_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rank_score: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_num_info: Option<ReleaseNumInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shame_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_class: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_evaluation_point: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_stadium_user: Option<TeamStadiumUser>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trophy_num_info: Option<TrophyNumInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BorrowStats {
    pub borrow_key: String,
    pub copy_count: i32,
    pub inheritance_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_copied_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_known_follower_num: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_recheck_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_viewed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_card_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub theoretical_copy_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub view_count: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer: Option<TrainerInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub borrow_stats: Option<BorrowStats>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle: Option<Circle>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_history: Option<Vec<CircleHistoryEntry>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fan_history: Option<FanHistory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inheritance: Option<Inheritance>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_card: Option<SupportCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team_stadium: Option<Vec<TeamStadiumChara>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub veterans: Option<Vec<Veteran>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Circle {
    pub circle_id: i64,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_viewer_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub leader_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_style: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub policy: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub monthly_point: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_month_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_month_point: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub archived: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_live_update: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_points: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_rank: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleHistoryEntry {
    pub year: i32,
    pub month: i32,
    pub circle_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_points: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleMemberFansMonthly {
    pub id: i32,
    pub circle_id: i64,
    pub viewer_id: i64,
    pub trainer_name: Option<String>,
    pub year: i32,
    pub month: i32,
    pub daily_fans: Vec<i64>,
    pub last_updated: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_circle_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_circle_name: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleResponse {
    pub circle: Circle,
    pub members: Vec<CircleMemberFansMonthly>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fans_to_next_tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fans_to_lower_tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_fans_to_next_tier: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_fans_to_lower_tier: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleWithRank {
    #[serde(flatten)]
    pub circle: Circle,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_rank: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CircleListResponse {
    pub circles: Vec<CircleWithRank>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankThreshold {
    pub rank_index: i32,
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_from: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranking_to: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_min_fans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_fans_per_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_vs_last_month_delta: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub daily_fans_delta: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_month_fans_per_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_month_min_fans: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_fans_per_day: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub yesterday_min_fans: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RankThresholdsResponse {
    pub thresholds: Vec<RankThreshold>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FanHistory {
    pub monthly: Vec<UserFanRankingMonthly>,
    pub rolling: UserFanRankingGains,
    pub alltime: UserFanRankingAlltime,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFanRankingMonthly {
    pub viewer_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_name: Option<String>,
    pub year: i32,
    pub month: i32,
    pub total_fans: i64,
    pub monthly_gain: i64,
    pub active_days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_daily: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_3d: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_7d: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_monthly: Option<f64>,
    pub rank: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub club_rank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_month_start: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shame_score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFanRankingGains {
    pub viewer_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_name: Option<String>,
    pub gain_3d: i64,
    pub gain_7d: i64,
    pub gain_30d: i64,
    pub rank_3d: i32,
    pub rank_7d: i32,
    pub rank_30d: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shame_score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserFanRankingAlltime {
    pub viewer_id: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_name: Option<String>,
    pub total_fans: i64,
    pub total_gain: i64,
    pub active_days: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_day: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_week: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avg_month: Option<f64>,
    pub rank: i32,
    pub rank_total_fans: i32,
    pub rank_total_gain: i32,
    pub rank_avg_day: i32,
    pub rank_avg_week: i32,
    pub rank_avg_month: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_id: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shame_score: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonthlyRankingsResponse {
    pub rankings: Vec<UserFanRankingMonthly>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub year: i32,
    pub month: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlltimeRankingsResponse {
    pub rankings: Vec<UserFanRankingAlltime>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GainsRankingsResponse {
    pub rankings: Vec<UserFanRankingGains>,
    pub total: i64,
    pub page: i64,
    pub limit: i64,
    pub total_pages: i64,
    pub sort_by: String,
}
