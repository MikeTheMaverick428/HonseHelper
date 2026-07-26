use crate::error::ApiError;
use crate::types::requests::*;
use crate::types::responses::*;

pub struct UmaMoeClient {
    base_url: String,
    client: reqwest::Client,
    api_key: Option<String>,
    user_agent: Option<String>,
}

impl UmaMoeClient {
    const LIB_USER_AGENT: &'static str = "uma-moe-api-rs/0.1.0";

    pub fn new() -> Self {
        let api_key = std::env::var("UMA_MOE_API_KEY").ok();
        Self {
            base_url: "https://uma.moe".to_string(),
            client: reqwest::Client::new(),
            api_key,
            user_agent: None,
        }
    }

    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.trim_end_matches('/').to_string();
        self
    }

    pub fn with_api_key(mut self, api_key: &str) -> Self {
        self.api_key = Some(api_key.to_string());
        self
    }

    pub fn with_user_agent(mut self, user_agent: &str) -> Self {
        self.user_agent = Some(user_agent.to_string());
        self
    }

    fn build_request(&self, method: reqwest::Method, url: String) -> reqwest::RequestBuilder {
        let mut builder = self.client.request(method, &url);

        if let Some(ref key) = self.api_key {
            builder = builder.header("X-API-Key", key);
        }

        // 1. Construct the formatted User-Agent header
        let ua_string = match &self.user_agent {
            // "Users' user agent + lib user agent"
            Some(app) => format!("{} {}", app, Self::LIB_USER_AGENT),
            // Fallback to just the library name if no app name is set
            None => Self::LIB_USER_AGENT.to_string(),
        };

        // 2. Inject it into the request
        builder = builder.header(reqwest::header::USER_AGENT, ua_string);

        builder
    }

    fn append_pair(qs: &mut String, key: &str, value: &str) {
        if !qs.is_empty() {
            qs.push('&');
        }
        qs.push_str(key);
        qs.push('=');
        qs.push_str(&url::form_urlencoded::byte_serialize(value.as_bytes()).collect::<String>());
    }

    fn append_opt_i32(qs: &mut String, key: &str, v: Option<i32>) {
        if let Some(val) = v {
            Self::append_pair(qs, key, &val.to_string());
        }
    }

    fn append_opt_str(qs: &mut String, key: &str, v: &Option<String>) {
        if let Some(val) = v {
            Self::append_pair(qs, key, val);
        }
    }

    fn append_vec_i32(qs: &mut String, key: &str, v: &[i32]) {
        for val in v {
            Self::append_pair(qs, key, &val.to_string());
        }
    }

    fn append_vec_str(qs: &mut String, key: &str, v: &[String]) {
        for val in v {
            Self::append_pair(qs, key, val);
        }
    }

    fn append_vec_comma(qs: &mut String, key: &str, v: &[String]) {
        if !v.is_empty() {
            Self::append_pair(qs, key, &v.join(","));
        }
    }

    fn search_params_to_query_string(params: &SearchParams) -> String {
        let mut qs = String::new();

        Self::append_opt_i32(&mut qs, "page", params.page);
        Self::append_opt_i32(&mut qs, "limit", params.limit);
        if let Some(ref st) = params.search_type {
            let s = match st {
                SearchType::Inheritance => "inheritance",
                SearchType::SupportCards => "support_cards",
                SearchType::All => "all",
            };
            Self::append_pair(&mut qs, "search_type", s);
        }
        Self::append_opt_str(&mut qs, "trainer_id", &params.trainer_id);
        Self::append_opt_str(&mut qs, "trainer_name", &params.trainer_name);
        Self::append_opt_i32(&mut qs, "max_follower_num", params.max_follower_num);
        Self::append_vec_i32(&mut qs, "main_parent_id", &params.main_parent_id);
        Self::append_vec_i32(
            &mut qs,
            "exclude_main_parent_id",
            &params.exclude_main_parent_id,
        );
        Self::append_vec_i32(&mut qs, "parent_id", &params.parent_id);
        Self::append_vec_i32(&mut qs, "parent_left_id", &params.parent_left_id);
        Self::append_vec_i32(&mut qs, "parent_right_id", &params.parent_right_id);
        Self::append_vec_i32(&mut qs, "exclude_parent_id", &params.exclude_parent_id);
        Self::append_opt_i32(&mut qs, "parent_rank", params.parent_rank);
        Self::append_opt_i32(&mut qs, "parent_rarity", params.parent_rarity);
        Self::append_vec_comma(&mut qs, "blue_sparks", &params.blue_sparks);
        Self::append_vec_comma(&mut qs, "pink_sparks", &params.pink_sparks);
        Self::append_vec_comma(&mut qs, "green_sparks", &params.green_sparks);
        Self::append_vec_comma(&mut qs, "white_sparks", &params.white_sparks);
        if let Some(v) = params.blue_sparks_9star {
            Self::append_pair(
                &mut qs,
                "blue_sparks_9star",
                if v { "true" } else { "false" },
            );
        }
        if let Some(v) = params.pink_sparks_9star {
            Self::append_pair(
                &mut qs,
                "pink_sparks_9star",
                if v { "true" } else { "false" },
            );
        }
        if let Some(v) = params.green_sparks_9star {
            Self::append_pair(
                &mut qs,
                "green_sparks_9star",
                if v { "true" } else { "false" },
            );
        }
        Self::append_vec_comma(
            &mut qs,
            "main_parent_blue_sparks",
            &params.main_parent_blue_sparks,
        );
        Self::append_vec_comma(
            &mut qs,
            "main_parent_pink_sparks",
            &params.main_parent_pink_sparks,
        );
        Self::append_vec_comma(
            &mut qs,
            "main_parent_green_sparks",
            &params.main_parent_green_sparks,
        );
        Self::append_vec_comma(
            &mut qs,
            "main_parent_white_sparks",
            &params.main_parent_white_sparks,
        );
        Self::append_opt_i32(
            &mut qs,
            "min_main_blue_factors",
            params.min_main_blue_factors,
        );
        Self::append_opt_i32(
            &mut qs,
            "min_main_pink_factors",
            params.min_main_pink_factors,
        );
        Self::append_opt_i32(
            &mut qs,
            "min_main_green_factors",
            params.min_main_green_factors,
        );
        Self::append_vec_str(&mut qs, "main_white_factors", &params.main_white_factors);
        Self::append_opt_i32(&mut qs, "min_main_white_count", params.min_main_white_count);
        Self::append_opt_i32(&mut qs, "min_blue_stars_sum", params.min_blue_stars_sum);
        Self::append_opt_i32(&mut qs, "max_blue_stars_sum", params.max_blue_stars_sum);
        Self::append_opt_i32(&mut qs, "min_pink_stars_sum", params.min_pink_stars_sum);
        Self::append_opt_i32(&mut qs, "max_pink_stars_sum", params.max_pink_stars_sum);
        Self::append_opt_i32(&mut qs, "min_green_stars_sum", params.min_green_stars_sum);
        Self::append_opt_i32(&mut qs, "max_green_stars_sum", params.max_green_stars_sum);
        Self::append_opt_i32(&mut qs, "min_white_stars_sum", params.min_white_stars_sum);
        Self::append_opt_i32(&mut qs, "max_white_stars_sum", params.max_white_stars_sum);
        Self::append_opt_i32(&mut qs, "min_win_count", params.min_win_count);
        Self::append_opt_i32(&mut qs, "min_white_count", params.min_white_count);
        Self::append_vec_str(
            &mut qs,
            "optional_white_sparks",
            &params.optional_white_sparks,
        );
        Self::append_vec_str(
            &mut qs,
            "optional_main_white_factors",
            &params.optional_main_white_factors,
        );
        Self::append_opt_i32(&mut qs, "support_card_id", params.support_card_id);
        Self::append_opt_i32(&mut qs, "min_limit_break", params.min_limit_break);
        Self::append_opt_i32(&mut qs, "max_limit_break", params.max_limit_break);
        Self::append_opt_i32(&mut qs, "min_experience", params.min_experience);
        Self::append_opt_str(&mut qs, "sort_by", &params.sort_by);
        if let Some(ref so) = params.sort_order {
            let s = match so {
                SortDir::Asc => "asc",
                SortDir::Desc => "desc",
            };
            Self::append_pair(&mut qs, "sort_order", s);
        }
        Self::append_opt_i32(&mut qs, "player_chara_id", params.player_chara_id);
        Self::append_opt_i32(&mut qs, "player_chara_id_2", params.player_chara_id_2);
        Self::append_opt_i32(
            &mut qs,
            "desired_main_chara_id",
            params.desired_main_chara_id,
        );

        qs
    }

    fn search_url(&self, params: &SearchParams, path: &str) -> String {
        let qs = Self::search_params_to_query_string(params);
        if qs.is_empty() {
            format!("{}{}", self.base_url, path)
        } else {
            format!("{}{}?{}", self.base_url, path, qs)
        }
    }

    pub async fn search(&self, params: SearchParams) -> Result<SearchResponse, ApiError> {
        let url = self.search_url(&params, "/api/v3/search");

        let response = self.build_request(reqwest::Method::GET, url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn get_profile(&self, account_id: &str) -> Result<ProfileResponse, ApiError> {
        let url = format!("{}/api/v4/user/profile/{}", self.base_url, account_id);

        let response = self.build_request(reqwest::Method::GET, url).send().await?;

        let status = response.status();
        if !status.is_success() {
            if status.as_u16() == 404 {
                return Err(ApiError::InvalidInput(format!(
                    "Trainer with ID {} not found",
                    account_id
                )));
            }
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn count(&self, params: SearchParams) -> Result<String, ApiError> {
        let url = self.search_url(&params, "/api/v3/count");

        let response = self.build_request(reqwest::Method::GET, url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        let json: serde_json::Value = response.json().await?;
        Ok(json["count"].as_str().unwrap_or("0").to_string())
    }

    pub async fn get_circle(
        &self,
        viewer_id: Option<i64>,
        circle_id: Option<i64>,
        month: Option<i32>,
        year: Option<i32>,
    ) -> Result<CircleResponse, ApiError> {
        let url = format!("{}/api/v4/circles", self.base_url);

        let mut request = self.build_request(reqwest::Method::GET, url);

        if let Some(vid) = viewer_id {
            request = request.query(&[("viewer_id", vid)]);
        }
        if let Some(cid) = circle_id {
            request = request.query(&[("circle_id", cid)]);
        }
        if let Some(m) = month {
            request = request.query(&[("month", m)]);
        }
        if let Some(y) = year {
            request = request.query(&[("year", y)]);
        }

        let response = request.send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn list_circles(
        &self,
        params: CircleListParams,
    ) -> Result<CircleListResponse, ApiError> {
        let url = format!("{}/api/v4/circles/list", self.base_url);

        let response = self
            .build_request(reqwest::Method::GET, url)
            .query(&params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn get_monthly_rankings(
        &self,
        params: MonthlyRankingParams,
    ) -> Result<MonthlyRankingsResponse, ApiError> {
        let url = format!("{}/api/v4/rankings/monthly", self.base_url);

        let response = self
            .build_request(reqwest::Method::GET, url)
            .query(&params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn get_alltime_rankings(
        &self,
        params: AlltimeRankingParams,
    ) -> Result<AlltimeRankingsResponse, ApiError> {
        let url = format!("{}/api/v4/rankings/alltime", self.base_url);

        let response = self
            .build_request(reqwest::Method::GET, url)
            .query(&params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn get_circle_rank_thresholds(&self) -> Result<RankThresholdsResponse, ApiError> {
        let url = format!("{}/api/v4/circles/rank-thresholds", self.base_url);

        let response = self.build_request(reqwest::Method::GET, url).send().await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }

    pub async fn get_gains_rankings(
        &self,
        params: GainsRankingParams,
    ) -> Result<GainsRankingsResponse, ApiError> {
        let url = format!("{}/api/v4/rankings/gains", self.base_url);

        let response = self
            .build_request(reqwest::Method::GET, url)
            .query(&params)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let error_body: Error = response.json().await?;
            return Err(ApiError::Api {
                error: error_body.error,
                status: status.as_u16(),
                details: error_body.details,
            });
        }

        Ok(response.json().await?)
    }
}

impl Default for UmaMoeClient {
    fn default() -> Self {
        Self::new()
    }
}
