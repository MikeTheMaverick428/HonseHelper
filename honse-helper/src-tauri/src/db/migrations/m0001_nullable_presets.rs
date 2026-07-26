use rusqlite::Connection;
use rusqlite::Result as SqliteResult;

pub fn migrate(conn: &Connection) -> SqliteResult<()> {
    conn.execute_batch(
        r#"
        CREATE TABLE IF NOT EXISTS browser_presets_new (
            browser_type TEXT NOT NULL DEFAULT 'veteran',
            name TEXT NOT NULL,
            filters TEXT,
            sort TEXT,
            created_at TEXT NOT NULL,
            active INTEGER NULL DEFAULT 0,
            PRIMARY KEY (browser_type, name)
        );

        INSERT INTO browser_presets_new (browser_type, name, filters, sort, created_at, active)
        SELECT browser_type, name, filters, sort, created_at, active FROM browser_presets;

        DROP TABLE browser_presets;

        ALTER TABLE browser_presets_new RENAME TO browser_presets;
        "#,
    )
}
