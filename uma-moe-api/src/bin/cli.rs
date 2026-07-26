use clap::{Parser, Subcommand};
use serde_json::to_string;
use serde_json::to_string_pretty;
use uma_moe_api::requests::*;
use uma_moe_api::UmaMoeClient;

#[derive(Parser)]
#[command(name = "uma-moe-cli")]
#[command(about = "CLI tool for uma.moe API", long_about = None)]
struct Cli {
    /// API key for authenticated requests (or set UMA_MOE_API_KEY env var)
    #[arg(long, global = true)]
    api_key: Option<String>,

    /// Pretty-print JSON output
    #[arg(long, global = true)]
    pretty_json: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Search inheritance and support card records
    Search {
        /// Page number (0-indexed)
        #[arg(short, long, default_value = "0")]
        page: i32,

        /// Results per page (max 100)
        #[arg(short, long, default_value = "20")]
        limit: i32,

        /// Search type: inheritance, support_cards, or all
        #[arg(short, long)]
        search_type: Option<String>,

        /// Exact trainer ID match (9-12 digits)
        #[arg(long)]
        trainer_id: Option<String>,

        /// Partial trainer name match
        #[arg(long)]
        trainer_name: Option<String>,

        /// Max follower count (999 = available for friend request)
        #[arg(long)]
        max_follower_num: Option<i32>,

        /// Main parent character IDs
        #[arg(long, value_delimiter = ',')]
        main_parent_id: Vec<i32>,

        /// Parent rank (1-5)
        #[arg(long)]
        parent_rank: Option<i32>,

        /// Parent rarity (1-5)
        #[arg(long)]
        parent_rarity: Option<i32>,

        /// Blue spark codes (e.g. 101-109 for blue level 1-9)
        #[arg(long, value_delimiter = ',')]
        blue_sparks: Vec<String>,

        /// Pink spark codes (e.g. 201-209)
        #[arg(long, value_delimiter = ',')]
        pink_sparks: Vec<String>,

        /// Green spark codes (e.g. 301-309)
        #[arg(long, value_delimiter = ',')]
        green_sparks: Vec<String>,

        /// White spark codes (e.g. 401-409)
        #[arg(long, value_delimiter = ',')]
        white_sparks: Vec<String>,

        /// Main parent blue spark codes (e.g. 101-109)
        #[arg(long, value_delimiter = ',')]
        main_parent_blue_sparks: Vec<String>,

        /// Main parent pink spark codes (e.g. 201-209)
        #[arg(long, value_delimiter = ',')]
        main_parent_pink_sparks: Vec<String>,

        /// Main parent green spark codes (e.g. 301-309)
        #[arg(long, value_delimiter = ',')]
        main_parent_green_sparks: Vec<String>,

        /// Main parent white spark codes (e.g. 401-409)
        #[arg(long, value_delimiter = ',')]
        main_parent_white_sparks: Vec<String>,

        /// Minimum race win count
        #[arg(long)]
        min_win_count: Option<i32>,

        /// Support card ID
        #[arg(long)]
        support_card_id: Option<i32>,

        /// Minimum limit break count
        #[arg(long)]
        min_limit_break: Option<i32>,

        /// Maximum limit break count
        #[arg(long)]
        max_limit_break: Option<i32>,

        /// Sort field (e.g. follower_num, updated_at, win_count)
        #[arg(long)]
        sort_by: Option<String>,

        /// Sort direction: asc or desc (default: desc)
        #[arg(long)]
        sort_order: Option<String>,
    },

    /// Get complete trainer profile by account ID
    Profile {
        /// Trainer account ID (9-12 digits)
        account_id: String,
    },

    /// Count records matching search filters
    Count {
        /// Exact trainer ID match
        #[arg(long)]
        trainer_id: Option<String>,

        /// Partial trainer name match
        #[arg(long)]
        trainer_name: Option<String>,

        /// Main parent character IDs
        #[arg(long, value_delimiter = ',')]
        main_parent_id: Vec<i32>,

        /// Support card ID
        #[arg(long)]
        support_card_id: Option<i32>,
    },

    /// Get circle details and member fan data
    Circle {
        /// Get circle for this trainer viewer ID
        #[arg(long)]
        viewer_id: Option<i64>,

        /// Get circle by circle ID
        #[arg(long)]
        circle_id: Option<i64>,

        /// Month (1-12)
        #[arg(long)]
        month: Option<i32>,

        /// Year
        #[arg(long)]
        year: Option<i32>,
    },

    /// List and search circles
    CircleList {
        /// Page number (0-indexed)
        #[arg(short, long, default_value = "0")]
        page: i64,

        /// Results per page
        #[arg(short, long)]
        limit: Option<i64>,

        /// Partial match on circle name
        #[arg(long)]
        name: Option<String>,

        /// Minimum member count
        #[arg(long)]
        min_members: Option<i32>,

        /// Maximum monthly rank (lower = better)
        #[arg(long)]
        max_rank: Option<i32>,

        /// Sort field: name, member_count, monthly_rank, monthly_point
        #[arg(long)]
        sort_by: Option<String>,

        /// Sort direction: asc or desc (default: desc)
        #[arg(long)]
        sort_dir: Option<String>,

        /// General search (circle ID/name, leader ID/name, member ID/name)
        #[arg(long)]
        query: Option<String>,
    },

    /// Get circle rank tier thresholds
    RankThresholds,

    /// Monthly fan gain rankings
    RankingsMonthly {
        /// Month (1-12, defaults to current JST)
        #[arg(short, long)]
        month: Option<i32>,

        /// Year (defaults to current JST)
        #[arg(short, long)]
        year: Option<i32>,

        /// Page number (0-indexed)
        #[arg(short, long, default_value = "0")]
        page: i64,

        /// Results per page (max 100)
        #[arg(short, long, default_value = "100")]
        limit: i64,

        /// Search query (viewer_id, trainer_name, or circle_name)
        #[arg(short, long)]
        query: Option<String>,

        /// Sort field: monthly_gain, total_fans, active_days, avg_daily, avg_3d, avg_7d, avg_monthly
        #[arg(long)]
        sort_by: Option<String>,
    },

    /// All-time fan rankings
    RankingsAlltime {
        /// Page number (0-indexed)
        #[arg(short, long, default_value = "0")]
        page: i64,

        /// Results per page (max 100)
        #[arg(short, long, default_value = "100")]
        limit: i64,

        /// Search query (viewer_id, trainer_name, or circle_name)
        #[arg(short, long)]
        query: Option<String>,

        /// Sort field: total_gain, total_fans, avg_day, avg_week, avg_month
        #[arg(long)]
        sort_by: Option<String>,
    },

    /// Rolling gain rankings (3d/7d/30d)
    RankingsGains {
        /// Page number (0-indexed)
        #[arg(short, long, default_value = "0")]
        page: i64,

        /// Results per page (max 100)
        #[arg(short, long, default_value = "100")]
        limit: i64,

        /// Search query (viewer_id, trainer_name, or circle_name)
        #[arg(short, long)]
        query: Option<String>,

        /// Sort field: gain_3d, gain_7d, gain_30d
        #[arg(long)]
        sort_by: Option<String>,
    },
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let mut client = UmaMoeClient::new();

    if let Some(ref base_url) = std::env::var("UMA_MOE_API_BASE").ok() {
        client = client.with_base_url(base_url);
    };

    if let Some(ref api_key) = cli.api_key {
        client = client.with_api_key(api_key);
    }

    match cli.command {
        Commands::Search {
            page,
            limit,
            search_type,
            trainer_id,
            trainer_name,
            max_follower_num,
            main_parent_id,
            parent_rank,
            parent_rarity,
            blue_sparks,
            pink_sparks,
            green_sparks,
            white_sparks,
            main_parent_blue_sparks,
            main_parent_pink_sparks,
            main_parent_green_sparks,
            main_parent_white_sparks,
            min_win_count,
            support_card_id,
            min_limit_break,
            max_limit_break,
            sort_by,
            sort_order,
        } => {
            let mut params = SearchParams::default();
            params.page = Some(page);
            params.limit = Some(limit);
            params.trainer_id = trainer_id;
            params.trainer_name = trainer_name;
            params.max_follower_num = max_follower_num;
            params.main_parent_id = main_parent_id;
            params.blue_sparks = blue_sparks;
            params.pink_sparks = pink_sparks;
            params.green_sparks = green_sparks;
            params.white_sparks = white_sparks;
            params.main_parent_blue_sparks = main_parent_blue_sparks;
            params.main_parent_pink_sparks = main_parent_pink_sparks;
            params.main_parent_green_sparks = main_parent_green_sparks;
            params.main_parent_white_sparks = main_parent_white_sparks;
            params.parent_rank = parent_rank;
            params.parent_rarity = parent_rarity;
            params.min_win_count = min_win_count;
            params.support_card_id = support_card_id;
            params.min_limit_break = min_limit_break;
            params.max_limit_break = max_limit_break;
            params.sort_by = sort_by;
            if let Some(s) = sort_order {
                params.sort_order = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }

            if let Some(st) = search_type {
                params.search_type = Some(match st.as_str() {
                    "inheritance" => SearchType::Inheritance,
                    "support_cards" => SearchType::SupportCards,
                    _ => SearchType::All,
                });
            }

            let result = client.search(params).await?;

            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::Profile { account_id } => {
            let result = client.get_profile(&account_id).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::Count {
            trainer_id,
            trainer_name,
            main_parent_id,
            support_card_id,
        } => {
            let mut params = SearchParams::default();
            params.trainer_id = trainer_id;
            params.trainer_name = trainer_name;
            params.main_parent_id = main_parent_id;
            params.support_card_id = support_card_id;

            let count = client.count(params).await?;
            println!("Count: {}", count);
        }

        Commands::Circle {
            viewer_id,
            circle_id,
            month,
            year,
        } => {
            let result = client.get_circle(viewer_id, circle_id, month, year).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::CircleList {
            page,
            limit,
            name,
            min_members,
            max_rank,
            sort_by,
            sort_dir,
            query,
        } => {
            let mut params = CircleListParams::default();
            params.page = Some(page);
            params.limit = limit;
            params.name = name;
            params.min_members = min_members;
            params.max_rank = max_rank;
            if let Some(s) = sort_by {
                params.sort_by = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }
            if let Some(s) = sort_dir {
                params.sort_dir = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }
            params.query = query;

            let result = client.list_circles(params).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::RankThresholds => {
            let result = client.get_circle_rank_thresholds().await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::RankingsMonthly {
            month,
            year,
            page,
            limit,
            query,
            sort_by,
        } => {
            let mut params = MonthlyRankingParams::default();
            params.month = month;
            params.year = year;
            params.page = Some(page);
            params.limit = Some(limit);
            params.query = query;
            if let Some(s) = sort_by {
                params.sort_by = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }

            let result = client.get_monthly_rankings(params).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::RankingsAlltime {
            page,
            limit,
            query,
            sort_by,
        } => {
            let mut params = AlltimeRankingParams::default();
            params.page = Some(page);
            params.limit = Some(limit);
            params.query = query;
            if let Some(s) = sort_by {
                params.sort_by = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }

            let result = client.get_alltime_rankings(params).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }

        Commands::RankingsGains {
            page,
            limit,
            query,
            sort_by,
        } => {
            let mut params = GainsRankingParams::default();
            params.page = Some(page);
            params.limit = Some(limit);
            params.query = query;
            if let Some(s) = sort_by {
                params.sort_by = Some(s.as_str().try_into().map_err(anyhow::Error::msg)?);
            }

            let result = client.get_gains_rankings(params).await?;
            if cli.pretty_json {
                println!("{}", to_string_pretty(&result)?);
            } else {
                println!("{}", to_string(&result)?);
            }
        }
    }

    Ok(())
}
