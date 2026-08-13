use rusqlite::Connection;
use shared::{
    db_models::UmaHash,
    models::PaginationResponse,
    trainer_browser::{
        max_level_for_limit_break, BorrowSupportCardInfo, BorrowVeteranInfo, TrainerBrowserQuery,
        TrainerFilter, TrainerPageItem, TrainerSortConfig,
    },
    veteran_browser::SparkGroupRow,
};
use std::collections::HashMap;

type SqlParam = Box<dyn rusqlite::types::ToSql>;

const BASE_COLS: &str = "\
    t.trainer_id, \
    COALESCE(t.name, '') AS name, \
    COALESCE(t.friend_state, 0) AS friend_state, \
    t.last_login, \
    COALESCE(t.comment, '') AS comment, \
    COALESCE(t.fan, 0) AS fan, \
    COALESCE(t.honor_id, 0) AS honor_id, \
    COALESCE(t.circle_id, 0) AS circle_id, \
    COALESCE(t.circle_name, '') AS circle_name, \
    COALESCE(t.is_following, 0) AS is_following, \
    t.borrow_uma_hash, \
    t.borrow_uma_character_id, \
    t.borrow_uma_rank, \
    t.borrow_uma_rank_score, \
    tsc.support_card_id, \
    COALESCE(scd.name, '') AS sc_name, \
    COALESCE(scd.character_id, 0) AS sc_character_id, \
    COALESCE(scd.card_type, 0) AS sc_card_type, \
    COALESCE(scd.rarity, 0) AS sc_rarity, \
    COALESCE(tsc.level, 0) AS sc_level, \
    COALESCE(tsc.limit_break_count, 0) AS sc_limit_break, \
    COALESCE(td.name, '') AS trainee_name, \
    t.follower_num, \
    t.last_recheck_at, \
    t.updated_at, \
    t.last_update_source, \
    CASE WHEN sco.support_card_id IS NOT NULL THEN 1 ELSE 0 END AS sc_owned";

const FROM_CLAUSE: &str = "\
    FROM trainers t \
    LEFT JOIN trainer_support_card tsc ON tsc.trainer_id = t.trainer_id \
    LEFT JOIN support_card_data scd ON scd.id = tsc.support_card_id \
    LEFT JOIN support_card_owned sco ON sco.support_card_id = tsc.support_card_id \
    LEFT JOIN trainee_data td ON td.id = t.borrow_uma_character_id";

fn make_page_item(row: &rusqlite::Row) -> rusqlite::Result<TrainerPageItem> {
    let borrow_veteran: Option<BorrowVeteranInfo> = match row.get::<_, Option<i64>>(10)? {
        Some(hash) => Some(BorrowVeteranInfo {
            hash: UmaHash::from(hash),
            character_id: row.get(11)?,
            rank: row.get(12)?,
            rank_score: row.get(13)?,
            blue_sparks: Vec::new(),
            min_hash: None,
            affinity: None,
        }),
        None => None,
    };

    let borrow_support_card: Option<BorrowSupportCardInfo> = match row.get::<_, Option<i64>>(14)? {
        Some(id) => {
            let card_rarity: i64 = row.get(18)?;
            let limit_break_count: i64 = row.get(20)?;
            let lvl = max_level_for_limit_break(card_rarity, limit_break_count);
            Some(BorrowSupportCardInfo {
                id,
                name: row.get(15)?,
                character_id: row.get(16)?,
                card_type: row.get(17)?,
                card_rarity,
                level: lvl,
                limit_break_count,
                owned: row.get::<_, i64>(26)? != 0,
            })
        }
        None => None,
    };

    Ok(TrainerPageItem {
        trainer_id: row.get(0)?,
        name: row.get(1)?,
        friend_state: row.get(2)?,
        last_login: row.get(3)?,
        comment: row.get(4)?,
        fan: row.get(5)?,
        honor_id: row.get(6)?,
        circle_id: row.get(7)?,
        circle_name: row.get(8)?,
        is_following: row.get::<_, i64>(9)? != 0,
        follower_num: row.get(22)?,
        last_recheck_at: row.get(23)?,
        updated_at: row.get(24)?,
        last_update_source: row.get(25)?,
        borrow_veteran,
        borrow_support_card,
        borrow_uma_trainee_name: row.get(21)?,
    })
}

fn build_filter_where(filters: &[TrainerFilter]) -> (String, Vec<SqlParam>) {
    let mut clauses: Vec<String> = Vec::new();
    let mut params: Vec<SqlParam> = Vec::new();

    for f in filters {
        match f {
            TrainerFilter::NameSearch { query } => {
                if !query.is_empty() {
                    clauses.push("t.name LIKE ?".to_string());
                    params.push(Box::new(format!("%{}%", query)));
                }
            }
            TrainerFilter::Following { is_following } => {
                clauses.push("t.is_following = ?".to_string());
                params.push(Box::new(i64::from(*is_following)));
            }
            TrainerFilter::VeteranTrainee { ids, negate } => {
                if !ids.is_empty() {
                    let placeholders = ids
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format!("?{}", i + 1))
                        .collect::<Vec<_>>()
                        .join(",");
                    clauses.push(format!(
                        "v.trainee_id {} IN ({placeholders})",
                        if *negate { "NOT" } else { "" }
                    ));
                    for id in ids {
                        params.push(Box::new(*id));
                    }
                }
            }
            TrainerFilter::VeteranRank { min } => {
                clauses.push("COALESCE(v.rank, 0) >= ?".to_string());
                params.push(Box::new(*min));
            }
            TrainerFilter::ScType { card_types } => {
                if !card_types.is_empty() {
                    let placeholders = card_types
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format!("?{}", i + 1))
                        .collect::<Vec<_>>()
                        .join(",");
                    clauses.push(format!("scd.card_type IN ({placeholders})"));
                    for ct in card_types {
                        params.push(Box::new(*ct));
                    }
                }
            }
            TrainerFilter::ScRarity { rarities } => {
                if !rarities.is_empty() {
                    let placeholders = rarities
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format!("?{}", i + 1))
                        .collect::<Vec<_>>()
                        .join(",");
                    clauses.push(format!("scd.rarity IN ({placeholders})"));
                    for r in rarities {
                        params.push(Box::new(*r));
                    }
                }
            }
            TrainerFilter::ScLimitBreak { min, max } => {
                clauses.push("COALESCE(tsc.limit_break_count, 0) >= ?".to_string());
                params.push(Box::new(*min));
                clauses.push("COALESCE(tsc.limit_break_count, 0) <= ?".to_string());
                params.push(Box::new(*max));
            }
            TrainerFilter::ScCharacter { character_ids } => {
                if !character_ids.is_empty() {
                    let placeholders = character_ids
                        .iter()
                        .enumerate()
                        .map(|(i, _)| format!("?{}", i + 1))
                        .collect::<Vec<_>>()
                        .join(",");
                    clauses.push(format!("scd.character_id IN ({placeholders})"));
                    for cid in character_ids {
                        params.push(Box::new(*cid));
                    }
                }
            }
        }
    }

    if clauses.is_empty() {
        ("1=1".to_string(), params)
    } else {
        (clauses.join(" AND "), params)
    }
}

fn build_order_clause(sort: &TrainerSortConfig) -> String {
    let dir = match sort.direction.as_str() {
        "Asc" => "ASC",
        _ => "DESC",
    };
    let col = match sort.key.as_str() {
        "Name" => "t.name",
        "Fan" => "COALESCE(t.fan, 0)",
        "RankScore" => "COALESCE(v.rank_score, 0)",
        "Rank" => "COALESCE(v.rank, 0)",
        "ScLevel" => "COALESCE(tsc.level, 0)",
        _ => "t.name",
    };
    format!("{} {}", col, dir)
}

pub fn query_trainers_page(
    conn: &Connection,
    query: &TrainerBrowserQuery,
) -> Result<PaginationResponse<TrainerPageItem>, String> {
    let (where_clause, where_params) = build_filter_where(&query.filters);
    let order_clause = build_order_clause(&query.sort);

    let count_sql = format!("SELECT COUNT(*) {} WHERE {}", FROM_CLAUSE, where_clause);
    let data_sql = format!(
        "SELECT {} {} WHERE {} ORDER BY {} LIMIT ? OFFSET ?",
        BASE_COLS, FROM_CLAUSE, where_clause, order_clause
    );

    let total: u32 = {
        let mut stmt = conn
            .prepare(&count_sql)
            .map_err(|e| format!("count prepare failed: {e}"))?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            where_params.iter().map(|p| p.as_ref()).collect();
        stmt.query_row(param_refs.as_slice(), |row| row.get::<_, u32>(0))
            .map_err(|e| format!("count query failed: {e}"))?
    };

    let offset = query.page.saturating_sub(1) * query.page_size;
    let mut all_params: Vec<SqlParam> = where_params;
    all_params.push(Box::new(query.page_size));
    all_params.push(Box::new(offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        all_params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn
        .prepare(&data_sql)
        .map_err(|e| format!("data prepare failed: {e}"))?;
    let rows = stmt
        .query_map(param_refs.as_slice(), make_page_item)
        .map_err(|e| format!("data query failed: {e}"))?;
    let mut results = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("data collect failed: {e}"))?;

    // Batch fetch blue (stat) spark groups for borrowed veterans
    if !results.is_empty() {
        let mut hashes: Vec<i64> = results
            .iter()
            .filter_map(|t| t.borrow_veteran.as_ref().map(|v| v.hash.as_i64()))
            .collect();
        hashes.sort_unstable();
        hashes.dedup();
        if !hashes.is_empty() {
            let placeholders: Vec<String> = hashes.iter().map(|_| "?".to_string()).collect();
            let spark_sql = format!(
                "SELECT vss.veteran_hash, vss.spark_group_id, vss.uma_count, vss.level_sum, \
                        COALESCE(sd.name, ''), COALESCE(sd.spark_type, 0), vss.veteran_level_sum \
                 FROM veteran_spark_summary vss \
                 LEFT JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
                 WHERE vss.veteran_hash IN ({}) AND COALESCE(sd.spark_type, 0) = 1 \
                 GROUP BY vss.veteran_hash, vss.spark_group_id \
                 ORDER BY vss.veteran_hash, sd.spark_type, vss.level_sum DESC",
                placeholders.join(",")
            );
            let mut spark_stmt = conn
                .prepare(&spark_sql)
                .map_err(|e| format!("trainer blue sparks prepare failed: {e}"))?;
            let spark_hash_refs: Vec<Box<dyn rusqlite::types::ToSql>> = hashes
                .iter()
                .map(|h| Box::new(*h) as Box<dyn rusqlite::types::ToSql>)
                .collect();
            let spark_hash_params: Vec<&dyn rusqlite::types::ToSql> =
                spark_hash_refs.iter().map(|p| p.as_ref()).collect();
            let spark_rows = spark_stmt
                .query_map(spark_hash_params.as_slice(), |row| {
                    Ok(SparkGroupRow {
                        veteran_hash: row.get(0)?,
                        spark_group_id: row.get(1)?,
                        uma_count: row.get(2)?,
                        level_sum: row.get(3)?,
                        name: row.get(4)?,
                        spark_type: row.get(5)?,
                        veteran_level_sum: row.get(6)?,
                    })
                })
                .map_err(|e| format!("trainer blue sparks query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("trainer blue sparks collect failed: {e}"))?;

            let mut spark_map: HashMap<i64, Vec<SparkGroupRow>> = HashMap::new();
            for s in spark_rows {
                spark_map.entry(s.veteran_hash).or_default().push(s);
            }
            for t in &mut results {
                if let Some(v) = &mut t.borrow_veteran {
                    if let Some(sparks) = spark_map.remove(&v.hash.as_i64()) {
                        v.blue_sparks = sparks;
                    }
                }
            }

            // Batch fetch parent identity hashes (min_hash) for borrowed veterans
            let min_hash_sql = format!(
                "SELECT v.hash, v.min_hash FROM veterans v WHERE v.hash IN ({})",
                placeholders.join(",")
            );
            let mut min_hash_stmt = conn
                .prepare(&min_hash_sql)
                .map_err(|e| format!("trainer min_hash prepare failed: {e}"))?;
            let min_hash_rows = min_hash_stmt
                .query_map(spark_hash_params.as_slice(), |row| {
                    Ok((row.get::<_, i64>(0)?, row.get::<_, Option<i64>>(1)?))
                })
                .map_err(|e| format!("trainer min_hash query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("trainer min_hash collect failed: {e}"))?;
            let min_hash_map: HashMap<i64, Option<i64>> = min_hash_rows.into_iter().collect();
            for t in &mut results {
                if let Some(v) = &mut t.borrow_veteran {
                    if let Some(min_hash) = min_hash_map.get(&v.hash.as_i64()) {
                        v.min_hash = min_hash.map(UmaHash::from);
                    }
                }
            }
        }
    }

    Ok(PaginationResponse {
        results,
        total,
        page: query.page,
        page_size: query.page_size,
    })
}
