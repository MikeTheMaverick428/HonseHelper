use crate::db::app_db;
use crate::handlers::legacy_planner::LegacyPlannerStateHandle;
use crate::storage::affinity::AffinityStorage;
use crate::storage::veterans::{make_veteran_row, veteran_select_cols, VeteranStore, VETERAN_FROM};
use rusqlite::{params, OptionalExtension};
use shared::{
    models::PaginationResponse, models::SparkType, veteran_browser::*, SkillDataRow, SkillType,
};
use std::sync::Mutex;
use tauri::State;

#[tauri::command]
pub fn query_veteran_store_page(
    query: VeteranBrowserQuery,
    planner_handle: State<'_, LegacyPlannerStateHandle>,
    veteran_store: State<'_, Mutex<VeteranStore>>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<PaginationResponse<VeteranPageItem>, String> {
    let conn = app_db::open_app_database_connection()?;
    let planner_state = planner_handle
        .state
        .lock()
        .map_err(|e| e.to_string())?
        .clone();

    let mut store = veteran_store.lock().map_err(|e| e.to_string())?;
    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;

    store.apply_filters(
        &conn,
        &affinity,
        &query.filters,
        &query.sort,
        &planner_state,
        &query.legacy_planner_slot,
    )?;
    let items = store.get_page(&conn, query.page as usize, query.page_size as usize)?;
    Ok(PaginationResponse {
        results: items,
        total: store.total_count() as u32,
        page: query.page,
        page_size: query.page_size,
    })
}

#[tauri::command]
pub fn get_veteran_detail(hash: String) -> Result<Option<VeteranRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let sql = format!(
        "SELECT {} {} WHERE v.hash = ?",
        veteran_select_cols(),
        VETERAN_FROM
    );
    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("detail prepare failed: {e}"))?;
    let row = stmt
        .query_row(params![hash], make_veteran_row)
        .optional()
        .map_err(|e| format!("detail query failed: {e}"))?;
    Ok(row)
}

#[tauri::command]
pub fn get_veteran_sparks(hash: String) -> Result<Vec<SparkGroupRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let mut stmt = conn
            .prepare(
                "SELECT vss.veteran_hash, vss.spark_group_id, vss.uma_count, vss.level_sum, \
                        COALESCE(sd.name, ''), COALESCE(sd.spark_type, 0), vss.veteran_level_sum \
                 FROM veteran_spark_summary vss \
                 LEFT JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
                 WHERE vss.veteran_hash = ? AND sd.spark_type != ? \
                 GROUP BY vss.spark_group_id \
                 ORDER BY \
                    CASE WHEN sd.spark_type IN (4, 5) THEN 4 ELSE sd.spark_type END, \
                    vss.veteran_level_sum DESC, \
                    vss.level_sum DESC",
            )
            .map_err(|e| format!("sparks prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash, SparkType::Event.into_raw()], |row| {
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
            .map_err(|e| format!("sparks query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("sparks collect failed: {e}"))?
    };
    Ok(rows)
}

#[tauri::command]
pub fn get_veteran_wins(hash: String) -> Result<Vec<MajorWinRow>, String> {
    let hash: i64 = hash
        .parse()
        .unwrap_or_else(|_| hash.parse::<u64>().map(|u| u as i64).expect("invalid hash"));
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let win_type_filter = if crate::app_config::win_saddle_version() == 2 {
            " AND mwd.win_saddle_type = 3"
        } else {
            ""
        };
        let sql = format!(
            "SELECT mwd.id, mwd.name, mwd.group_id, vwc.win_count, vwc.on_veteran, mwd.priority \
             FROM veteran_win_count vwc \
             JOIN major_wins_data mwd ON mwd.id = vwc.win_id \
             WHERE vwc.veteran_hash = ?{win_type_filter} \
             ORDER BY mwd.priority"
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("wins prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
                Ok(MajorWinRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    group_id: row.get(2)?,
                    shared_count: row.get(3)?,
                    on_veteran: row.get::<_, i64>(4)? != 0,
                    priority: row.get(5)?,
                })
            })
            .map_err(|e| format!("wins query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("wins collect failed: {e}"))?
    };
    Ok(rows)
}

#[tauri::command]
pub fn get_veteran_parents(hash: String) -> Result<Vec<ParentRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let mut rows = {
        let parent_wins_sub = if crate::app_config::win_saddle_version() == 2 {
            "(SELECT COUNT(*) FROM parent_has_win phw \
              JOIN major_wins_data mwd ON mwd.id = phw.win_id \
              WHERE phw.parent_hash = p.hash AND mwd.win_saddle_type = 3) AS major_wins_count"
        } else {
            "(SELECT COUNT(*) FROM parent_has_win phw WHERE phw.parent_hash = p.hash) AS major_wins_count"
        };
        let sql = format!(
            "SELECT p.hash, p.trainee_id, p.rank, p.rarity, p.talent_level, \
                    COALESCE(td.name, '') AS trainee_name, \
                    {parent_wins_sub}, \
                    (SELECT COUNT(DISTINCT phs.spark_id / 100) FROM parent_has_spark phs \
                     JOIN spark_data sd ON sd.group_id = phs.spark_id / 100 \
                     WHERE phs.parent_hash = p.hash AND sd.spark_type IN (1,2)) AS spark_count, \
                    p.owner_id, p.owned \
             FROM veterans v \
             JOIN parents p ON p.hash IN (v.parent_a, v.parent_b) \
             LEFT JOIN trainee_data td ON td.id = p.trainee_id \
             WHERE v.hash = ?"
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("parents prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
                Ok(ParentRow {
                    hash: row.get(0)?,
                    trainee_id: row.get(1)?,
                    rank: row.get(2)?,
                    rarity: row.get(3)?,
                    talent_level: row.get(4)?,
                    trainee_name: row.get::<_, Option<String>>(5)?,
                    major_wins_count: row.get(6)?,
                    spark_count: row.get(7)?,
                    blue_sparks: Vec::new(),
                    owner_id: row.get(8)?,
                    owned: row.get(9)?,
                })
            })
            .map_err(|e| format!("parents query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("parents collect failed: {e}"))?
    };

    if !rows.is_empty() {
        let placeholders: Vec<String> = rows.iter().map(|_| "?".to_string()).collect();
        let spark_sql = format!(
            "SELECT phs.parent_hash, \
                    phs.spark_id / 100 AS spark_group_id, \
                    1 AS uma_count, \
                    MAX(phs.spark_id % 100) AS level_sum, \
                    COALESCE(sd.name, ''), \
                    COALESCE(sd.spark_type, 0), \
                    0 AS veteran_level_sum \
             FROM parent_has_spark phs \
             LEFT JOIN spark_data sd ON sd.group_id = phs.spark_id / 100 \
             WHERE phs.parent_hash IN ({}) \
               AND sd.spark_type = 1 \
             GROUP BY phs.parent_hash, phs.spark_id / 100",
            placeholders.join(",")
        );
        let mut spark_stmt = conn
            .prepare(&spark_sql)
            .map_err(|e| format!("parent spark batch prepare failed: {e}"))?;
        let hash_params: Vec<&dyn rusqlite::types::ToSql> = rows
            .iter()
            .map(|p| &p.hash as &dyn rusqlite::types::ToSql)
            .collect();
        let spark_rows = spark_stmt
            .query_map(hash_params.as_slice(), |row| {
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
            .map_err(|e| format!("parent spark batch query failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("parent spark batch collect failed: {e}"))?;

        use std::collections::HashMap;
        let mut spark_map: HashMap<i64, Vec<SparkGroupRow>> = HashMap::new();
        for s in spark_rows {
            spark_map.entry(s.veteran_hash).or_default().push(s);
        }
        for p in &mut rows {
            if let Some(sparks) = spark_map.remove(&p.hash) {
                p.blue_sparks = sparks;
            }
        }
    }

    Ok(rows)
}

#[tauri::command]
pub fn get_parent_sparks(hash: String) -> Result<Vec<SparkGroupRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let mut stmt = conn
            .prepare(
                "SELECT ?1 AS veteran_hash, \
                         phs.spark_id / 100 AS spark_group_id, \
                         1 AS uma_count, \
                         MAX(phs.spark_id % 100) AS level_sum, \
                         COALESCE(sd.name, ''), \
                         COALESCE(sd.spark_type, 0), \
                         0 AS veteran_level_sum \
                  FROM parent_has_spark phs \
                  LEFT JOIN spark_data sd ON sd.group_id = phs.spark_id / 100 \
                  WHERE phs.parent_hash = ?1 \
                  GROUP BY phs.spark_id / 100 \
                  ORDER BY sd.spark_type, MAX(phs.spark_id % 100) DESC",
            )
            .map_err(|e| format!("parent sparks prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
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
            .map_err(|e| format!("parent sparks query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("parent sparks collect failed: {e}"))?
    };
    Ok(rows)
}

#[tauri::command]
pub fn get_parent_wins(hash: String) -> Result<Vec<MajorWinRow>, String> {
    let hash: i64 = hash
        .parse()
        .unwrap_or_else(|_| hash.parse::<u64>().map(|u| u as i64).expect("invalid hash"));
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let win_type_filter = if crate::app_config::win_saddle_version() == 2 {
            " AND mwd.win_saddle_type = 3"
        } else {
            ""
        };
        let sql = format!(
            "SELECT mwd.id, mwd.name, mwd.group_id, 1 AS shared_count, 0 AS on_veteran, mwd.priority \
             FROM parent_has_win phw \
             JOIN major_wins_data mwd ON mwd.id = phw.win_id \
             WHERE phw.parent_hash = ?{win_type_filter} \
             ORDER BY mwd.priority"
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("parent wins prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
                Ok(MajorWinRow {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    group_id: row.get(2)?,
                    shared_count: Some(row.get::<_, i64>(3)?),
                    on_veteran: row.get::<_, i64>(4)? != 0,
                    priority: row.get(5)?,
                })
            })
            .map_err(|e| format!("parent wins query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("parent wins collect failed: {e}"))?
    };
    Ok(rows)
}

#[tauri::command]
pub fn delete_veteran(hash: String) -> Result<(), String> {
    let hash_i64 = u64::from_str_radix(&hash, 16).map_err(|e| format!("invalid hash: {e}"))? as i64;
    let conn = app_db::open_app_database_connection()?;
    conn.execute("DELETE FROM veterans WHERE hash = ?1", params![hash_i64])
        .map_err(|e| format!("delete veteran failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_veteran_skills(hash: String) -> Result<Vec<VeteranSkillRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let mut stmt = conn
            .prepare(
                "SELECT vhs.skill_id, vhs.level, COALESCE(sd.name, ''), sd.skill_category, \
                 sd.icon_id, sd.ability_type, sd.target_type, sd.rarity \
                 FROM veteran_has_skill vhs \
                 JOIN skill_data sd ON sd.id = vhs.skill_id \
                 WHERE vhs.veteran_hash = ? \
                 ORDER BY sd.skill_category, sd.name",
            )
            .map_err(|e| format!("skills prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
                let icon_id: Option<i64> = row.get(4)?;
                let ability_type: Option<i64> = row.get(5)?;
                let target_type: Option<i64> = row.get(6)?;
                let skill_type = SkillType::from(&SkillDataRow {
                    icon_id,
                    ability_type,
                    target_type,
                    ..Default::default()
                });
                Ok(VeteranSkillRow {
                    skill_id: row.get(0)?,
                    level: row.get(1)?,
                    name: row.get(2)?,
                    category: row.get(3)?,
                    skill_type: skill_type.label().to_string(),
                    rarity: row.get(7)?,
                })
            })
            .map_err(|e| format!("skills query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("skills collect failed: {e}"))?
    };
    let rows: Vec<_> = rows.into_iter().filter(|r| r.skill_id >= 10000).collect();
    Ok(rows)
}

#[tauri::command]
pub fn get_veteran_support_cards(hash: String) -> Result<Vec<VeteranSupportCardRow>, String> {
    let hash: i64 = hash.parse().map_err(|e| format!("invalid hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let rows = {
        let mut stmt = conn
            .prepare(
                "SELECT vsc.position, vsc.support_card_id, \
                    COALESCE(scd.name, ''), COALESCE(scd.rarity, 0), \
                    COALESCE(scd.card_type, 0), vsc.exp, vsc.limit_break_count \
                 FROM veteran_support_card vsc \
                 LEFT JOIN support_card_data scd ON scd.id = vsc.support_card_id \
                 WHERE vsc.veteran_hash = ? \
                 ORDER BY vsc.position",
            )
            .map_err(|e| format!("support cards prepare failed: {e}"))?;
        let mapped = stmt
            .query_map(params![hash], |row| {
                Ok(VeteranSupportCardRow {
                    position: row.get(0)?,
                    support_card_id: row.get(1)?,
                    name: row.get(2)?,
                    rarity: row.get(3)?,
                    card_type: row.get(4)?,
                    exp: row.get(5)?,
                    limit_break_count: row.get(6)?,
                })
            })
            .map_err(|e| format!("support cards query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("support cards collect failed: {e}"))?
    };
    Ok(rows)
}

#[tauri::command]
pub fn get_skill_detail(skill_id: i64) -> Result<SkillDataRow, String> {
    use crate::storage::skills::SkillStorage;
    let conn = app_db::open_app_database_connection()?;
    let storage = SkillStorage::new(conn);
    storage
        .get_by_id(skill_id)
        .ok_or_else(|| format!("skill {skill_id} not found"))
}
