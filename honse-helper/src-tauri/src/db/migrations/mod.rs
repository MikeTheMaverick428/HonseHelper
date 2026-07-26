//! Veteran schema migrations.
//!
//! Each migration is a function that transforms the database from version N-1 to N.
//! Version 1 is the initial schema created by `VeteranSchema::create_tables`.
//! Add new migrations here as the schema evolves.

use rusqlite::Connection;
use rusqlite::Result as SqliteResult;

mod m0001_nullable_presets;

pub fn apply_migration(conn: &Connection, to_version: i64) -> SqliteResult<()> {
    match to_version {
        2 => m0001_nullable_presets::migrate(conn),
        _ => Err(rusqlite::Error::InvalidQuery),
    }
}
