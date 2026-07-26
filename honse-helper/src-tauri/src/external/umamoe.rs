use std::{
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    ops::Sub,
    str::FromStr,
};

use crate::storage::sparks::SparkGroupStorage;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use shared::{
    db_models::{
        veteran_data::{Parent, UmaGroup, Veteran},
        UmaHash,
    },
    filters::Filter,
    models::UmaRank,
    veteran_browser::{SparkGroupRow, VeteranBrowserQuery, VeteranRow},
};
use uma_moe_api::{requests::SearchParams, types::responses::Inheritance};

pub fn adapt_inheritance(inheritance: Inheritance, last_updated: &str) -> UmaGroup {
    let veteran_sparks = collect_sparks(
        inheritance.main_blue_factors,
        inheritance.main_pink_factors,
        inheritance.main_green_factors,
        &inheritance.main_white_factors,
    );
    let parent_a_sparks = collect_sparks(
        inheritance.left_blue_factors,
        inheritance.left_pink_factors,
        inheritance.left_green_factors,
        &inheritance.left_white_factors,
    );
    let parent_b_sparks = collect_sparks(
        inheritance.right_blue_factors,
        inheritance.right_pink_factors,
        inheritance.right_green_factors,
        &inheritance.right_white_factors,
    );

    let veteran_wins = collect_wins(&inheritance.main_win_saddles);
    let parent_a_wins = collect_wins(&inheritance.left_win_saddles);
    let parent_b_wins = collect_wins(&inheritance.right_win_saddles);

    let hash_min = hash_uma_chara(inheritance.main_parent_id as i64, &veteran_sparks);

    let hash_main = hash_uma_veteran(
        inheritance.main_parent_id as i64,
        &veteran_sparks,
        inheritance.parent_rank,
        inheritance.parent_rarity,
    );

    let parent_a_hash = hash_uma_chara(inheritance.parent_left_id as i64, &parent_a_sparks);

    let parent_b_hash = hash_uma_chara(inheritance.parent_right_id as i64, &parent_b_sparks);

    let mut sparks_sum = HashMap::<i64, u16>::new();
    let mut sparks_count = HashMap::<i64, u8>::new();
    let mut wins_count = HashMap::<i64, u8>::new();

    shared::db_models::veteran_data::UmaGroup::gather_sums(
        &mut sparks_sum,
        &mut sparks_count,
        &mut wins_count,
        &veteran_sparks,
        &veteran_wins,
    );

    shared::db_models::veteran_data::UmaGroup::gather_sums(
        &mut sparks_sum,
        &mut sparks_count,
        &mut wins_count,
        &parent_a_sparks,
        &parent_a_wins,
    );

    shared::db_models::veteran_data::UmaGroup::gather_sums(
        &mut sparks_sum,
        &mut sparks_count,
        &mut wins_count,
        &parent_b_sparks,
        &parent_b_wins,
    );

    let trainer_id = inheritance
        .account_id
        .parse::<u64>()
        .expect("Cannot parse trainer_id");

    let parent_a = Parent {
        hash: parent_a_hash,
        trainee_id: inheritance.parent_left_id as i64,
        rank: UmaRank::from_raw(inheritance.parent_rarity as u16).into(),
        rarity: 0,
        trained_chara_id: inheritance.parent_left_id as i64,
        owner_id: Some(trainer_id),
        owned: false,
        container_sparks: parent_a_sparks,
        container_major_wins: parent_a_wins,
        ..Default::default()
    };

    let parent_b = Parent {
        hash: parent_b_hash,
        trainee_id: inheritance.parent_right_id as i64,
        rank: UmaRank::from_raw(inheritance.parent_rarity as u16).into(),
        rarity: 0,
        trained_chara_id: inheritance.parent_right_id as i64,
        owner_id: Some(trainer_id),
        owned: false,
        container_sparks: parent_b_sparks,
        container_major_wins: parent_b_wins,
        ..Default::default()
    };

    let veteran = Veteran {
        trainee_id: inheritance.main_parent_id as i64,
        hash: hash_main,
        created_at: parse_umamoe_datetime(last_updated),
        rank: UmaRank::from_raw(inheritance.parent_rarity as u16).into(),
        owner_id: Some(trainer_id),
        rank_score: inheritance.parent_rank as u32,
        parent_a: Some(parent_a_hash),
        parent_b: Some(parent_b_hash),
        is_race_data: false,
        is_browser: true,
        owned: false,
        rarity: 0,
        container_sparks: veteran_sparks,
        container_major_wins: veteran_wins,
        min_hash: Some(hash_min),
        ..Default::default()
    };

    UmaGroup {
        veteran,
        parent_a,
        parent_b,
        sparks_sum,
        sparks_count,
        wins_count,
        ..Default::default()
    }
}

fn hash_uma_chara(trainee_id: i64, factor_id_array: &[i64]) -> UmaHash {
    shared::db_models::veteran_data::hash_uma_entity(trainee_id, factor_id_array)
}

fn hash_uma_veteran(
    trainee_id: i64,
    factor_id_array: &[i64],
    rank_score: i32,
    rank: i32,
) -> UmaHash {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    trainee_id.hash(&mut hasher);

    let mut sorted_factors = factor_id_array.to_vec();
    sorted_factors.sort();
    sorted_factors.hash(&mut hasher);

    rank_score.hash(&mut hasher);
    rank.hash(&mut hasher);

    let hash = hasher.finish();
    UmaHash::from(hash)
}

fn collect_sparks(blue: i32, pink: i32, green: i32, white: &[i32]) -> Vec<i64> {
    let mut sparks = white.iter().map(|f| *f as i64).collect::<Vec<_>>();
    sparks.push(blue as i64);
    sparks.push(pink as i64);
    sparks.push(green as i64);
    sparks.sort();
    sparks
}

fn collect_wins(wins: &[i32]) -> Vec<i64> {
    let mut wins = wins.iter().map(|w| *w as i64).collect::<Vec<_>>();
    wins.sort();
    wins
}

fn parse_umamoe_datetime(value: &str) -> DateTime<Utc> {
    if value.trim().is_empty() {
        return DateTime::default();
    }
    DateTime::from_str(value)
        .ok()
        .or_else(|| {
            DateTime::parse_from_rfc3339(value)
                .ok()
                .map(|dt| dt.with_timezone(&Utc))
        })
        .or_else(|| {
            NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                .ok()
                .map(|dt| Utc.from_utc_datetime(&dt))
        })
        .unwrap_or_default()
}

pub fn adapt_query(query: &VeteranBrowserQuery, chosen_character_id: Option<i64>) -> SearchParams {
    let mut params = SearchParams::default();

    for filter in &query.filters {
        let _ = apply_filter(&mut params, filter);
    }

    params.sort_by = Some(match query.sort.key.as_str() {
        "Rank" => "parent_rank".to_string(),
        "WhiteSparkCount" => "white_count".to_string(),
        "MajorWinCount" => "win_count".to_string(),
        "CreatedAt" => "updated_at".to_string(),
        "Affinity" => "affinity".to_string(),
        _ => "updated_at".to_string(),
    });

    params.sort_order = Some(match query.sort.direction.as_str() {
        "Asc" => uma_moe_api::requests::SortDir::Asc,
        _ => uma_moe_api::requests::SortDir::Desc,
    });

    params.page = Some((query.page as i32).sub(1).max(0));
    params.limit = Some(query.page_size as i32);

    if let Some(char_id) = chosen_character_id {
        params.player_chara_id = Some(char_id as i32);
    }

    params
}

fn apply_filter(params: &mut SearchParams, filter: &Filter) -> Result<(), String> {
    match filter {
        Filter::Trainee(trainee_id) => {
            params.main_parent_id.push(*trainee_id as i32);
            Ok(())
        }
        Filter::Ranking { min, .. } => {
            if let Some(min_val) = min {
                params.parent_rank = Some(*min_val as i32);
            }
            Ok(())
        }
        Filter::Spark(spark) => {
            let group_id = spark.group_id as i32;
            if spark.on_trainee {
                let min = spark.min_stars.unwrap_or(1);
                let max = spark.max_stars.unwrap_or(3).clamp(1, 3);
                let ids: String = (min..=max)
                    .map(|lvl| format!("{group_id}{lvl:02}"))
                    .collect::<Vec<_>>()
                    .join(",");
                match spark.spark_type {
                    Some(t) if t == 1 => params.main_parent_blue_sparks.push(ids),
                    Some(t) if t == 2 => params.main_parent_pink_sparks.push(ids),
                    Some(t) if t == 3 => params.main_parent_green_sparks.push(ids),
                    _ => params.main_parent_white_sparks.push(ids),
                }
            } else {
                let min = spark.min_stars.unwrap_or(1);
                let max = spark.max_stars.unwrap_or(9).clamp(1, 9);
                let ids: String = (min..=max)
                    .map(|lvl| format!("{group_id}{lvl:02}"))
                    .collect::<Vec<_>>()
                    .join(",");
                match spark.spark_type {
                    Some(t) if t == 1 => params.blue_sparks.push(ids),
                    Some(t) if t == 2 => params.pink_sparks.push(ids),
                    Some(t) if t == 3 => params.green_sparks.push(ids),
                    _ => params.white_sparks.push(ids),
                }
            }
            Ok(())
        }
        Filter::WhiteSparkCount { min, .. } => {
            if let Some(min_val) = min {
                params.min_white_count = Some(*min_val);
            }
            Ok(())
        }
        Filter::MajorWinsCount { min, .. } => {
            if let Some(min_val) = min {
                params.min_win_count = Some(*min_val);
            }
            Ok(())
        }
        _ => Ok(()),
    }
}

pub fn group_to_veteran_row(
    group: &UmaGroup,
    affinity_score: Option<i32>,
    resolve_name: impl Fn(i64) -> Option<String>,
    last_updated: &str,
    spark_storage: &SparkGroupStorage,
) -> VeteranRow {
    let v = &group.veteran;
    let trainee_name = resolve_name(v.trainee_id);

    let major_wins_count = group.wins_count.len() as i64;
    let major_wins_on_veteran = group.veteran.container_major_wins.len() as i64;

    let white_spark_count = group.sparks_count.len() as i64;
    let vet_spark_groups: HashSet<i64> = group
        .veteran
        .container_sparks
        .iter()
        .map(|sid| sid / 100)
        .collect();
    let white_spark_on_veteran = vet_spark_groups.len() as i64;

    let mut vet_spark_levels: HashMap<i64, i64> = HashMap::new();
    for &sid in &v.container_sparks {
        let base_id = sid / 100;
        let level = sid % 100;
        *vet_spark_levels.entry(base_id).or_insert(0) += level as i64;
    }

    let mut spark_groups: Vec<SparkGroupRow> = Vec::new();
    for (&spark_group_id, &level_sum) in &group.sparks_sum {
        let uma_count = group
            .sparks_count
            .get(&spark_group_id)
            .copied()
            .unwrap_or(0) as i64;
        spark_groups.push(SparkGroupRow {
            veteran_hash: v.hash.as_i64(),
            spark_group_id,
            uma_count,
            level_sum: level_sum as i64,
            veteran_level_sum: vet_spark_levels.get(&spark_group_id).copied().unwrap_or(0),
            name: spark_storage.name(spark_group_id).to_string(),
            spark_type: spark_storage.spark_type(spark_group_id),
        });
    }

    VeteranRow {
        hash: v.hash.as_i64(),
        trainee_id: v.trainee_id,
        scenario: None,
        favorite_icon_type: None,
        favorite_memo: None,
        created_at: last_updated.to_string(),
        rank: v.rank as i64,
        rank_score: v.rank_score as i64,
        stat_speed: None,
        stat_stamina: None,
        stat_power: None,
        stat_guts: None,
        stat_wit: None,
        aptitude_turf: None,
        aptitude_dirt: None,
        aptitude_sprint: None,
        aptitude_mile: None,
        aptitude_medium: None,
        aptitude_long: None,
        aptitude_front: None,
        aptitude_pace_chaser: None,
        aptitude_late_surger: None,
        aptitude_end_closer: None,
        owner_id: v.owner_id.map(|id| id as i64),
        owned: false,
        active: true,
        rarity: Some(v.rarity as i64),
        talent_level: v.talent_level.map(|v| v as i64),
        trainee_name,
        major_wins_count,
        major_wins_on_veteran_count: major_wins_on_veteran,
        white_spark_count,
        white_spark_on_veteran_count: white_spark_on_veteran,
        spark_groups,
        min_hash: None,
        affinity: affinity_score,
        nickname_id: None,
    }
}
