use crate::db::app_db;
use crate::external::umamoe;
use crate::handlers::api_config::ApiKeyState;
use crate::handlers::legacy_planner::LegacyPlannerStateHandle;
use crate::storage::sparks::SparkGroupStorage;
use crate::veterans::uma_moe_cache::UmaMoeCache;
use rusqlite::{params, Connection};
use serde::{Deserialize, Serialize};
use shared::{
    db_models::veteran_data::UmaGroup, filters::Filter,
    legacy_planner::lookup_dtos::AffinityResult, models::PaginationResponse, veteran_browser::*,
};
use std::collections::{HashMap, HashSet};
use tauri::State;

fn resolve_name(conn: &Connection, trainee_id: i64) -> Option<String> {
    conn.query_row(
        "SELECT name FROM trainee_data WHERE id = ?1",
        params![trainee_id],
        |row| row.get::<_, String>(0),
    )
    .ok()
}

fn load_spark_metadata(conn: &Connection) -> (HashMap<i64, String>, HashMap<i64, i32>) {
    let mut names = HashMap::new();
    let mut types = HashMap::new();
    let mut stmt = match conn.prepare("SELECT group_id, name, spark_type FROM spark_data") {
        Ok(s) => s,
        Err(_) => return (names, types),
    };
    let _ = stmt
        .query_map([], |row| {
            let group_id: i64 = row.get(0)?;
            let name: String = row.get(1)?;
            let spark_type: i32 = row.get(2)?;
            Ok((group_id, name, spark_type))
        })
        .map(|rows| {
            for r in rows.flatten() {
                names.insert(r.0, r.1);
                types.insert(r.0, r.2);
            }
        });
    (names, types)
}

fn load_win_metadata(conn: &Connection) -> (HashMap<i64, String>, HashMap<i64, i64>) {
    let mut names = HashMap::new();
    let mut priorities = HashMap::new();
    let sql = if crate::app_config::win_saddle_version() == 2 {
        "SELECT id, name, priority FROM major_wins_data WHERE win_saddle_type = 3"
    } else {
        "SELECT id, name, priority FROM major_wins_data"
    };
    let mut stmt = match conn.prepare(sql) {
        Ok(s) => s,
        Err(_) => return (names, priorities),
    };
    let _ = stmt
        .query_map([], |row| {
            let id: i64 = row.get(0)?;
            let name: Option<String> = row.get(1)?;
            let priority: Option<i64> = row.get(2)?;
            Ok((id, name, priority))
        })
        .map(|rows| {
            for r in rows.flatten() {
                if let Some(n) = r.1 {
                    names.insert(r.0, n);
                }
                if let Some(p) = r.2 {
                    priorities.insert(r.0, p);
                }
            }
        });
    (names, priorities)
}

#[tauri::command]
pub async fn query_uma_moe_veterans(
    query: VeteranBrowserQuery,
    planner_handle: State<'_, LegacyPlannerStateHandle>,
    api_key_state: State<'_, ApiKeyState>,
    cache: State<'_, UmaMoeCache>,
) -> Result<PaginationResponse<VeteranPageItem>, String> {
    let api_key = api_key_state
        .api_key
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "API key not configured".to_string())?;

    let chosen_character_id = planner_handle
        .state
        .lock()
        .map_err(|e| e.to_string())?
        .as_ref()
        .and_then(|s| s.chosen_character.as_ref().map(|c| c.character_id));

    let params = umamoe::adapt_query(&query, chosen_character_id);

    let client = uma_moe_api::UmaMoeClient::new().with_api_key(&api_key);
    let search_resp = client
        .search(params)
        .await
        .map_err(|e| format!("API error: {e}"))?;

    let api_total: u32 = search_resp
        .total
        .parse::<u32>()
        .unwrap_or((search_resp.total_pages as u32) * (search_resp.limit as u32).max(1));

    let conn = app_db::open_app_database_connection().ok();

    // Pass 1: adapt groups and collect all spark_group_ids
    let mut groups_batch: Vec<(UmaGroup, Option<i32>, String)> = Vec::new();
    let mut all_spark_ids: Vec<i64> = Vec::new();

    for record in search_resp.items {
        let affinity_score = record.inheritance.affinity_score;
        let last_updated = record.last_updated.as_deref().unwrap_or("").to_string();
        let mut group = umamoe::adapt_inheritance(record.inheritance, &last_updated);

        if crate::app_config::win_saddle_version() == 2 {
            if let Some(ref db_conn) = conn {
                filter_type3_wins(&mut group, db_conn);
            }
        }

        all_spark_ids.extend(group.sparks_sum.keys().copied());
        groups_batch.push((group, affinity_score, last_updated));
    }

    all_spark_ids.sort();
    all_spark_ids.dedup();

    let spark_storage = conn
        .as_ref()
        .map(|c| SparkGroupStorage::new(c, &all_spark_ids))
        .unwrap_or_default();

    // Pass 2: build rows
    let mut items: Vec<VeteranPageItem> = Vec::new();
    let mut cache_batch = Vec::new();

    for (group, affinity_score, last_updated) in groups_batch {
        let hash_u64 = group.veteran.hash.as_u64();

        let affinity = affinity_score.map(|s| AffinityResult {
            base: s as u32,
            bonus: 0,
        });

        let row = umamoe::group_to_veteran_row(
            &group,
            affinity_score,
            |trainee_id| conn.as_ref().and_then(|c| resolve_name(c, trainee_id)),
            &last_updated,
            &spark_storage,
        );
        items.push(VeteranPageItem {
            veteran: row,
            affinity,
            tags: Vec::new(),
        });
        cache_batch.push((hash_u64, group));
    }

    cache.store_batch(cache_batch);

    if let Some(Filter::Affinity { min }) = query
        .filters
        .iter()
        .find(|f| matches!(f, Filter::Affinity { .. }))
    {
        items.retain(|item| item.affinity.map(|af| af.total()).unwrap_or(0) >= *min);
    }

    Ok(PaginationResponse {
        results: items,
        total: api_total,
        page: query.page,
        page_size: query.page_size,
    })
}

#[tauri::command]
pub async fn save_uma_moe_veteran(
    hash: String,
    api_key_state: State<'_, ApiKeyState>,
    cache: State<'_, UmaMoeCache>,
) -> Result<String, String> {
    let hash_u64 = u64::from_str_radix(&hash, 16).map_err(|e| format!("invalid hash: {e}"))?;
    let group = cache
        .get(hash_u64)
        .ok_or_else(|| "veteran not found in cache".to_string())?;

    let owner_id_u64 = group
        .veteran
        .owner_id
        .ok_or_else(|| "veteran has no owner_id".to_string())?;
    let account_id = owner_id_u64.to_string();
    let owner_id_i64: i64 = owner_id_u64
        .try_into()
        .map_err(|_| format!("owner_id out of range: {owner_id_u64}"))?;

    let api_key = api_key_state
        .api_key
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "API key not configured".to_string())?;

    let client = uma_moe_api::UmaMoeClient::new().with_api_key(&api_key);
    let profile = client
        .get_profile(&account_id, Some(&uma_moe_api::types::requests::ProfileRequest::exclude_lite()))
        .await
        .map_err(|e| format!("API error: {e}"))?;

    let inh = profile
        .inheritance
        .ok_or_else(|| "trainer has no active borrow veteran".to_string())?;

    let last_updated = profile
        .borrow_stats
        .as_ref()
        .and_then(|b| b.last_recheck_at.clone())
        .unwrap_or_default();

    let mut group = umamoe::adapt_inheritance(inh, &last_updated);
    group.veteran.is_browser = true;

    let trainer_name = profile
        .trainer
        .as_ref()
        .map(|t| t.name.clone())
        .unwrap_or_default();
    let comment = profile
        .trainer
        .as_ref()
        .and_then(|t| t.comment.clone())
        .unwrap_or_default();
    let circle_id = profile
        .circle
        .as_ref()
        .map(|c| c.circle_id)
        .unwrap_or(0);
    let circle_name = profile
        .circle
        .as_ref()
        .map(|c| c.name.clone())
        .unwrap_or_default();
    let fan = profile
        .fan_history
        .as_ref()
        .map(|fh| fh.alltime.total_fans)
        .unwrap_or(0);

    let hash = group.veteran.hash.as_i64();
    let character_id = group.veteran.trainee_id;
    let rarity = i64::from(group.veteran.rarity);
    let rank = i64::from(group.veteran.rank);
    let rank_score = i64::from(group.veteran.rank_score);

    let mut conn = app_db::open_app_database_connection()?;
    let tx = conn
        .transaction()
        .map_err(|e| format!("failed to start save transaction: {e}"))?;

    crate::veterans::process_group_direct(&group, &tx)?;
    crate::veterans::deactivate_trainer_veterans(&tx, owner_id_i64)?;

    tx.execute(
        r#"INSERT INTO trainers (
            trainer_id, name, friend_state, is_following, honor_id, last_login,
            comment, fan, circle_id, circle_name, borrow_uma_hash, borrow_uma_character_id,
            borrow_uma_rarity, borrow_uma_rank, borrow_uma_rank_score, created_at, updated_at,
            last_update_source
        ) VALUES (?1, ?2, 0, 0, 0, NULL, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, datetime('now'), datetime('now'), 'uma_moe')
        ON CONFLICT(trainer_id) DO UPDATE SET
            name = excluded.name,
            comment = excluded.comment,
            fan = excluded.fan,
            circle_id = excluded.circle_id,
            circle_name = excluded.circle_name,
            borrow_uma_hash = excluded.borrow_uma_hash,
            borrow_uma_character_id = excluded.borrow_uma_character_id,
            borrow_uma_rarity = excluded.borrow_uma_rarity,
            borrow_uma_rank = excluded.borrow_uma_rank,
            borrow_uma_rank_score = excluded.borrow_uma_rank_score,
            updated_at = datetime('now'),
            last_update_source = 'uma_moe'
        "#,
        params![
            owner_id_i64,
            trainer_name,
            comment,
            fan,
            circle_id,
            circle_name,
            hash,
            character_id,
            rarity,
            rank,
            rank_score,
        ],
    )
    .map_err(|e| format!("upsert trainer: {e}"))?;

    if let Some(sc) = &profile.support_card {
        let support_card_id = i64::from(sc.support_card_id);
        if support_card_id > 0 {
            let limit_break_count = i64::from(sc.limit_break_count.unwrap_or(0));
            let sc_rarity: i64 = tx
                .query_row(
                    "SELECT rarity FROM support_card_data WHERE id = ?1",
                    params![support_card_id],
                    |row| row.get::<_, i64>(0),
                )
                .unwrap_or(0);
            let level =
                shared::trainer_browser::max_level_for_limit_break(sc_rarity, limit_break_count);
            tx.execute(
                r#"INSERT INTO trainer_support_card (
                    trainer_id, support_card_id, level, limit_break_count
                ) VALUES (?1, ?2, ?3, ?4)
                ON CONFLICT(trainer_id, support_card_id) DO UPDATE SET
                    level = excluded.level,
                    limit_break_count = excluded.limit_break_count
                "#,
                params![owner_id_i64, support_card_id, level, limit_break_count],
            )
            .map_err(|e| format!("upsert support card: {e}"))?;
        }
    }

    tx.commit()
        .map_err(|e| format!("commit: {e}"))?;

    Ok("saved".to_string())
}

fn filter_type3_wins(group: &mut UmaGroup, conn: &Connection) {
    let mut stmt = match conn.prepare("SELECT id FROM major_wins_data WHERE win_saddle_type = 3") {
        Ok(s) => s,
        Err(_) => return,
    };
    let type3_ids: HashSet<i64> = match stmt.query_map([], |row| row.get(0)) {
        Ok(rows) => rows.filter_map(|r| r.ok()).collect(),
        Err(_) => return,
    };

    group.wins_count.retain(|k, _| type3_ids.contains(k));
    group
        .veteran
        .container_major_wins
        .retain(|id| type3_ids.contains(id));
    group
        .parent_a
        .container_major_wins
        .retain(|id| type3_ids.contains(id));
    group
        .parent_b
        .container_major_wins
        .retain(|id| type3_ids.contains(id));
}

#[derive(Serialize, Deserialize)]
pub struct UmaMoeVeteranDetail {
    pub sparks: Vec<SparkGroupRow>,
    pub wins: Vec<MajorWinRow>,
    pub parents: Vec<ParentRow>,
}

#[tauri::command]
pub async fn get_uma_moe_veteran_detail(
    hash: String,
    cache: State<'_, UmaMoeCache>,
) -> Result<UmaMoeVeteranDetail, String> {
    let hash_u64 = u64::from_str_radix(&hash, 16).map_err(|e| format!("invalid hash: {e}"))?;

    let group = cache
        .get(hash_u64)
        .ok_or_else(|| "veteran not found in cache".to_string())?;
    let v = &group.veteran;

    let conn = app_db::open_app_database_connection().ok();

    let (spark_names, spark_types) = conn
        .as_ref()
        .map(|c| load_spark_metadata(c))
        .unwrap_or_default();

    let (win_names, win_priorities) = conn
        .as_ref()
        .map(|c| load_win_metadata(c))
        .unwrap_or_default();

    let mut vet_spark_levels: HashMap<i64, i64> = HashMap::new();
    for &sid in &v.container_sparks {
        let base_id = sid / 100;
        let level = sid % 100;
        *vet_spark_levels.entry(base_id).or_insert(0) += level as i64;
    }

    let mut sparks: Vec<SparkGroupRow> = Vec::new();
    for (&spark_group_id, &level_sum) in &group.sparks_sum {
        let uma_count = group
            .sparks_count
            .get(&spark_group_id)
            .copied()
            .unwrap_or(0) as i64;
        let vet_sum = vet_spark_levels.get(&spark_group_id).copied().unwrap_or(0);

        sparks.push(SparkGroupRow {
            veteran_hash: v.hash.as_i64(),
            spark_group_id,
            uma_count,
            level_sum: level_sum as i64,
            veteran_level_sum: vet_sum,
            name: spark_names
                .get(&spark_group_id)
                .cloned()
                .unwrap_or_default(),
            spark_type: spark_types.get(&spark_group_id).copied().unwrap_or(0) as i64,
        });
    }
    sparks.sort_by(|a, b| {
        let type_a = spark_types.get(&a.spark_group_id).copied().unwrap_or(0);
        let type_b = spark_types.get(&b.spark_group_id).copied().unwrap_or(0);
        let ta = if type_a == 4 || type_a == 5 {
            4
        } else {
            type_a
        };
        let tb = if type_b == 4 || type_b == 5 {
            4
        } else {
            type_b
        };
        ta.cmp(&tb)
            .then(b.veteran_level_sum.cmp(&a.veteran_level_sum))
            .then(b.level_sum.cmp(&a.level_sum))
    });

    let mut wins: Vec<MajorWinRow> = Vec::new();
    for (&win_saddle_id, &count) in &group.wins_count {
        let on_vet = v.container_major_wins.contains(&win_saddle_id);
        wins.push(MajorWinRow {
            id: win_saddle_id,
            name: win_names.get(&win_saddle_id).cloned(),
            group_id: Some(win_saddle_id),
            shared_count: Some(count as i64),
            on_veteran: on_vet,
            priority: win_priorities.get(&win_saddle_id).copied(),
        });
    }
    wins.sort_by_key(|w| w.priority.unwrap_or(i64::MAX));

    let mut parents: Vec<ParentRow> = Vec::new();
    for p in [&group.parent_a, &group.parent_b] {
        let parent_sparks: Vec<SparkGroupRow> = p
            .container_sparks
            .iter()
            .map(|&sid| {
                let base_id = sid / 100;
                let level = sid % 100;
                SparkGroupRow {
                    veteran_hash: p.hash.as_i64(),
                    spark_group_id: base_id,
                    uma_count: 0,
                    level_sum: level as i64,
                    veteran_level_sum: 0,
                    name: spark_names.get(&base_id).cloned().unwrap_or_default(),
                    spark_type: spark_types.get(&base_id).copied().unwrap_or(0) as i64,
                }
            })
            .collect();

        parents.push(ParentRow {
            hash: p.hash.as_i64(),
            trainee_id: p.trainee_id,
            rank: p.rank as i64,
            rarity: p.rarity as i64,
            talent_level: None,
            trainee_name: conn.as_ref().and_then(|c| resolve_name(c, p.trainee_id)),
            major_wins_count: p.container_major_wins.len() as i64,
            spark_count: p.container_sparks.len() as i64,
            blue_sparks: parent_sparks,
            owner_id: p.owner_id.map(i64::try_from).transpose().unwrap_or(None),
            owned: p.owned as i64,
        });
    }

    Ok(UmaMoeVeteranDetail {
        sparks,
        wins,
        parents,
    })
}

#[tauri::command]
pub fn get_uma_moe_parent_sparks(
    hash: String,
    cache: State<'_, UmaMoeCache>,
) -> Result<Vec<SparkGroupRow>, String> {
    let parent_hash = hash
        .parse::<i64>()
        .map(|v| v as u64)
        .map_err(|e| format!("invalid parent hash: {e}"))?;
    let parent = cache
        .find_parent(parent_hash)
        .ok_or_else(|| "parent not found in cache".to_string())?;
    let conn = app_db::open_app_database_connection()?;
    let (spark_names, spark_types) = load_spark_metadata(&conn);
    let mut rows: Vec<SparkGroupRow> = parent
        .container_sparks
        .iter()
        .map(|&sid| {
            let base_id = sid / 100;
            let level = sid % 100;
            SparkGroupRow {
                veteran_hash: parent.hash.as_i64(),
                spark_group_id: base_id,
                uma_count: 0,
                level_sum: level as i64,
                veteran_level_sum: 0,
                name: spark_names.get(&base_id).cloned().unwrap_or_default(),
                spark_type: spark_types.get(&base_id).copied().unwrap_or(0) as i64,
            }
        })
        .collect();
    rows.sort_by(|a, b| {
        let ta = if a.spark_type == 4 || a.spark_type == 5 {
            4
        } else {
            a.spark_type
        };
        let tb = if b.spark_type == 4 || b.spark_type == 5 {
            4
        } else {
            b.spark_type
        };
        ta.cmp(&tb).then(b.level_sum.cmp(&a.level_sum))
    });
    Ok(rows)
}

#[tauri::command]
pub fn get_uma_moe_parent_wins(
    hash: String,
    cache: State<'_, UmaMoeCache>,
) -> Result<Vec<MajorWinRow>, String> {
    let parent_hash = hash
        .parse::<i64>()
        .map(|v| v as u64)
        .map_err(|e| format!("invalid parent hash: {e}"))?;
    let parent = cache
        .find_parent(parent_hash)
        .ok_or_else(|| "parent not found in cache".to_string())?;
    let conn = app_db::open_app_database_connection()?;
    let (win_names, win_priorities) = load_win_metadata(&conn);
    let mut rows: Vec<MajorWinRow> = parent
        .container_major_wins
        .iter()
        .map(|&win_id| MajorWinRow {
            id: win_id,
            name: win_names.get(&win_id).cloned(),
            group_id: Some(win_id),
            shared_count: None,
            on_veteran: false,
            priority: win_priorities.get(&win_id).copied(),
        })
        .collect();
    rows.sort_by_key(|w| w.priority.unwrap_or(i64::MAX));
    Ok(rows)
}
