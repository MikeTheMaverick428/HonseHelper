use crate::db::app_db;
use crate::external::umamoe;
use crate::handlers::api_config::ApiKeyState;
use rusqlite::params;
use shared::db_models::veteran_data::UmaGroup;
use tauri::State;

#[tauri::command]
pub async fn add_uma_moe_trainer(
    account_id: String,
    api_key_state: State<'_, ApiKeyState>,
) -> Result<String, String> {
    let api_key = api_key_state
        .api_key
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "API key not configured".to_string())?;

    import_trainer_from_uma_moe(&api_key, &account_id).await
}

#[tauri::command]
pub async fn refresh_uma_moe_trainer(
    trainer_id: i64,
    api_key_state: State<'_, ApiKeyState>,
) -> Result<String, String> {
    let api_key = api_key_state
        .api_key
        .lock()
        .map_err(|e| e.to_string())?
        .clone()
        .ok_or_else(|| "API key not configured".to_string())?;

    import_trainer_from_uma_moe(&api_key, &trainer_id.to_string()).await
}

async fn import_trainer_from_uma_moe(
    api_key: &str,
    account_id: &str,
) -> Result<String, String> {
    let client = uma_moe_api::UmaMoeClient::new().with_api_key(api_key);
    let profile = client
        .get_profile(account_id, Some(&uma_moe_api::types::requests::ProfileRequest::exclude_lite()))
        .await
        .map_err(|e| format!("API error: {e}"))?;

    let trainer = profile
        .trainer
        .as_ref()
        .ok_or_else(|| "profile response has no trainer".to_string())?;
    let trainer_id: i64 = trainer
        .account_id
        .parse()
        .map_err(|e| format!("invalid trainer id '{}': {e}", trainer.account_id))?;

    let mut conn = app_db::open_app_database_connection()?;
    let tx = conn
        .transaction()
        .map_err(|err| format!("failed to start trainer import transaction: {err}"))?;

    let mut borrow = BorrowSnapshot::default();

    if let Some(inh) = &profile.inheritance {
        let last_updated = profile
            .borrow_stats
            .as_ref()
            .and_then(|b| b.last_recheck_at.clone())
            .unwrap_or_default();
        let mut group: UmaGroup = umamoe::adapt_inheritance(inh.clone(), &last_updated);

        let hash = group.veteran.hash.as_i64();

        crate::veterans::deactivate_trainer_veterans(&tx, trainer_id)?;

        group.veteran.is_browser = true;
        crate::veterans::process_group_direct(&group, &tx)?;

        borrow = BorrowSnapshot {
            hash: Some(hash),
            character_id: Some(group.veteran.trainee_id),
            rarity: Some(i64::from(group.veteran.rarity)),
            rank: Some(i64::from(group.veteran.rank)),
            rank_score: Some(i64::from(group.veteran.rank_score)),
        };
    }

    upsert_trainer(&tx, trainer_id, &trainer, &profile, &borrow)?;
    upsert_trainer_support_card(&tx, trainer_id, &profile)?;

    tx.commit()
        .map_err(|err| format!("failed to commit trainer import transaction: {err}"))?;

    Ok(trainer.name.clone())
}

struct BorrowSnapshot {
    hash: Option<i64>,
    character_id: Option<i64>,
    rarity: Option<i64>,
    rank: Option<i64>,
    rank_score: Option<i64>,
}

impl Default for BorrowSnapshot {
    fn default() -> Self {
        Self {
            hash: None,
            character_id: None,
            rarity: None,
            rank: None,
            rank_score: None,
        }
    }
}

fn upsert_trainer(
    tx: &rusqlite::Transaction,
    trainer_id: i64,
    trainer: &uma_moe_api::types::responses::TrainerInfo,
    profile: &uma_moe_api::types::responses::ProfileResponse,
    borrow: &BorrowSnapshot,
) -> Result<(), String> {
    let comment = trainer.comment.clone().unwrap_or_default();
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
    let last_recheck_at = profile
        .borrow_stats
        .as_ref()
        .and_then(|b| b.last_recheck_at.clone());

    tx.execute(
        r#"
        INSERT INTO trainers (
            trainer_id, name, friend_state, is_following, honor_id, last_login,
            comment, fan, circle_id, circle_name, follower_num, last_recheck_at, borrow_uma_hash, borrow_uma_character_id,
            borrow_uma_rarity, borrow_uma_rank, borrow_uma_rank_score, created_at, updated_at,
            last_update_source
        ) VALUES (?1, ?2, 0, 0, 0, NULL, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, datetime('now'), datetime('now'), 'uma_moe')
        ON CONFLICT(trainer_id) DO UPDATE SET
            name = excluded.name,
            comment = excluded.comment,
            fan = excluded.fan,
            circle_id = excluded.circle_id,
            circle_name = excluded.circle_name,
            follower_num = COALESCE(excluded.follower_num, trainers.follower_num),
            last_recheck_at = COALESCE(excluded.last_recheck_at, trainers.last_recheck_at),
            borrow_uma_hash = excluded.borrow_uma_hash,
            borrow_uma_character_id = excluded.borrow_uma_character_id,
            borrow_uma_rarity = excluded.borrow_uma_rarity,
            borrow_uma_rank = excluded.borrow_uma_rank,
            borrow_uma_rank_score = excluded.borrow_uma_rank_score,
            updated_at = datetime('now'),
            last_update_source = 'uma_moe'
        "#,
        params![
            trainer_id,
            trainer.name,
            comment,
            fan,
            circle_id,
            circle_name,
            trainer.follower_num,
            last_recheck_at,
            borrow.hash,
            borrow.character_id,
            borrow.rarity,
            borrow.rank,
            borrow.rank_score,
        ],
    )
    .map_err(|e| format!("upsert trainer {trainer_id}: {e}"))?;

    Ok(())
}

fn upsert_trainer_support_card(
    tx: &rusqlite::Transaction,
    trainer_id: i64,
    profile: &uma_moe_api::types::responses::ProfileResponse,
) -> Result<(), String> {
    let Some(sc) = &profile.support_card else {
        return Ok(());
    };

    let support_card_id = i64::from(sc.support_card_id);
    if support_card_id <= 0 {
        return Ok(());
    }

    let limit_break_count = i64::from(sc.limit_break_count.unwrap_or(0));

    let rarity: i64 = tx
        .query_row(
            "SELECT rarity FROM support_card_data WHERE id = ?1",
            params![support_card_id],
            |row| row.get::<_, i64>(0),
        )
        .unwrap_or(0);

    let level = shared::trainer_browser::max_level_for_limit_break(rarity, limit_break_count);

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
            trainer_id,
            support_card_id,
            level,
            limit_break_count,
        ],
    )
    .map_err(|e| format!("upsert trainer support card {support_card_id} for {trainer_id}: {e}"))?;

    Ok(())
}
