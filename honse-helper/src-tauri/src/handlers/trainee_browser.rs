use crate::db::app_db;
use crate::handlers::legacy_planner::LegacyPlannerStateHandle;
use crate::storage::affinity::AffinityStorage;
use rusqlite::Connection;
use serde_json;
use shared::{
    honse_db::{SkillDataRow, SkillType},
    legacy_planner::LegacyPlannerSlot,
    models::PaginationResponse,
    trainee_browser::{
        TraineeBrowserQuery, TraineeDetail, TraineeEventBranch, TraineeEventChoiceDetail,
        TraineeEventDetail, TraineeEventRewardDetail, TraineeFilterOptions, TraineePageItem,
        TraineeSkillDetail, TraineeSortConfig, BROWSER_TYPE,
    },
};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

pub struct TraineeBrowserConfig {
    pub modes: Mutex<HashMap<String, String>>,
}

const LABEL: &str = "trainee-browser";

#[tauri::command]
pub async fn open_trainee_browser(
    app: AppHandle,
    config: State<'_, TraineeBrowserConfig>,
    mode: Option<String>,
) -> Result<(), String> {
    {
        let mut modes = config.modes.lock().map_err(|e| e.to_string())?;
        if let Some(m) = &mode {
            modes.insert(LABEL.to_string(), m.clone());
        } else {
            modes.remove(LABEL);
        }
    }

    if let Some(win) = app.get_webview_window(LABEL) {
        let _ = win.set_focus();
        let _ = win.eval("window.location.reload()");
        return Ok(());
    }

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(LABEL);
    WebviewWindowBuilder::new(&app, LABEL, WebviewUrl::App("index.html".into()))
        .title("Trainee Browser")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_trainee_browser_mode(
    config: State<'_, TraineeBrowserConfig>,
    window_label: String,
) -> Result<Option<String>, String> {
    Ok(config
        .modes
        .lock()
        .map_err(|e| e.to_string())?
        .get(&window_label)
        .cloned())
}

const BASE_COLS: &str = "\
    td.id, \
    COALESCE(td.name, '') AS name, \
    td.character_id, \
    COALESCE(cd.name, '') AS character_name, \
    COALESCE(tor.rarity, 0) AS owned_rarity, \
    COALESCE(b.base_rarity, 3) AS base_rarity, \
    COALESCE(po.shard_count, 0) AS piece_count";

const FROM_CLAUSE_BASE: &str = "\
    FROM trainee_data td \
    JOIN character_data cd ON cd.id = td.character_id \
    LEFT JOIN trainee_owned tor ON tor.trainee_id = td.id \
    LEFT JOIN ( \
        SELECT trainee_id, MIN(rarity) AS base_rarity \
        FROM trainee_stats_data \
        GROUP BY trainee_id \
    ) b ON b.trainee_id = td.id \
    LEFT JOIN piece_owned po ON po.trainee_id = td.id";

const STATS_JOIN: &str = "\
    LEFT JOIN trainee_stats_data tsdf \
        ON tsdf.trainee_id = td.id \
        AND tsdf.rarity = COALESCE(tor.rarity, b.base_rarity)";

fn make_page_item(row: &rusqlite::Row) -> rusqlite::Result<TraineePageItem> {
    let owned_rarity: i64 = row.get(4)?;
    let base_rarity: i64 = row.get(5)?;
    let piece_needed = shared::trainee_browser::piece_needed(owned_rarity, base_rarity);
    Ok(TraineePageItem {
        id: row.get(0)?,
        name: row.get(1)?,
        character_id: row.get(2)?,
        character_name: row.get(3)?,
        owned_rarity,
        base_rarity,
        piece_count: row.get(6)?,
        piece_needed,
        affinity: None,
    })
}

fn build_order_clause(sort: &TraineeSortConfig) -> String {
    let dir = match sort.direction.as_str() {
        "Asc" => "ASC",
        _ => "DESC",
    };
    let col = match sort.key.as_str() {
        "id" => "td.id",
        "name" => "td.name",
        "owned" => "owned_rarity",
        "piece_count" => "piece_count",
        "Affinity" => "td.name",
        _ => "td.name",
    };
    format!("{} {}", col, dir)
}

#[tauri::command]
pub fn query_trainee_cards(
    query: TraineeBrowserQuery,
    planner_handle: State<'_, LegacyPlannerStateHandle>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<PaginationResponse<TraineePageItem>, String> {
    let conn = app_db::open_app_database_connection()?;

    let (where_clause, where_params, needs_stats) =
        crate::storage::trainee_browser::build_filter_where(&query.filters);
    let order_clause = build_order_clause(&query.sort);

    let from_clause = if needs_stats {
        format!("{} {}", FROM_CLAUSE_BASE, STATS_JOIN)
    } else {
        FROM_CLAUSE_BASE.to_string()
    };

    // Check if we need in-memory affinity sort
    let affinity_sort = query.sort.key == "Affinity" && query.planner_context;

    if affinity_sort {
        // Load ALL matching items, compute affinity, sort, then paginate
        let count_sql = format!("SELECT COUNT(*) {} WHERE {}", from_clause, where_clause);
        let total: u32 = {
            let mut stmt = conn
                .prepare(&count_sql)
                .map_err(|e| format!("count prepare failed: {e}"))?;
            let param_refs: Vec<&dyn rusqlite::types::ToSql> =
                where_params.iter().map(|p| p.as_ref()).collect();
            stmt.query_row(param_refs.as_slice(), |row| row.get::<_, u32>(0))
                .map_err(|e| format!("count query failed: {e}"))?
        };

        let data_sql = format!(
            "SELECT {} {} WHERE {} ORDER BY {}",
            BASE_COLS, from_clause, where_clause, order_clause
        );
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            where_params.iter().map(|p| p.as_ref()).collect();
        let mut stmt = conn
            .prepare(&data_sql)
            .map_err(|e| format!("data prepare failed: {e}"))?;
        let rows = stmt
            .query_map(param_refs.as_slice(), make_page_item)
            .map_err(|e| format!("data query failed: {e}"))?;
        let mut all_items: Vec<TraineePageItem> = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("data collect failed: {e}"))?;

        // Compute affinity for all items
        compute_trainee_affinities(&mut all_items, &planner_handle, &affinity_store, query.legacy_planner_slot)?;

        // Sort by affinity
        match query.sort.direction.as_str() {
            "Asc" => all_items.sort_by_key(|item| item.affinity.map(|a| a.total()).unwrap_or(0)),
            _ => all_items.sort_by(|a, b| {
                b.affinity.map(|a| a.total()).unwrap_or(0).cmp(&a.affinity.map(|a| a.total()).unwrap_or(0))
            }),
        }

        // Paginate in memory
        let offset = (query.page.saturating_sub(1) * query.page_size) as usize;
        let page_items: Vec<TraineePageItem> = all_items
            .into_iter()
            .skip(offset)
            .take(query.page_size as usize)
            .collect();

        Ok(PaginationResponse {
            results: page_items,
            total,
            page: query.page,
            page_size: query.page_size,
        })
    } else {
        // Normal SQL pagination
        let count_sql = format!("SELECT COUNT(*) {} WHERE {}", from_clause, where_clause);
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
        let mut all_params: Vec<Box<dyn rusqlite::types::ToSql>> = where_params;
        all_params.push(Box::new(query.page_size));
        all_params.push(Box::new(offset));

        let data_sql = format!(
            "SELECT {} {} WHERE {} ORDER BY {} LIMIT ? OFFSET ?",
            BASE_COLS, from_clause, where_clause, order_clause
        );

        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            all_params.iter().map(|p| p.as_ref()).collect();

        let mut stmt = conn
            .prepare(&data_sql)
            .map_err(|e| format!("data prepare failed: {e}"))?;
        let rows = stmt
            .query_map(param_refs.as_slice(), make_page_item)
            .map_err(|e| format!("data query failed: {e}"))?;
        let mut results: Vec<TraineePageItem> = rows
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("data collect failed: {e}"))?;

        // Compute affinity for current page if planner context is provided
        if query.planner_context {
            compute_trainee_affinities(&mut results, &planner_handle, &affinity_store, query.legacy_planner_slot)?;
        }

        Ok(PaginationResponse {
            results,
            total,
            page: query.page,
            page_size: query.page_size,
        })
    }
}

/// Compute affinity for a set of trainee page items against the current legacy planner state.
/// Each trainee's character_id is used to compute the total affinity the planner would have
/// if this trainee were placed in the specified slot.
fn compute_trainee_affinities(
    items: &mut [TraineePageItem],
    planner_handle: &State<'_, LegacyPlannerStateHandle>,
    affinity_store: &State<'_, Mutex<AffinityStorage>>,
    slot: Option<LegacyPlannerSlot>,
) -> Result<(), String> {
    let guard = planner_handle
        .state
        .lock()
        .map_err(|e| e.to_string())?;
    let state = match guard.as_ref() {
        Some(s) => s.clone(),
        None => {
            return Ok(());
        }
    };
    drop(guard);

    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;

    for item in items.iter_mut() {
        item.affinity = Some(crate::storage::affinity::compute_chosen_slot_affinity(
            item.character_id,
            &state,
            &affinity,
            slot,
        ));
    }

    Ok(())
}

#[tauri::command]
pub fn get_trainee_filter_options() -> Result<TraineeFilterOptions, String> {
    let conn = app_db::open_app_database_connection()?;

    let mut c_stmt = conn
        .prepare("SELECT id, COALESCE(name, '') FROM character_data ORDER BY name")
        .map_err(|e| format!("character query prepare: {e}"))?;
    let characters = c_stmt
        .query_map([], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })
        .map_err(|e| format!("character query: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("character collect: {e}"))?;

    let skills = {
        let mut stmt = conn
            .prepare("SELECT sd.group_id, sd.id, sd.name, sd.rarity FROM skill_data sd WHERE sd.group_id IS NOT NULL AND (EXISTS (SELECT 1 FROM trainee_skill WHERE skill_id = sd.id) OR EXISTS (SELECT 1 FROM support_event_reward WHERE skill_id = sd.id AND is_trainee_event = 1)) ORDER BY sd.group_id")
            .map_err(|e| format!("skills prepare: {e}"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })
            .map_err(|e| format!("skills query: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("skills collect: {e}"))?;

        let mut groups: std::collections::HashMap<i64, Vec<(i64, String, i64)>> =
            std::collections::HashMap::new();
        for (gid, sid, name, rarity) in rows {
            groups.entry(gid).or_default().push((sid, name, rarity));
        }

        let mut result: Vec<(i64, String)> = Vec::new();
        for (gid, members) in &groups {
            let min_rarity = members.iter().map(|(_, _, r)| *r).min().unwrap_or(0);
            let group_name = members
                .iter()
                .filter(|(_, _, r)| *r == min_rarity)
                .min_by_key(|(id, _, _)| *id)
                .map(|(_, n, _)| n.clone())
                .unwrap_or_default();
            result.push((*gid, group_name));
            if members.len() > 1 {
                for (sid, name, rarity) in members {
                    if *rarity > min_rarity {
                        result.push((-sid, name.clone()));
                    }
                }
            }
        }
        result
    };

    Ok(TraineeFilterOptions {
        characters,
        skills,
    })
}

#[tauri::command]
pub fn get_trainee_detail(trainee_id: i64, rarity: i64) -> Result<TraineeDetail, String> {
    let conn = app_db::open_app_database_connection()?;

    let (name, character_name, growth_spe, growth_sta, growth_str, growth_gut, growth_wit) = conn
        .query_row(
            "SELECT COALESCE(td.name, ''), COALESCE(cd.name, ''),
                    COALESCE(td.growth_rate_spe, 0), COALESCE(td.growth_rate_sta, 0),
                    COALESCE(td.growth_rate_str, 0), COALESCE(td.growth_rate_gut, 0),
                    COALESCE(td.growth_rate_wit, 0)
             FROM trainee_data td
             JOIN character_data cd ON cd.id = td.character_id
             WHERE td.id = ?1",
            [trainee_id],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                ))
            },
        )
        .map_err(|e| format!("trainee query failed: {e}"))?;

    let stats = conn
        .query_row(
            "SELECT spe, sta, pwr, gut, wit,
                    aptitude_dist_sprint, aptitude_dist_mile,
                    aptitude_dist_medium, aptitude_dist_long,
                    aptitude_ground_turf, aptitude_ground_dirt,
                    aptitude_style_front, aptitude_style_pace_chaser,
                    aptitude_style_late_surger, aptitude_style_end_closer,
                    unique_skill_id, unique_skill_level
             FROM trainee_stats_data
             WHERE trainee_id = ?1 AND rarity = ?2",
            rusqlite::params![trainee_id, rarity],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                    row.get::<_, i64>(3)?,
                    row.get::<_, i64>(4)?,
                    row.get::<_, i64>(5)?,
                    row.get::<_, i64>(6)?,
                    row.get::<_, i64>(7)?,
                    row.get::<_, i64>(8)?,
                    row.get::<_, i64>(9)?,
                    row.get::<_, i64>(10)?,
                    row.get::<_, i64>(11)?,
                    row.get::<_, i64>(12)?,
                    row.get::<_, i64>(13)?,
                    row.get::<_, i64>(14)?,
                    row.get::<_, Option<i64>>(15)?,
                    row.get::<_, Option<i64>>(16)?,
                ))
            },
        )
        .map_err(|e| format!("stats query failed for rarity {}: {e}", rarity))?;

    let events = load_trainee_events(&conn, trainee_id)?;
    let (talent_level, skills) =
        load_trainee_skills(&conn, trainee_id, stats.15, stats.16)?;

    Ok(TraineeDetail {
        id: trainee_id,
        name,
        character_name,
        rarity,
        talent_level,
        growth_spe,
        growth_sta,
        growth_str,
        growth_gut,
        growth_wit,
        stat_spe: stats.0,
        stat_sta: stats.1,
        stat_pwr: stats.2,
        stat_gut: stats.3,
        stat_wit: stats.4,
        aptitude_sprint: stats.5,
        aptitude_mile: stats.6,
        aptitude_medium: stats.7,
        aptitude_long: stats.8,
        aptitude_turf: stats.9,
        aptitude_dirt: stats.10,
        aptitude_front: stats.11,
        aptitude_pace_chaser: stats.12,
        aptitude_late_surger: stats.13,
        aptitude_end_closer: stats.14,
        events,
        skills,
    })
}

fn json_val_as_str(v: &serde_json::Value) -> Option<String> {
    match v {
        serde_json::Value::String(s) => Some(s.clone()),
        serde_json::Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn race_name(conn: &Connection, raw: &str) -> String {
    let id = raw.split('|').next().unwrap_or(raw);
    if let Ok(parsed) = id.parse::<i64>() {
        if let Ok(name) = conn.query_row(
            "SELECT race_name FROM race_data WHERE race_instance_id = ?1 LIMIT 1",
            [parsed],
            |r| r.get::<_, String>(0),
        ) {
            return name;
        }
    }
    raw.to_string()
}

fn char_name(conn: &Connection, id: &str) -> String {
    if let Ok(parsed) = id.parse::<i64>() {
        if let Ok(name) = conn.query_row(
            "SELECT name FROM character_data WHERE id = ?1 LIMIT 1",
            [parsed],
            |r| r.get::<_, String>(0),
        ) {
            return name;
        }
    }
    id.to_string()
}

fn race_names_vec(conn: &Connection, vals: &[serde_json::Value]) -> Vec<String> {
    vals.iter()
        .filter_map(|v| json_val_as_str(v))
        .map(|s| race_name(conn, &s))
        .collect()
}

fn race_names_json_arr(conn: &Connection, v: &serde_json::Value) -> Vec<String> {
    match v {
        serde_json::Value::Array(arr) => arr
            .iter()
            .filter_map(|x| json_val_as_str(x))
            .map(|s| race_name(conn, &s))
            .collect(),
        _ => vec![],
    }
}

fn resolve_conditions(conn: &Connection, conditions: &Option<String>) -> Option<String> {
    let json_str = conditions.as_ref()?;
    let parsed: Vec<Vec<serde_json::Value>> = serde_json::from_str(json_str).ok()?;
    if parsed.is_empty() {
        return None;
    }
    let parts: Vec<String> = parsed
        .iter()
        .filter_map(|group| {
            let key = group.first()?.as_str()?;
            let label: String = match key {
                "win" => {
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        "Win a race".into()
                    } else {
                        format!("Win: {}", names.join(", "))
                    }
                }
                "win_or" => {
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        "Win one of specified races".into()
                    } else {
                        format!(
                            "Win at {} or {}",
                            &names[..names.len() - 1].join(", "),
                            names.last().unwrap()
                        )
                    }
                }
                "win_all" => {
                    if group.len() > 1 {
                        let names = race_names_json_arr(conn, &group[1]);
                        if names.is_empty() {
                            "Win all target races".into()
                        } else {
                            format!("Win all: {}", names.join(", "))
                        }
                    } else {
                        "Win all races".into()
                    }
                }
                "win_n_of" => {
                    let count = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    if group.len() > 2 {
                        let names = race_names_json_arr(conn, &group[2]);
                        if !names.is_empty() {
                            format!("Win {} of: {}", count, names.join(", "))
                        } else {
                            format!("Win {} of target races", count)
                        }
                    } else {
                        format!("Win {} of target races", count)
                    }
                }
                "win_g1" => {
                    let p1 = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    if p1.parse::<i64>().is_ok() {
                        format!("Win G1 ×{}", p1)
                    } else {
                        let names = race_names_json_arr(conn, &group[1]);
                        if names.is_empty() {
                            "Win G1".into()
                        } else {
                            format!("Win G1 ×{}", names.join(", "))
                        }
                    }
                }
                "win_g1_cnt_class_distance" => {
                    let cnt = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    let cls = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    format!("Win G1 ×{} at class {}", cnt, cls)
                }
                "win_g1_length" => format!(
                    "Win G1 at length {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "win_g1_strat" => format!(
                    "Win G1 with strategy {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "win_g1_track" => {
                    let track = group.get(1).and_then(|v| json_val_as_str(v));
                    match track {
                        Some(t) => format!("Win G1 at {}", race_name(conn, &t)),
                        None => "Win G1 at specific track".into(),
                    }
                }
                "win_g1_year" => format!(
                    "Win G1 ×{} this year",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "race_w2" => {
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        "Win 2 different races".into()
                    } else {
                        names.join(", ")
                    }
                }
                "rn_race_w" => {
                    let names = if group.len() > 1 {
                        race_names_json_arr(conn, &group[1])
                    } else {
                        vec![]
                    };
                    let cnt = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    if !names.is_empty() {
                        format!(
                            "Win {} of: {} (total {})",
                            names.len(),
                            names.join(", "),
                            cnt
                        )
                    } else {
                        format!("Win {} races", cnt)
                    }
                }
                "rt_race_w" => {
                    let track = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    let cnt = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default();
                    format!("Win {} track {} times", race_name(conn, &track), cnt)
                }
                "win_on_streak" => {
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        "Win on streak".into()
                    } else {
                        format!("Win on streak at {}", names.join(", "))
                    }
                }
                "win_streak_graded" => format!(
                    "Win {} graded in a row",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "beat_rival" => {
                    let names = race_names_vec(conn, &group[1..]);
                    let rival = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .map(|s| char_name(conn, &s))
                        .unwrap_or_default();
                    match names.first() {
                        Some(race) if !rival.is_empty() => format!("Beat {} at {}", rival, race),
                        Some(race) => format!("Beat rival at {}", race),
                        None if !rival.is_empty() => format!("Beat {}", rival),
                        None => "Beat rival".into(),
                    }
                }
                "lose_to_rival" => {
                    let race = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .map(|s| race_name(conn, &s))
                        .unwrap_or_default();
                    let rival = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .map(|s| char_name(conn, &s))
                        .unwrap_or_default();
                    if !race.is_empty() && !rival.is_empty() {
                        format!("Lose to {} at {}", rival, race)
                    } else if !race.is_empty() {
                        format!("Lose to rival at {}", race)
                    } else if !rival.is_empty() {
                        format!("Lose to {}", rival)
                    } else {
                        "Lose to rival".into()
                    }
                }
                "rival_draw" => {
                    let rival = group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .map(|s| char_name(conn, &s))
                        .unwrap_or_default();
                    let race = group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .map(|s| race_name(conn, &s))
                        .unwrap_or_default();
                    if !race.is_empty() && !rival.is_empty() {
                        format!("Draw with {} at {}", rival, race)
                    } else if !rival.is_empty() {
                        format!("Draw with {}", rival)
                    } else {
                        "Rival draw".into()
                    }
                }
                "participate" | "do_not_participate" => {
                    let prefix = if key == "participate" { "" } else { "Don't " };
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        format!("{}participate in a race", prefix)
                    } else {
                        format!("{}participate in {}", prefix, names.join(", "))
                    }
                }
                "win_as_strat" => format!(
                    "Win as strategy {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "pick_and_win" | "dont_pick_and_win" => {
                    let prefix = if key == "pick_and_win" { "" } else { "Don't " };
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        format!("{}pick and win", prefix)
                    } else {
                        format!("{}pick and win at {}", prefix, names.join(", "))
                    }
                }
                "lose" => {
                    let names = race_names_vec(conn, &group[1..]);
                    if names.is_empty() {
                        "Lose a race".into()
                    } else {
                        format!("Lose at {}", names.join(", "))
                    }
                }
                "fan" => format!(
                    "Fans ≥ {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "fans_before_finals" => format!(
                    "Fans ≥ {} before finals",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "date" => format!(
                    "Month {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "obj" => format!(
                    "Objective {} completed",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "ev" => format!(
                    "Event {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "ct" => format!(
                    "Condition ID {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "dist_wins_branch" => format!(
                    "Win at {} distances",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "racetrack_wins_branch" => format!(
                    "Win at {} tracks",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "race_pn" => format!(
                    "Position ≤ {} at {}",
                    group
                        .get(2)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default(),
                    race_name(
                        conn,
                        &group
                            .get(1)
                            .and_then(|v| json_val_as_str(v))
                            .unwrap_or_default()
                    )
                ),
                "do_not_race" => format!(
                    "Don't race {} times",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "triple_crown" => "Triple Crown".into(),
                "spring_triple_crown" => "Spring Triple Crown".into(),
                "triple_tiara" => "Triple Tiara".into(),
                "3_crown_route" => "Triple Crown Route".into(),
                "autumn_triple_crown_same_year" => "Autumn Triple Crown (same year)".into(),
                "autumn_triple_crown_senior" => "Autumn Triple Crown (senior)".into(),
                "won_g1_before" => "Won G1 before".into(),
                "never_won_g1_before" => "Never won G1 before".into(),
                "gold_city_race" => "Gold City Race".into(),
                "brian_five" => "Brian Five".into(),
                "win_connect_live" => "Win connect live".into(),
                "use_strategy" => format!(
                    "Use strategy {}",
                    group
                        .get(1)
                        .and_then(|v| json_val_as_str(v))
                        .unwrap_or_default()
                ),
                "third_any_non_objective" => "3rd+ in non-objective".into(),
                "y_dt_gn_race_no_w" => {
                    let params: Vec<String> = group[1..]
                        .iter()
                        .filter_map(|v| json_val_as_str(v))
                        .collect();
                    if params.is_empty() {
                        "YDtGn race no win".into()
                    } else {
                        format!("YDtGn race no win: {}", params.join(", "))
                    }
                }
                other => {
                    let params: Vec<String> = group[1..]
                        .iter()
                        .filter_map(|v| json_val_as_str(v))
                        .collect();
                    if params.is_empty() {
                        format!("{}", other)
                    } else {
                        format!("{}: {}", other, params.join(", "))
                    }
                }
            };
            Some(label)
        })
        .collect();
    if parts.is_empty() {
        return None;
    }
    if parts.len() == 1 {
        return Some(parts.into_iter().next().unwrap());
    }
    let (last, rest) = parts.split_last().unwrap();
    Some(format!("{}, and {}", rest.join(", "), last))
}

fn load_trainee_events(
    conn: &Connection,
    trainee_id: i64,
) -> Result<Vec<TraineeEventDetail>, String> {
    let mut evt_stmt = conn
        .prepare(
            "SELECT story_id, event_name, category, conditions \
             FROM support_event \
             WHERE trainee_id = ?1 \
                OR character_id = (SELECT character_id FROM trainee_data WHERE id = ?1) \
             ORDER BY story_id",
        )
        .map_err(|e| format!("event query prepare: {e}"))?;

    let evt_rows = evt_stmt
        .query_map([trainee_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, String>(2)?,
                row.get::<_, Option<String>>(3)?,
            ))
        })
        .map_err(|e| format!("event query: {e}"))?;

    let mut events = Vec::new();
    for ev in evt_rows {
        let (story_id, event_name, category, conditions) =
            ev.map_err(|e| format!("event row: {e}"))?;

        let mut ch_stmt = conn
            .prepare(
                "SELECT id, choice_index FROM support_event_choice WHERE story_id = ?1 ORDER BY choice_index",
            )
            .map_err(|e| format!("choice prepare: {e}"))?;
        let ch_rows = ch_stmt
            .query_map([story_id], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
            })
            .map_err(|e| format!("choice query: {e}"))?;

        let mut choices = Vec::new();
        for cr in ch_rows {
            let (choice_id, choice_index) = cr.map_err(|e| format!("choice row: {e}"))?;

            let mut br_stmt = conn
                .prepare("SELECT id, probability FROM support_event_branch WHERE choice_id = ?1 ORDER BY branch_index")
                .map_err(|e| format!("branch prepare: {e}"))?;
            let br_rows = br_stmt
                .query_map([choice_id], |row| {
                    Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?))
                })
                .map_err(|e| format!("branch query: {e}"))?;

            let mut branches = Vec::new();
            for br in br_rows {
                let (branch_id, probability) = br.map_err(|e| format!("branch row: {e}"))?;

                let mut rw_stmt = conn
                    .prepare(
                        "SELECT reward_type, size, skill_id, negative, alternatives, effect_id FROM support_event_reward WHERE branch_id = ?1",
                    )
                    .map_err(|e| format!("reward prepare: {e}"))?;
                let rw_rows = rw_stmt
                    .query_map([branch_id], |row| {
                        let reward_type: i64 = row.get(0)?;
                        let skill_id: Option<i64> = row.get(2)?;
                        let skill_name = skill_id.and_then(|sid| {
                            conn.query_row(
                                "SELECT name FROM skill_data WHERE id = ?1",
                                [sid],
                                |r| r.get(0),
                            )
                            .ok()
                        });
                        let size: Option<i64> = row.get(1)?;
                        let alternatives_raw: Option<String> = row.get(4)?;
                        let alternatives = alternatives_raw
                            .and_then(|s| serde_json::from_str::<Vec<i64>>(&s).ok());
                        let effect_id: Option<i64> = row.get(5)?;
                        let mut negative: bool = row.get(3)?;
                        let effect_label: Option<String> = effect_id
                            .and_then(|id| shared::models::ScenarioStatus::from_id(id))
                            .map(|s| {
                                if s.negative() {
                                    negative = true;
                                }
                                s.label().to_string()
                            });
                        let reward_label = effect_label.clone().unwrap_or_else(|| {
                            shared::models::RewardType::from_raw(reward_type)
                                .label()
                                .to_string()
                        });
                        Ok(TraineeEventRewardDetail {
                            reward_type,
                            reward_label,
                            size,
                            skill_id,
                            skill_name,
                            negative,
                            alternatives,
                            effect_label,
                        })
                    })
                    .map_err(|e| format!("reward query: {e}"))?;

                let rewards: Vec<_> = rw_rows
                    .collect::<Result<_, _>>()
                    .map_err(|e| format!("rewards collect: {e}"))?;
                branches.push(TraineeEventBranch {
                    probability,
                    rewards,
                });
            }

            choices.push(TraineeEventChoiceDetail {
                choice_index,
                branches,
            });
        }

        let conditions_display = resolve_conditions(conn, &conditions);

        events.push(TraineeEventDetail {
            story_id,
            event_name,
            category,
            choices,
            conditions,
            conditions_display,
        });
    }

    Ok(events)
}

fn load_trainee_skills(
    conn: &Connection,
    trainee_id: i64,
    unique_skill_id: Option<i64>,
    unique_skill_level: Option<i64>,
) -> Result<(i64, Vec<TraineeSkillDetail>), String> {
    let talent_level: i64 = conn
        .query_row(
            "SELECT COALESCE(talent_level, 0) FROM trainee_owned WHERE trainee_id = ?1",
            [trainee_id],
            |row| row.get(0),
        )
        .unwrap_or(0);

    let mut skill_map: HashMap<i64, TraineeSkillDetail> = HashMap::new();

    {
        let mut stmt = conn
            .prepare(
                "SELECT ts.skill_id, sd.name, sd.icon_id, sd.ability_type, sd.target_type, \
                        COALESCE(sd.rarity, 1), ts.need_rank \
                 FROM trainee_skill ts \
                 JOIN skill_data sd ON sd.id = ts.skill_id \
                 WHERE ts.trainee_id = ?1 \
                 ORDER BY ts.need_rank",
            )
            .map_err(|e| format!("skill query prepare: {e}"))?;

        let rows = stmt
            .query_map([trainee_id], |row| {
                let skill_id: i64 = row.get(0)?;
                let name: String = row.get(1)?;
                let icon_id: Option<i64> = row.get(2)?;
                let ability_type: Option<i64> = row.get(3)?;
                let target_type: Option<i64> = row.get(4)?;
                let rarity: Option<i64> = row.get(5)?;
                let need_rank: i64 = row.get(6)?;
                let skill_type = SkillType::from(&SkillDataRow {
                    icon_id,
                    ability_type,
                    target_type,
                    rarity,
                    ..Default::default()
                });
                Ok((skill_id, name, skill_type, rarity.unwrap_or(1), need_rank))
            })
            .map_err(|e| format!("skill query: {e}"))?;

        for r in rows {
            let (skill_id, name, skill_type, rarity, need_rank) =
                r.map_err(|e| format!("skill row: {e}"))?;
            skill_map.insert(
                skill_id,
                TraineeSkillDetail {
                    skill_id,
                    name,
                    skill_type: skill_type.label().to_string(),
                    rarity,
                    level: 1,
                    need_rank,
                    source: if need_rank == 0 {
                        "base".to_string()
                    } else {
                        "potential".to_string()
                    },
                    source_name: String::new(),
                    unlocked: need_rank <= talent_level,
                },
            );
        }
    }

    if let (Some(skill_id), Some(level)) = (unique_skill_id.filter(|id| *id > 0), unique_skill_level) {
        let (sd_name, sd_rarity, skill_type): (String, i64, String) = conn
            .query_row(
                "SELECT name, COALESCE(rarity, 1), icon_id, ability_type, target_type \
                 FROM skill_data WHERE id = ?1",
                [skill_id],
                |row| {
                    let name: String = row.get(0)?;
                    let rarity: Option<i64> = row.get(1)?;
                    let icon_id: Option<i64> = row.get(2)?;
                    let ability_type: Option<i64> = row.get(3)?;
                    let target_type: Option<i64> = row.get(4)?;
                    let st = SkillType::from(&SkillDataRow {
                        icon_id,
                        ability_type,
                        target_type,
                        rarity,
                        ..Default::default()
                    });
                    Ok((name, rarity.unwrap_or(1), st.label().to_string()))
                },
            )
            .unwrap_or(("?".into(), 1, "Unknown".into()));
        skill_map.insert(
            skill_id,
            TraineeSkillDetail {
                skill_id,
                name: sd_name,
                skill_type,
                rarity: sd_rarity,
                level: level.max(1),
                need_rank: -2,
                source: "unique".to_string(),
                source_name: String::new(),
                unlocked: true,
            },
        );
    }

    let mut event_skills: Vec<TraineeSkillDetail> = Vec::new();

    {
        let character_id: Option<i64> = conn
            .query_row(
                "SELECT character_id FROM trainee_data WHERE id = ?1",
                [trainee_id],
                |row| row.get(0),
            )
            .ok();

        let mut evt_stmt = conn
            .prepare(
                "SELECT se.event_name, ser.skill_id, ser.size \
                 FROM support_event se \
                 JOIN support_event_choice sec ON sec.story_id = se.story_id \
                 JOIN support_event_reward ser ON ser.choice_id = sec.id \
                 WHERE (se.trainee_id = ?1 OR se.character_id = ?2) \
                   AND ser.skill_id IS NOT NULL \
                   AND ser.reward_type IN (11, 12)",
            )
            .map_err(|e| format!("event skill prepare: {e}"))?;

        let rows = evt_stmt
            .query_map(
                rusqlite::params![trainee_id, character_id],
                |row| {
                    let event_name: String = row.get(0)?;
                    let skill_id: i64 = row.get(1)?;
                    let size: Option<i64> = row.get(2)?;
                    Ok((event_name, skill_id, size.unwrap_or(1)))
                },
            )
            .map_err(|e| format!("event skill query: {e}"))?;

        for r in rows {
            let (event_name, skill_id, level) = r.map_err(|e| format!("event skill row: {e}"))?;
            let (sd_name, sd_rarity, skill_type): (String, i64, String) = conn
                .query_row(
                    "SELECT name, COALESCE(rarity, 1), icon_id, ability_type, target_type \
                     FROM skill_data WHERE id = ?1",
                    [skill_id],
                    |row| {
                        let name: String = row.get(0)?;
                        let rarity: Option<i64> = row.get(1)?;
                        let icon_id: Option<i64> = row.get(2)?;
                        let ability_type: Option<i64> = row.get(3)?;
                        let target_type: Option<i64> = row.get(4)?;
                        let st = SkillType::from(&SkillDataRow {
                            icon_id,
                            ability_type,
                            target_type,
                            rarity,
                            ..Default::default()
                        });
                        Ok((name, rarity.unwrap_or(1), st.label().to_string()))
                    },
                )
                .unwrap_or(("?".into(), 1, "Unknown".into()));

            let existing = event_skills.iter_mut().find(|es| es.skill_id == skill_id);
            if let Some(es) = existing {
                if level > es.level {
                    es.level = level;
                }
                if !es.source_name.contains(&event_name) {
                    es.source_name = format!("{}, {}", es.source_name, event_name);
                }
            } else {
                event_skills.push(TraineeSkillDetail {
                    skill_id,
                    name: sd_name,
                    skill_type,
                    rarity: sd_rarity,
                    level,
                    need_rank: -1,
                    source: "event".to_string(),
                    source_name: event_name.clone(),
                    unlocked: true,
                });
            }
        }
    }

    let mut skills: Vec<TraineeSkillDetail> = skill_map.into_values().collect();
    skills.sort_by_key(|s| {
        (
            match s.source.as_str() {
                "unique" => 0,
                "base" => 1,
                "potential" => 2,
                _ => 3,
            },
            s.need_rank,
            s.skill_id,
        )
    });

    skills.append(&mut event_skills);

    Ok((talent_level, skills))
}

// ── Selection return ──────────────────────────────────────────────

#[tauri::command]
pub fn return_trainee_selection(
    app: AppHandle,
    _config: State<'_, TraineeBrowserConfig>,
    trainee_id: i64,
    slot_label: Option<String>,
) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    let (trainee_name, character_id, character_name): (String, i64, String) = conn
        .query_row(
            "SELECT COALESCE(td.name, ''), td.character_id, COALESCE(cd.name, '') \
             FROM trainee_data td \
             JOIN character_data cd ON cd.id = td.character_id \
             WHERE td.id = ?1",
            [trainee_id],
            |row| Ok((row.get(0)?, row.get(1)?, row.get(2)?)),
        )
        .map_err(|e| format!("trainee lookup failed: {e}"))?;

    let mut payload = serde_json::json!({
        "trainee_id": trainee_id,
        "trainee_name": trainee_name,
        "character_id": character_id,
        "character_name": character_name,
    });
    if let Some(label) = &slot_label {
        payload["slot_label"] = serde_json::json!(label);
    }
    let _ = app.emit("trainee-selected", payload);
    if let Some(win) = app.get_webview_window(LABEL) {
        let _ = win.close();
    }
    Ok(())
}

// ── Preset delegation ────────────────────────────────────────────

#[tauri::command]
pub fn save_trainee_preset(name: String, filters: Option<String>, sort: Option<String>) -> Result<(), String> {
    crate::handlers::veteran_browser::save_preset(name, filters, sort, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn load_trainee_preset_active() -> Result<Option<shared::veteran_browser::PresetData>, String> {
    crate::handlers::veteran_browser::load_preset_active(Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn load_trainee_preset(
    name: String,
) -> Result<Option<shared::veteran_browser::PresetData>, String> {
    crate::handlers::veteran_browser::load_preset(name, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn delete_trainee_preset(name: String) -> Result<(), String> {
    crate::handlers::veteran_browser::delete_preset(name, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn list_trainee_presets(preset_type: Option<String>) -> Result<Vec<String>, String> {
    crate::handlers::veteran_browser::list_presets(Some(BROWSER_TYPE.into()), preset_type)
}
