use crate::db::app_db;
use crate::handlers::worker::{supervisor_discover_view, WorkerStatusState};
use crate::storage::skills::SkillStorage;
use crate::veterans;
use crate::worker::WorkerState;
use rusqlite::params;
use rusqlite::types::ToSql;
use serde::Serialize;
use serde_json::json;
use shared::models::PaginationResponse;
use shared::models::{
    RaceDistance, RawEvent, RawFrame, RawHorseData, ReplayEvent, ReplayFrame, ReplayHorseData,
};
use shared::mssgpack_data::MssgPackTrainedChara;
use shared::{models::ReplayEventData, veteran_browser::TagRow};
use shared::{
    date_time::normalize_bound, RaceDumpBrowserQuery, RaceDumpFilter, RaceDumpFilterOptions,
    RaceDumpPageItem, RaceDumpSummary,
};
use std::collections::HashMap;
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager, State};

pub struct RaceDumpDetailState(pub std::sync::Mutex<Option<i64>>);

const DEFAULT_RACE_DUMP_TIMEOUT_MS: u64 = 15_000;

#[derive(Debug, Clone, Serialize)]
pub struct SaveRaceDumpResult {
    pub race_dump_id: i64,
    pub participants: usize,
}

#[tauri::command]
pub async fn save_race_dump(
    app: AppHandle,
    mut request: honse_worker::protocol::WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<SaveRaceDumpResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();

        let ss: State<'_, WorkerStatusState> = app.state();
        supervisor_discover_view(&app, &state, &ss);
        if ss.snapshot(false).current_view_id_raw != Some(400) {
            return Err("Must be on the race view (id 400) to save a race dump".to_string());
        }

        let request_id = request.id.unwrap_or_else(|| state.next_request_id());
        request.id = Some(request_id);

        let receiver = state.register_pending(request_id)?;

        if let Err(err) = state.with_running_worker(|running| {
            honse_worker::protocol::write_msgpack_request_framed(&mut running.stdin, &request)
                .map_err(|write_err| format!("failed to write msgpack request: {write_err}"))
        }) {
            state.clear_pending(request_id);
            return Err(err);
        }

        let frame = await_worker_frame(
            &state,
            request_id,
            receiver,
            timeout_ms.unwrap_or(DEFAULT_RACE_DUMP_TIMEOUT_MS),
        )?;

        let value = honse_worker::protocol::parse_msgpack_frame_response(&frame)
            .and_then(|resp| {
                if let honse_worker::protocol::WorkerResponse::Ok(ok) = resp {
                    Some(super::rmpv_to_json(ok.payload))
                } else {
                    None
                }
            })
            .ok_or_else(|| "failed to parse race dump worker response".to_string())?;

        let mut conn = app_db::open_app_database_connection()?;
        let tx = conn
            .transaction()
            .map_err(|err| format!("failed to start race dump transaction: {err}"))?;

        let result = process_race_dump_value(&value, &tx)
            .map_err(|e| format!("failed to process race dump data: {}", e))?;

        tx.commit()
            .map_err(|err| format!("failed to commit race dump transaction: {err}"))?;

        let _ = app.emit("race-dump-saved", &result);

        Ok(result)
    })
    .await
    .map_err(|err| format!("failed to join race dump task: {err}"))?
}

fn process_race_dump_value(
    value: &serde_json::Value,
    tx: &rusqlite::Connection,
) -> Result<SaveRaceDumpResult, String> {
    let metadata = value.get("metadata").cloned().unwrap_or_default();
    let frames = value
        .get("frames")
        .unwrap_or(&serde_json::Value::Null)
        .to_string();
    let events = value
        .get("events")
        .unwrap_or(&serde_json::Value::Null)
        .to_string();

    let sim_data_base64 = metadata
        .get("sim_data_base64")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string())
        .unwrap_or_default();

    let race_course_set_id = metadata.get("race_course_set_id").and_then(|v| v.as_i64());
    let float_lane_max = metadata.get("float_lane_max").and_then(|v| v.as_i64());

    let race_instance_id = metadata.get("race_instance_id").and_then(|v| v.as_i64());
    let race_type = metadata.get("race_type").and_then(|v| v.as_i64());

    // Reject unsupported race types
    if let Some(rt) = race_type {
        if ![5, 6, 8, 14].contains(&rt) {
            return Err(format!("unsupported race type: {rt}"));
        }
    }

    let season = metadata.get("season").and_then(|v| v.as_i64());
    let weather = metadata
        .get("race_weather")
        .and_then(|v| v.as_i64())
        .or_else(|| metadata.get("weather").and_then(|v| v.as_i64()));
    let ground_condition = metadata.get("ground_condition").and_then(|v| v.as_i64());
    let distance = metadata.get("distance").and_then(|v| v.as_i64());
    let track_id = metadata.get("track_id").and_then(|v| v.as_i64());
    let ground_type = metadata.get("ground_type").and_then(|v| v.as_i64());
    let turn = metadata.get("turn").and_then(|v| v.as_i64());
    let inout = metadata.get("inout").and_then(|v| v.as_i64());
    let viewer_id = metadata.get("viewer_id").and_then(|v| v.as_i64());
    let race_id = metadata.get("race_id").and_then(|v| v.as_i64());
    let champions_id = metadata.get("champions_id").and_then(|v| v.as_i64());
    let league_type = metadata.get("league_type").and_then(|v| v.as_i64());
    let round = metadata.get("round").and_then(|v| v.as_i64());

    let horses = value
        .get("horses")
        .and_then(|v| v.as_array())
        .ok_or_else(|| "missing 'horses' array in race dump data".to_string())?;

    let race_dump_id = insert_race_dump_row(
        tx,
        race_instance_id,
        race_id,
        race_type,
        season,
        weather,
        ground_condition,
        distance,
        track_id,
        ground_type,
        turn,
        inout,
        champions_id,
        league_type,
        round,
        &frames,
        &events,
        &sim_data_base64,
        race_course_set_id,
        float_lane_max,
    )?;

    let mut participants = 0usize;

    for horse in horses {
        let horse_index = horse
            .get("horse_index")
            .and_then(|v| v.as_i64())
            .ok_or_else(|| "missing horse_index".to_string())?;
        let post_number = horse
            .get("post_number")
            .and_then(|v| v.as_i64())
            .unwrap_or(0);
        let finish_order = horse.get("finish_order").and_then(|v| v.as_i64());
        let popularity = horse.get("popularity").and_then(|v| v.as_i64());
        let chara_name = horse
            .get("chara_name")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string());

        let result = horse.get("result");
        let finish_time = result
            .and_then(|r| r.get("finish_time"))
            .and_then(|v| v.as_f64());
        let finish_diff_time = result
            .and_then(|r| r.get("finish_diff_time"))
            .and_then(|v| v.as_f64());
        let running_style = result
            .and_then(|r| r.get("running_style"))
            .and_then(|v| v.as_i64());

        let team_id = horse.get("team_id").and_then(|v| v.as_i64()).unwrap_or(0);
        let mut is_player = horse.get("is_player").and_then(|v| v.as_i64()).unwrap_or(0);

        let response_horse_data = horse
            .get("response_horse_data")
            .and_then(|v| if v.is_null() { None } else { Some(v) })
            .map(|v| v.to_string())
            .unwrap_or_default();

        // Try trained_chara_data first (full veteran horse)
        if let Some(trained_chara_val) =
            horse
                .get("trained_chara_data")
                .and_then(|v| if v.is_null() { None } else { Some(v) })
        {
            let chara: MssgPackTrainedChara = serde_json::from_value(trained_chara_val.clone())
                .map_err(|e| format!("failed to deserialize trained_chara_data: {e}"))?;

            let mut group =
                shared::db_models::veteran_data::UmaGroup::from_trained_chara_mssgpack(&chara)
                    .map_err(|e| format!("failed to build UmaGroup: {e}"))?;

            group.veteran.is_race_data = true;

            if is_player == 1 {
                // TeamStadium: team membership determines ownership
                group.veteran.owned = true;
            } else if let Some(vid) = viewer_id {
                group.veteran.owned = chara.viewer_id == vid;
                if chara.viewer_id == vid {
                    is_player = 1;
                }
                if chara.viewer_id != vid {
                    group.veteran.owner_id = Some(chara.viewer_id as u64);
                }
            }

            let _was_new = veterans::process_group_direct(&group, tx)?;

            let rows = tx
                .execute(
                    r#"INSERT OR IGNORE INTO race_dump_participant (
                    race_dump_id, horse_index, veteran_hash, post_number, finish_order,
                    finish_time, finish_diff_time, popularity, running_style, chara_name,
                    viewer_id, owner_viewer_id, team_id, is_player, response_horse_data
                ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15)"#,
                    params![
                        race_dump_id,
                        horse_index,
                        group.veteran.hash.as_i64(),
                        post_number,
                        finish_order,
                        finish_time,
                        finish_diff_time,
                        popularity,
                        running_style,
                        chara_name,
                        chara.viewer_id,
                        chara.owner_viewer_id,
                        team_id,
                        is_player,
                        &response_horse_data,
                    ],
                )
                .map_err(|e| format!("failed to insert race participant: {e}"))?;
            if rows > 0 {
                participants += 1;
            }
            continue;
        }

        // Fallback to response_horse_data (single-mode NPC/unregistered horse)
        if let Some(resp) =
            horse
                .get("response_horse_data")
                .and_then(|v| if v.is_null() { None } else { Some(v) })
        {
            let viewer_id_val = resp.get("viewer_id").and_then(|v| v.as_i64());
            let owner_viewer_id_val = resp.get("owner_viewer_id").and_then(|v| v.as_i64());
            let card_id_val = resp.get("card_id").and_then(|v| v.as_i64());
            let npc_type_val = resp.get("npc_type").and_then(|v| v.as_i64());
            let speed_val = resp.get("speed").and_then(|v| v.as_i64());
            let stamina_val = resp.get("stamina").and_then(|v| v.as_i64());
            let pow_val = resp.get("pow").and_then(|v| v.as_i64());
            let guts_val = resp.get("guts").and_then(|v| v.as_i64());
            let wiz_val = resp.get("wiz").and_then(|v| v.as_i64());

            if is_player == 0 {
                if let Some(vid) = viewer_id {
                    if viewer_id_val == Some(vid) {
                        is_player = 1;
                    }
                }
            }

            let rows = tx.execute(
                r#"INSERT OR IGNORE INTO race_dump_participant (
                    race_dump_id, horse_index, veteran_hash, post_number, finish_order,
                    finish_time, finish_diff_time, popularity, running_style,
                    viewer_id, owner_viewer_id, card_id, npc_type, chara_name,
                    speed, stamina, pow, guts, wiz,
                    team_id, is_player, response_horse_data
                ) VALUES (?1, ?2, NULL, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21)"#,
                params![
                    race_dump_id,
                    horse_index,
                    post_number,
                    finish_order,
                    finish_time,
                    finish_diff_time,
                    popularity,
                    running_style,
                    viewer_id_val,
                    owner_viewer_id_val,
                    card_id_val,
                    npc_type_val,
                    chara_name,
                    speed_val,
                    stamina_val,
                    pow_val,
                    guts_val,
                    wiz_val,
                    team_id,
                    is_player,
                    &response_horse_data,
                ],
            )
            .map_err(|e| format!("failed to insert race participant from response data: {e}"))?;
            if rows > 0 {
                participants += 1;
            }
            continue;
        }

        // Final fallback: basic HorseData only (filler NPC, no response data)
        let rows = tx
            .execute(
                r#"INSERT OR IGNORE INTO race_dump_participant (
                race_dump_id, horse_index, veteran_hash, post_number, finish_order,
                finish_time, finish_diff_time, popularity, running_style, chara_name,
                team_id, is_player
            ) VALUES (?1, ?2, NULL, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)"#,
                params![
                    race_dump_id,
                    horse_index,
                    post_number,
                    finish_order,
                    finish_time,
                    finish_diff_time,
                    popularity,
                    running_style,
                    chara_name,
                    team_id,
                    is_player,
                ],
            )
            .map_err(|e| format!("failed to insert minimal race participant: {e}"))?;
        if rows > 0 {
            participants += 1;
        }
    }

    Ok(SaveRaceDumpResult {
        race_dump_id,
        participants,
    })
}

fn insert_race_dump_row(
    conn: &rusqlite::Connection,
    race_instance_id: Option<i64>,
    race_id: Option<i64>,
    race_type: Option<i64>,
    season: Option<i64>,
    weather: Option<i64>,
    ground_condition: Option<i64>,
    distance: Option<i64>,
    track_id: Option<i64>,
    ground_type: Option<i64>,
    turn: Option<i64>,
    inout: Option<i64>,
    champions_id: Option<i64>,
    league_type: Option<i64>,
    round: Option<i64>,
    frames: &str,
    events: &str,
    sim_data_base64: &str,
    race_course_set_id: Option<i64>,
    float_lane_max: Option<i64>,
) -> Result<i64, String> {
    conn.execute(
        r#"INSERT INTO race_dump (
            race_type, race_instance_id, race_id, season, weather, ground_condition,
            distance, track_id, ground_type, turn, inout,
            champions_id, league_type, round, frames, events, sim_data_base64,
            race_course_set_id, float_lane_max
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)"#,
        params![
            race_type.unwrap_or(0),
            race_instance_id,
            race_id,
            season,
            weather,
            ground_condition,
            distance,
            track_id,
            ground_type,
            turn,
            inout,
            champions_id,
            league_type,
            round,
            frames,
            events,
            sim_data_base64,
            race_course_set_id,
            float_lane_max,
        ],
    )
    .map_err(|e| format!("failed to insert race_dump row: {e}"))?;

    Ok(conn.last_insert_rowid())
}

#[tauri::command]
pub fn get_race_dumps(
    _app: AppHandle,
    race_type_filter: Option<i64>,
) -> Result<Vec<RaceDumpSummary>, String> {
    let conn = app_db::open_app_database_connection()?;

    let mut sql = r#"
        SELECT
            rd.id, datetime(rd.capture_time, 'localtime') AS capture_time, rd.race_type,
            rd.race_instance_id, rd.race_id,
            rd.distance, rd.track_id, rd.ground_type,
            rd.season, rd.weather, rd.ground_condition,
            rd.turn, rd.inout,
            rd.champions_id, rd.league_type, rd.round,
            COALESCE(p.cnt, 0) AS participant_count,
            COALESCE(pp.cnt, 0) AS player_participant_count,
            COALESCE(pn.names, '') AS player_participant_names
        FROM race_dump rd
        LEFT JOIN (
            SELECT race_dump_id, COUNT(*) AS cnt
            FROM race_dump_participant
            GROUP BY race_dump_id
        ) p ON p.race_dump_id = rd.id
        LEFT JOIN (
            SELECT race_dump_id, COUNT(*) AS cnt
            FROM race_dump_participant
            WHERE is_player = 1
            GROUP BY race_dump_id
        ) pp ON pp.race_dump_id = rd.id
        LEFT JOIN (
            SELECT race_dump_id, GROUP_CONCAT(chara_name, '|!|') AS names
            FROM race_dump_participant
            WHERE is_player = 1 AND chara_name IS NOT NULL
            GROUP BY race_dump_id
        ) pn ON pn.race_dump_id = rd.id
    "#
    .to_string();

    let mut params_vec: Vec<Box<dyn rusqlite::types::ToSql>> = Vec::new();

    if let Some(filter) = race_type_filter {
        sql.push_str(" WHERE rd.race_type = ?");
        params_vec.push(Box::new(filter));
    }

    sql.push_str(" ORDER BY rd.id DESC");

    let mut stmt = conn
        .prepare(&sql)
        .map_err(|e| format!("prepare get_race_dumps failed: {e}"))?;

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        params_vec.iter().map(|p| p.as_ref()).collect();

    let rows = stmt
        .query_map(param_refs.as_slice(), |row| {
            let names_raw: String = row.get::<_, Option<String>>(18)?.unwrap_or_default();
            let player_names: Vec<String> = if names_raw.is_empty() {
                Vec::new()
            } else {
                names_raw.split("|!|").map(|s| s.to_string()).collect()
            };
            Ok(RaceDumpSummary {
                id: row.get(0)?,
                capture_time: row.get(1)?,
                race_type: row.get(2)?,
                race_instance_id: row.get(3)?,
                race_id: row.get(4)?,
                distance: row.get(5)?,
                track_id: row.get(6)?,
                ground_type: row.get(7)?,
                season: row.get(8)?,
                weather: row.get(9)?,
                ground_condition: row.get(10)?,
                turn: row.get(11)?,
                inout: row.get(12)?,
                champions_id: row.get(13)?,
                league_type: row.get(14)?,
                round: row.get(15)?,
                participant_count: row.get(16)?,
                player_participant_count: row.get(17)?,
                player_participants: player_names,
                race_name: None,
                track_name: None,
                tags: Vec::new(),
            })
        })
        .map_err(|e| format!("query get_race_dumps failed: {e}"))?;

    let mut dumps = Vec::new();
    for row in rows {
        dumps.push(row.map_err(|e| format!("row error: {e}"))?);
    }

    // Batch fetch tags for all dumps
    if !dumps.is_empty() {
        let dump_ids: Vec<i64> = dumps.iter().map(|d| d.id).collect();
        let tag_placeholders: Vec<String> = dump_ids.iter().map(|_| "?".to_string()).collect();
        let tag_sql = format!(
            "SELECT rdht.race_dump_id, t.id, t.tag_value, t.create_date \
             FROM race_dump_has_tag rdht \
             JOIN tag t ON t.id = rdht.tag_id \
             WHERE rdht.race_dump_id IN ({}) \
             ORDER BY t.tag_value",
            tag_placeholders.join(",")
        );
        if let Ok(mut tag_stmt) = conn.prepare(&tag_sql) {
            let tag_hash_refs: Vec<Box<dyn rusqlite::types::ToSql>> = dump_ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn rusqlite::types::ToSql>)
                .collect();
            let tag_hash_params: Vec<&dyn rusqlite::types::ToSql> =
                tag_hash_refs.iter().map(|p| p.as_ref()).collect();
            if let Ok(tag_rows) = tag_stmt.query_map(tag_hash_params.as_slice(), |row| {
                let dump_id: i64 = row.get(0)?;
                let tag = TagRow {
                    id: row.get(1)?,
                    tag_value: row.get(2)?,
                    create_date: row.get(3)?,
                };
                Ok((dump_id, tag))
            }) {
                let mut tag_map: HashMap<i64, Vec<TagRow>> = HashMap::new();
                for row in tag_rows.flatten() {
                    tag_map.entry(row.0).or_default().push(row.1);
                }
                for dump in &mut dumps {
                    if let Some(tags) = tag_map.remove(&dump.id) {
                        dump.tags = tags;
                    }
                }
            }
        }
    }

    Ok(dumps)
}

#[tauri::command]
pub fn delete_race_dump(id: i64) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    conn.execute("DELETE FROM race_dump WHERE id = ?1", params![id])
        .map_err(|e| format!("delete_race_dump failed: {e}"))?;
    Ok(())
}

pub(crate) fn build_race_dump_where(filters: &[RaceDumpFilter]) -> (String, Vec<Box<dyn ToSql>>) {
    if filters.is_empty() {
        return (String::new(), Vec::new());
    }

    let mut clauses = Vec::new();
    let mut params: Vec<Box<dyn ToSql>> = Vec::new();

    for filter in filters {
        match filter {
            RaceDumpFilter::RaceType(v) => {
                clauses.push("rd.race_type = ?".to_string());
                params.push(Box::new(*v as i64));
            }
            RaceDumpFilter::DistanceMeters { min, max } => {
                if let Some(v) = min {
                    clauses.push("rd.distance >= ?".to_string());
                    params.push(Box::new(*v));
                }
                if let Some(v) = max {
                    clauses.push("rd.distance <= ?".to_string());
                    params.push(Box::new(*v));
                }
            }
            RaceDumpFilter::Distance(d) => match d {
                RaceDistance::Sprint => {
                    clauses.push("rd.distance <= 1200".to_string());
                }
                RaceDistance::Mile => {
                    clauses.push("rd.distance > 1200 AND rd.distance <= 2000".to_string());
                }
                RaceDistance::Medium => {
                    clauses.push("rd.distance > 2000 AND rd.distance <= 2500".to_string());
                }
                RaceDistance::Long => {
                    clauses.push("rd.distance > 2500".to_string());
                }
            },
            RaceDumpFilter::GroundType(v) => {
                clauses.push("rd.ground_type = ?".to_string());
                params.push(Box::new(*v as i64));
            }
            RaceDumpFilter::Season(v) => {
                clauses.push("rd.season = ?".to_string());
                params.push(Box::new(*v as i64));
            }
            RaceDumpFilter::Weather(v) => {
                clauses.push("rd.weather = ?".to_string());
                params.push(Box::new(*v as i64));
            }
            RaceDumpFilter::GroundCondition(v) => {
                clauses.push("rd.ground_condition = ?".to_string());
                params.push(Box::new(*v as i64));
            }
            RaceDumpFilter::Character(id) => {
                clauses.push(
                    "EXISTS (\
                     SELECT 1 FROM race_dump_participant rdp \
                     JOIN veterans v ON v.hash = rdp.veteran_hash \
                     JOIN trainee_data td ON td.id = v.trainee_id \
                     WHERE rdp.race_dump_id = rd.id AND td.character_id = ?\
                     )"
                    .to_string(),
                );
                params.push(Box::new(*id));
            }
            RaceDumpFilter::Trainee(id) => {
                clauses.push(
                    "EXISTS (\
                     SELECT 1 FROM race_dump_participant rdp \
                     JOIN veterans v ON v.hash = rdp.veteran_hash \
                     JOIN trainee_data td ON td.id = v.trainee_id \
                     WHERE rdp.race_dump_id = rd.id AND td.id = ?\
                     )"
                    .to_string(),
                );
                params.push(Box::new(*id));
            }
            RaceDumpFilter::VeteranHash(h) => {
                clauses.push(
                    "EXISTS (\
                     SELECT 1 FROM race_dump_participant rdp \
                     WHERE rdp.race_dump_id = rd.id AND rdp.veteran_hash = ?\
                     )"
                    .to_string(),
                );
                params.push(Box::new(*h));
            }
            RaceDumpFilter::HasTag(s) => {
                clauses.push(
                    "EXISTS (\
                     SELECT 1 FROM race_dump_has_tag rdht \
                     JOIN tag t ON t.id = rdht.tag_id \
                     WHERE rdht.race_dump_id = rd.id AND t.tag_value = ?\
                     )"
                    .to_string(),
                );
                params.push(Box::new(s.clone()));
            }
            RaceDumpFilter::CaptureDate(r) => {
                if let Some(v) = &r.after {
                    clauses.push("datetime(rd.capture_time, 'localtime') >= ?".to_string());
                    params.push(Box::new(normalize_bound(v, true).unwrap_or_else(|| v.clone())));
                }
                if let Some(v) = &r.before {
                    clauses.push("datetime(rd.capture_time, 'localtime') <= ?".to_string());
                    params.push(Box::new(normalize_bound(v, false).unwrap_or_else(|| v.clone())));
                }
            }
        }
    }

    let where_clause = if clauses.is_empty() {
        String::new()
    } else {
        format!(" WHERE {}", clauses.join(" AND "))
    };

    (where_clause, params)
}

#[tauri::command]
pub fn query_race_dump_page(
    query: RaceDumpBrowserQuery,
) -> Result<PaginationResponse<RaceDumpPageItem>, String> {
    let conn = app_db::open_app_database_connection()?;

    let (where_clause, mut params) = build_race_dump_where(&query.filters);

    let count_sql = format!("SELECT COUNT(*) FROM race_dump rd{}", where_clause);
    let mut count_stmt = conn
        .prepare(&count_sql)
        .map_err(|e| format!("count prepare: {e}"))?;
    let count_refs: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let total: u32 = count_stmt
        .query_row(count_refs.as_slice(), |row| row.get::<_, i64>(0))
        .map_err(|e| format!("count query: {e}"))? as u32;

    let sort_col = match query.sort.key.as_str() {
        "participant_count" => "participant_count",
        "player_count" => "player_participant_count",
        "distance" => "rd.distance",
        "race_type" => "rd.race_type",
        "capture_time" => "rd.capture_time",
        _ => "rd.id",
    };
    let sort_dir = match query.sort.direction.as_str() {
        "asc" => "ASC",
        _ => "DESC",
    };

    let offset = (query.page.saturating_sub(1)) * query.page_size;
    params.push(Box::new(query.page_size as i64));
    params.push(Box::new(offset as i64));

    let select_sql = format!(
        r#"SELECT
            rd.id, datetime(rd.capture_time, 'localtime') AS capture_time, rd.race_type,
            rd.race_instance_id, rd.race_id,
            rd.distance, rd.track_id, rd.ground_type,
            rd.season, rd.weather, rd.ground_condition,
            rd.turn, rd.inout,
            rd.champions_id, rd.league_type, rd.round,
            COALESCE(p.cnt, 0) AS participant_count,
            COALESCE(pp.cnt, 0) AS player_participant_count,
            COALESCE(pn.names, '') AS player_participant_names,
             rdat.race_name,
             rdat.track_name
         FROM race_dump rd
         LEFT JOIN (
             SELECT race_dump_id, COUNT(*) AS cnt FROM race_dump_participant GROUP BY race_dump_id
         ) p ON p.race_dump_id = rd.id
         LEFT JOIN (
             SELECT race_dump_id, COUNT(*) AS cnt FROM race_dump_participant WHERE is_player = 1 GROUP BY race_dump_id
         ) pp ON pp.race_dump_id = rd.id
         LEFT JOIN (
             SELECT race_dump_id, GROUP_CONCAT(chara_name, '|!|') AS names FROM race_dump_participant WHERE is_player = 1 AND chara_name IS NOT NULL GROUP BY race_dump_id
         ) pn ON pn.race_dump_id = rd.id
         LEFT JOIN race_data rdat ON rdat.race_instance_id = rd.race_instance_id
         {}
         ORDER BY {} {}
         LIMIT ? OFFSET ?"#,
        where_clause, sort_col, sort_dir,
    );

    let mut select_stmt = conn
        .prepare(&select_sql)
        .map_err(|e| format!("select prepare: {e}"))?;
    let select_refs: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();

    let rows = select_stmt
        .query_map(select_refs.as_slice(), |row| {
            let names_raw: String = row.get::<_, Option<String>>(18)?.unwrap_or_default();
            let player_names: Vec<String> = if names_raw.is_empty() {
                Vec::new()
            } else {
                names_raw.split("|!|").map(|s| s.to_string()).collect()
            };
            Ok(RaceDumpPageItem {
                summary: RaceDumpSummary {
                    id: row.get(0)?,
                    capture_time: row.get(1)?,
                    race_type: row.get(2)?,
                    race_instance_id: row.get(3)?,
                    race_id: row.get(4)?,
                    distance: row.get(5)?,
                    track_id: row.get(6)?,
                    ground_type: row.get(7)?,
                    season: row.get(8)?,
                    weather: row.get(9)?,
                    ground_condition: row.get(10)?,
                    turn: row.get(11)?,
                    inout: row.get(12)?,
                    champions_id: row.get(13)?,
                    league_type: row.get(14)?,
                    round: row.get(15)?,
                    participant_count: row.get(16)?,
                    player_participant_count: row.get(17)?,
                    player_participants: player_names,
                    race_name: row.get(19)?,
                    track_name: row.get(20)?,
                    tags: Vec::new(),
                },
                race_name: row.get(19)?,
                tags: Vec::new(),
            })
        })
        .map_err(|e| format!("select query: {e}"))?;

    let mut items: Vec<RaceDumpPageItem> = Vec::new();
    for row in rows {
        items.push(row.map_err(|e| format!("row error: {e}"))?);
    }

    if !items.is_empty() {
        let dump_ids: Vec<i64> = items.iter().map(|i| i.summary.id).collect();
        let placeholders: Vec<String> = dump_ids.iter().map(|_| "?".to_string()).collect();
        let tag_sql = format!(
            "SELECT rdht.race_dump_id, t.id, t.tag_value, t.create_date \
             FROM race_dump_has_tag rdht \
             JOIN tag t ON t.id = rdht.tag_id \
             WHERE rdht.race_dump_id IN ({}) \
             ORDER BY t.tag_value",
            placeholders.join(",")
        );
        if let Ok(mut tag_stmt) = conn.prepare(&tag_sql) {
            let tag_params: Vec<Box<dyn ToSql>> = dump_ids
                .iter()
                .map(|id| Box::new(*id) as Box<dyn ToSql>)
                .collect();
            let tag_refs: Vec<&dyn ToSql> = tag_params.iter().map(|p| p.as_ref()).collect();
            if let Ok(tag_rows) = tag_stmt.query_map(tag_refs.as_slice(), |row| {
                let dump_id: i64 = row.get(0)?;
                let tag = TagRow {
                    id: row.get(1)?,
                    tag_value: row.get(2)?,
                    create_date: row.get(3)?,
                };
                Ok((dump_id, tag))
            }) {
                let mut tag_map: HashMap<i64, Vec<TagRow>> = HashMap::new();
                for row in tag_rows.flatten() {
                    tag_map.entry(row.0).or_default().push(row.1);
                }
                for item in &mut items {
                    if let Some(tags) = tag_map.remove(&item.summary.id) {
                        item.tags = tags;
                    }
                }
            }
        }
    }

    Ok(PaginationResponse {
        results: items,
        total,
        page: query.page,
        page_size: query.page_size,
    })
}

#[tauri::command]
pub fn get_race_dump_detail(
    state: State<'_, RaceDumpDetailState>,
) -> Result<shared::RaceDumpDetail, String> {
    let dump_id = state
        .0
        .lock()
        .unwrap()
        .ok_or_else(|| "no race dump selected".to_string())?;
    let conn = app_db::open_app_database_connection()?;
    let skill_storage = SkillStorage::new(app_db::open_app_database_connection()?);

    let mut stmt = conn
        .prepare(r#"
            SELECT
                rd.id, datetime(rd.capture_time, 'localtime') AS capture_time, rd.race_type, rd.race_instance_id, rd.race_id,
                rd.distance, rd.track_id, rd.ground_type, rd.season, rd.weather, rd.ground_condition,
                rd.turn, rd.inout, rd.champions_id, rd.league_type, rd.round, rd.frames, rd.events,
                rdat.race_name, rdat.track_name
            FROM race_dump rd
            LEFT JOIN race_data rdat ON rdat.race_instance_id = rd.race_instance_id
            WHERE rd.id = ?1
        "#)
        .map_err(|e| format!("prepare get_race_dump_detail failed: {e}"))?;

    let (summary, frames_raw, events_raw) = stmt
        .query_row(params![dump_id], |row| {
            let frames_text: String = row.get(16)?;
            let events_text: String = row.get(17)?;
            let frames_val: serde_json::Value =
                serde_json::from_str(&frames_text).unwrap_or(serde_json::Value::Null);
            let events_val: serde_json::Value =
                serde_json::from_str(&events_text).unwrap_or(serde_json::Value::Null);
            Ok((
                shared::RaceDumpSummary {
                    id: row.get(0)?,
                    capture_time: row.get(1)?,
                    race_type: row.get(2)?,
                    race_instance_id: row.get(3)?,
                    race_id: row.get(4)?,
                    distance: row.get(5)?,
                    track_id: row.get(6)?,
                    ground_type: row.get(7)?,
                    season: row.get(8)?,
                    weather: row.get(9)?,
                    ground_condition: row.get(10)?,
                    turn: row.get(11)?,
                    inout: row.get(12)?,
                    champions_id: row.get(13)?,
                    league_type: row.get(14)?,
                    round: row.get(15)?,
                    participant_count: 0,
                    player_participant_count: 0,
                    player_participants: Vec::new(),
                    race_name: row.get(18)?,
                    track_name: row.get(19)?,
                    tags: Vec::new(),
                },
                frames_val,
                events_val,
            ))
        })
        .map_err(|e| format!("query race_dump row failed: {e}"))?;

    let mut pstmt = conn
        .prepare(
            r#"
            SELECT
                horse_index, post_number, chara_name, is_player,
                finish_order, finish_time, running_style,
                speed, stamina, pow, guts, wiz,
                veteran_hash, viewer_id
            FROM race_dump_participant
            WHERE race_dump_id = ?1
            ORDER BY post_number
        "#,
        )
        .map_err(|e| format!("prepare participants query failed: {e}"))?;

    let participants: Vec<shared::RaceDumpParticipant> = pstmt
        .query_map(params![dump_id], |row| {
            Ok(shared::RaceDumpParticipant {
                horse_index: row.get(0)?,
                post_number: row.get(1)?,
                chara_name: row.get(2)?,
                is_player: row.get(3)?,
                finish_order: row.get(4)?,
                finish_time: row.get(5)?,
                running_style: row.get(6)?,
                speed: row.get(7)?,
                stamina: row.get(8)?,
                pow: row.get(9)?,
                guts: row.get(10)?,
                wiz: row.get(11)?,
                veteran_hash: row.get(12)?,
                viewer_id: row.get(13)?,
            })
        })
        .map_err(|e| format!("query participants failed: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("participant row error: {e}"))?;

    let participant_count = participants.len() as i64;
    let player_participant_count = participants.iter().filter(|p| p.is_player == 1).count() as i64;
    let player_participants: Vec<String> = participants
        .iter()
        .filter(|p| p.is_player == 1)
        .filter_map(|p| p.chara_name.clone())
        .collect();

    let summary = shared::RaceDumpSummary {
        participant_count,
        player_participant_count,
        player_participants,
        ..summary
    };

    let frames_raw: Vec<RawFrame> = serde_json::from_value(frames_raw.clone()).unwrap_or_default();
    let frames = frames_raw.into_iter().map(adapt_frame).collect();

    let events_raw: Vec<RawEvent> = serde_json::from_value(events_raw.clone()).unwrap_or_default();
    let events = events_raw
        .into_iter()
        .map(|e| adapt_event(&skill_storage, e))
        .collect();

    Ok(shared::RaceDumpDetail {
        summary,
        participants,
        frames,
        events,
    })
}

#[tauri::command]
pub fn get_race_dump_filter_options() -> Result<RaceDumpFilterOptions, String> {
    let conn = app_db::open_app_database_connection()?;

    let pairs = |sql: &str| -> Result<Vec<(i64, String)>, String> {
        let mut stmt = conn.prepare(sql).map_err(|e| format!("prepare: {e}"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
            })
            .map_err(|e| format!("query: {e}"))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| format!("row: {e}"))?);
        }
        Ok(result)
    };

    let race_types = pairs(
        "SELECT DISTINCT race_type, \
         CASE CAST(race_type AS INTEGER) \
           WHEN 5 THEN 'Champions' \
           WHEN 6 THEN 'Single' \
           WHEN 8 THEN 'RoomMatch' \
           WHEN 14 THEN 'TeamStadium' \
           ELSE 'Unknown' END \
         FROM race_dump ORDER BY race_type",
    )?;

    let seasons = pairs(
        "SELECT DISTINCT season, \
         CASE CAST(season AS INTEGER) \
           WHEN 1 THEN 'Spring' WHEN 2 THEN 'Summer' \
           WHEN 3 THEN 'Fall' WHEN 4 THEN 'Winter' \
           WHEN 5 THEN 'CherryBlossom' ELSE 'Random' END \
         FROM race_dump WHERE season IS NOT NULL ORDER BY season",
    )?;

    let weather_types = pairs(
        "SELECT DISTINCT weather, \
         CASE CAST(weather AS INTEGER) \
           WHEN 1 THEN 'Sunny' WHEN 2 THEN 'Rainy' \
           WHEN 3 THEN 'Snow' WHEN 4 THEN 'Cloudy' \
           WHEN 5 THEN 'Star' WHEN 6 THEN 'Firework' \
           ELSE 'None' END \
         FROM race_dump WHERE weather IS NOT NULL ORDER BY weather",
    )?;

    let ground_types = pairs(
        "SELECT DISTINCT ground_type, \
         CASE CAST(ground_type AS INTEGER) \
           WHEN 1 THEN 'Turf' WHEN 2 THEN 'Dirt' \
           ELSE 'Undefined' END \
         FROM race_dump WHERE ground_type IS NOT NULL ORDER BY ground_type",
    )?;

    let ground_conditions = pairs(
        "SELECT DISTINCT ground_condition, \
         CASE CAST(ground_condition AS INTEGER) \
           WHEN 1 THEN 'Firm' WHEN 2 THEN 'Good' \
           WHEN 3 THEN 'Soft' WHEN 4 THEN 'Heavy' \
           ELSE 'Good' END \
         FROM race_dump WHERE ground_condition IS NOT NULL ORDER BY ground_condition",
    )?;

    let characters = pairs(
        "SELECT DISTINCT cd.id, cd.name \
         FROM race_dump_participant rdp \
         JOIN veterans v ON v.hash = rdp.veteran_hash \
         JOIN trainee_data td ON td.id = v.trainee_id \
         JOIN character_data cd ON cd.id = td.character_id \
         ORDER BY cd.name",
    )?;

    let trainees = pairs(
        "SELECT DISTINCT td.id, td.name \
         FROM race_dump_participant rdp \
         JOIN veterans v ON v.hash = rdp.veteran_hash \
         JOIN trainee_data td ON td.id = v.trainee_id \
         ORDER BY td.name",
    )?;

    let tags: Vec<String> = {
        let mut stmt = conn
            .prepare(
                "SELECT DISTINCT t.tag_value \
                 FROM race_dump_has_tag rdht \
                 JOIN tag t ON t.id = rdht.tag_id \
                 ORDER BY t.tag_value",
            )
            .map_err(|e| format!("prepare tags: {e}"))?;
        let rows = stmt
            .query_map([], |row| row.get::<_, String>(0))
            .map_err(|e| format!("query tags: {e}"))?;
        let mut result = Vec::new();
        for row in rows {
            result.push(row.map_err(|e| format!("row: {e}"))?);
        }
        result
    };

    let (distance_min, distance_max) = {
        let mut stmt = conn
            .prepare("SELECT COALESCE(MIN(distance), 0), COALESCE(MAX(distance), 0) FROM race_dump WHERE distance IS NOT NULL")
            .map_err(|e| format!("prepare distance range: {e}"))?;
        stmt.query_row([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?)))
            .map_err(|e| format!("query distance range: {e}"))?
    };

    Ok(RaceDumpFilterOptions {
        race_types,
        seasons,
        weather_types,
        ground_types,
        ground_conditions,
        characters,
        trainees,
        tags,
        distance_min,
        distance_max,
    })
}

#[tauri::command]
pub async fn open_race_dump_detail_window(app: AppHandle, id: i64) -> Result<(), String> {
    {
        let state = app.state::<RaceDumpDetailState>();
        *state.0.lock().unwrap() = Some(id);
    }

    let label = "race-dump-detail";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    tauri::WebviewWindowBuilder::new(&app, label, tauri::WebviewUrl::App("index.html".into()))
        .title("Race Dump Detail")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn emit_race_dump_tags_changed(app: AppHandle, race_dump_id: i64) -> Result<(), String> {
    let payload = json!({ "raceDumpId": race_dump_id });
    let _ = app.emit("race-dump-tags-changed", payload);
    Ok(())
}

#[tauri::command]
pub fn return_race_dump_selection(app: AppHandle, id: i64) -> Result<(), String> {
    let payload = json!({ "id": id });
    let _ = app.emit("race-dump-selected", payload);
    if let Some(win) = app.get_webview_window("race-dump") {
        let _ = win.close();
    }
    Ok(())
}

fn await_worker_frame(
    state: &WorkerState,
    request_id: u64,
    receiver: Receiver<Vec<u8>>,
    timeout_ms: u64,
) -> Result<Vec<u8>, String> {
    receiver
        .recv_timeout(Duration::from_millis(timeout_ms))
        .map_err(|_| {
            state.clear_pending(request_id);
            format!("timed out waiting for worker response for request id {request_id}")
        })
}

fn adapt_frame(frame: RawFrame) -> ReplayFrame {
    ReplayFrame {
        time: frame.time,
        horse_data_array: frame
            .horse_data_array
            .into_iter()
            .map(adapt_horse_data)
            .collect(),
    }
}

fn adapt_horse_data(horse: RawHorseData) -> ReplayHorseData {
    ReplayHorseData {
        distance: horse.distance,
        lane_position: horse.lane_position,
        speed: horse.speed,
        hp: horse.hp,
        is_tempted: horse.temptation_mode != 0,
        is_blocked: horse.block_front_horse_index != -1 && horse.block_front_horse_index != 255,
    }
}

fn adapt_event(skill_storage: &SkillStorage, event: RawEvent) -> ReplayEvent {
    let idx = event.param.get(0).copied();

    let data = match event.event_type {
        ReplayEventData::TYPE_ID_SCORE => Some(ReplayEventData::Score),
        ReplayEventData::TYPE_ID_SKILL => {
            if let Some(skill_id) = event.param.get(1).copied() {
                skill_storage
                    .get_by_id(skill_id)
                    .map(|skill| ReplayEventData::Skill(skill.name))
            } else {
                None
            }
        }
        ReplayEventData::TYPE_ID_COMP_TOP => Some(ReplayEventData::CompTop),
        ReplayEventData::TYPE_ID_COMP_FIGHT => Some(ReplayEventData::CompFight),
        ReplayEventData::TYPE_ID_REL_CONS => Some(ReplayEventData::RelCons),
        ReplayEventData::TYPE_ID_STAM_BRK => Some(ReplayEventData::StamBrk),
        ReplayEventData::TYPE_ID_COMP_SPURT => Some(ReplayEventData::CompSpurt),
        ReplayEventData::TYPE_ID_STAM_KEEP => Some(ReplayEventData::StamKeep),
        ReplayEventData::TYPE_ID_SEC_LEAD => Some(ReplayEventData::SecLead),
        _ => None,
    };

    ReplayEvent {
        frame_time: event.frame_time,
        horse_idx: idx,
        event_data: data,
    }
}

#[tauri::command]
pub async fn export_race_dump_hakuraku(
    state: State<'_, RaceDumpDetailState>,
) -> Result<String, String> {
    let dump_id = state
        .0
        .lock()
        .unwrap()
        .ok_or_else(|| "no race dump selected".to_string())?;

    tauri::async_runtime::spawn_blocking(move || {
        let conn = app_db::open_app_database_connection().map_err(|e| format!("open db: {e}"))?;

        let (json, capture_time) = build_hakuraku_json_for_dump(&conn, dump_id)?;

        let race_name: Option<String> = conn
            .prepare(
                "SELECT rdat.race_name FROM race_dump rd \
                 LEFT JOIN race_data rdat ON rdat.race_instance_id = rd.race_instance_id \
                 WHERE rd.id = ?1",
            )
            .ok()
            .and_then(|mut s| s.query_row(params![dump_id], |row| row.get(0)).ok())
            .flatten();

        let player_names: String = conn
            .prepare(
                "SELECT GROUP_CONCAT(chara_name, '_') FROM race_dump_participant \
                 WHERE race_dump_id = ?1 AND is_player = 1 AND chara_name IS NOT NULL",
            )
            .ok()
            .and_then(|mut s| s.query_row(params![dump_id], |row| row.get(0)).ok())
            .unwrap_or_default();

        let date = &capture_time[0..10];
        let mut parts: Vec<String> = vec![dump_id.to_string()];
        if let Some(ref rn) = race_name {
            if !rn.is_empty() {
                parts.push(sanitize_filename(rn));
            }
        }
        if !player_names.is_empty() {
            parts.push(sanitize_filename(&player_names));
        }
        parts.push(date.to_string());
        let default_name = format!("{}.json", parts.join("_"));

        let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .set_file_name(&default_name)
            .save_file()
        else {
            return Ok("canceled".to_string());
        };

        std::fs::write(&path, &json).map_err(|e| format!("write: {e}"))?;

        Ok(path.display().to_string())
    })
    .await
    .map_err(|e| format!("join export task: {e}"))?
}

#[tauri::command]
pub async fn export_race_dumps_batch(query: RaceDumpBrowserQuery) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = app_db::open_app_database_connection().map_err(|e| format!("open db: {e}"))?;

        let (where_clause, params) = build_race_dump_where(&query.filters);
        let query_params: Vec<&dyn ToSql> = params.iter().map(|p| p.as_ref()).collect();

        let sql = format!(
            r#"SELECT
                rd.id, rd.capture_time,
                rdat.race_name,
                COALESCE(pn.names, '') AS player_names
            FROM race_dump rd
            LEFT JOIN race_data rdat ON rdat.race_instance_id = rd.race_instance_id
            LEFT JOIN (
                SELECT race_dump_id, GROUP_CONCAT(chara_name, '_') AS names
                FROM race_dump_participant
                WHERE is_player = 1 AND chara_name IS NOT NULL
                GROUP BY race_dump_id
            ) pn ON pn.race_dump_id = rd.id
            {} ORDER BY rd.id"#,
            where_clause
        );
        let mut stmt = conn.prepare(&sql).map_err(|e| format!("prepare: {e}"))?;
        let rows: Vec<(i64, String, Option<String>, String)> = stmt
            .query_map(query_params.as_slice(), |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    row.get::<_, String>(3)?,
                ))
            })
            .map_err(|e| format!("query: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("collect: {e}"))?;

        if rows.is_empty() {
            return Ok("No matching race dumps found.".to_string());
        }

        let Some(dir) = rfd::FileDialog::new().pick_folder() else {
            return Ok("canceled".to_string());
        };

        let mut exported = 0usize;
        for (id, caps, race_name, player_names) in &rows {
            let (json, _) = build_hakuraku_json_for_dump(&conn, *id)?;

            let date = &caps[0..10];
            let mut parts: Vec<String> = vec![id.to_string()];

            if let Some(rn) = race_name {
                if !rn.is_empty() {
                    parts.push(sanitize_filename(rn));
                }
            }
            if !player_names.is_empty() {
                parts.push(sanitize_filename(player_names));
            }
            parts.push(date.to_string());

            let filename = format!("{}.json", parts.join("_"));
            let filepath = dir.join(&filename);

            std::fs::write(&filepath, &json)
                .map_err(|e| format!("write {}: {e}", filepath.display()))?;
            exported += 1;
        }

        Ok(format!("Exported {} files to {}", exported, dir.display()))
    })
    .await
    .map_err(|e| format!("join export task: {e}"))?
}

fn build_hakuraku_json_for_dump(
    conn: &rusqlite::Connection,
    dump_id: i64,
) -> Result<(String, String), String> {
    let mut rd_stmt = conn
        .prepare(
            r#"SELECT
                    id, capture_time, race_type, race_instance_id, race_id,
                    season, weather, ground_condition, distance, track_id, ground_type,
                    turn, inout, champions_id, league_type, round,
                    sim_data_base64, race_course_set_id, float_lane_max
                FROM race_dump WHERE id = ?1"#,
        )
        .map_err(|e| format!("prepare: {e}"))?;

    let (
        rd_id,
        capture_time,
        race_type,
        _race_instance_id,
        _race_id,
        season,
        weather,
        ground_condition,
        distance,
        track_id,
        ground_type_val,
        turn,
        inout,
        _champions_id,
        _league_type,
        _round,
        sim_data_base64,
        race_course_set_id,
        float_lane_max,
    ) = rd_stmt
        .query_row(params![dump_id], |row| {
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, String>(1)?,
                row.get::<_, i64>(2)?,
                row.get::<_, Option<i64>>(3)?,
                row.get::<_, Option<i64>>(4)?,
                row.get::<_, Option<i64>>(5)?,
                row.get::<_, Option<i64>>(6)?,
                row.get::<_, Option<i64>>(7)?,
                row.get::<_, Option<i64>>(8)?,
                row.get::<_, Option<i64>>(9)?,
                row.get::<_, Option<i64>>(10)?,
                row.get::<_, Option<i64>>(11)?,
                row.get::<_, Option<i64>>(12)?,
                row.get::<_, Option<i64>>(13)?,
                row.get::<_, Option<i64>>(14)?,
                row.get::<_, Option<i64>>(15)?,
                row.get::<_, String>(16)?,
                row.get::<_, Option<i64>>(17)?,
                row.get::<_, Option<i64>>(18)?,
            ))
        })
        .map_err(|e| format!("query race_dump: {e}"))?;

    let mut p_stmt = conn
        .prepare(
            r#"SELECT
                    horse_index, post_number, chara_name, is_player,
                    finish_order, finish_time, finish_diff_time, running_style,
                    speed, stamina, pow, guts, wiz,
                    veteran_hash, popularity, team_id, response_horse_data
                FROM race_dump_participant
                WHERE race_dump_id = ?1
                ORDER BY horse_index"#,
        )
        .map_err(|e| format!("prepare participants: {e}"))?;

    let participants_raw: Vec<_> = p_stmt
        .query_map(params![dump_id], |row| {
            let response_data_str: String = row.get::<_, String>(16)?;
            let response_data: serde_json::Value =
                serde_json::from_str(&response_data_str).unwrap_or(serde_json::Value::Null);
            Ok((
                row.get::<_, i64>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, Option<String>>(2)?,
                row.get::<_, i64>(3)?,
                row.get::<_, Option<i64>>(4)?,
                row.get::<_, Option<f64>>(5)?,
                row.get::<_, Option<f64>>(6)?,
                row.get::<_, Option<i64>>(7)?,
                response_data,
                row.get::<_, Option<i64>>(14)?,
                row.get::<_, i64>(15)?,
                row.get::<_, Option<i64>>(13)?,
            ))
        })
        .map_err(|e| format!("query participants: {e}"))?
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect participants: {e}"))?;

    let participants: Vec<serde_json::Value> = participants_raw
        .into_iter()
        .map(|(hi, pn, cn, ip, fo, ft, fdt, rs, rd, pop, tid, vhash)| {
            let tcd = vhash
                .and_then(|h| build_trained_chara_data_for_hash(&conn, h).ok())
                .unwrap_or(serde_json::Value::Null);
            build_hakuraku_horse(hi, pn, cn, ip, fo, ft, fdt, rs, rd, pop, tid, tcd)
        })
        .collect();

    let player_participants: Vec<serde_json::Value> = participants
        .iter()
        .filter(|h| h.get("_is_player").and_then(|v| v.as_i64()).unwrap_or(0) == 1)
        .cloned()
        .collect();

    let player_horse_index = player_participants
        .first()
        .and_then(|h| h.get("horseIndex").and_then(|v| v.as_i64()))
        .unwrap_or(0);

    let num_horses = participants.len() as i64;
    let post_max = participants
        .iter()
        .filter_map(|h| h.get("postNumber").and_then(|v| v.as_i64()))
        .max()
        .unwrap_or(8);

    let horse_index_by_popularity: Vec<i64> = (0..num_horses).collect();

    let course_furlong_num = distance.map(|d| d / 200).unwrap_or(6);

    let distance_type = match distance {
        Some(d) if d <= 1400 => "Short",
        Some(d) if d <= 1800 => "Mile",
        Some(d) if d <= 2400 => "Middle",
        _ => "Long",
    };

    let turn_label = match turn {
        Some(4) => "Right",
        _ => "Left",
    };

    let mut root = serde_json::Map::new();

    root.insert(
        "<RaceType>k__BackingField".into(),
        serde_json::Value::String(race_type_string(race_type)),
    );
    root.insert("<IsExistPlayerRace>k__BackingField".into(), true.into());
    root.insert("<IsExistGhostRace>k__BackingField".into(), false.into());
    root.insert("<IsExistFollowRace>k__BackingField".into(), false.into());
    root.insert(
        "<IsMultiplePlayerRace>k__BackingField".into(),
        (num_horses > 1).into(),
    );
    root.insert("<RandomSeed>k__BackingField".into(), 0.into());
    root.insert("<SingleRaceProgramId>k__BackingField".into(), 0.into());
    root.insert(
        "<IsSingleRaceExportRetryEnable>k__BackingField".into(),
        false.into(),
    );
    root.insert("<SingleRaceRetryCount>k__BackingField".into(), 0.into());
    root.insert("<OpponentEvaluate>k__BackingField".into(), 0.into());
    root.insert("<SelfEvaluate>k__BackingField".into(), 0.into());
    root.insert("<SupportCardScoreBonus>k__BackingField".into(), 0.into());
    root.insert("<ScoreCalcTeamId>k__BackingField".into(), 0.into());
    root.insert("<RaceNo>k__BackingField".into(), 0.into());

    let mut race_course_set = serde_json::Map::new();
    race_course_set.insert("Id".into(), race_course_set_id.unwrap_or(0).into());
    race_course_set.insert("RaceTrackId".into(), track_id.unwrap_or(0).into());
    race_course_set.insert("Distance".into(), distance.unwrap_or(0).into());
    race_course_set.insert("Ground".into(), ground_type_val.unwrap_or(1).into());
    race_course_set.insert("Inout".into(), inout.unwrap_or(1).into());
    race_course_set.insert("Turn".into(), turn.unwrap_or(2).into());
    race_course_set.insert("FenceSet".into(), 0.into());
    race_course_set.insert(
        "FloatLaneMax".into(),
        float_lane_max.unwrap_or(15000).into(),
    );
    race_course_set.insert("CourseSetStatusId".into(), 8.into());
    race_course_set.insert("FinishTimeMin".into(), 908000.into());
    race_course_set.insert("FinishTimeMinRandomRange".into(), 10000.into());
    race_course_set.insert("FinishTimeMax".into(), 950000.into());
    race_course_set.insert("FinishTimeMaxRandomRange".into(), 10000.into());
    root.insert(
        "<RaceCourseSet>k__BackingField".into(),
        serde_json::Value::Object(race_course_set),
    );

    let mut fence_set = serde_json::Map::new();
    fence_set.insert("Id".into(), race_course_set_id.unwrap_or(0).into());
    for i in 1..=8 {
        fence_set.insert(format!("Fence{i}"), (if i <= 2 { 1 } else { 0 }).into());
    }
    root.insert(
        "<FenceSet>k__BackingField".into(),
        serde_json::Value::Object(fence_set),
    );

    let mut race_track = serde_json::Map::new();
    race_track.insert("Id".into(), track_id.unwrap_or(10006).into());
    race_track.insert("InitialLaneType".into(), 1.into());
    race_track.insert("EnableHalfGate".into(), 1.into());
    race_track.insert("HorseNumGateVariation".into(), 0.into());
    race_track.insert("TurfVisionType".into(), 1.into());
    race_track.insert("FootsmokeColorType".into(), 1.into());
    race_track.insert("Area".into(), 2.into());
    race_track.insert("FlagType".into(), 0.into());
    race_track.insert("GatePanelType".into(), 1.into());
    race_track.insert("GateLampType".into(), 1.into());
    root.insert(
        "<RaceTrack>k__BackingField".into(),
        serde_json::Value::Object(race_track),
    );

    root.insert("<GoalGate>k__BackingField".into(), 0.into());
    root.insert("<GoalGateFlower>k__BackingField".into(), 0.into());
    root.insert(
        "<InitialLaneType>k__BackingField".into(),
        "ExtraSpaceAfter9".into(),
    );
    root.insert(
        "<RotationCategory>k__BackingField".into(),
        turn_label.into(),
    );
    root.insert(
        "<GroundTypeAvailable>k__BackingField".into(),
        "TurfAndDirt".into(),
    );
    root.insert(
        "<CourseSectionDistance>k__BackingField".into(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(
                distance.unwrap_or(1600) as f64 / course_furlong_num as f64,
            )
            .unwrap_or(serde_json::Number::from(200)),
        ),
    );

    root.insert(
        "<CourseDistanceType>k__BackingField".into(),
        distance_type.into(),
    );
    root.insert(
        "<CourseFurlongNum>k__BackingField".into(),
        course_furlong_num.into(),
    );
    root.insert("<IsHalfGate>k__BackingField".into(), false.into());
    root.insert(
        "<IsHorseNumVariationGate>k__BackingField".into(),
        false.into(),
    );
    root.insert("<TurfVisionType>k__BackingField".into(), "URA".into());

    // String enum mappings
    root.insert(
        "<GroundCondition>k__BackingField".into(),
        serde_json::Value::String(ground_condition_string(ground_condition.unwrap_or(1))),
    );
    root.insert(
        "<Weather>k__BackingField".into(),
        serde_json::Value::String(weather_string(weather.unwrap_or(1))),
    );
    root.insert(
        "<Season>k__BackingField".into(),
        serde_json::Value::String(season_string(season.unwrap_or(1))),
    );

    root.insert("<Time>k__BackingField".into(), "Daytime".into());
    root.insert("_baseSpeed".into(), (-1.0f64).into());
    root.insert("<BorderTimeScaled>k__BackingField".into(), 0.0.into());
    root.insert(
        "<ChallengeMatchDifficulty>k__BackingField".into(),
        "Easy".into(),
    );
    root.insert("<NumRaceHorses>k__BackingField".into(), num_horses.into());
    root.insert("<PostNumberMax>k__BackingField".into(), post_max.into());
    root.insert("_playerHorseIndex".into(), player_horse_index.into());
    root.insert(
        "<PlayerTeamMemberArray>k__BackingField".into(),
        serde_json::Value::Array(player_participants.clone()),
    );
    root.insert(
        "<RaceHorse>k__BackingField".into(),
        serde_json::Value::Array(participants.clone()),
    );

    root.insert(
        "<SimDataBase64>k__BackingField".into(),
        serde_json::Value::String(sim_data_base64),
    );

    if let Some(first) = participants.first().cloned() {
        root.insert(
            "<PlayerTeamTopFinishOrderHorse>k__BackingField".into(),
            first,
        );
    }

    root.insert(
        "<HorseIndexByPopularity>k__BackingField".into(),
        serde_json::Value::Array(
            horse_index_by_popularity
                .into_iter()
                .map(|v| v.into())
                .collect(),
        ),
    );

    root.insert("horseACT_version".into(), "1.1.3".into());

    let mut root_value = serde_json::Value::Object(root);
    normalize_hakuraku_json(&mut root_value);

    if let serde_json::Value::Object(ref mut map) = root_value {
        if let Some(weather_val) = map.get("weather") {
            map.insert("<Weather>k__BackingField".into(), weather_val.clone());
        }
        if let Some(race_horse_val) = map.get("raceHorse") {
            map.insert("<RaceHorse>k__BackingField".into(), race_horse_val.clone());
        }
    }

    let json = serde_json::to_string_pretty(&root_value).map_err(|e| format!("serialize: {e}"))?;

    Ok((json, capture_time))
}

fn build_trained_chara_data_for_hash(
    conn: &rusqlite::Connection,
    veteran_hash: i64,
) -> Result<serde_json::Value, String> {
    let mut tcd = serde_json::Map::new();

    let support_cards: Vec<serde_json::Value> = {
        let mut stmt = conn
            .prepare("SELECT position, support_card_id, exp, limit_break_count FROM veteran_support_card WHERE veteran_hash = ?1 ORDER BY position")
            .map_err(|e| format!("prepare support cards: {e}"))?;
        let mapped = stmt
            .query_map([veteran_hash], |row| {
                let mut card = serde_json::Map::new();
                card.insert(
                    "<Position>k__BackingField".into(),
                    serde_json::Value::from(row.get::<_, i64>(0)?),
                );
                card.insert(
                    "<SupportCardId>k__BackingField".into(),
                    serde_json::Value::from(row.get::<_, i64>(1)?),
                );
                card.insert(
                    "<Exp>k__BackingField".into(),
                    serde_json::Value::from(row.get::<_, i64>(2)?),
                );
                card.insert(
                    "<LimitBreakCount>k__BackingField".into(),
                    serde_json::Value::from(row.get::<_, i64>(3)?),
                );
                Ok(serde_json::Value::Object(card))
            })
            .map_err(|e| format!("query support cards: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("collect support cards: {e}"))?
    };
    tcd.insert(
        "<SupportCardArray>k__BackingField".into(),
        serde_json::Value::Array(support_cards),
    );

    let (parent_a, parent_b): (Option<i64>, Option<i64>) = {
        let mut stmt = conn
            .prepare("SELECT parent_a, parent_b FROM veterans WHERE hash = ?1")
            .map_err(|e| format!("prepare veteran parents: {e}"))?;
        stmt.query_row([veteran_hash], |row| {
            Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, Option<i64>>(1)?))
        })
        .map_err(|e| format!("query veteran parents: {e}"))?
    };

    let mut succession_items: Vec<serde_json::Value> = Vec::new();

    let build_entry = |conn: &rusqlite::Connection,
                       parent_hash: i64,
                       position_id: i64|
     -> Result<serde_json::Value, String> {
        let parent_row: (i64, i64, i64) = {
            let mut stmt = conn
                .prepare("SELECT trainee_id, rank, rarity FROM parents WHERE hash = ?1")
                .map_err(|e| format!("prepare parent: {e}"))?;
            stmt.query_row([parent_hash], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| format!("query parent {parent_hash}: {e}"))?
        };

        let factor_ids: Vec<serde_json::Value> = {
            let mut stmt = conn
                .prepare("SELECT spark_id FROM parent_has_spark WHERE parent_hash = ?1")
                .map_err(|e| format!("prepare parent sparks: {e}"))?;
            let mapped = stmt
                .query_map([parent_hash], |row| {
                    Ok(serde_json::Value::from(row.get::<_, i64>(0)?))
                })
                .map_err(|e| format!("query parent sparks: {e}"))?;
            mapped
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("collect parent sparks: {e}"))?
        };

        let mut entry = serde_json::Map::new();
        entry.insert("_positionId".into(), serde_json::Value::from(position_id));
        entry.insert(
            "<CardId>k__BackingField".into(),
            serde_json::Value::from(parent_row.0),
        );
        entry.insert("_rank".into(), serde_json::Value::from(parent_row.1));
        entry.insert(
            "<FactorDataArray>k__BackingField".into(),
            serde_json::Value::Array(factor_ids),
        );
        Ok(serde_json::Value::Object(entry))
    };

    let load_grandparents = |conn: &rusqlite::Connection,
                             parent_hash: i64|
     -> Result<(Option<i64>, Option<i64>), String> {
        let mut stmt = conn
            .prepare("SELECT parent_a, parent_b FROM parents WHERE hash = ?1")
            .map_err(|e| format!("prepare grandparent: {e}"))?;
        stmt.query_row([parent_hash], |row| {
            Ok((row.get::<_, Option<i64>>(0)?, row.get::<_, Option<i64>>(1)?))
        })
        .map_err(|e| format!("query grandparent {parent_hash}: {e}"))
    };

    if let Some(pa_hash) = parent_a {
        if let Ok(entry) = build_entry(conn, pa_hash, 10) {
            succession_items.push(entry);
        }
        if let Ok((gpa, gpb)) = load_grandparents(conn, pa_hash) {
            if let Some(gp_hash) = gpa {
                if let Ok(entry) = build_entry(conn, gp_hash, 11) {
                    succession_items.push(entry);
                }
            }
            if let Some(gp_hash) = gpb {
                if let Ok(entry) = build_entry(conn, gp_hash, 12) {
                    succession_items.push(entry);
                }
            }
        }
    }
    if let Some(pb_hash) = parent_b {
        if let Ok(entry) = build_entry(conn, pb_hash, 20) {
            succession_items.push(entry);
        }
        if let Ok((gpa, gpb)) = load_grandparents(conn, pb_hash) {
            if let Some(gp_hash) = gpa {
                if let Ok(entry) = build_entry(conn, gp_hash, 21) {
                    succession_items.push(entry);
                }
            }
            if let Some(gp_hash) = gpb {
                if let Ok(entry) = build_entry(conn, gp_hash, 22) {
                    succession_items.push(entry);
                }
            }
        }
    }

    let mut scl = serde_json::Map::new();
    let item_count = succession_items.len() as i64;
    scl.insert("_items".into(), serde_json::Value::Array(succession_items));
    scl.insert("_size".into(), serde_json::Value::from(item_count));
    scl.insert("_version".into(), serde_json::Value::from(1i64));
    tcd.insert(
        "<SuccessionCharaList>k__BackingField".into(),
        serde_json::Value::Object(scl),
    );

    Ok(serde_json::Value::Object(tcd))
}

fn build_hakuraku_horse(
    horse_index: i64,
    post_number: i64,
    chara_name: Option<String>,
    is_player: i64,
    finish_order: Option<i64>,
    finish_time: Option<f64>,
    finish_diff_time: Option<f64>,
    running_style: Option<i64>,
    response_data: serde_json::Value,
    popularity: Option<i64>,
    team_id: i64,
    trained_chara_data: serde_json::Value,
) -> serde_json::Value {
    let mut horse = serde_json::Map::new();
    horse.insert("horseIndex".into(), horse_index.into());
    horse.insert("postNumber".into(), post_number.into());
    horse.insert("charaId".into(), 0.into());
    horse.insert(
        "<charaName>k__BackingField".into(),
        chara_name.unwrap_or_default().into(),
    );
    horse.insert("FinishOrder".into(), finish_order.unwrap_or(0).into());
    horse.insert(
        "FinishTimeRaw".into(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(finish_time.unwrap_or(0.0))
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );
    horse.insert("FinishTimeScaled".into(), 0.0.into());
    horse.insert(
        "FinishDiffTimeFromPrev".into(),
        serde_json::Value::Number(
            serde_json::Number::from_f64(finish_diff_time.unwrap_or(0.0))
                .unwrap_or(serde_json::Number::from(0)),
        ),
    );

    let mut race_param = serde_json::Map::new();
    race_param.insert("<RawSpeed>k__BackingField".into(), 0.into());
    race_param.insert("<RawStamina>k__BackingField".into(), 0.into());
    race_param.insert("<RawPow>k__BackingField".into(), 0.into());
    race_param.insert("<RawGuts>k__BackingField".into(), 0.into());
    race_param.insert("<RawWiz>k__BackingField".into(), 0.into());
    race_param.insert("<BaseSpeed>k__BackingField".into(), 0.0.into());
    race_param.insert("<BaseStamina>k__BackingField".into(), 0.0.into());
    race_param.insert("<BasePow>k__BackingField".into(), 0.0.into());
    race_param.insert("<BaseGuts>k__BackingField".into(), 0.0.into());
    race_param.insert("<BaseWiz>k__BackingField".into(), 0.0.into());
    race_param.insert("<Motivation>k__BackingField".into(), "Max".into());
    race_param.insert("<MotivationCoef>k__BackingField".into(), 1.04.into());
    horse.insert("_raceParam".into(), serde_json::Value::Object(race_param));

    let mut resp_data = if response_data.is_object() {
        response_data
    } else {
        serde_json::Value::Object(serde_json::Map::new())
    };
    if let serde_json::Value::Object(ref mut rmap) = resp_data {
        rmap.entry("speed".to_string()).or_insert(0.into());
        rmap.entry("stamina".to_string()).or_insert(0.into());
        rmap.entry("pow".to_string()).or_insert(0.into());
        rmap.entry("guts".to_string()).or_insert(0.into());
        rmap.entry("wiz".to_string()).or_insert(0.into());
        rmap.entry("running_style".to_string())
            .or_insert(running_style.unwrap_or(4).into());
        rmap.entry("popularity".to_string())
            .or_insert(popularity.unwrap_or(1).into());
        rmap.entry("team_id".to_string()).or_insert(team_id.into());
        rmap.entry("team_member_id".to_string())
            .or_insert((if is_player == 1 { 1 } else { 0 }).into());
        rmap.entry("motivation".to_string()).or_insert(5.into());
        rmap.entry("rank_score".to_string()).or_insert(0.into());
        rmap.entry("single_mode_win_count".to_string())
            .or_insert(0.into());
        rmap.entry("final_grade".to_string()).or_insert(0.into());
        rmap.entry("rarity".to_string()).or_insert(3.into());
        rmap.entry("talent_level".to_string()).or_insert(3.into());
        let card_id_val = rmap.get("card_id").and_then(|v| v.as_i64()).unwrap_or(0);
        rmap.entry("chara_id".to_string()).or_insert_with(|| {
            if card_id_val > 0 {
                serde_json::Value::from(card_id_val / 100)
            } else {
                0.into()
            }
        });
        rmap.entry("trained_chara_id".to_string())
            .or_insert(0.into());
        rmap.entry("frame_order".to_string())
            .or_insert((horse_index + 1).into());
        rmap.entry("owner_trainer_name".to_string())
            .or_insert("".into());
        rmap.entry("mob_id".to_string()).or_insert(0.into());
        rmap.entry("race_dress_id".to_string()).or_insert(0.into());
        rmap.entry("chara_color_type".to_string())
            .or_insert(0.into());
        rmap.entry("item_id_array".to_string())
            .or_insert(serde_json::Value::Array(vec![]));
        rmap.entry("win_saddle_id_array".to_string())
            .or_insert(serde_json::Value::Array(vec![]));
        rmap.entry("race_result_array".to_string())
            .or_insert(serde_json::Value::Array(vec![]));
        rmap.entry("motivation_change_flag".to_string())
            .or_insert(0.into());
        rmap.entry("frame_order_change_flag".to_string())
            .or_insert(0.into());
        rmap.entry("team_rank".to_string()).or_insert(0.into());
        rmap.entry("skill_array".to_string())
            .or_insert(serde_json::Value::Array(vec![]));
        rmap.entry("popularity_mark_rank_array".to_string())
            .or_insert(serde_json::json!([1, 1, 9]));
        rmap.entry("proper_distance_short".to_string())
            .or_insert(7.into());
        rmap.entry("proper_distance_mile".to_string())
            .or_insert(7.into());
        rmap.entry("proper_distance_middle".to_string())
            .or_insert(6.into());
        rmap.entry("proper_distance_long".to_string())
            .or_insert(6.into());
        rmap.entry("proper_running_style_nige".to_string())
            .or_insert(1.into());
        rmap.entry("proper_running_style_senko".to_string())
            .or_insert(6.into());
        rmap.entry("proper_running_style_sashi".to_string())
            .or_insert(7.into());
        rmap.entry("proper_running_style_oikomi".to_string())
            .or_insert(7.into());
        rmap.entry("proper_ground_turf".to_string())
            .or_insert(7.into());
        rmap.entry("proper_ground_dirt".to_string())
            .or_insert(1.into());
    }
    horse.insert("_responseHorseData".into(), resp_data);

    horse.insert(
        "<Popularity>k__BackingField".into(),
        popularity.unwrap_or(1).into(),
    );
    horse.insert("<PopularityRankLeft>k__BackingField".into(), 0.into());
    horse.insert("<PopularityRankCenter>k__BackingField".into(), 0.into());
    horse.insert("<PopularityRankRight>k__BackingField".into(), 0.into());
    horse.insert("_gateInPopularity".into(), 0.into());
    horse.insert("<Rarity>k__BackingField".into(), "Rare3".into());
    horse.insert(
        "<TrainerName>k__BackingField".into(),
        serde_json::Value::Null,
    );
    horse.insert("IsGhost".into(), false.into());
    horse.insert(
        "<Defeat>k__BackingField".into(),
        (finish_order == Some(0))
            .then_some("Win")
            .unwrap_or("Lose")
            .into(),
    );
    horse.insert("<RaceDressId>k__BackingField".into(), 0.into());
    horse.insert("<RaceDressIdWithOption>k__BackingField".into(), 0.into());
    horse.insert("<RunningType>k__BackingField".into(), "Base".into());
    horse.insert("<ActiveProperDistance>k__BackingField".into(), "A".into());
    horse.insert("<ActiveProperGroundType>k__BackingField".into(), "A".into());
    horse.insert("<MobId>k__BackingField".into(), 0.into());
    horse.insert("<FinishOrderRawScore>k__BackingField".into(), 0.into());

    let mut race_record = serde_json::Map::new();
    race_record.insert("<IsUndefeated>k__BackingField".into(), false.into());
    let mut items = serde_json::Map::new();
    items.insert("_items".into(), serde_json::Value::Array(vec![]));
    items.insert("_size".into(), 0.into());
    items.insert("_version".into(), 1.into());
    items.insert("_syncRoot".into(), 0.into());
    race_record.insert(
        "<WinRaceInstanceIdList>k__BackingField".into(),
        serde_json::Value::Object(items.clone()),
    );
    race_record.insert(
        "_raceInstanceIdList".into(),
        serde_json::Value::Object(items),
    );
    horse.insert("_raceRecord".into(), serde_json::Value::Object(race_record));

    horse.insert(
        "<TrainedCharaData>k__BackingField".into(),
        trained_chara_data,
    );
    horse.insert("_is_player".into(), is_player.into());

    serde_json::Value::Object(horse)
}

fn normalize_hakuraku_key(raw: &str) -> String {
    let name = raw.trim_start_matches('_');
    if name.is_empty() {
        return String::new();
    }
    let name = if let Some(inner) = name
        .strip_prefix('<')
        .and_then(|s| s.strip_suffix(">k__BackingField"))
    {
        inner.trim_start_matches('_')
    } else {
        name
    };
    if name.is_empty() {
        return String::new();
    }
    let mut chars = name.chars();
    let first = chars.next().unwrap();
    first.to_lowercase().chain(chars).collect()
}

fn normalize_hakuraku_json(value: &mut serde_json::Value) {
    match value {
        serde_json::Value::Array(items) => {
            for item in items {
                normalize_hakuraku_json(item);
            }
        }
        serde_json::Value::Object(map) => {
            let old = std::mem::take(map);
            for (key, mut child) in old {
                if key == "<TrainedCharaData>k__BackingField" && !child.is_null() {
                    map.insert(key, child);
                    continue;
                }
                normalize_hakuraku_json(&mut child);
                map.insert(normalize_hakuraku_key(&key), child);
            }
        }
        _ => {}
    }
}
fn race_type_string(rt: i64) -> String {
    match rt {
        5 => "Champions",
        6 => "Standard",
        8 => "RoomMatch",
        14 => "TeamStadium",
        _ => "Unknown",
    }
    .into()
}

fn season_string(s: i64) -> String {
    match s {
        1 => "Spring",
        2 => "Summer",
        3 => "Fall",
        4 => "Winter",
        5 => "CherryBlossom",
        _ => "Unknown",
    }
    .into()
}

fn weather_string(w: i64) -> String {
    match w {
        1 => "Sunny",
        2 => "Rainy",
        3 => "Snow",
        4 => "Cloudy",
        5 => "Star",
        6 => "Firework",
        _ => "Unknown",
    }
    .into()
}

fn ground_condition_string(g: i64) -> String {
    match g {
        1 => "Good",
        2 => "Soft",
        3 => "Hard",
        4 => "Bad",
        _ => "Good",
    }
    .into()
}

fn sanitize_filename(raw: &str) -> String {
    let safe: String = raw
        .chars()
        .map(|c| {
            if c.is_alphanumeric() || c == ' ' || c == '-' {
                c
            } else {
                '_'
            }
        })
        .collect();
    safe.trim_matches('_').replace(' ', "_")
}
