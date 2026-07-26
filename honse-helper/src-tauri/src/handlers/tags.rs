use crate::db::app_db;
use rusqlite::params;
use shared::veteran_browser::TagRow;

#[tauri::command]
pub fn search_tags(query: String) -> Result<Vec<TagRow>, String> {
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, tag_value, create_date FROM tag WHERE tag_value LIKE ?1 ORDER BY tag_value LIMIT 20")
        .map_err(|e| format!("prepare search_tags failed: {e}"))?;
    let pattern = format!("%{}%", query);
    let rows = stmt
        .query_map(params![pattern], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                tag_value: row.get(1)?,
                create_date: row.get(2)?,
            })
        })
        .map_err(|e| format!("query search_tags failed: {e}"))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| format!("row error: {e}"))?);
    }
    Ok(tags)
}

#[tauri::command]
pub fn add_tag(tag_value: String) -> Result<TagRow, String> {
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "INSERT OR IGNORE INTO tag (tag_value, create_date) VALUES (?1, datetime('now'))",
        params![tag_value],
    )
    .map_err(|e| format!("add_tag insert failed: {e}"))?;
    let tag = conn
        .query_row(
            "SELECT id, tag_value, create_date FROM tag WHERE tag_value = ?1",
            params![tag_value],
            |row| {
                Ok(TagRow {
                    id: row.get(0)?,
                    tag_value: row.get(1)?,
                    create_date: row.get(2)?,
                })
            },
        )
        .map_err(|e| format!("add_tag select failed: {e}"))?;
    Ok(tag)
}

#[tauri::command]
pub fn get_veteran_tags(veteran_hash: String) -> Result<Vec<TagRow>, String> {
    let hash: i64 = veteran_hash
        .parse()
        .map_err(|e| format!("invalid veteran_hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.tag_value, t.create_date FROM tag t \
             JOIN veteran_has_tag vht ON vht.tag_id = t.id \
             WHERE vht.veteran_hash = ?1 \
             ORDER BY t.tag_value",
        )
        .map_err(|e| format!("prepare get_veteran_tags failed: {e}"))?;
    let rows = stmt
        .query_map(params![hash], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                tag_value: row.get(1)?,
                create_date: row.get(2)?,
            })
        })
        .map_err(|e| format!("query get_veteran_tags failed: {e}"))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| format!("row error: {e}"))?);
    }
    Ok(tags)
}

#[tauri::command]
pub fn tag_veteran(tag_id: i64, veteran_hash: String) -> Result<(), String> {
    let hash: i64 = veteran_hash
        .parse()
        .map_err(|e| format!("invalid veteran_hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "INSERT OR IGNORE INTO veteran_has_tag (tag_id, veteran_hash) VALUES (?1, ?2)",
        params![tag_id, hash],
    )
    .map_err(|e| format!("tag_veteran failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn untag_veteran(tag_id: i64, veteran_hash: String) -> Result<(), String> {
    let hash: i64 = veteran_hash
        .parse()
        .map_err(|e| format!("invalid veteran_hash: {e}"))?;
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "DELETE FROM veteran_has_tag WHERE tag_id = ?1 AND veteran_hash = ?2",
        params![tag_id, hash],
    )
    .map_err(|e| format!("untag_veteran failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_race_dump_tags(race_dump_id: i64) -> Result<Vec<TagRow>, String> {
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare(
            "SELECT t.id, t.tag_value, t.create_date FROM tag t \
             JOIN race_dump_has_tag rdht ON rdht.tag_id = t.id \
             WHERE rdht.race_dump_id = ?1 \
             ORDER BY t.tag_value",
        )
        .map_err(|e| format!("prepare get_race_dump_tags failed: {e}"))?;
    let rows = stmt
        .query_map(params![race_dump_id], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                tag_value: row.get(1)?,
                create_date: row.get(2)?,
            })
        })
        .map_err(|e| format!("query get_race_dump_tags failed: {e}"))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| format!("row error: {e}"))?);
    }
    Ok(tags)
}

#[tauri::command]
pub fn tag_race_dump(tag_id: i64, race_dump_id: i64) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "INSERT OR IGNORE INTO race_dump_has_tag (tag_id, race_dump_id) VALUES (?1, ?2)",
        params![tag_id, race_dump_id],
    )
    .map_err(|e| format!("tag_race_dump failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn untag_race_dump(tag_id: i64, race_dump_id: i64) -> Result<(), String> {
    let conn = app_db::open_app_database_connection()?;
    conn.execute(
        "DELETE FROM race_dump_has_tag WHERE tag_id = ?1 AND race_dump_id = ?2",
        params![tag_id, race_dump_id],
    )
    .map_err(|e| format!("untag_race_dump failed: {e}"))?;
    Ok(())
}

#[tauri::command]
pub fn get_all_tags() -> Result<Vec<TagRow>, String> {
    let conn = app_db::open_app_database_connection()?;
    let mut stmt = conn
        .prepare("SELECT id, tag_value, create_date FROM tag ORDER BY tag_value")
        .map_err(|e| format!("prepare get_all_tags failed: {e}"))?;
    let rows = stmt
        .query_map([], |row| {
            Ok(TagRow {
                id: row.get(0)?,
                tag_value: row.get(1)?,
                create_date: row.get(2)?,
            })
        })
        .map_err(|e| format!("query get_all_tags failed: {e}"))?;
    let mut tags = Vec::new();
    for row in rows {
        tags.push(row.map_err(|e| format!("row error: {e}"))?);
    }
    Ok(tags)
}
