use crate::db::{app_db, schema::VeteranSchema};
use crate::storage::veterans::VeteranStore;
use crate::veterans;
use crate::worker::WorkerState;
use chrono::Utc;
use honse_worker::protocol::{write_msgpack_request_framed, WorkerCommand, WorkerRequest};
use shared::GatherVeteransResult;
use std::collections::HashSet;
use std::sync::mpsc::Receiver;
use std::sync::Mutex;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

const DEFAULT_VETERAN_TIMEOUT_MS: u64 = 15_000;

#[tauri::command]
pub async fn gather_veterans(
    app: AppHandle,
    mut request: WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<GatherVeteransResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();
        validate_gather_veterans_request(&request)?;

        let request_id = request.id.unwrap_or_else(|| state.next_request_id());
        request.id = Some(request_id);

        let mut app_conn = app_db::open_app_database_connection()?;

        VeteranSchema::ensure_current(&app_conn)
            .map_err(|err| format!("failed to initialize veteran schema: {err}"))?;

        let receiver = state.register_pending(request_id)?;

        if let Err(err) = state.with_running_worker(|running| {
            write_msgpack_request_framed(&mut running.stdin, &request)
                .map_err(|write_err| format!("failed to write msgpack request: {write_err}"))
        }) {
            state.clear_pending(request_id);
            return Err(err);
        }

        let frame = await_worker_frame(
            &state,
            request_id,
            receiver,
            timeout_ms.unwrap_or(DEFAULT_VETERAN_TIMEOUT_MS),
        )?;

        let tx = app_conn
            .transaction()
            .map_err(|err| format!("failed to start veteran import transaction: {err}"))?;

        let (uma_groups, added) = veterans::process_messagepack(&frame, &tx)
            .map_err(|e| format!("failed to process veteran data: {}", e))?;

        let current_hashes: HashSet<i64> = uma_groups.iter()
            .map(|g| g.veteran.hash.as_i64())
            .collect();

        let mut removed = 0usize;

        if !current_hashes.is_empty() {
            let placeholders = current_hashes.iter()
                .enumerate()
                .map(|(i, _)| format!("?{}", i + 1))
                .collect::<Vec<_>>()
                .join(",");
            let params: Vec<&dyn rusqlite::ToSql> = current_hashes.iter()
                .map(|h| h as &dyn rusqlite::ToSql)
                .collect();

            removed = tx.execute(
                &format!("UPDATE veterans SET active = 0 WHERE owned = 1 AND is_browser = 1 AND active != 0 AND hash NOT IN ({placeholders})"),
                rusqlite::params_from_iter(params),
            ).map_err(|e| format!("deactivate stale veterans: {e}"))?;
        }

        tx.commit()
            .map_err(|err| format!("failed to commit veteran import transaction: {err}"))?;

        // Invalidate in-memory store cache so next query re-reads from DB
        if let Some(store) = app.try_state::<Mutex<VeteranStore>>() {
            if let Ok(mut s) = store.lock() {
                s.invalidate_cache();
            }
        }

        let now = Utc::now().to_rfc3339();
        app_conn
            .execute(
                "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
                rusqlite::params!["last_veterans_gathered", now, now],
            )
            .map_err(|e| format!("metadata write: {e}"))?;

        Ok(GatherVeteransResult {
            added,
            removed,
            total: uma_groups.len(),
        })
    })
    .await
    .map_err(|err| format!("failed to join veteran import task: {err}"))?
}

fn validate_gather_veterans_request(request: &WorkerRequest) -> Result<(), String> {
    if matches!(request.command, WorkerCommand::GetVeteranData { .. }) {
        Ok(())
    } else {
        Err("gather_veterans expects worker command 'get_veteran_data'".to_string())
    }
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

#[tauri::command]
pub async fn export_veterans_to_json(_app: AppHandle) -> Result<String, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let conn = app_db::open_app_database_connection()
            .map_err(|e| format!("open db: {e}"))?;

        VeteranSchema::ensure_current(&conn)
            .map_err(|e| format!("ensure schema: {e}"))?;

        let mut stmt_veterans = conn
            .prepare(
                r#"
                SELECT hash, trainee_id, scenario, trained_chara_id, favorite_icon_type,
                       favorite_memo, created_at, rank, rank_score,
                       stat_speed, stat_stamina, stat_power, stat_guts, stat_wit,
                       aptitude_turf, aptitude_dirt, aptitude_sprint, aptitude_mile,
                       aptitude_medium, aptitude_long, aptitude_front, aptitude_pace_chaser,
                       aptitude_late_surger, aptitude_end_closer,
                       parent_a, parent_b, owner_id, min_hash, owned, rarity, talent_level,
                       use_type, fans, succession_num, is_saved, is_locked,
                       chara_grade, veteran_running_style, nickname_id, wins
                FROM veterans
                WHERE owned = 1 AND is_browser = 1 AND active = 1
                ORDER BY rank_score DESC
                "#,
            )
            .map_err(|e| format!("prepare veterans query: {e}"))?;

        let veteran_rows = stmt_veterans
            .query_map([], |row| {
                Ok(VeteranExportRow {
                    hash: row.get::<_, i64>(0)?,
                    trainee_id: row.get::<_, i64>(1)?,
                    scenario: row.get::<_, Option<i64>>(2)?,
                    trained_chara_id: row.get::<_, Option<i64>>(3)?,
                    favorite_icon_type: row.get::<_, Option<i64>>(4)?,
                    _favorite_memo: row.get::<_, Option<String>>(5)?,
                    created_at: row.get::<_, String>(6)?,
                    rank: row.get::<_, i64>(7)?,
                    rank_score: row.get::<_, i64>(8)?,
                    stat_speed: row.get::<_, Option<i64>>(9)?,
                    stat_stamina: row.get::<_, Option<i64>>(10)?,
                    stat_power: row.get::<_, Option<i64>>(11)?,
                    stat_guts: row.get::<_, Option<i64>>(12)?,
                    stat_wit: row.get::<_, Option<i64>>(13)?,
                    aptitude_turf: row.get::<_, Option<i64>>(14)?,
                    aptitude_dirt: row.get::<_, Option<i64>>(15)?,
                    aptitude_sprint: row.get::<_, Option<i64>>(16)?,
                    aptitude_mile: row.get::<_, Option<i64>>(17)?,
                    aptitude_medium: row.get::<_, Option<i64>>(18)?,
                    aptitude_long: row.get::<_, Option<i64>>(19)?,
                    aptitude_front: row.get::<_, Option<i64>>(20)?,
                    aptitude_pace_chaser: row.get::<_, Option<i64>>(21)?,
                    aptitude_late_surger: row.get::<_, Option<i64>>(22)?,
                    aptitude_end_closer: row.get::<_, Option<i64>>(23)?,
                    parent_a: row.get::<_, Option<i64>>(24)?,
                    parent_b: row.get::<_, Option<i64>>(25)?,
                    owner_id: row.get::<_, Option<i64>>(26)?,
                    min_hash: row.get::<_, Option<i64>>(27)?,
                    _owned: row.get::<_, i64>(28)?,
                    rarity: row.get::<_, i64>(29)?,
                    talent_level: row.get::<_, Option<i64>>(30)?,
                    use_type: row.get::<_, i64>(31)?,
                    fans: row.get::<_, i64>(32)?,
                    succession_num: row.get::<_, i64>(33)?,
                    is_saved: row.get::<_, i64>(34)?,
                    is_locked: row.get::<_, i64>(35)?,
                    chara_grade: row.get::<_, i64>(36)?,
                    running_style: row.get::<_, i64>(37)?,
                    nickname_id: row.get::<_, i64>(38)?,
                    wins: row.get::<_, i64>(39)?,
                })
            })
            .map_err(|e| format!("query veterans: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("collect veterans: {e}"))?;

        let mut output = Vec::with_capacity(veteran_rows.len());

        for vrow in &veteran_rows {
            let hash = vrow.hash;

            let skills: Vec<serde_json::Value> = query_json_rows(
                &conn,
                "SELECT skill_id, level FROM veteran_has_skill WHERE veteran_hash = ?1 ORDER BY rowid",
                [hash],
                |row| {
                    let mut m = serde_json::Map::new();
                    m.insert("skill_id".into(), serde_json::json!(row.get::<_, i64>(0)?));
                    m.insert("level".into(), serde_json::json!(row.get::<_, i64>(1)?));
                    Ok(serde_json::Value::Object(m))
                },
            )?;

            let support_cards: Vec<serde_json::Value> = query_json_rows(
                &conn,
                "SELECT position, support_card_id, exp, limit_break_count FROM veteran_support_card WHERE veteran_hash = ?1 ORDER BY position",
                [hash],
                |row| {
                    let mut m = serde_json::Map::new();
                    m.insert("position".into(), serde_json::json!(row.get::<_, i64>(0)?));
                    m.insert("support_card_id".into(), serde_json::json!(row.get::<_, i64>(1)?));
                    m.insert("exp".into(), serde_json::json!(row.get::<_, i64>(2)?));
                    m.insert("limit_break_count".into(), serde_json::json!(row.get::<_, i64>(3)?));
                    Ok(serde_json::Value::Object(m))
                },
            )?;

            let factor_info: Vec<serde_json::Value> = query_json_rows(
                &conn,
                "SELECT spark_id FROM veteran_has_spark WHERE veteran_hash = ?1",
                [hash],
                |row| {
                    let spark_id: i64 = row.get(0)?;
                    let mut m = serde_json::Map::new();
                    m.insert("factor_id".into(), serde_json::json!(spark_id));
                    m.insert("level".into(), serde_json::json!(0i64));
                    Ok(serde_json::Value::Object(m))
                },
            )?;

            let win_saddle_id_array: Vec<i64> = {
                let mut stmt = conn
                    .prepare("SELECT win_id FROM veteran_has_win WHERE veteran_hash = ?1")
                    .map_err(|e| format!("prepare wins: {e}"))?;
                let mapped = stmt.query_map([hash], |row| row.get(0))
                    .map_err(|e| format!("query wins: {e}"))?;
                mapped
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("collect wins: {e}"))?
            };

            let race_result_list: Vec<serde_json::Value> = query_json_rows(
                &conn,
                "SELECT turn, program_id, weather, ground_condition, running_style, \
                 popularity, result_rank, result_time, prize_money \
                 FROM veteran_race_results WHERE veteran_hash = ?1 ORDER BY idx",
                [hash],
                |row| {
                    let mut m = serde_json::Map::new();
                    m.insert("turn".into(), serde_json::json!(row.get::<_, i64>(0)?));
                    m.insert("program_id".into(), serde_json::json!(row.get::<_, i64>(1)?));
                    m.insert("weather".into(), serde_json::json!(row.get::<_, i64>(2)?));
                    m.insert("ground_condition".into(), serde_json::json!(row.get::<_, i64>(3)?));
                    m.insert("running_style".into(), serde_json::json!(row.get::<_, i64>(4)?));
                    m.insert("popularity".into(), serde_json::json!(row.get::<_, i64>(5)?));
                    m.insert("result_rank".into(), serde_json::json!(row.get::<_, i64>(6)?));
                    m.insert("result_time".into(), serde_json::json!(row.get::<_, i64>(7)?));
                    m.insert("prize_money".into(), serde_json::json!(row.get::<_, i64>(8)?));
                    Ok(serde_json::Value::Object(m))
                },
            )?;

            let nickname_id_array: Vec<i64> = {
                let mut stmt = conn
                    .prepare("SELECT nickname_id FROM veteran_nickname_ids WHERE veteran_hash = ?1 ORDER BY idx")
                    .map_err(|e| format!("prepare nickname ids: {e}"))?;
                let mapped = stmt.query_map([hash], |row| row.get(0))
                    .map_err(|e| format!("query nickname ids: {e}"))?;
                mapped
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("collect nickname ids: {e}"))?
            };

            let succession_chara_array = build_succession_chara_array(&conn, vrow)?;

            let mut entry = serde_json::Map::new();
            entry.insert("trained_chara_id".into(), serde_json::json!(vrow.trained_chara_id.unwrap_or(0)));
            entry.insert("owner_trained_chara_id".into(), serde_json::json!(0i64));
            entry.insert("single_mode_chara_id".into(), serde_json::json!(0i64));
            entry.insert("chara_seed".into(), serde_json::json!(0i64));
            entry.insert("card_id".into(), serde_json::json!(vrow.trainee_id));
            entry.insert("succession_trained_chara_id_1".into(), serde_json::json!(0i64));
            entry.insert("succession_trained_chara_id_2".into(), serde_json::json!(0i64));
            entry.insert("use_type".into(), serde_json::json!(vrow.use_type));
            entry.insert("speed".into(), serde_json::json!(vrow.stat_speed.unwrap_or(0)));
            entry.insert("stamina".into(), serde_json::json!(vrow.stat_stamina.unwrap_or(0)));
            entry.insert("power".into(), serde_json::json!(vrow.stat_power.unwrap_or(0)));
            entry.insert("wiz".into(), serde_json::json!(vrow.stat_wit.unwrap_or(0)));
            entry.insert("guts".into(), serde_json::json!(vrow.stat_guts.unwrap_or(0)));
            entry.insert("fans".into(), serde_json::json!(vrow.fans));
            entry.insert("rank_score".into(), serde_json::json!(vrow.rank_score));
            entry.insert("rank".into(), serde_json::json!(vrow.rank));
            entry.insert("scenario_id".into(), serde_json::json!(vrow.scenario.unwrap_or(0)));
            entry.insert("route_id".into(), serde_json::json!(0i64));
            entry.insert("arrive_route_race_id".into(), serde_json::json!(0i64));
            entry.insert("proper_ground_turf".into(), serde_json::json!(vrow.aptitude_turf.unwrap_or(0)));
            entry.insert("proper_ground_dirt".into(), serde_json::json!(vrow.aptitude_dirt.unwrap_or(0)));
            entry.insert("proper_running_style_nige".into(), serde_json::json!(vrow.aptitude_front.unwrap_or(0)));
            entry.insert("proper_running_style_senko".into(), serde_json::json!(vrow.aptitude_pace_chaser.unwrap_or(0)));
            entry.insert("proper_running_style_sashi".into(), serde_json::json!(vrow.aptitude_late_surger.unwrap_or(0)));
            entry.insert("proper_running_style_oikomi".into(), serde_json::json!(vrow.aptitude_end_closer.unwrap_or(0)));
            entry.insert("proper_distance_short".into(), serde_json::json!(vrow.aptitude_sprint.unwrap_or(0)));
            entry.insert("proper_distance_mile".into(), serde_json::json!(vrow.aptitude_mile.unwrap_or(0)));
            entry.insert("proper_distance_middle".into(), serde_json::json!(vrow.aptitude_medium.unwrap_or(0)));
            entry.insert("proper_distance_long".into(), serde_json::json!(vrow.aptitude_long.unwrap_or(0)));
            entry.insert("succession_num".into(), serde_json::json!(vrow.succession_num));
            entry.insert("rarity".into(), serde_json::json!(vrow.rarity));
            entry.insert("is_saved".into(), serde_json::json!(vrow.is_saved));
            entry.insert("is_locked".into(), serde_json::json!(vrow.is_locked));
            entry.insert("talent_level".into(), serde_json::json!(vrow.talent_level.unwrap_or(0)));
            entry.insert("race_cloth_id".into(), serde_json::json!(0i64));
            entry.insert("chara_grade".into(), serde_json::json!(vrow.chara_grade));
            entry.insert("running_style".into(), serde_json::json!(vrow.running_style));
            entry.insert("nickname_id".into(), serde_json::json!(vrow.nickname_id));
            entry.insert("wins".into(), serde_json::json!(vrow.wins));
            entry.insert("register_time".into(), serde_json::json!(""));
            entry.insert("create_time".into(), serde_json::json!(&vrow.created_at));
            entry.insert("skill_array".into(), serde_json::Value::Array(skills));
            entry.insert("support_card_list".into(), serde_json::Value::Array(support_cards));
            entry.insert("race_result_list".into(), serde_json::Value::Array(race_result_list));
            entry.insert("win_saddle_id_array".into(), serde_json::json!(win_saddle_id_array));
            entry.insert("nickname_id_array".into(), serde_json::json!(nickname_id_array));
            entry.insert("factor_info_array".into(), serde_json::Value::Array(factor_info));
            entry.insert("factor_extend_array".into(), serde_json::json!([]));
            entry.insert("succession_chara_array".into(), serde_json::Value::Array(succession_chara_array));

            output.push(serde_json::Value::Object(entry));
        }

        let json = serde_json::to_string_pretty(&output)
            .map_err(|e| format!("serialize output: {e}"))?;

        let Some(path) = rfd::FileDialog::new()
            .add_filter("JSON", &["json"])
            .set_file_name("data.json")
            .save_file()
        else {
            return Ok("canceled".to_string());
        };

        std::fs::write(&path, &json)
            .map_err(|e| format!("failed to write file '{}': {e}", path.display()))?;

        Ok(path.display().to_string())
    })
    .await
    .map_err(|e| format!("join export task: {e}"))?
}

fn query_json_rows<P: rusqlite::Params>(
    conn: &rusqlite::Connection,
    sql: &str,
    params: P,
    mapper: impl FnMut(&rusqlite::Row<'_>) -> Result<serde_json::Value, rusqlite::Error>,
) -> Result<Vec<serde_json::Value>, String> {
    let mut stmt = conn.prepare(sql).map_err(|e| format!("prepare: {e}"))?;
    let mut mapper = mapper;
    let mapped = stmt
        .query_map(params, |row| mapper(row))
        .map_err(|e| format!("query: {e}"))?;
    mapped
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect: {e}"))
}

struct VeteranExportRow {
    hash: i64,
    trainee_id: i64,
    scenario: Option<i64>,
    trained_chara_id: Option<i64>,
    favorite_icon_type: Option<i64>,
    _favorite_memo: Option<String>,
    created_at: String,
    rank: i64,
    rank_score: i64,
    stat_speed: Option<i64>,
    stat_stamina: Option<i64>,
    stat_power: Option<i64>,
    stat_guts: Option<i64>,
    stat_wit: Option<i64>,
    aptitude_turf: Option<i64>,
    aptitude_dirt: Option<i64>,
    aptitude_sprint: Option<i64>,
    aptitude_mile: Option<i64>,
    aptitude_medium: Option<i64>,
    aptitude_long: Option<i64>,
    aptitude_front: Option<i64>,
    aptitude_pace_chaser: Option<i64>,
    aptitude_late_surger: Option<i64>,
    aptitude_end_closer: Option<i64>,
    parent_a: Option<i64>,
    parent_b: Option<i64>,
    owner_id: Option<i64>,
    min_hash: Option<i64>,
    _owned: i64,
    rarity: i64,
    talent_level: Option<i64>,
    use_type: i64,
    fans: i64,
    succession_num: i64,
    is_saved: i64,
    is_locked: i64,
    chara_grade: i64,
    running_style: i64,
    nickname_id: i64,
    wins: i64,
}

struct ParentExportRow {
    trainee_id: i64,
    rank: i64,
    rarity: i64,
    talent_level: Option<i64>,
    parent_a: Option<i64>,
    parent_b: Option<i64>,
    owner_id: Option<i64>,
}

fn load_parent(conn: &rusqlite::Connection, hash: i64) -> Result<Option<ParentExportRow>, String> {
    let mut stmt = conn
        .prepare(
            "SELECT trainee_id, rank, rarity, talent_level, parent_a, parent_b, owner_id \
             FROM parents WHERE hash = ?1",
        )
        .map_err(|e| format!("prepare load parent: {e}"))?;

    let mut rows = stmt
        .query_map([hash], |row| {
            Ok(ParentExportRow {
                trainee_id: row.get(0)?,
                rank: row.get(1)?,
                rarity: row.get(2)?,
                talent_level: row.get(3)?,
                parent_a: row.get(4)?,
                parent_b: row.get(5)?,
                owner_id: row.get(6)?,
            })
        })
        .map_err(|e| format!("query parent {hash}: {e}"))?;

    rows.next()
        .transpose()
        .map_err(|e| format!("read parent {hash}: {e}"))
}

fn load_parent_sparks(
    conn: &rusqlite::Connection,
    hash: i64,
) -> Result<Vec<serde_json::Value>, String> {
    let mut stmt = conn
        .prepare("SELECT spark_id FROM parent_has_spark WHERE parent_hash = ?1")
        .map_err(|e| format!("prepare parent sparks: {e}"))?;
    let mapped = stmt
        .query_map([hash], |row| {
            let spark_id: i64 = row.get(0)?;
            Ok(serde_json::json!({
                "factor_id": spark_id,
                "level": 0i64
            }))
        })
        .map_err(|e| format!("query parent sparks {hash}: {e}"))?;
    mapped
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect parent sparks {hash}: {e}"))
}

fn load_parent_wins(conn: &rusqlite::Connection, hash: i64) -> Result<Vec<i64>, String> {
    let mut stmt = conn
        .prepare("SELECT win_id FROM parent_has_win WHERE parent_hash = ?1")
        .map_err(|e| format!("prepare parent wins: {e}"))?;
    let mapped = stmt
        .query_map([hash], |row| row.get(0))
        .map_err(|e| format!("query parent wins {hash}: {e}"))?;
    mapped
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("collect parent wins {hash}: {e}"))
}

const POS_PARENT1: i64 = 10;
const POS_PARENT2: i64 = 20;
const POS_GP1_P1: i64 = 11;
const POS_GP1_P2: i64 = 12;
const POS_GP2_P1: i64 = 21;
const POS_GP2_P2: i64 = 22;

fn build_succession_chara_array(
    conn: &rusqlite::Connection,
    vrow: &VeteranExportRow,
) -> Result<Vec<serde_json::Value>, String> {
    let mut out = Vec::new();

    if let Some(parent_a_hash) = vrow.parent_a {
        let pa = load_parent(conn, parent_a_hash)?.unwrap_or_default();
        let factor_info = load_parent_sparks(conn, parent_a_hash)?;
        let win_saddle = load_parent_wins(conn, parent_a_hash)?;
        out.push(build_succession_entry(
            POS_PARENT1,
            &pa,
            parent_a_hash,
            factor_info,
            win_saddle,
        ));

        if let Some(gpa_hash) = pa.parent_a {
            let gpa = load_parent(conn, gpa_hash)?.unwrap_or_default();
            let fi = load_parent_sparks(conn, gpa_hash)?;
            let ws = load_parent_wins(conn, gpa_hash)?;
            out.push(build_succession_entry(POS_GP1_P1, &gpa, gpa_hash, fi, ws));
        }
        if let Some(gpb_hash) = pa.parent_b {
            let gpb = load_parent(conn, gpb_hash)?.unwrap_or_default();
            let fi = load_parent_sparks(conn, gpb_hash)?;
            let ws = load_parent_wins(conn, gpb_hash)?;
            out.push(build_succession_entry(POS_GP1_P2, &gpb, gpb_hash, fi, ws));
        }
    }

    if let Some(parent_b_hash) = vrow.parent_b {
        let pb = load_parent(conn, parent_b_hash)?.unwrap_or_default();
        let factor_info = load_parent_sparks(conn, parent_b_hash)?;
        let win_saddle = load_parent_wins(conn, parent_b_hash)?;
        out.push(build_succession_entry(
            POS_PARENT2,
            &pb,
            parent_b_hash,
            factor_info,
            win_saddle,
        ));

        if let Some(gpa_hash) = pb.parent_a {
            let gpa = load_parent(conn, gpa_hash)?.unwrap_or_default();
            let fi = load_parent_sparks(conn, gpa_hash)?;
            let ws = load_parent_wins(conn, gpa_hash)?;
            out.push(build_succession_entry(POS_GP2_P1, &gpa, gpa_hash, fi, ws));
        }
        if let Some(gpb_hash) = pb.parent_b {
            let gpb = load_parent(conn, gpb_hash)?.unwrap_or_default();
            let fi = load_parent_sparks(conn, gpb_hash)?;
            let ws = load_parent_wins(conn, gpb_hash)?;
            out.push(build_succession_entry(POS_GP2_P2, &gpb, gpb_hash, fi, ws));
        }
    }

    Ok(out)
}

fn build_succession_entry(
    position_id: i64,
    parent: &ParentExportRow,
    _parent_hash: i64,
    factor_info_array: Vec<serde_json::Value>,
    win_saddle_id_array: Vec<i64>,
) -> serde_json::Value {
    serde_json::json!({
        "position_id": position_id,
        "card_id": parent.trainee_id,
        "rank": parent.rank,
        "rarity": parent.rarity,
        "talent_level": parent.talent_level.unwrap_or(0),
        "factor_info_array": factor_info_array,
        "win_saddle_id_array": win_saddle_id_array,
        "owner_viewer_id": parent.owner_id.unwrap_or(0),
    })
}

impl Default for ParentExportRow {
    fn default() -> Self {
        Self {
            trainee_id: 0,
            rank: 0,
            rarity: 0,
            talent_level: None,
            parent_a: None,
            parent_b: None,
            owner_id: None,
        }
    }
}
