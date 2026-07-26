//! Database schema definitions and migrations.
//!
//! Provides SQL table definitions and creation abstractions for veteran data storage.

use rusqlite::Connection;
use rusqlite::OptionalExtension;
use rusqlite::Result as SqliteResult;

pub const VETERAN_SCHEMA_VERSION_KEY: &str = "veteran_schema_version";
pub const CURRENT_VETERAN_SCHEMA_VERSION: i64 = 2;

/// SQL schema for the veterans table.
pub const VETERANS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veterans (
    hash INTEGER PRIMARY KEY NOT NULL,
    trainee_id INTEGER NOT NULL,
    scenario INTEGER,
    favorite_icon_type INTEGER,
    favorite_memo TEXT,
    created_at TEXT NOT NULL,
    rank INTEGER NOT NULL,
    rank_score INTEGER NOT NULL,
    stat_speed INTEGER,
    stat_stamina INTEGER,
    stat_power INTEGER,
    stat_guts INTEGER,
    stat_wit INTEGER,
    aptitude_turf INTEGER,
    aptitude_dirt INTEGER,
    aptitude_sprint INTEGER,
    aptitude_mile INTEGER,
    aptitude_medium INTEGER,
    aptitude_long INTEGER,
    aptitude_front INTEGER,
    aptitude_pace_chaser INTEGER,
    aptitude_late_surger INTEGER,
    aptitude_end_closer INTEGER,
    parent_a INTEGER,
    parent_b INTEGER,
    owner_id INTEGER,
    is_race_data INTEGER NOT NULL DEFAULT 0,
    is_browser INTEGER NOT NULL DEFAULT 0,
    active INTEGER NOT NULL DEFAULT 1,
    min_hash INTEGER NOT NULL,
    owned INTEGER NOT NULL,
    rarity INTEGER,
    talent_level INTEGER,
    updated_at TEXT,
    trained_chara_id INTEGER NOT NULL DEFAULT 0,
    use_type INTEGER NOT NULL DEFAULT 0,
    fans INTEGER NOT NULL DEFAULT 0,
    succession_num INTEGER NOT NULL DEFAULT 0,
    is_saved INTEGER NOT NULL DEFAULT 0,
    is_locked INTEGER NOT NULL DEFAULT 0,
    chara_grade INTEGER NOT NULL DEFAULT 0,
    veteran_running_style INTEGER NOT NULL DEFAULT 0,
    nickname_id INTEGER NOT NULL DEFAULT 0,
    wins INTEGER NOT NULL DEFAULT 0
);
"#;

/// SQL schema for the parents table.
pub const PARENTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS parents (
    hash INTEGER PRIMARY KEY NOT NULL,
    trainee_id INTEGER NOT NULL,
    rank INTEGER NOT NULL,
    rarity INTEGER NOT NULL,
    talent_level INTEGER,
    parent_a INTEGER,
    parent_b INTEGER,
    owner_id INTEGER,
    owned INTEGER NOT NULL DEFAULT 1,
    updated_at TEXT
);
"#;

/// SQL schema for the parent_has_win junction table.
pub const PARENT_HAS_WIN_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS parent_has_win (
    parent_hash INTEGER NOT NULL,
    win_id INTEGER NOT NULL,
    PRIMARY KEY (parent_hash, win_id),
    FOREIGN KEY (parent_hash) REFERENCES parents(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for the parent_has_spark junction table.
pub const PARENT_HAS_SPARK_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS parent_has_spark (
    parent_hash INTEGER NOT NULL,
    spark_id INTEGER NOT NULL,
    PRIMARY KEY (parent_hash, spark_id),
    FOREIGN KEY (parent_hash) REFERENCES parents(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for the veteran_has_win junction table.
pub const VETERAN_HAS_WIN_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_has_win (
    veteran_hash INTEGER NOT NULL,
    win_id INTEGER NOT NULL,
    PRIMARY KEY (veteran_hash, win_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for the veteran_has_spark junction table.
pub const VETERAN_HAS_SPARK_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_has_spark (
    veteran_hash INTEGER NOT NULL,
    spark_id INTEGER NOT NULL,
    PRIMARY KEY (veteran_hash, spark_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for the veteran_has_skill junction table.
pub const VETERAN_HAS_SKILL_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_has_skill (
    veteran_hash INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    level INTEGER NOT NULL DEFAULT 1,
    PRIMARY KEY (veteran_hash, skill_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for per-veteran spark summary rows.
pub const VETERAN_SPARK_SUMMARY_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_spark_summary (
    veteran_hash INTEGER NOT NULL,
    spark_group_id INTEGER NOT NULL,
    uma_count INTEGER NOT NULL,
    level_sum INTEGER NOT NULL,
    veteran_level_sum INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (veteran_hash, spark_group_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for browser presets.
pub const BROWSER_PRESETS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS browser_presets (
    browser_type TEXT NOT NULL DEFAULT 'veteran',
    name TEXT NOT NULL,
    filters TEXT,
    sort TEXT,
    created_at TEXT NOT NULL,
    active INTEGER NULL DEFAULT 0,
    PRIMARY KEY (browser_type, name)
);
"#;

/// SQL schema for legacy planner state persistence.
pub const LEGACY_PLANNER_STATE_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS legacy_planner_state (
    key TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL
);
"#;

/// SQL schema for per-veteran major win count rows.
pub const VETERAN_WIN_COUNT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_win_count (
    veteran_hash INTEGER NOT NULL,
    win_id INTEGER NOT NULL,
    win_count INTEGER NOT NULL,
    on_veteran INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (veteran_hash, win_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for character-trophy junction table.
pub const CHARACTER_HAS_TROPHY_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS character_has_trophy (
    character_id INTEGER NOT NULL,
    trophy_id INTEGER NOT NULL,
    PRIMARY KEY (character_id, trophy_id)
);
"#;

/// SQL schema for race dump sessions.
pub const RACE_DUMP_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS race_dump (
    id                INTEGER PRIMARY KEY AUTOINCREMENT,
    capture_time      TEXT NOT NULL DEFAULT (datetime('now')),
    race_type         INTEGER NOT NULL DEFAULT 0,
    race_instance_id  INTEGER,
    race_id           INTEGER,
    season            INTEGER,
    weather           INTEGER,
    ground_condition  INTEGER,
    distance          INTEGER,
    track_id          INTEGER,
    ground_type       INTEGER,
    turn              INTEGER,
    inout             INTEGER,
    champions_id      INTEGER,
    league_type       INTEGER,
    round             INTEGER,
    frames            TEXT NOT NULL DEFAULT '[]',
    events            TEXT NOT NULL DEFAULT '[]',
    sim_data_base64   TEXT NOT NULL DEFAULT '',
    race_course_set_id INTEGER,
    float_lane_max    INTEGER
);
"#;

/// SQL schema for race dump participant junction table.
/// veteran_hash is NULL for unregistered horses (single-mode NPCs without TrainedCharaData).
pub const RACE_DUMP_PARTICIPANT_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS race_dump_participant (
    race_dump_id      INTEGER NOT NULL,
    horse_index       INTEGER NOT NULL,
    veteran_hash      INTEGER,
    post_number       INTEGER NOT NULL,
    finish_order      INTEGER,
    finish_time       REAL,
    finish_diff_time  REAL,
    popularity        INTEGER,
    running_style     INTEGER,
    viewer_id         INTEGER,
    owner_viewer_id   INTEGER,
    card_id           INTEGER,
    npc_type          INTEGER,
    chara_name        TEXT,
    speed             INTEGER,
    stamina           INTEGER,
    pow               INTEGER,
    guts              INTEGER,
    wiz               INTEGER,
    team_id           INTEGER NOT NULL DEFAULT 0,
    is_player         INTEGER NOT NULL DEFAULT 0,
    response_horse_data TEXT NOT NULL DEFAULT '{}',
    PRIMARY KEY (race_dump_id, horse_index),
    FOREIGN KEY (race_dump_id) REFERENCES race_dump(id) ON DELETE CASCADE
);
"#;

/// SQL schema for trainee-owned cards table.
pub const TRAINEE_OWNED_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS trainee_owned (
    trainee_id INTEGER PRIMARY KEY,
    rarity INTEGER NOT NULL DEFAULT 0,
    talent_level INTEGER NOT NULL DEFAULT 0
);
"#;

/// SQL schema for owned piece/shard counts.
pub const PIECE_OWNED_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS piece_owned (
    trainee_id INTEGER PRIMARY KEY,
    shard_count INTEGER NOT NULL DEFAULT 0
);
"#;

/// SQL schema for owned support cards table.
pub const SUPPORT_CARD_OWNED_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS support_card_owned (
    support_card_id INTEGER PRIMARY KEY,
    level INTEGER NOT NULL DEFAULT 0,
    max_level INTEGER NOT NULL DEFAULT 0,
    exp INTEGER NOT NULL DEFAULT 0,
    limit_break_count INTEGER NOT NULL DEFAULT 0,
    favorite_flag INTEGER NOT NULL DEFAULT 0,
    stock INTEGER NOT NULL DEFAULT 0,
    create_time INTEGER NOT NULL DEFAULT 0,
    possess_time INTEGER NOT NULL DEFAULT 0,
    best_training INTEGER NOT NULL DEFAULT 0
);
"#;

/// SQL schema for reusable tag table.
pub const TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS tag (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    tag_value TEXT NOT NULL UNIQUE,
    create_date TEXT NOT NULL DEFAULT (datetime('now'))
);
"#;

/// SQL schema for veteran-tag junction table.
pub const VETERAN_HAS_TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_has_tag (
    tag_id INTEGER NOT NULL,
    veteran_hash INTEGER NOT NULL,
    PRIMARY KEY (tag_id, veteran_hash),
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE,
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for race_dump-tag junction table.
pub const RACE_DUMP_HAS_TAG_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS race_dump_has_tag (
    tag_id INTEGER NOT NULL,
    race_dump_id INTEGER NOT NULL,
    PRIMARY KEY (tag_id, race_dump_id),
    FOREIGN KEY (tag_id) REFERENCES tag(id) ON DELETE CASCADE,
    FOREIGN KEY (race_dump_id) REFERENCES race_dump(id) ON DELETE CASCADE
);
"#;

/// SQL schema for veteran_support_card junction table — support cards used per training run.
pub const VETERAN_SUPPORT_CARD_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_support_card (
    veteran_hash       INTEGER NOT NULL,
    support_card_id    INTEGER NOT NULL,
    position           INTEGER NOT NULL,
    exp                INTEGER NOT NULL,
    limit_break_count  INTEGER NOT NULL,
    PRIMARY KEY (veteran_hash, support_card_id),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for veteran race results (single-mode race history).
pub const VETERAN_RACE_RESULTS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_race_results (
    veteran_hash    INTEGER NOT NULL,
    idx             INTEGER NOT NULL,
    turn            INTEGER NOT NULL,
    program_id      INTEGER NOT NULL,
    weather         INTEGER NOT NULL,
    ground_condition INTEGER NOT NULL,
    running_style   INTEGER NOT NULL,
    popularity      INTEGER NOT NULL,
    result_rank     INTEGER NOT NULL,
    result_time     INTEGER NOT NULL,
    prize_money     INTEGER NOT NULL,
    PRIMARY KEY (veteran_hash, idx),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// SQL schema for veteran nickname ID array.
pub const VETERAN_NICKNAME_IDS_TABLE: &str = r#"
CREATE TABLE IF NOT EXISTS veteran_nickname_ids (
    veteran_hash INTEGER NOT NULL,
    idx          INTEGER NOT NULL,
    nickname_id  INTEGER NOT NULL,
    PRIMARY KEY (veteran_hash, idx),
    FOREIGN KEY (veteran_hash) REFERENCES veterans(hash) ON DELETE CASCADE
);
"#;

/// Abstraction for database table creation and schema management.
pub struct VeteranSchema;

impl VeteranSchema {
    pub fn current_version() -> i64 {
        CURRENT_VETERAN_SCHEMA_VERSION
    }

    fn ensure_metadata_table(conn: &Connection) -> SqliteResult<()> {
        conn.execute_batch(
            r#"
            CREATE TABLE IF NOT EXISTS db_metadata (
                key TEXT NOT NULL PRIMARY KEY,
                value TEXT NOT NULL,
                created_at TEXT NOT NULL
            );
            "#,
        )?;
        Ok(())
    }

    fn load_stored_schema_version(conn: &Connection) -> SqliteResult<Option<i64>> {
        let version_raw: Option<String> = conn
            .query_row(
                "SELECT value FROM db_metadata WHERE key = ?1 LIMIT 1",
                [VETERAN_SCHEMA_VERSION_KEY],
                |row| row.get(0),
            )
            .optional()?;

        match version_raw {
            Some(value) => value
                .parse::<i64>()
                .map(Some)
                .map_err(|_| rusqlite::Error::InvalidQuery),
            None => Ok(None),
        }
    }

    fn store_schema_version(conn: &Connection, version: i64) -> SqliteResult<()> {
        conn.execute(
            r#"
            INSERT INTO db_metadata (key, value, created_at)
            VALUES (?1, ?2, datetime('now'))
            ON CONFLICT(key) DO UPDATE SET
                value = excluded.value,
                created_at = excluded.created_at
            "#,
            rusqlite::params![VETERAN_SCHEMA_VERSION_KEY, version.to_string()],
        )?;
        Ok(())
    }

    /// Applies a single migration step from version N-1 to N.
    fn apply_migration_step(conn: &Connection, to_version: i64) -> SqliteResult<()> {
        crate::db::migrations::apply_migration(conn, to_version)
    }

    fn migrate_incrementally(
        conn: &Connection,
        from_version: i64,
        to_version: i64,
    ) -> SqliteResult<()> {
        for version in (from_version + 1)..=to_version {
            Self::apply_migration_step(conn, version)?;
        }
        Ok(())
    }

    /// Creates all veteran-related tables if they do not exist.
    ///
    /// # Errors
    /// Returns `rusqlite::Error` if table creation fails.
    pub fn create_tables(conn: &Connection) -> SqliteResult<()> {
        conn.execute_batch(VETERANS_TABLE)?;
        conn.execute_batch(VETERAN_HAS_WIN_TABLE)?;
        conn.execute_batch(VETERAN_HAS_SPARK_TABLE)?;
        conn.execute_batch(VETERAN_SPARK_SUMMARY_TABLE)?;
        conn.execute_batch(VETERAN_HAS_SKILL_TABLE)?;
        conn.execute_batch(VETERAN_WIN_COUNT_TABLE)?;
        conn.execute_batch(PARENTS_TABLE)?;
        conn.execute_batch(PARENT_HAS_WIN_TABLE)?;
        conn.execute_batch(PARENT_HAS_SPARK_TABLE)?;
        conn.execute_batch(BROWSER_PRESETS_TABLE)?;
        conn.execute_batch(LEGACY_PLANNER_STATE_TABLE)?;
        conn.execute_batch(CHARACTER_HAS_TROPHY_TABLE)?;
        conn.execute_batch(TRAINEE_OWNED_TABLE)?;
        conn.execute_batch(PIECE_OWNED_TABLE)?;
        conn.execute_batch(SUPPORT_CARD_OWNED_TABLE)?;
        conn.execute_batch(RACE_DUMP_TABLE)?;
        conn.execute_batch(RACE_DUMP_PARTICIPANT_TABLE)?;
        conn.execute_batch(TAG_TABLE)?;
        conn.execute_batch(VETERAN_HAS_TAG_TABLE)?;
        conn.execute_batch(RACE_DUMP_HAS_TAG_TABLE)?;
        conn.execute_batch(VETERAN_SUPPORT_CARD_TABLE)?;
        conn.execute_batch(VETERAN_RACE_RESULTS_TABLE)?;
        conn.execute_batch(VETERAN_NICKNAME_IDS_TABLE)?;
        Ok(())
    }

    /// Creates indices for common query patterns.
    ///
    /// # Errors
    /// Returns `rusqlite::Error` if index creation fails.
    pub fn create_indices(conn: &Connection) -> SqliteResult<()> {
        conn.execute_batch(
            r#"
            CREATE INDEX IF NOT EXISTS idx_veterans_trainee_id ON veterans(trainee_id);
            CREATE INDEX IF NOT EXISTS idx_veterans_created_at ON veterans(created_at);
            CREATE INDEX IF NOT EXISTS idx_veteran_has_win_win_id ON veteran_has_win(win_id);
            CREATE INDEX IF NOT EXISTS idx_veteran_has_spark_spark_id ON veteran_has_spark(spark_id);
            CREATE INDEX IF NOT EXISTS idx_veteran_spark_summary_group_id ON veteran_spark_summary(spark_group_id);
            CREATE INDEX IF NOT EXISTS idx_veteran_win_count_win_id ON veteran_win_count(win_id);
            CREATE INDEX IF NOT EXISTS idx_parents_trainee_id ON parents(trainee_id);
            CREATE INDEX IF NOT EXISTS idx_parent_has_win_win_id ON parent_has_win(win_id);
            CREATE INDEX IF NOT EXISTS idx_parent_has_spark_spark_id ON parent_has_spark(spark_id);
            CREATE INDEX IF NOT EXISTS idx_veteran_race_results_hash ON veteran_race_results(veteran_hash);
            CREATE INDEX IF NOT EXISTS idx_veteran_nickname_ids_hash ON veteran_nickname_ids(veteran_hash);
            "#,
        )?;
        Ok(())
    }

    /// Initializes the complete schema: creates tables and indices.
    ///
    /// # Errors
    /// Returns `rusqlite::Error` if initialization fails.
    pub fn init(conn: &Connection) -> SqliteResult<()> {
        Self::create_tables(conn)?;
        Self::create_indices(conn)?;
        Ok(())
    }

    /// Ensures veteran schema is at the current version by applying incremental migrations.
    pub fn ensure_current(conn: &Connection) -> SqliteResult<()> {
        Self::ensure_metadata_table(conn)?;

        let current_version = Self::current_version();

        match Self::load_stored_schema_version(conn)? {
            Some(version) if version == current_version => {
                Self::init(conn)?;
            }
            Some(version) if version < current_version => {
                Self::migrate_incrementally(conn, version, current_version)?;
            }
            Some(version) if version > current_version => {
                return Err(rusqlite::Error::InvalidQuery);
            }
            Some(_) => return Err(rusqlite::Error::InvalidQuery),
            None => {
                Self::init(conn)?;
            }
        }

        Self::store_schema_version(conn, current_version)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    #[test]
    fn test_schema_creation() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory DB");
        VeteranSchema::init(&conn).expect("Failed to initialize schema");

        // Verify tables exist
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name IN ('veterans','veteran_has_win','veteran_has_spark','veteran_spark_summary','veteran_win_count','parents','parent_has_win','parent_has_spark','browser_presets','character_has_trophy','trainee_owned','support_card_owned','tag','veteran_has_tag','race_dump_has_tag','veteran_support_card','veteran_race_results','veteran_nickname_ids');")
            .expect("Failed to query tables")
            .query_map([], |row| row.get(0))
            .expect("Failed to map rows")
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to collect tables");

        assert!(tables.contains(&"veterans".to_string()));
        assert!(tables.contains(&"veteran_has_win".to_string()));
        assert!(tables.contains(&"veteran_has_spark".to_string()));
        assert!(tables.contains(&"veteran_spark_summary".to_string()));
        assert!(tables.contains(&"veteran_win_count".to_string()));
        assert!(tables.contains(&"parents".to_string()));
        assert!(tables.contains(&"parent_has_win".to_string()));
        assert!(tables.contains(&"parent_has_spark".to_string()));
        assert!(tables.contains(&"browser_presets".to_string()));
        assert!(tables.contains(&"character_has_trophy".to_string()));
        assert!(tables.contains(&"trainee_owned".to_string()));
        assert!(tables.contains(&"support_card_owned".to_string()));
        assert!(tables.contains(&"tag".to_string()));
        assert!(tables.contains(&"veteran_has_tag".to_string()));
        assert!(tables.contains(&"race_dump_has_tag".to_string()));
        assert!(tables.contains(&"veteran_support_card".to_string()));
        assert!(tables.contains(&"veteran_race_results".to_string()));
        assert!(tables.contains(&"veteran_nickname_ids".to_string()));
    }

    #[test]
    fn test_ensure_current_stores_schema_version_in_metadata() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory DB");

        VeteranSchema::ensure_current(&conn).expect("Failed to ensure current schema");

        let version: String = conn
            .query_row(
                "SELECT value FROM db_metadata WHERE key = ?1",
                [VETERAN_SCHEMA_VERSION_KEY],
                |row| row.get(0),
            )
            .expect("Failed to read veteran schema version");

        assert_eq!(version, VeteranSchema::current_version().to_string());

        // Verify tables were actually created
        let tables: Vec<String> = conn
            .prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='veterans'")
            .expect("Failed to query tables")
            .query_map([], |row| row.get(0))
            .expect("Failed to map rows")
            .collect::<Result<Vec<_>, _>>()
            .expect("Failed to collect tables");

        assert!(tables.contains(&"veterans".to_string()));
    }

    #[test]
    fn test_ensure_current_idempotent_on_existing_schema() {
        let conn = Connection::open_in_memory().expect("Failed to create in-memory DB");

        VeteranSchema::ensure_current(&conn).expect("First call failed");
        VeteranSchema::ensure_current(&conn).expect("Second call (idempotent) failed");

        let version: String = conn
            .query_row(
                "SELECT value FROM db_metadata WHERE key = ?1",
                [VETERAN_SCHEMA_VERSION_KEY],
                |row| row.get(0),
            )
            .expect("Failed to read veteran schema version");

        assert_eq!(version, VeteranSchema::current_version().to_string());
    }
}
