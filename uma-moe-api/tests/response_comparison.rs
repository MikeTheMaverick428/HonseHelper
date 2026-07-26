use dotenvy::dotenv;
use serde_json::Value;
use uma_moe_api::{types::requests::*, UmaMoeClient};

#[tokio::test]
async fn compare_search_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let params = SearchParams {
        limit: Some(2),
        search_type: Some(SearchType::All),
        ..Default::default()
    };

    let typed_response = client.search(params.clone()).await;
    let raw_response = fetch_raw_json("/api/v3/search", Some(&params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "search");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_profile_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let account_id = std::env::var("TEST_ACCOUNT_ID")
        .expect("TEST_ACCOUNT_ID must be set in .env (copy .env.example to .env)");

    let raw_response =
        fetch_raw_json(&format!("/api/v4/user/profile/{}", account_id), None::<&()>).await;

    let typed_response = client.get_profile(&account_id).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "profile");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_circle_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let circle_id: i64 = std::env::var("TEST_CIRCLE_ID")
        .expect("TEST_CIRCLE_ID must be set in .env (copy .env.example to .env)")
        .parse()
        .expect("TEST_CIRCLE_ID must be a valid integer");

    let typed_response = client.get_circle(None, Some(circle_id), None, None).await;

    let raw_params = [("circle_id", circle_id.to_string())];
    let raw_response = fetch_raw_json("/api/v4/circles", Some(&raw_params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "circle");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_circle_list_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let params = CircleListParams {
        limit: Some(2),
        sort_by: Some(CircleListSortBy::MonthlyPoint),
        sort_dir: Some(SortDir::Desc),
        ..Default::default()
    };

    let typed_response = client.list_circles(params.clone()).await;
    let raw_response = fetch_raw_json("/api/v4/circles/list", Some(&params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "circle_list");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_monthly_rankings_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let params = MonthlyRankingParams {
        limit: Some(2),
        sort_by: Some(MonthlyRankingSortBy::MonthlyGain),
        ..Default::default()
    };

    let typed_response = client.get_monthly_rankings(params.clone()).await;
    let raw_response = fetch_raw_json("/api/v4/rankings/monthly", Some(&params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "monthly_rankings");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_alltime_rankings_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let params = AlltimeRankingParams {
        limit: Some(2),
        ..Default::default()
    };

    let typed_response = client.get_alltime_rankings(params.clone()).await;
    let raw_response = fetch_raw_json("/api/v4/rankings/alltime", Some(&params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "alltime_rankings");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_gains_rankings_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let params = GainsRankingParams {
        limit: Some(2),
        sort_by: Some(GainsRankingSortBy::Gain30d),
        ..Default::default()
    };

    let typed_response = client.get_gains_rankings(params.clone()).await;
    let raw_response = fetch_raw_json("/api/v4/rankings/gains", Some(&params)).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "gains_rankings");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

#[tokio::test]
async fn compare_rank_thresholds_response() {
    dotenv().ok();
    let client = UmaMoeClient::new();

    let typed_response = client.get_circle_rank_thresholds().await;
    let raw_response = fetch_raw_json::<&()>("/api/v4/circles/rank-thresholds", None).await;

    match (typed_response, raw_response) {
        (Ok(typed), Ok(raw)) => {
            let typed_json = serde_json::to_value(&typed).unwrap();
            compare_json_values(&typed_json, &raw, "rank_thresholds");
        }
        (Err(e), _) => panic!("Typed client failed: {}", e),
        (_, Err(e)) => panic!("Raw request failed: {}", e),
    }
}

async fn fetch_raw_json<T>(
    endpoint: &str,
    params: Option<&T>,
) -> Result<Value, Box<dyn std::error::Error>>
where
    T: serde::Serialize + ?Sized,
{
    let base_url = "https://uma.moe";
    let client = reqwest::Client::new();
    let api_key = std::env::var("UMA_MOE_API_KEY").ok();

    let url = format!("{}{}", base_url, endpoint);
    let mut request = client.get(&url);

    if let Some(api_key) = api_key.as_deref() {
        request = request.header("X-API-Key", api_key);
    }

    if let Some(p) = params {
        request = request.query(p);
    }

    let response = request.send().await?;

    if !response.status().is_success() {
        return Err(format!("HTTP error: {}", response.status()).into());
    }

    let json = response.json::<Value>().await?;
    Ok(json)
}

fn compare_json_values(typed: &Value, raw: &Value, context: &str) {
    match (typed, raw) {
        (Value::Object(typed_obj), Value::Object(raw_obj)) => {
            for (key, typed_val) in typed_obj {
                if let Some(raw_val) = raw_obj.get(key) {
                    compare_json_values(typed_val, raw_val, &format!("{}.{}", context, key));
                } else if !is_optional_field(typed_val) {
                    panic!("Key '{}' missing in raw response for {}", key, context);
                }
            }

            for (key, raw_val) in raw_obj {
                if !typed_obj.contains_key(key) && !raw_val.is_null() {
                    panic!(
                        "Key '{}' in raw response but not in typed struct for {} (raw value: {})",
                        key, context, raw_val
                    );
                }
            }
        }
        (Value::Array(typed_arr), Value::Array(raw_arr)) => {
            if typed_arr.len() != raw_arr.len() {
                panic!(
                    "Array length mismatch in {}: typed={}, raw={}",
                    context,
                    typed_arr.len(),
                    raw_arr.len()
                );
            }
            for (i, (t, r)) in typed_arr.iter().zip(raw_arr.iter()).enumerate() {
                compare_json_values(t, r, &format!("{}[{}]", context, i));
            }
        }
        (Value::Number(t), Value::Number(r)) => {
            assert_eq!(t, r, "Numeric mismatch in {}", context);
        }
        (Value::String(t), Value::String(r)) => {
            assert_eq!(t, r, "String mismatch in {}", context);
        }
        (Value::Bool(t), Value::Bool(r)) => {
            assert_eq!(t, r, "Bool mismatch in {}", context);
        }
        (Value::Null, Value::Null) => {}
        (Value::Null, _) => {
            panic!("Typed has null but raw has value in {}", context);
        }
        (_, Value::Null) => {
            if !is_optional_field(typed) {
                panic!("Raw has null but typed has value in {}", context);
            }
        }
        _ => panic!(
            "Type mismatch in {}: typed={:?}, raw={:?}",
            context, typed, raw
        ),
    }
}

fn is_optional_field(value: &Value) -> bool {
    matches!(value, Value::Null)
}
