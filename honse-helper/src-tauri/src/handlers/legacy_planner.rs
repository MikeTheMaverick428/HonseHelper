use crate::db::app_db;
use crate::storage::affinity::AffinityStorage;
use crate::storage::sparks::SparkGroupStorage;
use crate::veterans::uma_moe_cache::UmaMoeCache;
use rusqlite::{params, Connection, OptionalExtension};
use shared::legacy_planner::lookup_dtos::{AffinityResult, SlimUma};
use shared::{
    legacy_planner::{
        AffinityPairInfo, InspirationSummaryRow, LegacyPlannerSlot, LegacyPlannerState,
        LegacySlotValue, LegacyUma, ParentUma, PlannerAffinities, PlannerAffinitySummary,
        SelectedTrainee, SparkGroupInfo, SparkSummaryRow, VeteranAffinity,
    },
    models::CharacterOption,
    models::SparkType,
    TraineeStatsDataRow,
};
use std::collections::{BTreeMap, HashMap, HashSet};
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};

pub struct LegacyPlannerStateHandle {
    pub state: Mutex<Option<LegacyPlannerState>>,
}

const PLANNER_STATE_KEY: &str = "planner_state";

fn load_from_db(conn: &Connection) -> Option<LegacyPlannerState> {
    let json_str = conn
        .query_row(
            "SELECT value FROM legacy_planner_state WHERE key = ?1",
            params![PLANNER_STATE_KEY],
            |row| row.get::<_, String>(0),
        )
        .optional()
        .ok()
        .flatten()?;
    serde_json::from_str(&json_str).ok()
}

fn save_to_db(conn: &Connection, state: &LegacyPlannerState) -> Result<(), String> {
    let json_str =
        serde_json::to_string(state).map_err(|e| format!("serialize state failed: {e}"))?;
    conn.execute(
        "INSERT INTO legacy_planner_state (key, value) VALUES (?1, ?2)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value",
        params![PLANNER_STATE_KEY, json_str],
    )
    .map_err(|e| format!("save state failed: {e}"))?;
    Ok(())
}

fn get_or_load_state<'a>(
    handle: &'a LegacyPlannerStateHandle,
    conn: &Connection,
) -> Result<std::sync::MutexGuard<'a, Option<LegacyPlannerState>>, String> {
    let mut guard = handle.state.lock().map_err(|e| e.to_string())?;
    if guard.is_none() {
        *guard = load_from_db(conn);
    }
    Ok(guard)
}

fn persist_and_emit(
    handle: &LegacyPlannerStateHandle,
    app: &AppHandle,
    conn: &Connection,
) -> Result<(), String> {
    let guard = handle.state.lock().map_err(|e| e.to_string())?;
    if let Some(state) = guard.as_ref() {
        save_to_db(conn, state)?;
        let _ = app.emit("legacy-planner-state-changed", state.clone());
    }
    Ok(())
}

fn get_character_id(value: &LegacySlotValue) -> i64 {
    match value {
        LegacySlotValue::LegacyUma(u) => u.character_id,
        LegacySlotValue::ParentUma(u) => u.character_id,
        LegacySlotValue::Character(u) => u.character_id,
    }
}

fn is_borrowed(value: &LegacySlotValue) -> bool {
    match value {
        LegacySlotValue::LegacyUma(u) => u.is_borrowed,
        LegacySlotValue::ParentUma(_) => false,
        LegacySlotValue::Character(_) => false,
    }
}

fn slot_value(state: &LegacyPlannerState, slot: LegacyPlannerSlot) -> Option<&LegacySlotValue> {
    match slot {
        LegacyPlannerSlot::ParentA => state.parent_a.as_ref(),
        LegacyPlannerSlot::ParentB => state.parent_b.as_ref(),
        LegacyPlannerSlot::GrandparentAA => state.grandparent_aa.as_ref(),
        LegacyPlannerSlot::GrandparentAB => state.grandparent_ab.as_ref(),
        LegacyPlannerSlot::GrandparentBA => state.grandparent_ba.as_ref(),
        LegacyPlannerSlot::GrandparentBB => state.grandparent_bb.as_ref(),
    }
}

fn set_slot_value(
    state: &mut LegacyPlannerState,
    slot: LegacyPlannerSlot,
    value: Option<LegacySlotValue>,
) {
    match slot {
        LegacyPlannerSlot::ParentA => state.parent_a = value,
        LegacyPlannerSlot::ParentB => state.parent_b = value,
        LegacyPlannerSlot::GrandparentAA => state.grandparent_aa = value,
        LegacyPlannerSlot::GrandparentAB => state.grandparent_ab = value,
        LegacyPlannerSlot::GrandparentBA => state.grandparent_ba = value,
        LegacyPlannerSlot::GrandparentBB => state.grandparent_bb = value,
    }
}

fn is_grandparent_locked(value: Option<&LegacySlotValue>) -> bool {
    matches!(value, Some(LegacySlotValue::ParentUma(_)))
}

fn can_add_character_to_slot(
    state: &LegacyPlannerState,
    slot: LegacyPlannerSlot,
    character_id: i64,
) -> bool {
    match slot {
        LegacyPlannerSlot::ParentA => {
            state
                .parent_b
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .chosen_character
                    .as_ref()
                    .is_none_or(|c| c.character_id != character_id)
        }
        LegacyPlannerSlot::ParentB => {
            state
                .parent_a
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .chosen_character
                    .as_ref()
                    .is_none_or(|c| c.character_id != character_id)
        }
        LegacyPlannerSlot::GrandparentAA => {
            state
                .grandparent_ab
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .parent_a
                    .as_ref()
                    .is_none_or(|c| get_character_id(c) != character_id)
        }
        LegacyPlannerSlot::GrandparentAB => {
            state
                .grandparent_aa
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .parent_a
                    .as_ref()
                    .is_none_or(|c| get_character_id(c) != character_id)
        }
        LegacyPlannerSlot::GrandparentBA => {
            state
                .grandparent_bb
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .parent_b
                    .as_ref()
                    .is_none_or(|c| get_character_id(c) != character_id)
        }
        LegacyPlannerSlot::GrandparentBB => {
            state
                .grandparent_ba
                .as_ref()
                .is_none_or(|c| get_character_id(c) != character_id)
                && state
                    .parent_b
                    .as_ref()
                    .is_none_or(|c| get_character_id(c) != character_id)
        }
    }
}

fn can_add_borrow_to_slot(state: &LegacyPlannerState, slot: LegacyPlannerSlot) -> bool {
    match slot {
        LegacyPlannerSlot::ParentA => state.parent_b.as_ref().is_none_or(|c| !is_borrowed(c)),
        LegacyPlannerSlot::ParentB => state.parent_a.as_ref().is_none_or(|c| !is_borrowed(c)),
        LegacyPlannerSlot::GrandparentAA => state
            .grandparent_ab
            .as_ref()
            .is_none_or(|c| !is_borrowed(c)),
        LegacyPlannerSlot::GrandparentAB => state
            .grandparent_aa
            .as_ref()
            .is_none_or(|c| !is_borrowed(c)),
        LegacyPlannerSlot::GrandparentBA => state
            .grandparent_bb
            .as_ref()
            .is_none_or(|c| !is_borrowed(c)),
        LegacyPlannerSlot::GrandparentBB => state
            .grandparent_ba
            .as_ref()
            .is_none_or(|c| !is_borrowed(c)),
    }
}

fn can_set_as_trainee(state: &LegacyPlannerState, character_id: i64) -> bool {
    state
        .parent_a
        .as_ref()
        .is_none_or(|c| get_character_id(c) != character_id)
        && state
            .parent_b
            .as_ref()
            .is_none_or(|c| get_character_id(c) != character_id)
}

fn fetch_veteran_uma(conn: &Connection, hash: u64) -> Result<Option<LegacyUma>, String> {
    let row: Option<(String, i64, Option<i64>, Option<i64>, bool)> = conn
        .query_row(
            "SELECT COALESCE(td.name, 'Unknown'), v.trainee_id, v.parent_a, v.parent_b, v.owned
             FROM veterans v
             LEFT JOIN trainee_data td ON td.id = v.trainee_id
             WHERE v.hash = ?1",
            params![hash as i64],
            |row| {
                Ok((
                    row.get::<_, String>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                    row.get::<_, bool>(4)?,
                ))
            },
        )
        .optional()
        .map_err(|e| format!("query veteran failed: {e}"))?;

    let Some((name, trainee_id, parent_a, parent_b, owned)) = row else {
        return Ok(None);
    };

    let character_id: i64 = conn
        .query_row(
            "SELECT character_id FROM trainee_data WHERE id = ?1",
            params![trainee_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("query character_id failed: {e}"))?
        .unwrap_or(0);

    // Fetch spark summary
    let mut spark_stmt = conn
        .prepare(
            "SELECT vss.spark_group_id, COALESCE(sd.name, ''), COALESCE(sd.spark_type, 0), vss.level_sum, vss.uma_count, vss.veteran_level_sum
             FROM veteran_spark_summary vss
             LEFT JOIN spark_data sd ON sd.group_id = vss.spark_group_id
             WHERE vss.veteran_hash = ?1
             GROUP BY vss.spark_group_id",
        )
        .map_err(|e| format!("prepare spark query failed: {e}"))?;
    let spark_groups: Vec<SparkGroupInfo> = spark_stmt
        .query_map(params![hash as i64], |row| {
            Ok(SparkGroupInfo {
                spark_group_id: row.get(0)?,
                name: row.get(1)?,
                spark_type: shared::models::SparkType::from_raw(row.get(2)?),
                total_stars: row.get::<_, i64>(3)? as i8,
                uma_count: row.get::<_, i64>(4)? as i8,
                trainee_stars_veteran: row.get::<_, i64>(5)? as i8,
            })
        })
        .map_err(|e| format!("query sparks failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect sparks failed: {e}"))?;

    // Fetch win IDs (filter to type-3 in V2 mode)
    let win_type_filter = if crate::app_config::win_saddle_version() == 2 {
        " AND mwd.win_saddle_type = 3"
    } else {
        ""
    };
    let win_sql = format!(
        "SELECT vwc.win_id FROM veteran_win_count vwc \
         JOIN major_wins_data mwd ON mwd.id = vwc.win_id \
         WHERE vwc.veteran_hash = ?1{win_type_filter}"
    );
    let mut win_stmt = conn
        .prepare(&win_sql)
        .map_err(|e| format!("prepare win query failed: {e}"))?;
    let major_wins: Vec<i64> = win_stmt
        .query_map(params![hash as i64], |row| row.get(0))
        .map_err(|e| format!("query wins failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect wins failed: {e}"))?;

    Ok(Some(LegacyUma {
        name,
        character_id,
        hash,
        spark_groups,
        major_wins,
        parent1_hash: parent_a.map(|v| v as u64),
        parent2_hash: parent_b.map(|v| v as u64),
        is_borrowed: !owned,
    }))
}

fn fetch_parent_uma(conn: &Connection, hash: u64) -> Result<Option<ParentUma>, String> {
    let row: Option<(String, i64)> = conn
        .query_row(
            "SELECT COALESCE(td.name, 'Unknown'), p.trainee_id
             FROM parents p
             LEFT JOIN trainee_data td ON td.id = p.trainee_id
             WHERE p.hash = ?1",
            params![hash as i64],
            |row| Ok((row.get::<_, String>(0)?, row.get::<_, i64>(1)?)),
        )
        .optional()
        .map_err(|e| format!("query parent failed: {e}"))?;

    let Some((name, trainee_id)) = row else {
        return Ok(None);
    };

    let character_id: i64 = conn
        .query_row(
            "SELECT character_id FROM trainee_data WHERE id = ?1",
            params![trainee_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("query character_id failed: {e}"))?
        .unwrap_or(0);

    let mut spark_stmt = conn
        .prepare(
            "SELECT phs.spark_id / 100 AS spark_group_id,
                    COALESCE(sd.name, ''),
                    COALESCE(sd.spark_type, 0),
                    MAX(phs.spark_id % 100) AS level_sum,
                    1 AS uma_count,
                    0 AS veteran_level_sum
             FROM parent_has_spark phs
             LEFT JOIN spark_data sd ON sd.group_id = phs.spark_id / 100
             WHERE phs.parent_hash = ?1
             GROUP BY phs.spark_id / 100",
        )
        .map_err(|e| format!("prepare parent spark query failed: {e}"))?;

    let spark_groups: Vec<SparkGroupInfo> = spark_stmt
        .query_map(params![hash as i64], |row| {
            Ok(SparkGroupInfo {
                spark_group_id: row.get(0)?,
                name: row.get(1)?,
                spark_type: shared::models::SparkType::from_raw(row.get(2)?),
                total_stars: row.get::<_, i64>(3)? as i8,
                uma_count: row.get::<_, i64>(4)? as i8,
                trainee_stars_veteran: row.get::<_, i64>(5)? as i8,
            })
        })
        .map_err(|e| format!("query parent sparks failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect parent sparks failed: {e}"))?;

    let win_type_filter = if crate::app_config::win_saddle_version() == 2 {
        " AND mwd.win_saddle_type = 3"
    } else {
        ""
    };
    let win_sql = format!(
        "SELECT phw.win_id FROM parent_has_win phw \
         JOIN major_wins_data mwd ON mwd.id = phw.win_id \
         WHERE phw.parent_hash = ?1{win_type_filter}"
    );
    let mut win_stmt = conn
        .prepare(&win_sql)
        .map_err(|e| format!("prepare parent win query failed: {e}"))?;
    let major_wins: Vec<i64> = win_stmt
        .query_map(params![hash as i64], |row| row.get(0))
        .map_err(|e| format!("query parent wins failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect parent wins failed: {e}"))?;

    Ok(Some(ParentUma {
        name,
        character_id,
        hash,
        spark_groups,
        major_wins,
        api_mode: false,
    }))
}

// ── Tauri Commands ─────────────────────────────────────────────────

#[tauri::command]
pub fn get_legacy_planner_state(
    handle: State<'_, LegacyPlannerStateHandle>,
) -> Result<LegacyPlannerState, String> {
    let conn = app_db::open_app_database_connection()?;
    let guard = get_or_load_state(&handle, &conn)?;
    Ok(guard.clone().unwrap_or_default())
}

#[tauri::command]
pub fn set_legacy_planner_chosen(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
    trainee_id: i64,
    character_id: i64,
    trainee_name: String,
    character_name: String,
) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    let mut guard = get_or_load_state(&handle, &conn)?;
    let state = guard.get_or_insert_with(LegacyPlannerState::default);

    if !can_set_as_trainee(state, character_id) {
        return Err("Cannot select this trainee: its character is already used as a parent".into());
    }

    state.chosen_character = Some(SelectedTrainee {
        trainee_id,
        character_id,
        trainee_name,
        character_name,
    });

    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn clear_legacy_planner_chosen(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    let mut guard = get_or_load_state(&handle, &conn)?;
    if let Some(state) = guard.as_mut() {
        state.chosen_character = None;
    }
    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn set_legacy_planner_slot_veteran(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
    slot_label: String,
    hash: String,
) -> Result<(), String> {
    let slot = LegacyPlannerSlot::from_label(&slot_label)
        .ok_or_else(|| format!("invalid slot label: {slot_label}"))?;
    let hash: u64 = u64::from_str_radix(&hash, 16).map_err(|e| format!("invalid hash: {e}"))?;

    let conn = app_db::open_app_database_connection()?;
    let mut guard = get_or_load_state(&handle, &conn)?;
    let state = guard.get_or_insert_with(LegacyPlannerState::default);

    if is_grandparent_locked(slot_value(state, slot)) {
        return Err("Slot is locked (grandparent inherited from parent)".into());
    }

    let uma =
        fetch_veteran_uma(&conn, hash)?.ok_or_else(|| format!("veteran hash {hash} not found"))?;

    if !can_add_character_to_slot(state, slot, uma.character_id) {
        return Err("Cannot add: this character is already used in a related slot".into());
    }
    if uma.is_borrowed && !can_add_borrow_to_slot(state, slot) {
        return Err("Cannot add borrowed: only one borrowed per pair".into());
    }
    // For parent slots, auto-populate grandparents
    if matches!(
        slot,
        LegacyPlannerSlot::ParentA | LegacyPlannerSlot::ParentB
    ) {
        let (gp1_slot, gp2_slot) = match slot {
            LegacyPlannerSlot::ParentA => (
                LegacyPlannerSlot::GrandparentAA,
                LegacyPlannerSlot::GrandparentAB,
            ),
            LegacyPlannerSlot::ParentB => (
                LegacyPlannerSlot::GrandparentBA,
                LegacyPlannerSlot::GrandparentBB,
            ),
            _ => unreachable!(),
        };

        if let Some(gp1_hash) = uma.parent1_hash {
            if let Some(gp1_uma) = fetch_parent_uma(&conn, gp1_hash)? {
                set_slot_value(state, gp1_slot, Some(LegacySlotValue::ParentUma(gp1_uma)));
            }
        }
        if let Some(gp2_hash) = uma.parent2_hash {
            if let Some(gp2_uma) = fetch_parent_uma(&conn, gp2_hash)? {
                set_slot_value(state, gp2_slot, Some(LegacySlotValue::ParentUma(gp2_uma)));
            }
        }
    }

    set_slot_value(state, slot, Some(LegacySlotValue::LegacyUma(uma)));

    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn set_legacy_planner_slot_uma_moe_veteran(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
    cache: State<'_, UmaMoeCache>,
    slot_label: String,
    hash: String,
) -> Result<(), String> {
    let slot = LegacyPlannerSlot::from_label(&slot_label)
        .ok_or_else(|| format!("invalid slot label: {slot_label}"))?;
    let hash_u64: u64 = u64::from_str_radix(&hash, 16).map_err(|e| format!("invalid hash: {e}"))?;

    let mut group = cache
        .get(hash_u64)
        .ok_or_else(|| format!("veteran hash {hash} not found in cache"))?;

    let conn = app_db::open_app_database_connection()?;

    // Filter to type-3 wins in V2 mode (group was cloned from cache)
    if crate::app_config::win_saddle_version() == 2 {
        if let Ok(mut stmt) =
            conn.prepare("SELECT id FROM major_wins_data WHERE win_saddle_type = 3")
        {
            if let Ok(rows) = stmt.query_map([], |row| row.get::<_, i64>(0)) {
                let type3_ids: HashSet<i64> = rows.filter_map(|r| r.ok()).collect();
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
        }
    }

    let mut guard = get_or_load_state(&handle, &conn)?;
    let state = guard.get_or_insert_with(LegacyPlannerState::default);

    if is_grandparent_locked(slot_value(state, slot)) {
        return Err("Slot is locked (grandparent inherited from parent)".into());
    }

    let v = &group.veteran;
    let trainee_id = v.trainee_id;

    let character_id: i64 = conn
        .query_row(
            "SELECT character_id FROM trainee_data WHERE id = ?1",
            params![trainee_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("query character_id failed: {e}"))?
        .unwrap_or(0);

    if !can_add_character_to_slot(state, slot, character_id) {
        return Err("Cannot add: this character is already used in a related slot".into());
    }

    let name: String = conn
        .query_row(
            "SELECT COALESCE(td.name, 'Unknown') FROM trainee_data td WHERE td.id = ?1",
            params![trainee_id],
            |row| row.get(0),
        )
        .optional()
        .map_err(|e| format!("query name failed: {e}"))?
        .unwrap_or_else(|| format!("Trainee {}", trainee_id));

    let mut vet_spark_max: HashMap<i64, u16> = HashMap::new();
    for &sid in &v.container_sparks {
        let gid = sid / 100;
        let lvl = sid % 100;
        let entry = vet_spark_max.entry(gid).or_insert(0);
        *entry = (*entry).max(lvl as u16);
    }

    let spark_ids: Vec<i64> = vet_spark_max.keys().copied().collect();
    let spark_storage = SparkGroupStorage::new(&conn, &spark_ids);

    let mut spark_groups: Vec<SparkGroupInfo> = Vec::new();
    for (&spark_group_id, &max_lvl) in &vet_spark_max {
        spark_groups.push(SparkGroupInfo {
            spark_group_id,
            name: spark_storage.name(spark_group_id).to_string(),
            spark_type: SparkType::from_raw(spark_storage.spark_type(spark_group_id)),
            total_stars: max_lvl as i8,
            trainee_stars_veteran: max_lvl as i8,
            uma_count: 1,
        });
    }

    let major_wins: Vec<i64> = v.container_major_wins.clone();

    let uma = LegacyUma {
        name,
        character_id,
        hash: hash_u64,
        spark_groups,
        major_wins,
        parent1_hash: v.parent_a.map(|h| h.as_u64()),
        parent2_hash: v.parent_b.map(|h| h.as_u64()),
        is_borrowed: true,
    };

    if matches!(
        slot,
        LegacyPlannerSlot::ParentA | LegacyPlannerSlot::ParentB
    ) {
        let (gp1_slot, gp2_slot) = match slot {
            LegacyPlannerSlot::ParentA => (
                LegacyPlannerSlot::GrandparentAA,
                LegacyPlannerSlot::GrandparentAB,
            ),
            LegacyPlannerSlot::ParentB => (
                LegacyPlannerSlot::GrandparentBA,
                LegacyPlannerSlot::GrandparentBB,
            ),
            _ => unreachable!(),
        };

        for (gp_slot, parent_hash) in [(gp1_slot, uma.parent1_hash), (gp2_slot, uma.parent2_hash)] {
            if let Some(p_hash) = parent_hash {
                if let Some(parent) = cache.find_parent(p_hash) {
                    let mut parent_spark_max: HashMap<i64, u16> = HashMap::new();
                    for &sid in &parent.container_sparks {
                        let gid = sid / 100;
                        let lvl = sid % 100;
                        let entry = parent_spark_max.entry(gid).or_insert(0);
                        *entry = (*entry).max(lvl as u16);
                    }

                    let p_spark_ids: Vec<i64> = parent_spark_max.keys().copied().collect();
                    let p_storage = SparkGroupStorage::new(&conn, &p_spark_ids);

                    let mut p_spark_groups: Vec<SparkGroupInfo> = Vec::new();
                    for (&gid, &max_lvl) in &parent_spark_max {
                        p_spark_groups.push(SparkGroupInfo {
                            spark_group_id: gid,
                            name: p_storage.name(gid).to_string(),
                            spark_type: SparkType::from_raw(p_storage.spark_type(gid)),
                            total_stars: max_lvl as i8,
                            trainee_stars_veteran: 0,
                            uma_count: 1,
                        });
                    }

                    let p_name: String = conn
                        .query_row(
                            "SELECT COALESCE(name, 'Unknown') FROM trainee_data WHERE id = ?1",
                            params![parent.trainee_id],
                            |row| row.get(0),
                        )
                        .optional()
                        .map_err(|e| format!("query parent name failed: {e}"))?
                        .unwrap_or_else(|| format!("Trainee {}", parent.trainee_id));

                    let p_char_id: i64 = conn
                        .query_row(
                            "SELECT character_id FROM trainee_data WHERE id = ?1",
                            params![parent.trainee_id],
                            |row| row.get(0),
                        )
                        .optional()
                        .map_err(|e| format!("query parent character_id failed: {e}"))?
                        .unwrap_or(0);

                    let parent_uma = ParentUma {
                        name: p_name,
                        character_id: p_char_id,
                        hash: p_hash,
                        spark_groups: p_spark_groups,
                        major_wins: parent.container_major_wins.clone(),
                        api_mode: true,
                    };

                    set_slot_value(state, gp_slot, Some(LegacySlotValue::ParentUma(parent_uma)));
                }
            }
        }
    }

    set_slot_value(state, slot, Some(LegacySlotValue::LegacyUma(uma)));

    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn set_legacy_planner_slot_character(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
    slot_label: String,
    character_option: CharacterOption,
) -> Result<(), String> {
    let slot = LegacyPlannerSlot::from_label(&slot_label)
        .ok_or_else(|| format!("invalid slot label: {slot_label}"))?;

    let conn = app_db::open_app_database_connection()?;
    let mut guard = get_or_load_state(&handle, &conn)?;
    let state = guard.get_or_insert_with(LegacyPlannerState::default);

    if is_grandparent_locked(slot_value(state, slot)) {
        return Err("Slot is locked (grandparent inherited from parent)".into());
    }

    if !can_add_character_to_slot(state, slot, character_option.character_id) {
        return Err("Cannot add: this character is already used in a related slot".into());
    }

    set_slot_value(
        state,
        slot,
        Some(LegacySlotValue::Character(character_option)),
    );

    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn clear_legacy_planner_slot(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
    slot_label: String,
) -> Result<(), String> {
    let slot = LegacyPlannerSlot::from_label(&slot_label)
        .ok_or_else(|| format!("invalid slot label: {slot_label}"))?;

    let conn = app_db::open_app_database_connection()?;
    let mut guard = get_or_load_state(&handle, &conn)?;
    let state = guard.get_or_insert_with(LegacyPlannerState::default);

    if is_grandparent_locked(slot_value(state, slot)) {
        return Err("Slot is locked (grandparent inherited from parent)".into());
    }

    // If clearing a parent, also clear its locked grandparents
    match slot {
        LegacyPlannerSlot::ParentA => {
            set_slot_value(state, LegacyPlannerSlot::ParentA, None);
            if is_grandparent_locked(slot_value(state, LegacyPlannerSlot::GrandparentAA)) {
                set_slot_value(state, LegacyPlannerSlot::GrandparentAA, None);
            }
            if is_grandparent_locked(slot_value(state, LegacyPlannerSlot::GrandparentAB)) {
                set_slot_value(state, LegacyPlannerSlot::GrandparentAB, None);
            }
        }
        LegacyPlannerSlot::ParentB => {
            set_slot_value(state, LegacyPlannerSlot::ParentB, None);
            if is_grandparent_locked(slot_value(state, LegacyPlannerSlot::GrandparentBA)) {
                set_slot_value(state, LegacyPlannerSlot::GrandparentBA, None);
            }
            if is_grandparent_locked(slot_value(state, LegacyPlannerSlot::GrandparentBB)) {
                set_slot_value(state, LegacyPlannerSlot::GrandparentBB, None);
            }
        }
        _ => {
            set_slot_value(state, slot, None);
        }
    }

    drop(guard);
    persist_and_emit(&handle, &app, &conn)
}

#[tauri::command]
pub fn clear_legacy_planner(
    app: AppHandle,
    handle: State<'_, LegacyPlannerStateHandle>,
) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    let mut guard = handle.state.lock().map_err(|e| e.to_string())?;
    *guard = None;
    // Clear from DB too
    conn.execute(
        "DELETE FROM legacy_planner_state WHERE key = ?1",
        params![PLANNER_STATE_KEY],
    )
    .map_err(|e| format!("clear state failed: {e}"))?;
    drop(guard);
    let _ = app.emit(
        "legacy-planner-state-changed",
        LegacyPlannerState::default(),
    );
    Ok(())
}

// ── Affinity Computation ───────────────────────────────────────────

fn compute_pair_affinity(conn: &Connection, char_id_1: i64, char_id_2: i64) -> Option<i32> {
    let point: Option<i32> = conn
        .query_row(
            "SELECT ag.affinity_point
             FROM affinity_member am1
             JOIN affinity_member am2 ON am1.affinity_group = am2.affinity_group
             JOIN affinity_groups ag ON ag.affinity_group = am1.affinity_group
             WHERE am1.chara_id = ?1 AND am2.chara_id = ?2
             LIMIT 1",
            params![char_id_1, char_id_2],
            |row| row.get(0),
        )
        .optional()
        .ok()
        .flatten();
    point
}

fn compute_trio_affinity(
    conn: &Connection,
    char_id_1: i64,
    char_id_2: i64,
    char_id_3: i64,
) -> Option<i32> {
    // Three characters share an affinity group — we need 3-way intersection.
    // Find groups where all 3 are members.
    let point: Option<i32> = conn
        .query_row(
            "SELECT ag.affinity_point
             FROM affinity_member am1
             JOIN affinity_member am2 ON am1.affinity_group = am2.affinity_group
             JOIN affinity_member am3 ON am1.affinity_group = am3.affinity_group
             JOIN affinity_groups ag ON ag.affinity_group = am1.affinity_group
             WHERE am1.chara_id = ?1 AND am2.chara_id = ?2 AND am3.chara_id = ?3
             LIMIT 1",
            params![char_id_1, char_id_2, char_id_3],
            |row| row.get(0),
        )
        .optional()
        .ok()
        .flatten();
    point
}

fn get_parent_character_id(conn: &Connection, parent_hash: i64) -> Option<i64> {
    let trainee_id: Option<i64> = conn
        .query_row(
            "SELECT trainee_id FROM parents WHERE hash = ?1",
            params![parent_hash],
            |row| row.get(0),
        )
        .optional()
        .ok()
        .flatten()?;

    conn.query_row(
        "SELECT character_id FROM trainee_data WHERE id = ?1",
        params![trainee_id],
        |row| row.get(0),
    )
    .optional()
    .ok()
    .flatten()
}

/// Compute preview affinity for a veteran as a candidate parent
pub fn compute_veteran_affinity_internal(
    conn: &Connection,
    chosen_char_id: i64,
    veteran_hash: i64,
) -> Option<i32> {
    // Get veteran's character_id and parent hashes
    let (veteran_character_id, parent_a_hash, parent_b_hash): (i64, Option<i64>, Option<i64>) =
        conn.query_row(
            "SELECT v.trainee_id, v.parent_a, v.parent_b FROM veterans v WHERE v.hash = ?1",
            params![veteran_hash],
            |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, Option<i64>>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                ))
            },
        )
        .optional()
        .ok()
        .flatten()
        .and_then(|(trainee_id, pa, pb)| {
            let char_id: i64 = conn
                .query_row(
                    "SELECT character_id FROM trainee_data WHERE id = ?1",
                    params![trainee_id],
                    |row| row.get(0),
                )
                .optional()
                .ok()
                .flatten()
                .unwrap_or(0);
            Some((char_id, pa, pb))
        })?;

    let grandparent_1_char_id = parent_a_hash.and_then(|h| get_parent_character_id(conn, h));
    let grandparent_2_char_id = parent_b_hash.and_then(|h| get_parent_character_id(conn, h));

    let mut total = 0;

    // 1. Chosen character ↔ Veteran (direct pair)
    if let Some(aff) = compute_pair_affinity(conn, chosen_char_id, veteran_character_id) {
        total += aff;
    }

    // 2. Chosen character ↔ Veteran ↔ Grandparent 1 (trio)
    if let Some(gp1) = grandparent_1_char_id {
        if let Some(aff) = compute_trio_affinity(conn, chosen_char_id, veteran_character_id, gp1) {
            total += aff;
        }
    }

    // 3. Chosen character ↔ Veteran ↔ Grandparent 2 (trio)
    if let Some(gp2) = grandparent_2_char_id {
        if let Some(aff) = compute_trio_affinity(conn, chosen_char_id, veteran_character_id, gp2) {
            total += aff;
        }
    }

    Some(total)
}

#[tauri::command]
pub fn compute_veteran_affinities(
    handle: State<'_, LegacyPlannerStateHandle>,
) -> Result<Vec<VeteranAffinity>, String> {
    let conn = app_db::open_app_database_connection()?;
    let guard = get_or_load_state(&handle, &conn)?;

    let chosen_char_id = match guard.as_ref().and_then(|s| s.chosen_character.as_ref()) {
        Some(c) => c.character_id,
        None => return Ok(Vec::new()),
    };
    drop(guard);

    // Get all veteran hashes
    let mut stmt = conn
        .prepare("SELECT hash FROM veterans")
        .map_err(|e| format!("prepare veteran list failed: {e}"))?;
    let hashes: Vec<i64> = stmt
        .query_map([], |row| row.get(0))
        .map_err(|e| format!("query veteran list failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect veteran list failed: {e}"))?;

    let affinities: Vec<VeteranAffinity> = hashes
        .into_iter()
        .map(|hash| VeteranAffinity {
            hash,
            affinity: compute_veteran_affinity_internal(&conn, chosen_char_id, hash),
        })
        .collect();

    Ok(affinities)
}

/// Bulk compute affinities for specific hashes (for pagination)
#[allow(dead_code)]
pub fn compute_affinities_for_hashes(
    conn: &Connection,
    chosen_char_id: i64,
    hashes: &[i64],
) -> HashMap<i64, Option<i32>> {
    let mut map = HashMap::new();
    for &hash in hashes {
        let aff = compute_veteran_affinity_internal(conn, chosen_char_id, hash);
        map.insert(hash, aff);
    }
    map
}

#[tauri::command]
pub fn get_planner_trainee_characters() -> Result<Vec<(i64, String, i64, String)>, String> {
    use crate::db::app_db;
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare(
            "SELECT td.id, td.name, td.character_id, COALESCE(cd.name, '') \
             FROM trainee_data td \
             LEFT JOIN character_data cd ON cd.id = td.character_id \
             ORDER BY cd.name, td.name",
        )
        .map_err(|e| format!("prepare failed: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, String>(3)?,
            ))
        })
        .map_err(|e| format!("query failed: {e}"))?;
    rows.collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect failed: {e}"))
}

#[tauri::command]
pub fn compute_planner_affinities(
    handle: State<'_, LegacyPlannerStateHandle>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<PlannerAffinities, String> {
    let guard = handle.state.lock().map_err(|e| e.to_string())?;
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return Ok(PlannerAffinities::default()),
    };
    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;

    let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id);

    let pa = state
        .parent_a
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let pb = state
        .parent_b
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpaa = state
        .grandparent_aa
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpab = state
        .grandparent_ab
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpba = state
        .grandparent_ba
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpbb = state
        .grandparent_bb
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    drop(guard);

    let mut result = PlannerAffinities::default();

    // trainee ↔ parent_a (pair base only)
    if let (Some(c), Some(p)) = (chosen_id, &pa) {
        result.trainee_parent_a = Some(AffinityResult {
            base: affinity.pair_base(c, p.character_id),
            bonus: 0,
        });
    }

    // trainee ↔ parent_b (pair base only)
    if let (Some(c), Some(p)) = (chosen_id, &pb) {
        result.trainee_parent_b = Some(AffinityResult {
            base: affinity.pair_base(c, p.character_id),
            bonus: 0,
        });
    }

    // parent_a ↔ parent_b (pair base + shared wins)
    if let (Some(a), Some(b)) = (&pa, &pb) {
        result.parent_a_parent_b = Some(AffinityResult {
            base: affinity.pair_base(a.character_id, b.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&a.wins, &b.wins),
        });
    }

    fn compute_grandparent_affinity(
        chosen_id: Option<i64>,
        parent: &SlimUma,
        grandparent: &SlimUma,
        affinity: &AffinityStorage,
    ) -> AffinityResult {
        let base = match chosen_id {
            Some(c) => affinity.trio_base(c, parent.character_id, grandparent.character_id),
            None => affinity.pair_base(parent.character_id, grandparent.character_id),
        };
        let bonus = AffinityStorage::shared_wins_bonus(&parent.wins, &grandparent.wins);
        AffinityResult { base, bonus }
    }

    // parent_a ↔ grandparent_aa
    if let (Some(p), Some(g)) = (&pa, &gpaa) {
        result.parent_a_grandparent_aa =
            Some(compute_grandparent_affinity(chosen_id, p, g, &affinity));
    }

    // parent_a ↔ grandparent_ab
    if let (Some(p), Some(g)) = (&pa, &gpab) {
        result.parent_a_grandparent_ab =
            Some(compute_grandparent_affinity(chosen_id, p, g, &affinity));
    }

    // parent_b ↔ grandparent_ba
    if let (Some(p), Some(g)) = (&pb, &gpba) {
        result.parent_b_grandparent_ba =
            Some(compute_grandparent_affinity(chosen_id, p, g, &affinity));
    }

    // parent_b ↔ grandparent_bb
    if let (Some(p), Some(g)) = (&pb, &gpbb) {
        result.parent_b_grandparent_bb =
            Some(compute_grandparent_affinity(chosen_id, p, g, &affinity));
    }

    Ok(result)
}

// ── Trainee Stats ────────────────────────────────────────────────

#[tauri::command]
pub fn get_trainee_stats(
    trainee_id: i64,
    rarity: i64,
) -> Result<Option<TraineeStatsDataRow>, String> {
    let conn = app_db::open_app_database_connection()?;
    let row = conn
        .query_row(
            "SELECT trainee_id, rarity, spe, sta, pwr, gut, wit,
                    aptitude_dist_sprint, aptitude_dist_mile, aptitude_dist_medium, aptitude_dist_long,
                    aptitude_ground_turf, aptitude_ground_dirt,
                    aptitude_style_front, aptitude_style_pace_chaser, aptitude_style_late_surger, aptitude_style_end_closer
             FROM trainee_stats_data
             WHERE trainee_id = ?1 AND rarity = ?2",
            params![trainee_id, rarity],
            |row| {
                Ok(TraineeStatsDataRow {
                    trainee_id: row.get(0)?,
                    rarity: row.get(1)?,
                    spe: row.get(2)?,
                    sta: row.get(3)?,
                    pwr: row.get(4)?,
                    gut: row.get(5)?,
                    wit: row.get(6)?,
                    aptitude_dist_sprint: row.get(7)?,
                    aptitude_dist_mile: row.get(8)?,
                    aptitude_dist_medium: row.get(9)?,
                    aptitude_dist_long: row.get(10)?,
                    aptitude_ground_turf: row.get(11)?,
                    aptitude_ground_dirt: row.get(12)?,
                    aptitude_style_front: row.get(13)?,
                    aptitude_style_pace_chaser: row.get(14)?,
                    aptitude_style_late_surger: row.get(15)?,
                    aptitude_style_end_closer: row.get(16)?,
                })
            },
        )
        .optional()
        .map_err(|e| format!("query trainee stats failed: {e}"))?;
    Ok(row)
}

#[tauri::command]
pub fn get_trainee_available_rarities(trainee_id: i64) -> Result<Vec<i64>, String> {
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare(
            "SELECT DISTINCT rarity FROM trainee_stats_data WHERE trainee_id = ?1 ORDER BY rarity",
        )
        .map_err(|e| format!("prepare rarities query failed: {e}"))?;
    let rarities = stmt
        .query_map(params![trainee_id], |row| row.get::<_, i64>(0))
        .map_err(|e| format!("query rarities failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect rarities failed: {e}"))?;
    Ok(rarities)
}

// ── Affinity Summary (extended with pairs + hash map) ───────────

fn compute_affinity_summary_internal(
    state: &LegacyPlannerState,
    affinity: &AffinityStorage,
) -> PlannerAffinitySummary {
    let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id);

    let pa = state
        .parent_a
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let pb = state
        .parent_b
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpaa = state
        .grandparent_aa
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpab = state
        .grandparent_ab
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpba = state
        .grandparent_ba
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpbb = state
        .grandparent_bb
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));

    let pa_hash = state.parent_a.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });
    let pb_hash = state.parent_b.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });
    let gpaa_hash = state.grandparent_aa.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });
    let gpab_hash = state.grandparent_ab.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });
    let gpba_hash = state.grandparent_ba.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });
    let gpbb_hash = state.grandparent_bb.as_ref().and_then(|v| match v {
        LegacySlotValue::LegacyUma(u) => Some(u.hash),
        LegacySlotValue::ParentUma(u) => Some(u.hash),
        LegacySlotValue::Character(_) => None,
    });

    let mut pairs = Vec::new();
    let mut total_affinity_by_hash = BTreeMap::new();
    let mut total_base: i32 = 0;
    let mut total_bonus: i32 = 0;

    macro_rules! push_pair {
        ($label:expr, $base:expr, $bonus:expr, $hash:expr) => {
            let b = $base.unwrap_or(0);
            let bn = $bonus.unwrap_or(0);
            total_base += b;
            total_bonus += bn;
            pairs.push(AffinityPairInfo {
                label: $label.to_string(),
                base_affinity: $base,
                bonus_affinity: $bonus,
            });
            if let Some(h) = $hash {
                *total_affinity_by_hash.entry(h).or_insert(0) += b + bn;
            }
        };
    }

    // 0: Trainee ↔ Parent A
    {
        let (base, bonus) = match (chosen_id, &pa) {
            (Some(c), Some(p)) => (Some(affinity.pair_base(c, p.character_id) as i32), None),
            _ => (None, None),
        };
        push_pair!("Trainee ↔ Parent A", base, bonus, None);
    }

    // 1: Trainee ↔ Parent B
    {
        let (base, bonus) = match (chosen_id, &pb) {
            (Some(c), Some(p)) => (Some(affinity.pair_base(c, p.character_id) as i32), None),
            _ => (None, None),
        };
        push_pair!("Trainee ↔ Parent B", base, bonus, None);
    }

    // 2: Parent A ↔ Parent B
    {
        let (base, bonus) = match (&pa, &pb) {
            (Some(a), Some(b)) => (
                Some(affinity.pair_base(a.character_id, b.character_id) as i32),
                Some(AffinityStorage::shared_wins_bonus(&a.wins, &b.wins) as i32),
            ),
            _ => (None, None),
        };
        push_pair!("Parent A ↔ Parent B", base, bonus, None);
    }

    // 3: Parent A ↔ Grandparent AA
    {
        let (base, bonus) = match (chosen_id, &pa, &gpaa) {
            (Some(c), Some(p), Some(g)) => (
                Some(affinity.trio_base(c, p.character_id, g.character_id) as i32),
                Some(AffinityStorage::shared_wins_bonus(&p.wins, &g.wins) as i32),
            ),
            _ => (None, None),
        };
        push_pair!("Parent A ↔ Grandparent AA", base, bonus, pa_hash);
        if let Some(h) = gpaa_hash {
            *total_affinity_by_hash.entry(h).or_insert(0) += base.unwrap_or(0) + bonus.unwrap_or(0);
        }
    }

    // 4: Parent A ↔ Grandparent AB
    {
        let (base, bonus) = match (chosen_id, &pa, &gpab) {
            (Some(c), Some(p), Some(g)) => (
                Some(affinity.trio_base(c, p.character_id, g.character_id) as i32),
                Some(AffinityStorage::shared_wins_bonus(&p.wins, &g.wins) as i32),
            ),
            _ => (None, None),
        };
        push_pair!("Parent A ↔ Grandparent AB", base, bonus, pa_hash);
        if let Some(h) = gpab_hash {
            *total_affinity_by_hash.entry(h).or_insert(0) += base.unwrap_or(0) + bonus.unwrap_or(0);
        }
    }

    // 5: Parent B ↔ Grandparent BA
    {
        let (base, bonus) = match (chosen_id, &pb, &gpba) {
            (Some(c), Some(p), Some(g)) => (
                Some(affinity.trio_base(c, p.character_id, g.character_id) as i32),
                Some(AffinityStorage::shared_wins_bonus(&p.wins, &g.wins) as i32),
            ),
            _ => (None, None),
        };
        push_pair!("Parent B ↔ Grandparent BA", base, bonus, pb_hash);
        if let Some(h) = gpba_hash {
            *total_affinity_by_hash.entry(h).or_insert(0) += base.unwrap_or(0) + bonus.unwrap_or(0);
        }
    }

    // 6: Parent B ↔ Grandparent BB
    {
        let (base, bonus) = match (chosen_id, &pb, &gpbb) {
            (Some(c), Some(p), Some(g)) => (
                Some(affinity.trio_base(c, p.character_id, g.character_id) as i32),
                Some(AffinityStorage::shared_wins_bonus(&p.wins, &g.wins) as i32),
            ),
            _ => (None, None),
        };
        push_pair!("Parent B ↔ Grandparent BB", base, bonus, pb_hash);
        if let Some(h) = gpbb_hash {
            *total_affinity_by_hash.entry(h).or_insert(0) += base.unwrap_or(0) + bonus.unwrap_or(0);
        }
    }

    // Base affinity (hash 0) = sum of chosen↔parent pair affinities
    let base_aff = pairs[0].base_affinity.unwrap_or(0) + pairs[1].base_affinity.unwrap_or(0);
    total_affinity_by_hash.insert(0, base_aff);

    PlannerAffinitySummary {
        pairs,
        total: total_base + total_bonus,
        base: total_base,
        bonus: total_bonus,
        total_affinity_by_hash,
    }
}

#[tauri::command]
pub fn get_planner_affinity_summary(
    handle: State<'_, LegacyPlannerStateHandle>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<PlannerAffinitySummary, String> {
    let guard = handle.state.lock().map_err(|e| e.to_string())?;
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return Ok(PlannerAffinitySummary::default()),
    };
    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;
    Ok(compute_affinity_summary_internal(state, &affinity))
}

// ── Spark Summary (white spark generating chance) ───────────────

fn chance_tables_from_parent_grandparent_count(carriers: usize) -> (f64, f64, f64) {
    let idx = carriers.min(6);
    const WHITE_SKILL_RACE_SCENARIO: [f64; 7] = [20.0, 22.5, 25.0, 27.5, 30.0, 32.5, 35.0];
    const MARU_SKILL: [f64; 7] = [25.0, 27.5, 30.0, 32.5, 35.0, 37.5, 40.0];
    const GOLD_SKILL: [f64; 7] = [40.0, 45.0, 50.0, 55.0, 60.0, 65.0, 70.0];
    (
        WHITE_SKILL_RACE_SCENARIO[idx],
        MARU_SKILL[idx],
        GOLD_SKILL[idx],
    )
}

fn collect_spark_groups(state: &LegacyPlannerState) -> Vec<SparkGroupInfo> {
    let slots = [
        &state.parent_a,
        &state.parent_b,
        &state.grandparent_aa,
        &state.grandparent_ab,
        &state.grandparent_ba,
        &state.grandparent_bb,
    ];
    let mut all = Vec::new();
    for slot in slots {
        match slot {
            Some(LegacySlotValue::LegacyUma(u)) => {
                let mut sgs: Vec<SparkGroupInfo> = u.spark_groups.iter().cloned().collect();
                for sg in &mut sgs {
                    sg.total_stars = sg.trainee_stars_veteran;
                }
                all.extend(sgs);
            }
            Some(LegacySlotValue::ParentUma(u)) => all.extend(u.spark_groups.iter().cloned()),
            _ => {}
        }
    }
    all
}

#[tauri::command]
pub fn get_planner_spark_summary(
    handle: State<'_, LegacyPlannerStateHandle>,
) -> Result<Vec<SparkSummaryRow>, String> {
    let guard = handle.state.lock().map_err(|e| e.to_string())?;
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return Ok(Vec::new()),
    };

    let all_sparks = collect_spark_groups(state);
    drop(guard);

    let mut grouped: BTreeMap<i64, (String, SparkType, i32, usize)> = BTreeMap::new();

    for spark in all_sparks {
        if spark.total_stars == 0 {
            continue;
        }
        grouped
            .entry(spark.spark_group_id)
            .and_modify(|(_, _, stars, carriers)| {
                *stars += spark.total_stars as i32;
                *carriers += 1;
            })
            .or_insert((
                spark.name.clone(),
                spark.spark_type,
                spark.total_stars as i32,
                1,
            ));
    }

    let mut rows: Vec<SparkSummaryRow> = grouped
        .into_iter()
        .map(
            |(spark_group_id, (spark_name, spark_type, total_stars, carriers))| {
                let (white, maru, gold) = if spark_type.is_white() {
                    let (w, m, g) = chance_tables_from_parent_grandparent_count(carriers);
                    (Some(w), Some(m), Some(g))
                } else {
                    (None, None, None)
                };
                SparkSummaryRow {
                    spark_group_id,
                    spark_name,
                    spark_type,
                    total_stars,
                    legacy_uma_count: carriers,
                    white_probability_pct: white,
                    maru_skill_probability_pct: maru,
                    gold_skill_probability_pct: gold,
                }
            },
        )
        .collect();

    rows.sort_by(|a, b| {
        b.total_stars
            .cmp(&a.total_stars)
            .then(a.spark_name.cmp(&b.spark_name))
    });
    Ok(rows)
}

// ── Inspiration Summary ─────────────────────────────────────────

fn calc_spark_chance(spark_type: SparkType, total_stars: i8, total_affinity: i32) -> f64 {
    let total_affinity = total_affinity as f64;
    let base_chance = match (spark_type, total_stars) {
        (SparkType::Stat, 1) => 70.0,
        (SparkType::Stat, 2) => 80.0,
        (SparkType::Stat, 3) => 90.0,
        (SparkType::Aptitude, 1) => 1.0,
        (SparkType::Aptitude, 2) => 3.0,
        (SparkType::Aptitude, 3) => 5.0,
        (SparkType::Unique, 1) => 5.0,
        (SparkType::Unique, 2) => 10.0,
        (SparkType::Unique, 3) => 15.0,
        (SparkType::Skill | SparkType::Race | SparkType::Scenario | SparkType::Event, 1) => 3.0,
        (SparkType::Skill | SparkType::Race | SparkType::Scenario | SparkType::Event, 2) => 6.0,
        (SparkType::Skill | SparkType::Race | SparkType::Scenario | SparkType::Event, 3) => 9.0,
        _ => 0.0,
    };
    base_chance * (1.0 + total_affinity / 100.0)
}

#[tauri::command]
pub fn get_planner_inspiration_summary(
    handle: State<'_, LegacyPlannerStateHandle>,
    affinity_store: State<'_, Mutex<AffinityStorage>>,
) -> Result<Vec<InspirationSummaryRow>, String> {
    let guard = handle.state.lock().map_err(|e| e.to_string())?;
    let state = match guard.as_ref() {
        Some(s) => s,
        None => return Ok(Vec::new()),
    };
    let affinity = affinity_store.lock().map_err(|e| e.to_string())?;
    let affinity_summary = compute_affinity_summary_internal(state, &affinity);
    let total_affinity_by_hash = affinity_summary.total_affinity_by_hash;

    let mut rows: BTreeMap<i64, InspirationSummaryRow> = BTreeMap::new();

    let slots_with_affinity: [(&Option<LegacySlotValue>, bool); 6] = [
        (&state.parent_a, true),
        (&state.parent_b, true),
        (&state.grandparent_aa, false),
        (&state.grandparent_ab, false),
        (&state.grandparent_ba, false),
        (&state.grandparent_bb, false),
    ];

    for (slot, is_parent) in &slots_with_affinity {
        let Some(value) = slot else { continue };
        let (hash, spark_groups, is_legacy) = match value {
            LegacySlotValue::LegacyUma(u) => (u.hash, &u.spark_groups, true),
            LegacySlotValue::ParentUma(u) => (u.hash, &u.spark_groups, false),
            _ => continue,
        };

        let mut uma_affinity = total_affinity_by_hash.get(&hash).copied().unwrap_or(0);
        if *is_parent {
            uma_affinity += total_affinity_by_hash.get(&0).copied().unwrap_or(0);
        }

        for spark in spark_groups {
            let stars = if is_legacy {
                spark.trainee_stars_veteran
            } else {
                spark.total_stars
            };

            if stars == 0 {
                continue;
            }

            let sparking_chance = calc_spark_chance(spark.spark_type, stars, uma_affinity);

            if let Some(existing) = rows.get_mut(&spark.spark_group_id) {
                existing.sparking_chance += sparking_chance;
            } else {
                rows.insert(
                    spark.spark_group_id,
                    InspirationSummaryRow {
                        spark_group_id: spark.spark_group_id,
                        spark_name: spark.name.clone(),
                        spark_type: spark.spark_type,
                        sparking_chance,
                    },
                );
            }
        }
    }

    Ok(rows.into_values().collect())
}

#[tauri::command]
pub async fn open_legacy_planner_window(app: AppHandle) -> Result<(), String> {
    let label = "legacy-planner";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("Legacy Planner")
        .inner_size(1100.0, 850.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}
