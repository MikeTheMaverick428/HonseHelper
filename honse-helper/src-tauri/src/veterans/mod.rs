pub mod uma_moe_cache;

use std::collections::{HashMap, HashSet};

use chrono::Utc;
use honse_worker::protocol::{parse_msgpack_frame_response, WorkerResponse};
use rusqlite::{params, params_from_iter, Connection};
use serde::Deserialize;
use shared::{
    db_models::veteran_data::{Parent, UmaGroup, Veteran},
    mssgpack_data::{
        MssgPackDataContainer, MssgPackRaceResult, MssgPackSkill, MssgPackSupportCard,
    },
};

pub fn process_messagepack(
    msg: &[u8],
    conn: &Connection,
) -> Result<(Vec<UmaGroup>, usize), String> {
    let container = decode_container(msg)?;

    let favourite_data = HashMap::<i64, (Option<i16>, Option<String>)>::from_iter(
        container
            .trained_chara_favorite_array
            .iter()
            .map(|fav| (fav.trained_chara_id, (fav.icon_type, fav.memo.clone()))),
    );

    let mut uma_groups = Vec::new();
    for chara in &container.trained_chara {
        let mut group = match UmaGroup::from_trained_chara_mssgpack(chara) {
            Ok(group) => group,
            Err(_) => {
                continue;
            }
        };

        if let Some(fav) = favourite_data.get(&chara.trained_chara_id) {
            group.veteran.favorite_icon_type = fav.0.map(|i| i as u16);
            group.veteran.favorite_memo = fav.1.clone();
        }

        group.veteran.is_browser = true;

        if !group.veteran.owned {
            continue;
        }

        uma_groups.push(group);
    }

    let mut new_count = 0;

    for group in &mut uma_groups {
        let was_new = process_group(group, conn)?;
        if was_new {
            new_count += 1;
        }
    }

    Ok((uma_groups, new_count))
}

#[derive(Deserialize)]
struct MsgpackWorkerEnvelope {
    payload: MssgPackDataContainer,
}

#[derive(Deserialize)]
struct MsgpackWorkerEnvelopeData {
    payload: MsgpackDataField,
}

#[derive(Deserialize)]
struct MsgpackDataField {
    #[serde(alias = "Data")]
    data: MssgPackDataContainer,
}

#[derive(Deserialize)]
struct JsonDataField {
    #[serde(alias = "Data")]
    data: MssgPackDataContainer,
}

fn decode_container(msg: &[u8]) -> Result<MssgPackDataContainer, String> {
    if let Some(response) = parse_msgpack_frame_response(msg) {
        if let WorkerResponse::Ok(ok) = response {
            let payload = crate::handlers::rmpv_to_json(ok.payload);
            if let Ok(container) = serde_json::from_value::<MssgPackDataContainer>(payload.clone())
            {
                return Ok(container);
            }

            if let Ok(wrapper) = serde_json::from_value::<JsonDataField>(payload.clone()) {
                return Ok(wrapper.data);
            }
        }
    }

    if let Ok(container) = rmp_serde::from_slice::<MssgPackDataContainer>(msg) {
        return Ok(container);
    }

    if let Ok(envelope) = rmp_serde::from_slice::<MsgpackWorkerEnvelope>(msg) {
        return Ok(envelope.payload);
    }

    if let Ok(envelope) = rmp_serde::from_slice::<MsgpackWorkerEnvelopeData>(msg) {
        return Ok(envelope.payload.data);
    }

    Err("failed to deserialize veteran payload: expected trained_chara container or worker envelope payload".to_string())
}

pub fn process_group_direct(group: &UmaGroup, conn: &Connection) -> Result<bool, String> {
    let mut group = group.clone();
    process_group(&mut group, conn)
}

fn process_group(group: &mut UmaGroup, conn: &Connection) -> Result<bool, String> {
    for parent in [
        group.grandparent_aa.as_ref(),
        group.grandparent_ab.as_ref(),
        group.grandparent_ba.as_ref(),
        group.grandparent_bb.as_ref(),
        Some(&group.parent_a),
        Some(&group.parent_b),
    ]
    .into_iter()
    .flatten()
    {
        upsert_parent(conn, parent)?;
    }

    let was_new = upsert_veteran(conn, &group.veteran)?;
    if was_new {
        sync_group_relations(conn, &group)?;
        sync_veteran_skills(conn, group.veteran.hash.as_i64(), &group.skills)?;
        for parent in [
            group.grandparent_aa.as_ref(),
            group.grandparent_ab.as_ref(),
            group.grandparent_ba.as_ref(),
            group.grandparent_bb.as_ref(),
            Some(&group.parent_a),
            Some(&group.parent_b),
        ]
        .into_iter()
        .flatten()
        {
            sync_parent_relations(conn, parent)?;
        }
    } else if !group.veteran.is_race_data {
        sync_veteran_relations(conn, &group.veteran)?;
        sync_group_relations(conn, &group)?;
        for parent in [
            group.grandparent_aa.as_ref(),
            group.grandparent_ab.as_ref(),
            group.grandparent_ba.as_ref(),
            group.grandparent_bb.as_ref(),
            Some(&group.parent_a),
            Some(&group.parent_b),
        ]
        .into_iter()
        .flatten()
        {
            sync_parent_relations(conn, parent)?;
        }
    }

    sync_veteran_race_results(conn, group.veteran.hash.as_i64(), &group.race_result_list)?;
    sync_veteran_nickname_ids(conn, group.veteran.hash.as_i64(), &group.nickname_id_array)?;

    if let Some(cards) = &group.support_card_list {
        if cards.len() == 6 {
            sync_veteran_support_cards(conn, group.veteran.hash.as_i64(), cards)?;
        }
    }

    Ok(was_new)
}

fn upsert_parent(conn: &Connection, parent: &Parent) -> Result<(), String> {
    let now = Utc::now().to_rfc3339();

    let owner_id: Option<i64> = parent
        .owner_id
        .map(i64::try_from)
        .transpose()
        .map_err(|_| {
            format!(
                "owner_id out of i64 range for parent {}",
                parent.hash.as_i64()
            )
        })?;

    conn.execute(
        r#"
        INSERT INTO parents (
            hash, trainee_id, rank, rarity, talent_level,
            parent_a, parent_b, owner_id, owned, updated_at
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)
        ON CONFLICT(hash) DO UPDATE SET
            parent_a = COALESCE(parents.parent_a, excluded.parent_a),
            parent_b = COALESCE(parents.parent_b, excluded.parent_b),
            owner_id = COALESCE(parents.owner_id, excluded.owner_id),
            owned = COALESCE(parents.owned, excluded.owned),
            updated_at = excluded.updated_at
        "#,
        params![
            parent.hash.as_i64(),
            parent.trainee_id,
            i64::from(parent.rank),
            i64::from(parent.rarity),
            parent.talent_level.map(i64::from),
            parent.parent_a.map(|hash| hash.as_i64()),
            parent.parent_b.map(|hash| hash.as_i64()),
            owner_id,
            parent.owned,
            now,
        ],
    )
    .map_err(|e| format!("failed to upsert parent {}: {e}", parent.hash.as_i64()))?;

    Ok(())
}

fn sync_parent_relations(conn: &Connection, parent: &Parent) -> Result<(), String> {
    let parent_hash = parent.hash.as_i64();

    sync_relation_table(
        conn,
        "parent_has_spark",
        "parent_hash",
        parent_hash,
        "spark_id",
        &parent.container_sparks,
        "parent sparks",
    )?;
    sync_relation_table(
        conn,
        "parent_has_win",
        "parent_hash",
        parent_hash,
        "win_id",
        &parent.container_major_wins,
        "parent wins",
    )?;

    Ok(())
}

fn upsert_veteran(conn: &Connection, veteran: &Veteran) -> Result<bool, String> {
    let now = Utc::now().to_rfc3339();
    let owner_id_i64 = veteran
        .owner_id
        .map(i64::try_from)
        .transpose()
        .map_err(|_| {
            format!(
                "owner_id out of i64 range for veteran {}",
                veteran.hash.as_i64()
            )
        })?;

    let inserted = conn
        .execute(
            r#"
        INSERT OR IGNORE INTO veterans (
            hash,
            trainee_id,
            scenario,
            favorite_icon_type,
            favorite_memo,
            created_at,
            rank,
            rank_score,
            stat_speed,
            stat_stamina,
            stat_power,
            stat_guts,
            stat_wit,
            aptitude_turf,
            aptitude_dirt,
            aptitude_sprint,
            aptitude_mile,
            aptitude_medium,
            aptitude_long,
            aptitude_front,
            aptitude_pace_chaser,
            aptitude_late_surger,
            aptitude_end_closer,
            parent_a,
            parent_b,
            owner_id,
            is_race_data,
            is_browser,
            active,
            min_hash,
            owned,
            rarity,
            talent_level,
            updated_at,
            trained_chara_id,
            use_type,
            fans,
            succession_num,
            is_saved,
            is_locked,
            chara_grade,
            veteran_running_style,
            nickname_id,
            wins
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10,
            ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19,
            ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27, ?28, ?29, ?30, ?31, ?32, ?33, ?34,
            ?35, ?36, ?37, ?38, ?39, ?40, ?41, ?42, ?43, ?44
        )
        "#,
            params![
                veteran.hash.as_i64(),
                veteran.trainee_id,
                veteran.scenario.map(i64::from),
                veteran.favorite_icon_type.map(i64::from),
                veteran.favorite_memo,
                veteran.created_at.to_rfc3339(),
                i64::from(veteran.rank),
                i64::from(veteran.rank_score),
                veteran.stat_speed.map(i64::from),
                veteran.stat_stamina.map(i64::from),
                veteran.stat_power.map(i64::from),
                veteran.stat_guts.map(i64::from),
                veteran.stat_wit.map(i64::from),
                veteran.aptitude_turf.map(i64::from),
                veteran.aptitude_dirt.map(i64::from),
                veteran.aptitude_sprint.map(i64::from),
                veteran.aptitude_mile.map(i64::from),
                veteran.aptitude_medium.map(i64::from),
                veteran.aptitude_long.map(i64::from),
                veteran.aptitude_front.map(i64::from),
                veteran.aptitude_pace_chaser.map(i64::from),
                veteran.aptitude_late_surger.map(i64::from),
                veteran.aptitude_end_closer.map(i64::from),
                veteran.parent_a.map(|hash| hash.as_i64()),
                veteran.parent_b.map(|hash| hash.as_i64()),
                owner_id_i64,
                i64::from(veteran.is_race_data),
                i64::from(veteran.is_browser),
                1i64,
                veteran.min_hash.map(|h| h.as_i64()),
                veteran.owned,
                i64::from(veteran.rarity),
                veteran.talent_level.map(i64::from),
                now,
                veteran.trained_chara_id.unwrap_or(0),
                veteran.use_type,
                veteran.fans,
                veteran.succession_num,
                veteran.is_saved,
                veteran.is_locked,
                veteran.chara_grade,
                veteran.veteran_running_style,
                veteran.nickname_id,
                veteran.wins,
            ],
        )
        .map_err(|e| format!("failed to upsert veteran {}: {e}", veteran.hash.as_i64()))?;

    if inserted == 0 && veteran.is_browser {
        conn.execute(
            r#"
            UPDATE veterans
            SET favorite_icon_type = ?2,
                favorite_memo = ?3,
                is_browser = MAX(is_browser, ?4),
                active = 1,
                updated_at = ?5
            WHERE hash = ?1
            "#,
            params![
                veteran.hash.as_i64(),
                veteran.favorite_icon_type.map(i64::from),
                veteran.favorite_memo,
                i64::from(veteran.is_browser),
                now,
            ],
        )
        .map_err(|e| {
            format!(
                "failed to update veteran favorite fields {}: {e}",
                veteran.hash.as_i64()
            )
        })?;
    }

    if inserted > 0 {
        sync_veteran_relations(conn, veteran)?;
        Ok(true)
    } else {
        Ok(false)
    }
}

fn sync_veteran_relations(conn: &Connection, veteran: &Veteran) -> Result<(), String> {
    let veteran_hash = veteran.hash.as_i64();

    sync_relation_table(
        conn,
        "veteran_has_spark",
        "veteran_hash",
        veteran_hash,
        "spark_id",
        &veteran.container_sparks,
        "veteran sparks",
    )?;
    sync_relation_table(
        conn,
        "veteran_has_win",
        "veteran_hash",
        veteran_hash,
        "win_id",
        &veteran.container_major_wins,
        "veteran wins",
    )?;

    Ok(())
}

fn sync_veteran_race_results(
    conn: &Connection,
    veteran_hash: i64,
    race_results: &[MssgPackRaceResult],
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_race_results WHERE veteran_hash = ?1",
        params![veteran_hash],
    )
    .map_err(|err| format!("failed to clear veteran race results for {veteran_hash}: {err}"))?;

    let mut stmt = conn
        .prepare(
            r#"
            INSERT INTO veteran_race_results (
                veteran_hash, idx, turn, program_id, weather, ground_condition,
                running_style, popularity, result_rank, result_time, prize_money
            ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
            "#,
        )
        .map_err(|err| format!("failed to prepare insert race results: {err}"))?;

    for (idx, rr) in race_results.iter().enumerate() {
        stmt.execute(params![
            veteran_hash,
            idx as i64,
            rr.turn,
            rr.program_id,
            rr.weather,
            rr.ground_condition,
            rr.running_style,
            rr.popularity,
            rr.result_rank,
            rr.result_time,
            rr.prize_money,
        ])
        .map_err(|err| {
            format!("failed to insert veteran race result {idx} for {veteran_hash}: {err}")
        })?;
    }

    Ok(())
}

fn sync_veteran_nickname_ids(
    conn: &Connection,
    veteran_hash: i64,
    nickname_ids: &[i64],
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_nickname_ids WHERE veteran_hash = ?1",
        params![veteran_hash],
    )
    .map_err(|err| format!("failed to clear veteran nickname ids for {veteran_hash}: {err}"))?;

    let mut stmt = conn
        .prepare(
            "INSERT INTO veteran_nickname_ids (veteran_hash, idx, nickname_id) VALUES (?1, ?2, ?3)",
        )
        .map_err(|err| format!("failed to prepare insert nickname ids: {err}"))?;

    for (idx, nid) in nickname_ids.iter().enumerate() {
        stmt.execute(params![veteran_hash, idx as i64, nid])
            .map_err(|err| {
                format!("failed to insert veteran nickname id {idx} for {veteran_hash}: {err}")
            })?;
    }

    Ok(())
}

fn sync_group_relations(conn: &Connection, group: &UmaGroup) -> Result<(), String> {
    let veteran_hash = group.veteran.hash.as_i64();

    sync_veteran_spark_summary(conn, veteran_hash, group)?;
    sync_veteran_win_count(conn, veteran_hash, group)?;

    Ok(())
}

fn sync_veteran_spark_summary(
    conn: &Connection,
    veteran_hash: i64,
    group: &UmaGroup,
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_spark_summary WHERE veteran_hash = ?1",
        params![veteran_hash],
    )
    .map_err(|err| format!("failed to clear veteran spark summary for {veteran_hash}: {err}"))?;

    let mut veteran_level_sums: HashMap<i64, u16> = HashMap::new();
    for spark in &group.veteran.container_sparks {
        let (group_id, level) = (spark / 100, spark % 100);
        *veteran_level_sums.entry(group_id).or_insert(0u16) += level as u16;
    }

    for (&spark_group_id, &level_sum) in &group.sparks_sum {
        let uma_count = group
            .sparks_count
            .get(&spark_group_id)
            .copied()
            .unwrap_or(0);

        let veteran_level_sum = veteran_level_sums
            .get(&spark_group_id)
            .copied()
            .unwrap_or(0);

        conn.execute(
            r#"
            INSERT OR REPLACE INTO veteran_spark_summary (
                veteran_hash,
                spark_group_id,
                uma_count,
                level_sum,
                veteran_level_sum
            ) VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
            params![
                veteran_hash,
                spark_group_id,
                i64::from(uma_count),
                i64::from(level_sum),
                i64::from(veteran_level_sum),
            ],
        )
        .map_err(|err| {
            format!(
                "failed to upsert veteran spark summary row ({veteran_hash}, {spark_group_id}): {err}"
            )
        })?;
    }

    Ok(())
}

fn sync_veteran_win_count(
    conn: &Connection,
    veteran_hash: i64,
    group: &UmaGroup,
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_win_count WHERE veteran_hash = ?1",
        params![veteran_hash],
    )
    .map_err(|err| format!("failed to clear veteran win counts for {veteran_hash}: {err}"))?;

    let veteran_wins: HashSet<i64> = group.veteran.container_major_wins.iter().copied().collect();

    for (&win_id, &win_count) in &group.wins_count {
        let on_veteran = veteran_wins.contains(&win_id);

        conn.execute(
            r#"
            INSERT OR REPLACE INTO veteran_win_count (
                veteran_hash,
                win_id,
                win_count,
                on_veteran
            ) VALUES (?1, ?2, ?3, ?4)
            "#,
            params![veteran_hash, win_id, i64::from(win_count), on_veteran],
        )
        .map_err(|err| {
            format!("failed to upsert veteran win count row ({veteran_hash}, {win_id}): {err}")
        })?;
    }

    Ok(())
}

fn sync_veteran_support_cards(
    conn: &Connection,
    hash: i64,
    cards: &[MssgPackSupportCard],
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_support_card WHERE veteran_hash = ?1",
        params![hash],
    )
    .map_err(|e| format!("failed to clear support cards for {hash}: {e}"))?;

    if cards.is_empty() {
        return Ok(());
    }

    let mut stmt = conn
        .prepare(
            r#"
            INSERT INTO veteran_support_card (
                veteran_hash, support_card_id, position, exp, limit_break_count
            ) VALUES (?1, ?2, ?3, ?4, ?5)
            "#,
        )
        .map_err(|e| format!("failed to prepare veteran support card insert: {e}"))?;

    for card in cards {
        stmt.execute(params![
            hash,
            card.support_card_id,
            card.position,
            card.exp,
            card.limit_break_count,
        ])
        .map_err(|e| {
            format!(
                "failed to insert veteran support card ({hash}, {}): {e}",
                card.support_card_id
            )
        })?;
    }

    Ok(())
}

fn sync_veteran_skills(
    conn: &Connection,
    hash: i64,
    skills: &[MssgPackSkill],
) -> Result<(), String> {
    conn.execute(
        "DELETE FROM veteran_has_skill WHERE veteran_hash = ?1",
        params![hash],
    )
    .map_err(|e| format!("sync_veteran_skills delete: {e}"))?;

    for s in skills {
        conn.execute(
            "INSERT OR IGNORE INTO veteran_has_skill (veteran_hash, skill_id, level) \
             VALUES (?1, ?2, ?3)",
            params![hash, s.skill_id, s.level],
        )
        .map_err(|e| format!("sync_veteran_skills insert: {e}"))?;
    }
    Ok(())
}

fn sync_relation_table<T>(
    conn: &Connection,
    table_name: &str,
    owner_column: &str,
    owner_value: i64,
    value_column: &str,
    values: &[T],
    label: &str,
) -> Result<(), String>
where
    T: rusqlite::ToSql,
{
    let delete_sql = format!("DELETE FROM {table_name} WHERE {owner_column} = ?1");
    conn.execute(&delete_sql, params![owner_value])
        .map_err(|err| format!("failed to clear {label} for {owner_value}: {err}"))?;

    if values.is_empty() {
        return Ok(());
    }

    let placeholders = values
        .iter()
        .enumerate()
        .map(|(index, _)| format!("(?1, ?{})", index + 2))
        .collect::<Vec<_>>()
        .join(", ");
    let insert_sql = format!(
        "INSERT OR IGNORE INTO {table_name} ({owner_column}, {value_column}) VALUES {placeholders}"
    );
    let mut stmt = conn
        .prepare(&insert_sql)
        .map_err(|err| format!("failed to prepare {label} insert: {err}"))?;

    let mut params: Vec<&dyn rusqlite::ToSql> = vec![&owner_value];
    for value in values {
        params.push(value as &dyn rusqlite::ToSql);
    }

    stmt.execute(params_from_iter(params))
        .map_err(|err| format!("failed to insert {label}: {err}"))?;

    Ok(())
}
