use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SearchType {
    #[serde(rename = "inheritance")]
    Inheritance,
    #[serde(rename = "support_cards")]
    SupportCards,
    #[serde(rename = "all")]
    All,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SortDir {
    #[serde(rename = "asc")]
    Asc,
    #[serde(rename = "desc")]
    Desc,
}

impl TryFrom<&str> for SortDir {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "asc" => Ok(Self::Asc),
            "desc" => Ok(Self::Desc),
            _ => Err(format!("Invalid sort_dir '{}': expected asc or desc", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CircleListSortBy {
    #[serde(rename = "name")]
    Name,
    #[serde(rename = "member_count")]
    MemberCount,
    #[serde(rename = "monthly_rank")]
    MonthlyRank,
    #[serde(rename = "monthly_point")]
    MonthlyPoint,
}

impl TryFrom<&str> for CircleListSortBy {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "name" => Ok(Self::Name),
            "member_count" => Ok(Self::MemberCount),
            "monthly_rank" => Ok(Self::MonthlyRank),
            "monthly_point" => Ok(Self::MonthlyPoint),
            _ => Err(format!(
                "Invalid sort_by '{}': expected name, member_count, monthly_rank, or monthly_point",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MonthlyRankingSortBy {
    #[serde(rename = "monthly_gain")]
    MonthlyGain,
    #[serde(rename = "total_fans")]
    TotalFans,
    #[serde(rename = "active_days")]
    ActiveDays,
    #[serde(rename = "avg_daily")]
    AvgDaily,
    #[serde(rename = "avg_3d")]
    Avg3d,
    #[serde(rename = "avg_7d")]
    Avg7d,
    #[serde(rename = "avg_monthly")]
    AvgMonthly,
}

impl TryFrom<&str> for MonthlyRankingSortBy {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "monthly_gain" => Ok(Self::MonthlyGain),
            "total_fans" => Ok(Self::TotalFans),
            "active_days" => Ok(Self::ActiveDays),
            "avg_daily" => Ok(Self::AvgDaily),
            "avg_3d" => Ok(Self::Avg3d),
            "avg_7d" => Ok(Self::Avg7d),
            "avg_monthly" => Ok(Self::AvgMonthly),
            _ => Err(format!("Invalid sort_by '{}': expected monthly_gain, total_fans, active_days, avg_daily, avg_3d, avg_7d, or avg_monthly", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlltimeRankingSortBy {
    #[serde(rename = "total_gain")]
    TotalGain,
    #[serde(rename = "total_fans")]
    TotalFans,
    #[serde(rename = "avg_day")]
    AvgDay,
    #[serde(rename = "avg_week")]
    AvgWeek,
    #[serde(rename = "avg_month")]
    AvgMonth,
}

impl TryFrom<&str> for AlltimeRankingSortBy {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "total_gain" => Ok(Self::TotalGain),
            "total_fans" => Ok(Self::TotalFans),
            "avg_day" => Ok(Self::AvgDay),
            "avg_week" => Ok(Self::AvgWeek),
            "avg_month" => Ok(Self::AvgMonth),
            _ => Err(format!("Invalid sort_by '{}': expected total_gain, total_fans, avg_day, avg_week, or avg_month", s)),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GainsRankingSortBy {
    #[serde(rename = "gain_3d")]
    Gain3d,
    #[serde(rename = "gain_7d")]
    Gain7d,
    #[serde(rename = "gain_30d")]
    Gain30d,
}

impl TryFrom<&str> for GainsRankingSortBy {
    type Error = String;
    fn try_from(s: &str) -> Result<Self, Self::Error> {
        match s {
            "gain_3d" => Ok(Self::Gain3d),
            "gain_7d" => Ok(Self::Gain7d),
            "gain_30d" => Ok(Self::Gain30d),
            _ => Err(format!(
                "Invalid sort_by '{}': expected gain_3d, gain_7d, or gain_30d",
                s
            )),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CircleListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_members: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<CircleListSortBy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_dir: Option<SortDir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct MonthlyRankingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<MonthlyRankingSortBy>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AlltimeRankingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<AlltimeRankingSortBy>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct GainsRankingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<GainsRankingSortBy>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct SearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_type: Option<SearchType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trainer_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_follower_num: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_parent_id: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_main_parent_id: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_id: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_left_id: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parent_right_id: Vec<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub exclude_parent_id: Vec<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rank: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_rarity: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blue_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pink_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub green_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub white_sparks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blue_sparks_9star: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pink_sparks_9star: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub green_sparks_9star: Option<bool>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_parent_blue_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_parent_pink_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_parent_green_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_parent_white_sparks: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_main_blue_factors: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_main_pink_factors: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_main_green_factors: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_white_factors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_main_white_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_blue_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_blue_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_pink_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_pink_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_green_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_green_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_white_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_white_stars_sum: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_win_count: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_white_count: Option<i32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional_white_sparks: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub optional_main_white_factors: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_card_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_limit_break: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_limit_break: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_experience: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_order: Option<SortDir>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_chara_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_chara_id_2: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desired_main_chara_id: Option<i32>,
}

pub struct SearchParamsBuilder {
    params: SearchParams,
}

impl SearchParamsBuilder {
    pub fn new() -> Self {
        Self {
            params: SearchParams::default(),
        }
    }

    pub fn page(mut self, page: i32) -> Self {
        self.params.page = Some(page);
        self
    }

    pub fn limit(mut self, limit: i32) -> Self {
        self.params.limit = Some(limit);
        self
    }

    pub fn search_type(mut self, search_type: SearchType) -> Self {
        self.params.search_type = Some(search_type);
        self
    }

    pub fn trainer_id(mut self, trainer_id: &str) -> Self {
        self.params.trainer_id = Some(trainer_id.to_string());
        self
    }

    pub fn trainer_name(mut self, trainer_name: &str) -> Self {
        self.params.trainer_name = Some(trainer_name.to_string());
        self
    }

    pub fn max_follower_num(mut self, max: i32) -> Self {
        self.params.max_follower_num = Some(max);
        self
    }

    pub fn main_parent_id(mut self, ids: Vec<i32>) -> Self {
        self.params.main_parent_id = ids;
        self
    }

    pub fn exclude_main_parent_id(mut self, ids: Vec<i32>) -> Self {
        self.params.exclude_main_parent_id = ids;
        self
    }

    pub fn parent_id(mut self, ids: Vec<i32>) -> Self {
        self.params.parent_id = ids;
        self
    }

    pub fn parent_left_id(mut self, ids: Vec<i32>) -> Self {
        self.params.parent_left_id = ids;
        self
    }

    pub fn parent_right_id(mut self, ids: Vec<i32>) -> Self {
        self.params.parent_right_id = ids;
        self
    }

    pub fn exclude_parent_id(mut self, ids: Vec<i32>) -> Self {
        self.params.exclude_parent_id = ids;
        self
    }

    pub fn parent_rank(mut self, rank: i32) -> Self {
        self.params.parent_rank = Some(rank);
        self
    }

    pub fn parent_rarity(mut self, rarity: i32) -> Self {
        self.params.parent_rarity = Some(rarity);
        self
    }

    pub fn blue_sparks(mut self, ids: Vec<String>) -> Self {
        self.params.blue_sparks = ids;
        self
    }

    pub fn pink_sparks(mut self, ids: Vec<String>) -> Self {
        self.params.pink_sparks = ids;
        self
    }

    pub fn green_sparks(mut self, ids: Vec<String>) -> Self {
        self.params.green_sparks = ids;
        self
    }

    pub fn white_sparks(mut self, ids: Vec<String>) -> Self {
        self.params.white_sparks = ids;
        self
    }

    pub fn blue_sparks_9star(mut self, value: bool) -> Self {
        self.params.blue_sparks_9star = Some(value);
        self
    }

    pub fn pink_sparks_9star(mut self, value: bool) -> Self {
        self.params.pink_sparks_9star = Some(value);
        self
    }

    pub fn green_sparks_9star(mut self, value: bool) -> Self {
        self.params.green_sparks_9star = Some(value);
        self
    }

    pub fn main_parent_blue_sparks(mut self, sparks: Vec<String>) -> Self {
        self.params.main_parent_blue_sparks = sparks;
        self
    }

    pub fn main_parent_pink_sparks(mut self, sparks: Vec<String>) -> Self {
        self.params.main_parent_pink_sparks = sparks;
        self
    }

    pub fn main_parent_green_sparks(mut self, sparks: Vec<String>) -> Self {
        self.params.main_parent_green_sparks = sparks;
        self
    }

    pub fn main_parent_white_sparks(mut self, sparks: Vec<String>) -> Self {
        self.params.main_parent_white_sparks = sparks;
        self
    }

    pub fn min_main_blue_factors(mut self, min: i32) -> Self {
        self.params.min_main_blue_factors = Some(min);
        self
    }

    pub fn min_main_pink_factors(mut self, min: i32) -> Self {
        self.params.min_main_pink_factors = Some(min);
        self
    }

    pub fn min_main_green_factors(mut self, min: i32) -> Self {
        self.params.min_main_green_factors = Some(min);
        self
    }

    pub fn min_main_white_count(mut self, min: i32) -> Self {
        self.params.min_main_white_count = Some(min);
        self
    }

    pub fn min_win_count(mut self, min: i32) -> Self {
        self.params.min_win_count = Some(min);
        self
    }

    pub fn min_white_count(mut self, min: i32) -> Self {
        self.params.min_white_count = Some(min);
        self
    }

    pub fn support_card_id(mut self, id: i32) -> Self {
        self.params.support_card_id = Some(id);
        self
    }

    pub fn min_limit_break(mut self, min: i32) -> Self {
        self.params.min_limit_break = Some(min);
        self
    }

    pub fn max_limit_break(mut self, max: i32) -> Self {
        self.params.max_limit_break = Some(max);
        self
    }

    pub fn min_experience(mut self, min: i32) -> Self {
        self.params.min_experience = Some(min);
        self
    }

    pub fn sort_by(mut self, field: &str) -> Self {
        self.params.sort_by = Some(field.to_string());
        self
    }

    pub fn sort_order(mut self, order: SortDir) -> Self {
        self.params.sort_order = Some(order);
        self
    }

    pub fn player_chara_id(mut self, id: i32) -> Self {
        self.params.player_chara_id = Some(id);
        self
    }

    pub fn desired_main_chara_id(mut self, id: i32) -> Self {
        self.params.desired_main_chara_id = Some(id);
        self
    }

    pub fn build(self) -> SearchParams {
        self.params
    }
}

impl Default for SearchParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}
