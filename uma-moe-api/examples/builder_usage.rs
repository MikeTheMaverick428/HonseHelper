use serde_json::to_string_pretty;
use uma_moe_api::*;

#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // Without API key (public endpoints)
    let client = UmaMoeClient::new();

    // With API key (set via environment variable)
    // export UMA_MOE_API_KEY="your-api-key-here"
    // let client = UmaMoeClient::new();

    // Or pass API key programmatically
    // let client = UmaMoeClient::new().with_api_key("your-api-key");

    // Using builder pattern
    let params = requests::SearchParamsBuilder::new()
        .trainer_name("PlayerName")
        .min_win_count(5)
        .limit(3)
        .build();

    let results = client.search(params).await?;
    println!(
        "Builder pattern results:\n{}\n",
        to_string_pretty(&results)?
    );

    // Using struct directly (hybrid approach)
    let mut params2 = requests::SearchParams::default();
    params2.trainer_name = Some("PlayerName".to_string());
    params2.min_white_count = Some(10);
    params2.limit = Some(2);

    let results2 = client.search(params2).await?;
    println!("Struct pattern results:\n{}", to_string_pretty(&results2)?);

    Ok(())
}
