use rusqlite::{Connection, OpenFlags};
use std::fs;
use std::path::PathBuf;

use super::AppDbError;

pub fn app_db_path() -> Result<PathBuf, AppDbError> {
    let data_dir = dirs::data_dir().ok_or(AppDbError::Io(std::io::Error::new(
        std::io::ErrorKind::NotFound,
        "data directory unavailable",
    )))?;
    let db_dir = data_dir.join("honse-helper");
    fs::create_dir_all(&db_dir)?;
    Ok(db_dir.join("honse_helper.db"))
}

pub(super) fn open_app_connection() -> Result<Connection, AppDbError> {
    let path = app_db_path()?;
    let conn = Connection::open_with_flags(
        path,
        OpenFlags::SQLITE_OPEN_READ_WRITE | OpenFlags::SQLITE_OPEN_CREATE,
    )?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}
