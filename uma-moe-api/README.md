# uma-moe-api

Rust client library and CLI tool for the [uma.moe](https://uma.moe) API.

## Features

- Typed Rust structs for all API responses and request parameters
- Search inheritance and support card records with composable filters
- Retrieve trainer profiles, circle data, and ranking leaderboards
- Builder pattern or direct struct mutation for parameter construction
- API key support via `UMA_MOE_API_KEY` env var or explicit setter
- Comparison test suite that validates typed deserialization against raw JSON responses

## Security: API Key Management

The API key is passed via the `X-API-Key` header. Two authorization methods are supported:

### 1. Environment Variable (Recommended)

```bash
export UMA_MOE_API_KEY="your-api-key-here"
cargo run -- search --trainer-name "PlayerName"
```

The library automatically reads `UMA_MOE_API_KEY` from the environment at initialization.

### 2. CLI Flag (Less Secure)

```bash
cargo run -- --api-key "your-api-key-here" search --trainer-name "PlayerName"
```

**Note**: CLI arguments may be visible in shell history or process listings. Prefer the environment variable method.

## Usage

### CLI

```bash
# Search for trainers
cargo run -- search --trainer-name "PlayerName" --limit 10

# Search with specific filters
cargo run -- search --trainer-id 100000000000 --min-win-count 5

# Get trainer profile
cargo run -- profile 100000000000

# Count matching records
cargo run -- count --trainer-name "PlayerName"

# Get circle details and member fan data
cargo run -- circle --circle-id 100000000
```

### Library

```rust
use uma_moe_api::*;

let client = UmaMoeClient::new(); // Reads UMA_MOE_API_KEY automatically

// Or with explicit key
let client = UmaMoeClient::new().with_api_key("your-key");

// Builder pattern
let params = SearchParamsBuilder::new()
    .trainer_name("PlayerName")
    .min_win_count(5)
    .build();

let results = client.search(params).await?;

// Direct struct mutation
let mut params = SearchParams::default();
params.trainer_name = Some("PlayerName".to_string());
params.limit = Some(10);

let results = client.search(params).await?;
```

## API Endpoints

| Method | HTTP | Endpoint | Description |
|--------|------|----------|-------------|
| `search()` | GET | `/api/v3/search` | Search inheritance/support card records |
| `count()` | GET | `/api/v3/count` | Count matching records |
| `get_profile(id)` | GET | `/api/v4/user/profile/{id}` | Get trainer profile |
| `get_circle(...)` | GET | `/api/v4/circles` | Get circle details with member fan data |
| `list_circles(params)` | GET | `/api/v4/circles/list` | List/search circles |
| `get_circle_rank_thresholds()` | GET | `/api/v4/circles/rank-thresholds` | Get circle rank thresholds |
| `get_monthly_rankings(params)` | GET | `/api/v4/rankings/monthly` | Monthly fan ranking |
| `get_alltime_rankings(params)` | GET | `/api/v4/rankings/alltime` | All-time fan ranking |
| `get_gains_rankings(params)` | GET | `/api/v4/rankings/gains` | Fan gains ranking |

## Error Handling

```rust
use uma_moe_api::*;

match client.search(params).await {
    Ok(response) => println!("Results: {}", response.total),
    Err(ApiError::Api { error, status, details }) => {
        eprintln!("API error {}: {} ({:?})", status, error, details);
    }
    Err(e) => eprintln!("Request failed: {}", e),
}
```

## Project Structure

```
src/
├── bin/cli.rs           # CLI binary (clap-based subcommands)
├── client.rs            # UmaMoeClient — all API methods
├── error.rs             # ApiError enum (Reqwest, Serde, Api, InvalidInput)
├── lib.rs               # Crate root — re-exports
└── types/
    ├── mod.rs           # Module declarations + re-exports
    ├── requests.rs      # Request params (SearchParams, CircleListParams, etc.)
    └── responses.rs     # Response types (ProfileResponse, SearchResponse, etc.)
tests/
└── response_comparison.rs  # 8 integration tests comparing typed vs raw JSON
```

## Testing

Requires `.env` file with test IDs:

```bash
cp .env.example .env
# Edit .env with valid test IDs
cargo test
```

The test suite fetches each endpoint twice — once through typed structs and once as raw `serde_json::Value` — then compares every field to catch deserialization mismatches.
