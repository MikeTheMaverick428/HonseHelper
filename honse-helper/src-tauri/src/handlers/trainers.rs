use crate::db::{app_db, schema::VeteranSchema};
use crate::handlers::rmpv_to_json;
use crate::veterans;
use crate::worker::WorkerState;
use chrono::Utc;
use honse_worker::protocol::{
    parse_msgpack_frame_response, write_msgpack_request_framed, WorkerCommand, WorkerRequest,
    WorkerResponse,
};
use rusqlite::params;
use serde::Deserialize;
use shared::{GatherVeteransResult, mssgpack_data::MssgPackTrainedChara};
use std::collections::{HashMap, HashSet};
use std::sync::mpsc::Receiver;
use std::time::Duration;
use tauri::{AppHandle, Manager, State};

const DEFAULT_TRAINER_TIMEOUT_MS: u64 = 15_000;

#[tauri::command]
pub async fn gather_followed_trainers(
    app: AppHandle,
    mut request: WorkerRequest,
    timeout_ms: Option<u64>,
) -> Result<GatherVeteransResult, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let state: State<'_, WorkerState> = app.state();
        validate_gather_trainers_request(&request)?;

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
            timeout_ms.unwrap_or(DEFAULT_TRAINER_TIMEOUT_MS),
        )?;

        let payload = parse_ok_payload(&frame)?;
        let entry: EntryInfoPayload = serde_json::from_value(
            payload
                .get("entry")
                .cloned()
                .unwrap_or(serde_json::Value::Null),
        )
        .map_err(|e| format!("failed to parse single mode start friends payload: {e}"))?;

        let tx = app_conn
            .transaction()
            .map_err(|err| format!("failed to start trainer import transaction: {err}"))?;

        let result = import_friends(&tx, &entry)?;

        tx.commit()
            .map_err(|err| format!("failed to commit trainer import transaction: {err}"))?;

        let now = Utc::now().to_rfc3339();
        app_conn
            .execute(
                "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
                 ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
                params!["last_trainers_gathered", now, now],
            )
            .map_err(|e| format!("metadata write: {e}"))?;

        Ok(result)
    })
    .await
    .map_err(|err| format!("failed to join trainer import task: {err}"))?
}

fn validate_gather_trainers_request(request: &WorkerRequest) -> Result<(), String> {
    if matches!(request.command, WorkerCommand::GetSingleModeStartFriends) {
        Ok(())
    } else {
        Err("gather_followed_trainers expects worker command 'get_single_mode_start_friends'".to_string())
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

fn parse_ok_payload(frame: &[u8]) -> Result<serde_json::Value, String> {
    match parse_msgpack_frame_response(frame) {
        Some(WorkerResponse::Ok(ok)) => Ok(rmpv_to_json(ok.payload)),
        Some(WorkerResponse::Err(err)) => Err(format!("worker error: {}", err.error)),
        _ => Err("worker response was not an Ok payload".to_string()),
    }
}

struct ImportSummary {
    added: usize,
    removed: usize,
    total: usize,
}

fn import_friends(tx: &rusqlite::Transaction, entry: &EntryInfoPayload) -> Result<GatherVeteransResult, String> {
    let mut current_trainer_ids: HashSet<i64> = HashSet::new();
    let mut added_veterans = 0usize;

    // Map each full rental uma to its owner viewer id (obscured long -> plain id).
    let mut chara_index_by_owner: HashMap<i64, usize> = HashMap::new();
    for (i, chara) in entry.rental_trained_chara_array.iter().enumerate() {
        chara_index_by_owner.entry(chara.owner_viewer_id).or_insert(i);
    }

    let friend_card_by_viewer: HashMap<i64, &FriendCardInfoPayload> = entry
        .friend_card_info_list
        .iter()
        .map(|c| (c.viewer_id, c))
        .collect();

    for (idx, friend) in entry.rental_user_info_array.iter().enumerate() {
        let rental = chara_index_by_owner
            .get(&friend.viewer_id)
            .and_then(|i| entry.rental_trained_chara_array.get(*i))
            .or_else(|| entry.rental_trained_chara_array.get(idx));

        let mut borrow = BorrowSnapshot::default();

        if let Some(chara) = rental {
            let hash = shared::db_models::veteran_data::hash_rental_veteran(
                chara.card_id,
                &chara.factor_id_array,
                chara.rank_score,
                chara.rank,
            );
            let hash_i64 = hash.as_i64();

            match shared::db_models::veteran_data::UmaGroup::from_trained_chara_mssgpack(chara) {
                Ok(mut group) => {
                    group.veteran.is_browser = true;
                    group.veteran.hash = hash;

                    veterans::deactivate_trainer_veterans(tx, friend.viewer_id)?;

                    veterans::process_group_direct(&group, tx)?;

                    borrow = BorrowSnapshot {
                        hash: Some(hash_i64),
                        character_id: Some(group.veteran.trainee_id),
                        rarity: Some(i64::from(group.veteran.rarity)),
                        rank: Some(i64::from(group.veteran.rank)),
                        rank_score: Some(i64::from(group.veteran.rank_score)),
                    };
                    added_veterans += 1;
                }
                Err(err) => {
                    eprintln!(
                        "trainer {} ({}): skipping borrow uma, group build failed: {err}",
                        friend.viewer_id, friend.name
                    );
                }
            }
        }

        upsert_trainer(tx, friend, &borrow)?;
        upsert_trainer_support_card(tx, friend, friend_card_by_viewer.get(&friend.viewer_id).copied())?;

        current_trainer_ids.insert(friend.viewer_id);
    }

    let mut removed = 0usize;

    if !current_trainer_ids.is_empty() {
        let placeholders = current_trainer_ids
            .iter()
            .enumerate()
            .map(|(i, _)| format!("?{}", i + 1))
            .collect::<Vec<_>>()
            .join(",");
        let ids: Vec<i64> = current_trainer_ids.iter().copied().collect();

        let stale = tx
            .query_row(
                &format!(
                    "SELECT COUNT(*) FROM trainers WHERE last_update_source = 'game' AND is_following = 1 AND trainer_id NOT IN ({placeholders})"
                ),
                rusqlite::params_from_iter(ids.iter()),
                |row| row.get::<_, i64>(0),
            )
            .map_err(|e| format!("count stale followed trainers: {e}"))?;

        tx.execute(
            &format!(
                "UPDATE trainers SET is_following = 0, updated_at = datetime('now') \
                 WHERE last_update_source = 'game' AND is_following = 1 AND trainer_id NOT IN ({placeholders})"
            ),
            rusqlite::params_from_iter(ids.iter()),
        )
        .map_err(|e| format!("unfollow stale trainers: {e}"))?;

        if stale > 0 {
            tx.execute(
                "UPDATE veterans SET active = 0 \
                 WHERE owned = 0 AND is_browser = 1 AND active = 1 AND owner_id IN (\
                   SELECT trainer_id FROM trainers WHERE last_update_source = 'game' AND is_following = 0\
                 )",
                [],
            )
            .map_err(|e| format!("deactivate borrow veterans of unfollowed trainers: {e}"))?;
        }

        removed = stale as usize;
    }

    Ok(GatherVeteransResult {
        added: added_veterans,
        removed,
        total: entry.rental_user_info_array.len(),
    })
}

#[derive(Default)]
struct BorrowSnapshot {
    hash: Option<i64>,
    character_id: Option<i64>,
    rarity: Option<i64>,
    rank: Option<i64>,
    rank_score: Option<i64>,
}

fn upsert_trainer(
    tx: &rusqlite::Transaction,
    friend: &UserInfoAtFriendPayload,
    borrow: &BorrowSnapshot,
) -> Result<(), String> {
    let honor_id = friend.honor_data.as_ref().map(|h| h.honor_id).unwrap_or(0);
    let circle_id = friend.circle_info.as_ref().map(|c| c.circle_id).unwrap_or(0);
    let circle_name = friend
        .circle_info
        .as_ref()
        .map(|c| c.circle_name.clone())
        .unwrap_or_default();
    let now = Utc::now().to_rfc3339();

    tx.execute(
        r#"
        INSERT INTO trainers (
            trainer_id, name, friend_state, is_following, honor_id, last_login,
            comment, fan, circle_id, circle_name, borrow_uma_hash, borrow_uma_character_id,
            borrow_uma_rarity, borrow_uma_rank, borrow_uma_rank_score, created_at, updated_at,
            last_update_source
        ) VALUES (?1, ?2, ?3, 1, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, 'game')
        ON CONFLICT(trainer_id) DO UPDATE SET
            name = excluded.name,
            friend_state = excluded.friend_state,
            is_following = 1,
            honor_id = excluded.honor_id,
            last_login = excluded.last_login,
            comment = excluded.comment,
            fan = excluded.fan,
            circle_id = excluded.circle_id,
            circle_name = excluded.circle_name,
            borrow_uma_hash = excluded.borrow_uma_hash,
            borrow_uma_character_id = excluded.borrow_uma_character_id,
            borrow_uma_rarity = excluded.borrow_uma_rarity,
            borrow_uma_rank = excluded.borrow_uma_rank,
            borrow_uma_rank_score = excluded.borrow_uma_rank_score,
            updated_at = excluded.updated_at,
            last_update_source = 'game'
        "#,
        params![
            friend.viewer_id,
            friend.name,
            friend.friend_state,
            honor_id,
            if friend.last_login_time.is_empty() {
                None
            } else {
                Some(friend.last_login_time.as_str())
            },
            friend.comment,
            friend.fan,
            circle_id,
            circle_name,
            borrow.hash,
            borrow.character_id,
            borrow.rarity,
            borrow.rank,
            borrow.rank_score,
            now,
            now,
        ],
    )
    .map_err(|e| format!("upsert trainer {}: {e}", friend.viewer_id))?;

    Ok(())
}

fn upsert_trainer_support_card(
    tx: &rusqlite::Transaction,
    friend: &UserInfoAtFriendPayload,
    friend_card: Option<&FriendCardInfoPayload>,
) -> Result<(), String> {
    tx.execute(
        "DELETE FROM trainer_support_card WHERE trainer_id = ?1",
        params![friend.viewer_id],
    )
    .map_err(|e| format!("clean trainer support cards for {}: {e}", friend.viewer_id))?;

    let Some(sc) = &friend.user_support_card else {
        return Ok(());
    };

    let support_card_id = friend_card
        .map(|c| c.support_card_id)
        .filter(|id| *id > 0)
        .unwrap_or(sc.support_card_id);
    if support_card_id <= 0 {
        return Ok(());
    }

    let limit_break_count = sc.limit_break_count.max(
        friend_card.map(|c| c.limit_break_count).unwrap_or(0),
    );

    let rarity: i64 = tx
        .query_row(
            "SELECT rarity FROM support_card_data WHERE id = ?1",
            params![support_card_id],
            |row| row.get::<_, i64>(0),
        )
        .unwrap_or(0);

    let lvl = friend_card
        .filter(|c| c.support_card_id == support_card_id)
        .map(|c| c.support_card_level)
        .filter(|lvl| *lvl > 0)
        .unwrap_or_else(|| shared::trainer_browser::max_level_for_limit_break(rarity, limit_break_count));

    tx.execute(
        r#"
        INSERT INTO trainer_support_card (
            trainer_id, support_card_id, level, limit_break_count
        ) VALUES (?1, ?2, ?3, ?4)
        ON CONFLICT(trainer_id, support_card_id) DO UPDATE SET
            level = excluded.level,
            limit_break_count = excluded.limit_break_count
        "#,
        params![
            friend.viewer_id,
            support_card_id,
            lvl,
            limit_break_count,
        ],
    )
    .map_err(|e| format!("upsert trainer support card {support_card_id} for {}: {e}", friend.viewer_id))?;

    Ok(())
}

#[derive(Deserialize)]
struct EntryInfoPayload {
    #[serde(default, alias = "rental_user_info_array")]
    rental_user_info_array: Vec<UserInfoAtFriendPayload>,
    #[serde(default, alias = "rental_trained_chara_array")]
    rental_trained_chara_array: Vec<MssgPackTrainedChara>,
    #[serde(default, alias = "friend_card_info_list")]
    friend_card_info_list: Vec<FriendCardInfoPayload>,
}

#[derive(Deserialize)]
struct UserInfoAtFriendPayload {
    viewer_id: i64,
    #[serde(default)]
    name: String,
    #[serde(default)]
    friend_state: i64,
    #[serde(default)]
    last_login_time: String,
    #[serde(default)]
    comment: String,
    #[serde(default)]
    fan: i64,
    #[serde(default)]
    leader_chara_id: i64,
    #[serde(default)]
    honor_data: Option<HonorDataPayload>,
    #[serde(default)]
    circle_info: Option<CircleInfoPayload>,
    #[serde(default)]
    user_support_card: Option<UserSupportCardPayload>,
}

#[derive(Deserialize)]
struct HonorDataPayload {
    #[serde(default)]
    honor_id: i64,
}

#[derive(Deserialize)]
struct CircleInfoPayload {
    #[serde(default)]
    circle_id: i64,
    #[serde(default)]
    circle_name: String,
}

#[derive(Deserialize)]
struct UserSupportCardPayload {
    #[serde(default)]
    support_card_id: i64,
    #[serde(default)]
    limit_break_count: i64,
}

#[derive(Deserialize)]
struct FriendCardInfoPayload {
    viewer_id: i64,
    #[serde(default)]
    support_card_id: i64,
    #[serde(default)]
    support_card_level: i64,
    #[serde(default)]
    limit_break_count: i64,
}
