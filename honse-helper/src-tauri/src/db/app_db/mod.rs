mod connection;

use chrono::Utc;
use rusqlite::{params, types::Value, Connection, OpenFlags, OptionalExtension};
use shared::{AppDbSyncReport, AppDbTableSyncState};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter, Manager, State};
use thiserror::Error;

use super::master_db::{current_master_db_path, discover_master_db_path_impl, MasterDbState};
use super::schema::VeteranSchema;
use connection::{app_db_path, open_app_connection};

const APP_DB_SCHEMA_VERSION: i64 = 2;
const APP_VERSION: &str = env!("CARGO_PKG_VERSION");

const APP_DB_SCHEMA_SQL: &str = r#"
CREATE TABLE IF NOT EXISTS db_metadata (
    key TEXT NOT NULL PRIMARY KEY,
    value TEXT NOT NULL,
    created_at TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS db_sync_state (
    table_name TEXT NOT NULL PRIMARY KEY,
    source_table TEXT NOT NULL,
    row_count INTEGER NOT NULL,
    app_version TEXT NOT NULL,
    source_db_path TEXT NOT NULL,
    synced_at TEXT NOT NULL
);

CREATE INDEX IF NOT EXISTS idx_db_sync_state_source_table ON db_sync_state(source_table);
CREATE INDEX IF NOT EXISTS idx_db_metadata_created_at ON db_metadata(created_at);

CREATE TABLE IF NOT EXISTS character_data (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    birth_day INTEGER,
    birth_month INTEGER,
    birth_year INTEGER,
    trainee BOOLEAN NOT NULL DEFAULT FALSE,
    support BOOLEAN NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS skill_data (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT,
    precondition1 TEXT,
    condition1 TEXT,
    precondition2 TEXT,
    condition2 TEXT,
    skill_category INTEGER,
    group_id INTEGER,
    rarity INTEGER,
    icon_id INTEGER,
    ability_type INTEGER,
    target_type INTEGER,
    ability_type_2 INTEGER,
    ability_type_3 INTEGER,
    target_type_2 INTEGER,
    target_type_3 INTEGER,
    effect_value_1 INTEGER,
    effect_value_2 INTEGER,
    effect_value_3 INTEGER,
    target_value_1 INTEGER,
    target_value_2 INTEGER,
    target_value_3 INTEGER,
    effect_duration INTEGER,
    effect_cooldown INTEGER,
    activate_lot INTEGER,
    skill_cost INTEGER
);

CREATE INDEX IF NOT EXISTS idx_skill_category ON skill_data (skill_category);

CREATE TABLE IF NOT EXISTS trainee_data (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    character_id INTEGER NOT NULL,
    growth_rate_spe INTEGER,
    growth_rate_sta INTEGER,
    growth_rate_str INTEGER,
    growth_rate_gut INTEGER,
    growth_rate_wit INTEGER,
    FOREIGN KEY (character_id) REFERENCES character_data(id)
);

CREATE TABLE IF NOT EXISTS spark_data (
    id INTEGER PRIMARY KEY,
    group_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    stars_count INTEGER,
    spark_type INTEGER
);

CREATE TABLE IF NOT EXISTS race_data (
    program_id INTEGER NOT NULL,
    race_id INTEGER NOT NULL,
    race_instance_id INTEGER NOT NULL,
    course_set_id INTEGER,
    race_name TEXT,
    track_name TEXT,
    race_grade INTEGER,
    race_group INTEGER,
    distance INTEGER,
    ground INTEGER,
    program_grade INTEGER,
    PRIMARY KEY (program_id, race_id, race_instance_id)
);

CREATE INDEX IF NOT EXISTS idx_race_instance ON race_data (race_instance_id);
CREATE INDEX IF NOT EXISTS idx_race_grade ON race_data (race_grade);

CREATE TABLE IF NOT EXISTS affinity_member (
    id INTEGER NOT NULL,
    affinity_group INTEGER NOT NULL,
    chara_id INTEGER NOT NULL,
    PRIMARY KEY(id)
);

CREATE INDEX IF NOT EXISTS idx_affinity_member_chara_id ON affinity_member(chara_id);

CREATE TABLE IF NOT EXISTS affinity_groups (
    affinity_group INTEGER NOT NULL,
    affinity_point INTEGER NOT NULL,
    PRIMARY KEY(affinity_group)
);

CREATE TABLE IF NOT EXISTS major_wins_data (
    id INTEGER PRIMARY KEY,
    name TEXT,
    priority INTEGER,
    group_id INTEGER,
    condition INTEGER,
    win_saddle_type INTEGER,
    race_instance_ids TEXT
);

CREATE TABLE IF NOT EXISTS support_card_data (
    id INTEGER PRIMARY KEY,
    character_id INTEGER NOT NULL,
    name TEXT NOT NULL,
    rarity INTEGER NOT NULL,
    card_type INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS trophy_race (
    trophy_id INTEGER PRIMARY KEY,
    trophy_type INTEGER NOT NULL DEFAULT 0,
    trophy_name TEXT NOT NULL DEFAULT '',
    race_grade INTEGER NULL DEFAULT NULL,
    race_instance_ids TEXT NOT NULL DEFAULT '[]'
);

CREATE TABLE IF NOT EXISTS scenario_data (
    id INTEGER PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS trainee_stats_data (
    trainee_id INTEGER NOT NULL,
    rarity INTEGER NOT NULL,
    spe INTEGER NOT NULL,
    sta INTEGER NOT NULL,
    pwr INTEGER NOT NULL,
    gut INTEGER NOT NULL,
    wit INTEGER NOT NULL,
    aptitude_dist_sprint INTEGER NOT NULL,
    aptitude_dist_mile INTEGER NOT NULL,
    aptitude_dist_medium INTEGER NOT NULL,
    aptitude_dist_long INTEGER NOT NULL,
    aptitude_ground_turf INTEGER NOT NULL,
    aptitude_ground_dirt INTEGER NOT NULL,
    aptitude_style_front INTEGER NOT NULL,
    aptitude_style_pace_chaser INTEGER NOT NULL,
    aptitude_style_late_surger INTEGER NOT NULL,
    aptitude_style_end_closer INTEGER NOT NULL,
    unique_skill_id INTEGER,
    unique_skill_level INTEGER,
    PRIMARY KEY (trainee_id, rarity)
);

CREATE TABLE IF NOT EXISTS trainee_skill (
    trainee_id INTEGER NOT NULL,
    skill_id   INTEGER NOT NULL,
    need_rank  INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY (trainee_id, skill_id)
);

CREATE INDEX IF NOT EXISTS idx_trainee_skill_trainee ON trainee_skill(trainee_id);

CREATE TABLE IF NOT EXISTS support_event (
    story_id INTEGER PRIMARY KEY,
    support_card_id INTEGER,
    character_id INTEGER,
    trainee_id INTEGER,
    event_name TEXT NOT NULL,
    category TEXT NOT NULL,
    conditions TEXT
);

CREATE INDEX IF NOT EXISTS idx_support_event_sc ON support_event(support_card_id);
CREATE INDEX IF NOT EXISTS idx_support_event_char ON support_event(character_id);
CREATE INDEX IF NOT EXISTS idx_support_event_trainee ON support_event(trainee_id);

CREATE TABLE IF NOT EXISTS support_event_choice (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    story_id INTEGER NOT NULL,
    choice_index INTEGER NOT NULL,
    FOREIGN KEY (story_id) REFERENCES support_event(story_id)
);

CREATE INDEX IF NOT EXISTS idx_support_event_choice_story ON support_event_choice(story_id);

CREATE TABLE IF NOT EXISTS support_event_branch (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    branch_index INTEGER NOT NULL,
    probability TEXT,
    FOREIGN KEY (choice_id) REFERENCES support_event_choice(id)
);

CREATE INDEX IF NOT EXISTS idx_support_event_branch_choice ON support_event_branch(choice_id);

CREATE TABLE IF NOT EXISTS support_event_reward (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    branch_id INTEGER,
    reward_type INTEGER NOT NULL,
    size INTEGER DEFAULT NULL,
    skill_id INTEGER,
    negative BOOLEAN NOT NULL DEFAULT FALSE,
    alternatives TEXT,
    effect_id INTEGER,
    is_support_event BOOLEAN NOT NULL DEFAULT 0,
    is_trainee_event BOOLEAN NOT NULL DEFAULT 0,
    FOREIGN KEY (choice_id) REFERENCES support_event_choice(id),
    FOREIGN KEY (branch_id) REFERENCES support_event_branch(id)
);

CREATE INDEX IF NOT EXISTS idx_support_event_reward_choice ON support_event_reward(choice_id);
CREATE INDEX IF NOT EXISTS idx_support_event_reward_branch ON support_event_reward(branch_id);

CREATE TABLE IF NOT EXISTS support_card_effect (
    support_card_id INTEGER NOT NULL,
    effect_type INTEGER NOT NULL,
    init_value INTEGER NOT NULL DEFAULT -1,
    lv5 INTEGER NOT NULL DEFAULT -1,
    lv10 INTEGER NOT NULL DEFAULT -1,
    lv15 INTEGER NOT NULL DEFAULT -1,
    lv20 INTEGER NOT NULL DEFAULT -1,
    lv25 INTEGER NOT NULL DEFAULT -1,
    lv30 INTEGER NOT NULL DEFAULT -1,
    lv35 INTEGER NOT NULL DEFAULT -1,
    lv40 INTEGER NOT NULL DEFAULT -1,
    lv45 INTEGER NOT NULL DEFAULT -1,
    lv50 INTEGER NOT NULL DEFAULT -1,
    PRIMARY KEY (support_card_id, effect_type)
);

CREATE TABLE IF NOT EXISTS support_card_unique_effect (
    support_card_id INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    limit_break_level INTEGER NOT NULL
);

CREATE TABLE IF NOT EXISTS support_card_unique_effect_entry (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    support_card_id INTEGER NOT NULL,
    sort_order INTEGER NOT NULL,
    effect_label TEXT NOT NULL,
    effect_value INTEGER NOT NULL DEFAULT 0,
    FOREIGN KEY (support_card_id) REFERENCES support_card_unique_effect(support_card_id)
);
CREATE INDEX IF NOT EXISTS idx_scue_card ON support_card_unique_effect_entry(support_card_id);

CREATE TABLE IF NOT EXISTS support_card_has_skill_hint (
    support_card_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    skill_level INTEGER NOT NULL,
    alt_level INTEGER,
    PRIMARY KEY (support_card_id, skill_id)
);
"#;

pub const SUPPORT_EVENT_DROP_SQL: &str = "\
    DROP TABLE IF EXISTS support_event_reward; \
    DROP TABLE IF EXISTS support_event_branch; \
    DROP TABLE IF EXISTS support_event_choice; \
    DROP TABLE IF EXISTS support_event;";

pub const SUPPORT_EVENT_CREATE_SQL: &str = "\
    CREATE TABLE IF NOT EXISTS support_event (\
        story_id INTEGER PRIMARY KEY, \
        support_card_id INTEGER, \
        character_id INTEGER, \
        trainee_id INTEGER, \
        event_name TEXT NOT NULL, \
        category TEXT NOT NULL, \
        conditions TEXT \
    ); \
    CREATE INDEX IF NOT EXISTS idx_support_event_sc ON support_event(support_card_id); \
    CREATE INDEX IF NOT EXISTS idx_support_event_char ON support_event(character_id); \
    CREATE INDEX IF NOT EXISTS idx_support_event_trainee ON support_event(trainee_id); \
    CREATE TABLE IF NOT EXISTS support_event_choice (\
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        story_id INTEGER NOT NULL, \
        choice_index INTEGER NOT NULL, \
        FOREIGN KEY (story_id) REFERENCES support_event(story_id) \
    ); \
    CREATE INDEX IF NOT EXISTS idx_support_event_choice_story ON support_event_choice(story_id); \
    CREATE TABLE IF NOT EXISTS support_event_branch (\
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        choice_id INTEGER NOT NULL, \
        branch_index INTEGER NOT NULL, \
        probability TEXT, \
        FOREIGN KEY (choice_id) REFERENCES support_event_choice(id) \
    ); \
    CREATE INDEX IF NOT EXISTS idx_support_event_branch_choice ON support_event_branch(choice_id); \
    CREATE TABLE IF NOT EXISTS support_event_reward (\
        id INTEGER PRIMARY KEY AUTOINCREMENT, \
        choice_id INTEGER NOT NULL, \
        branch_id INTEGER, \
        reward_type INTEGER NOT NULL, \
        size INTEGER DEFAULT NULL, \
        skill_id INTEGER, \
        negative BOOLEAN NOT NULL DEFAULT FALSE, \
        alternatives TEXT, \
        effect_id INTEGER, \
        is_support_event BOOLEAN NOT NULL DEFAULT 0, \
        is_trainee_event BOOLEAN NOT NULL DEFAULT 0, \
        FOREIGN KEY (choice_id) REFERENCES support_event_choice(id), \
        FOREIGN KEY (branch_id) REFERENCES support_event_branch(id) \
    ); \
    CREATE INDEX IF NOT EXISTS idx_support_event_reward_choice ON support_event_reward(choice_id); \
    CREATE INDEX IF NOT EXISTS idx_support_event_reward_branch ON support_event_reward(branch_id);";

const APP_METADATA_KEYS: &[(&str, &str)] = &[("app_version", APP_VERSION), ("schema_version", "2")];

#[derive(Debug, Error)]
pub enum AppDbError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

#[derive(Clone, Copy)]
struct TransferJob {
    table_name: &'static str,
    source_table: &'static str,
    select_sql: &'static str,
    insert_sql: &'static str,
}

const TRANSFER_JOBS: &[TransferJob] = &[
    TransferJob {
        table_name: "character_data",
        source_table: "chara_data",
        select_sql: r#"
SELECT
   cd.id,
   td."text" AS name,
   cd.birth_day,
   cd.birth_month,
   cd.birth_year,
   cd2.id IS NOT NULL AS trainee,
   scd.id IS NOT NULL AS support
FROM chara_data cd
LEFT JOIN card_data cd2 ON cd.id = cd2.chara_id
LEFT JOIN support_card_data scd ON cd.id = scd.chara_id
LEFT JOIN text_data td ON td."index" = cd.id AND td.category = 6
GROUP BY cd.id
ORDER BY cd.id;
"#,
        insert_sql: r#"INSERT INTO character_data (id, name, birth_day, birth_month, birth_year, trainee, support) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);"#,
    },
    TransferJob {
        table_name: "skill_data",
        source_table: "skill_data",
        select_sql: r#"
SELECT
    sk."id" AS "id",
    td."text" AS "name",
    td_desc.text AS description,
    sk.precondition_1 AS precondition1,
    sk.condition_1 AS condition1,
    sk.precondition_2 AS precondition2,
    sk.condition_2 AS condition2,
    sk.skill_category AS skill_category,
    sk.group_id AS group_id,
    sk.rarity AS rarity,
    sk.icon_id AS icon_id,
    sk.ability_type_1_1 AS ability_type,
    sk.target_type_1_1 AS target_type,
    sk.ability_type_1_2 AS ability_type_2,
    sk.ability_type_1_3 AS ability_type_3,
    sk.target_type_1_2 AS target_type_2,
    sk.target_type_1_3 AS target_type_3,
    sk.float_ability_value_1_1 AS effect_value_1,
    sk.float_ability_value_1_2 AS effect_value_2,
    sk.float_ability_value_1_3 AS effect_value_3,
    sk.target_value_1_1 AS target_value_1,
    sk.target_value_1_2 AS target_value_2,
    sk.target_value_1_3 AS target_value_3,
    sk.float_ability_time_1 AS effect_duration,
    sk.float_cooldown_time_1 AS effect_cooldown,
    sk.activate_lot AS activate_lot,
    smnp.need_skill_point AS skill_cost
FROM skill_data AS sk
JOIN text_data AS td ON td."index" = sk.id AND td.category = 47
JOIN text_data AS td_desc ON td_desc."index" = sk.id AND td_desc.category = 48
LEFT JOIN single_mode_skill_need_point AS smnp ON smnp.id = sk.id
ORDER BY sk.id;
"#,
        insert_sql: r#"INSERT INTO skill_data (id, name, description, precondition1, condition1, precondition2, condition2, skill_category, group_id, rarity, icon_id, ability_type, target_type, ability_type_2, ability_type_3, target_type_2, target_type_3, effect_value_1, effect_value_2, effect_value_3, target_value_1, target_value_2, target_value_3, effect_duration, effect_cooldown, activate_lot, skill_cost) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22, ?23, ?24, ?25, ?26, ?27);"#,
    },
    TransferJob {
        table_name: "trainee_data",
        source_table: "card_data",
        select_sql: r#"
SELECT
    t."id" AS "id",
    td."text" AS "name",
    t.chara_id AS character_id,
    t.talent_speed AS growth_rate_spe,
    t.talent_stamina AS growth_rate_sta,
    t.talent_pow AS growth_rate_str,
    t.talent_guts AS growth_rate_gut,
    t.talent_wiz AS growth_rate_wit
FROM card_data AS t
JOIN text_data AS td ON td."index" = t.id AND td.category = 4
ORDER BY t.id;
"#,
        insert_sql: r#"INSERT INTO trainee_data (id, name, character_id, growth_rate_spe, growth_rate_sta, growth_rate_str, growth_rate_gut, growth_rate_wit) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8);"#,
    },
    TransferJob {
        table_name: "trainee_stats_data",
        source_table: "card_rarity_data",
        select_sql: r#"
SELECT
    `card_id` as `trainee_id`,
    `rarity`,
    `speed` as `spe`,
    `stamina` as `sta`,
    `pow` as `pwr`,
    `guts` as `gut`,
    `wiz` as `wit`,
    `proper_distance_short` as `aptitude_dist_sprint`,
    `proper_distance_mile` as `aptitude_dist_mile`,
    `proper_distance_middle` as `aptitude_dist_medium`,
    `proper_distance_long` as `aptitude_dist_long`,
    `proper_ground_turf` as `aptitude_ground_turf`,
    `proper_ground_dirt` as `aptitude_ground_dirt`,
    `proper_running_style_nige` as `aptitude_style_front`,
    `proper_running_style_senko` as `aptitude_style_pace_chaser`,
    `proper_running_style_sashi` as `aptitude_style_late_surger`,
    `proper_running_style_oikomi` as `aptitude_style_end_closer`,
    ss.skill_id1 AS unique_skill_id,
    ss.skill_level1 AS unique_skill_level
FROM `card_rarity_data`
LEFT JOIN skill_set ss ON ss.id = `skill_set`
ORDER BY `card_id`, `rarity`;
"#,
        insert_sql: r#"INSERT INTO trainee_stats_data (trainee_id, rarity, spe, sta, pwr, gut, wit, aptitude_dist_sprint, aptitude_dist_mile, aptitude_dist_medium, aptitude_dist_long, aptitude_ground_turf, aptitude_ground_dirt, aptitude_style_front, aptitude_style_pace_chaser, aptitude_style_late_surger, aptitude_style_end_closer, unique_skill_id, unique_skill_level) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19);"#,
    },
    TransferJob {
        table_name: "spark_data",
        source_table: "succession_factor",
        select_sql: r#"
SELECT
    spa."factor_id" AS "id",
    spa."factor_group_id" AS "group_id",
    td."text" AS "name",
    td_desc.text AS description,
    spa.rarity AS stars_count,
    spa.factor_type AS spark_type
FROM succession_factor AS spa
JOIN text_data AS td ON td."index" = spa.factor_id AND td.category = 147
JOIN text_data AS td_desc ON td_desc."index" = spa.factor_id AND td_desc.category = 172
ORDER BY spa.factor_id;
"#,
        insert_sql: r#"INSERT INTO spark_data (id, group_id, name, description, stars_count, spark_type) VALUES (?1, ?2, ?3, ?4, ?5, ?6);"#,
    },
    TransferJob {
        table_name: "race_data",
        source_table: "single_mode_program",
        select_sql: r#"
SELECT
   smp."id" AS program_id,
   r."id" AS race_id,
   ri."id" AS race_instance_id,
   r."course_set" AS course_set_id,
   td."text" AS race_name,
   td_track."text" AS track_name,
   r."grade" AS race_grade,
   r."group" AS race_group,
   rcs."distance" AS distance,
   rcs."ground" AS ground,
   smp."grade_rate_id" AS program_grade
FROM single_mode_program smp
LEFT JOIN race_instance ri ON smp.race_instance_id = ri.id
LEFT JOIN race r ON ri.race_id = r.id
LEFT JOIN race_course_set rcs ON rcs.id = r.course_set
LEFT JOIN text_data td ON r.id = td."index" AND td.category = 32
LEFT JOIN text_data td_track ON td_track."index" = rcs.race_track_id AND td_track.category = 31
ORDER BY smp.id;
"#,
        insert_sql: r#"INSERT INTO race_data (program_id, race_id, race_instance_id, course_set_id, race_name, track_name, race_grade, race_group, distance, ground, program_grade) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);"#,
    },
    // Include TeamStadium/stadium race instances not linked from single_mode_program
    TransferJob {
        table_name: "race_data",
        source_table: "race_instance",
        select_sql: r#"
SELECT
   0 AS program_id,
   r."id" AS race_id,
   ri."id" AS race_instance_id,
   r."course_set" AS course_set_id,
   td."text" AS race_name,
   td_track."text" AS track_name,
   r."grade" AS race_grade,
   r."group" AS race_group,
   rcs."distance" AS distance,
   rcs."ground" AS ground,
   NULL AS program_grade
FROM race_instance ri
LEFT JOIN race r ON ri.race_id = r.id
LEFT JOIN race_course_set rcs ON rcs.id = r.course_set
LEFT JOIN text_data td ON r.id = td."index" AND td.category = 32
LEFT JOIN text_data td_track ON td_track."index" = rcs.race_track_id AND td_track.category = 31
WHERE ri.id NOT IN (SELECT race_instance_id FROM single_mode_program WHERE race_instance_id IS NOT NULL)
ORDER BY ri.id;
"#,
        insert_sql: r#"INSERT INTO race_data (program_id, race_id, race_instance_id, course_set_id, race_name, track_name, race_grade, race_group, distance, ground, program_grade) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11);"#,
    },
    TransferJob {
        table_name: "affinity_member",
        source_table: "succession_relation_member",
        select_sql: r#"
SELECT
    id,
    relation_type as affinity_group,
    chara_id
FROM succession_relation_member
ORDER BY relation_type, id;
"#,
        insert_sql: r#"INSERT INTO affinity_member (id, affinity_group, chara_id) VALUES (?1, ?2, ?3);"#,
    },
    TransferJob {
        table_name: "affinity_groups",
        source_table: "succession_relation",
        select_sql: r#"
SELECT
    relation_type as affinity_group,
    relation_point as affinity_point
FROM succession_relation
ORDER BY relation_type;
"#,
        insert_sql: r#"INSERT INTO affinity_groups (affinity_group, affinity_point) VALUES (?1, ?2);"#,
    },
    TransferJob {
        table_name: "major_wins_data",
        source_table: "single_mode_wins_saddle",
        select_sql: r#"
SELECT
    smw.id,
    td."text" AS name,
    smw.priority,
    smw.group_id,
    smw.condition,
    smw.win_saddle_type,
    '[' || rtrim(
        CASE WHEN smw.race_instance_id_1 != 0 THEN smw.race_instance_id_1 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_2 != 0 THEN smw.race_instance_id_2 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_3 != 0 THEN smw.race_instance_id_3 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_4 != 0 THEN smw.race_instance_id_4 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_5 != 0 THEN smw.race_instance_id_5 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_6 != 0 THEN smw.race_instance_id_6 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_7 != 0 THEN smw.race_instance_id_7 || ',' ELSE '' END ||
        CASE WHEN smw.race_instance_id_8 != 0 THEN smw.race_instance_id_8 || ',' ELSE '' END
        , ',') || ']' AS race_instance_ids
FROM single_mode_wins_saddle smw
JOIN text_data td ON smw.id = td."index" AND td.category = 111
ORDER BY smw.id;
"#,
        insert_sql: r#"INSERT INTO major_wins_data (id, name, priority, group_id, condition, win_saddle_type, race_instance_ids) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7);"#,
    },
    TransferJob {
        table_name: "support_card_data",
        source_table: "support_card_data",
        select_sql: r#"
SELECT
    scd.id,
    scd.chara_id,
    td."text" as "name",
    scd.rarity,
    CASE
        WHEN scg.id IS NOT NULL THEN 7
        WHEN scd.command_id = 101 THEN 1
        WHEN scd.command_id = 102 THEN 3
        WHEN scd.command_id = 103 THEN 4
        WHEN scd.command_id = 105 THEN 2
        WHEN scd.command_id = 106 THEN 5
        WHEN scd.command_id = 0 THEN 6
        ELSE 0
    END as "card_type"
FROM support_card_data scd
LEFT JOIN text_data td ON td."index" = scd.id AND td.category = 75
LEFT JOIN support_card_group scg ON scg.support_card_id = scd.id
GROUP BY scd.id
ORDER BY scd.id;
"#,
        insert_sql: r#"INSERT INTO support_card_data (id, character_id, name, rarity, card_type) VALUES (?1, ?2, ?3, ?4, ?5);"#,
    },
    TransferJob {
        table_name: "scenario_data",
        source_table: "single_mode_scenario",
        select_sql: r#"
SELECT
    sms.id,
    td."text" AS name
FROM single_mode_scenario sms
JOIN text_data td ON sms.id = td."index" AND td.category = 237
ORDER BY sms.id;
"#,
        insert_sql: r#"INSERT INTO scenario_data (id, name) VALUES (?1, ?2);"#,
    },
    TransferJob {
        table_name: "trophy_race",
        source_table: "race_trophy",
        select_sql: r#"
SELECT
    rt.trophy_id,
    CASE
        WHEN MIN(rt.disp_order) < 500 THEN 1
        WHEN MIN(rt.disp_order) < 1000 THEN 2
        ELSE 3
    END AS trophy_type,
    COALESCE(td."text", '') AS trophy_name,
    json_group_array(rt.race_instance_id) AS race_instance_ids,
    r.grade as race_grade
FROM race_trophy rt
LEFT JOIN text_data td ON rt.trophy_id = td."index" AND td.category = 36
LEFT JOIN race_instance ri ON ri.id = rt.race_instance_id 
LEFT JOIN race r ON r.id = ri.race_id
GROUP BY rt.trophy_id
ORDER BY rt.trophy_id;
"#,
        insert_sql: r#"INSERT INTO trophy_race (trophy_id, trophy_type, trophy_name, race_instance_ids, race_grade) VALUES (?1, ?2, ?3, ?4, ?5);"#,
    },
    TransferJob {
        table_name: "support_card_effect",
        source_table: "support_card_effect_table",
        select_sql: r#"
SELECT
    id AS support_card_id,
    type AS effect_type,
    init AS init_value,
    limit_lv5 AS lv5,
    limit_lv10 AS lv10,
    limit_lv15 AS lv15,
    limit_lv20 AS lv20,
    limit_lv25 AS lv25,
    limit_lv30 AS lv30,
    limit_lv35 AS lv35,
    limit_lv40 AS lv40,
    limit_lv45 AS lv45,
    limit_lv50 AS lv50
FROM support_card_effect_table
ORDER BY id, type;
"#,
        insert_sql: r#"INSERT INTO support_card_effect (support_card_id, effect_type, init_value, lv5, lv10, lv15, lv20, lv25, lv30, lv35, lv40, lv45, lv50) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13);"#,
    },
    TransferJob {
        table_name: "support_card_has_skill_hint",
        source_table: "single_mode_hint_gain",
        select_sql: r#"
SELECT
    support_card_id,
    hint_value_1 AS skill_id,
    MIN(hint_value_2) AS skill_level,
    CASE WHEN COUNT(*) > 1 THEN MAX(hint_value_2) ELSE NULL END AS alt_level
FROM single_mode_hint_gain
WHERE hint_value_1 > 0
GROUP BY support_card_id, hint_value_1
ORDER BY support_card_id, hint_value_1;
"#,
        insert_sql: r#"INSERT INTO support_card_has_skill_hint (support_card_id, skill_id, skill_level, alt_level) VALUES (?1, ?2, ?3, ?4);"#,
    },
    TransferJob {
        table_name: "trainee_skill",
        source_table: "available_skill_set",
        select_sql: r#"
SELECT
    cd.id AS trainee_id,
    ass.skill_id,
    ass.need_rank
FROM available_skill_set ass
JOIN card_data cd ON cd.available_skill_set_id = ass.available_skill_set_id
ORDER BY cd.id, ass.need_rank;
"#,
        insert_sql: r#"INSERT INTO trainee_skill (trainee_id, skill_id, need_rank) VALUES (?1, ?2, ?3);"#,
    },
];

#[derive(Debug)]
struct SyncNeed {
    needs_refresh: bool,
    reason: Option<String>,
    current_counts: HashMap<&'static str, i64>,
}

fn open_master_connection(path: &Path) -> Result<Connection, AppDbError> {
    let conn = Connection::open_with_flags(path, OpenFlags::SQLITE_OPEN_READ_ONLY)?;
    conn.execute_batch("PRAGMA foreign_keys = ON;")?;
    Ok(conn)
}

fn init_app_schema(conn: &Connection) -> Result<(), AppDbError> {
    conn.execute_batch(APP_DB_SCHEMA_SQL)?;
    VeteranSchema::ensure_current(conn)?;
    Ok(())
}

fn table_exists(conn: &Connection, table_name: &str) -> Result<bool, AppDbError> {
    let exists = conn
        .query_row(
            "SELECT 1 FROM sqlite_master WHERE type = 'table' AND name = ?1 LIMIT 1",
            params![table_name],
            |_| Ok(()),
        )
        .optional()?
        .is_some();
    Ok(exists)
}

fn current_app_metadata(conn: &Connection, key: &str) -> Result<Option<String>, AppDbError> {
    conn.query_row(
        "SELECT value FROM db_metadata WHERE key = ?1",
        params![key],
        |row| row.get(0),
    )
    .optional()
    .map_err(AppDbError::from)
}

fn upsert_metadata(tx: &Connection, key: &str, value: &str) -> Result<(), AppDbError> {
    let now = Utc::now().to_rfc3339();
    tx.execute(
        "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)\n         ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
        params![key, value, now],
    )?;
    Ok(())
}

fn collect_source_counts(
    master_conn: &Connection,
) -> Result<HashMap<&'static str, i64>, AppDbError> {
    let mut counts = HashMap::new();
    for job in TRANSFER_JOBS {
        let sql = format!("SELECT COUNT(*) FROM {}", job.source_table);
        let count: i64 = master_conn.query_row(&sql, [], |row| row.get(0))?;
        counts.insert(job.table_name, count);
    }
    Ok(counts)
}

fn collect_destination_counts(
    app_conn: &Connection,
) -> Result<HashMap<&'static str, i64>, AppDbError> {
    let mut counts = HashMap::new();
    for job in TRANSFER_JOBS {
        if !table_exists(app_conn, job.table_name)? {
            counts.insert(job.table_name, -1);
            continue;
        }
        let sql = format!("SELECT COUNT(*) FROM {}", job.table_name);
        let count: i64 = app_conn.query_row(&sql, [], |row| row.get(0))?;
        counts.insert(job.table_name, count);
    }
    Ok(counts)
}

fn load_existing_sync_state(
    app_conn: &Connection,
) -> Result<HashMap<String, AppDbTableSyncState>, AppDbError> {
    if !table_exists(app_conn, "db_sync_state")? {
        return Ok(HashMap::new());
    }

    let mut stmt = app_conn.prepare(
        "SELECT table_name, source_table, row_count, app_version, source_db_path, synced_at FROM db_sync_state",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok(AppDbTableSyncState {
            table_name: row.get(0)?,
            source_table: row.get(1)?,
            row_count: row.get(2)?,
            app_version: row.get(3)?,
            source_db_path: row.get(4)?,
            synced_at: row.get(5)?,
        })
    })?;

    let mut state = HashMap::new();
    for row in rows {
        let item = row?;
        state.insert(item.table_name.clone(), item);
    }
    Ok(state)
}

fn should_refresh(
    app_conn: &Connection,
    master_path: &Path,
    source_counts: &HashMap<&'static str, i64>,
) -> Result<SyncNeed, AppDbError> {
    let destination_counts = collect_destination_counts(app_conn)?;
    let existing_state = load_existing_sync_state(app_conn)?;
    let stored_app_version = current_app_metadata(app_conn, "app_version")?;
    let stored_schema_version = current_app_metadata(app_conn, "schema_version")?;

    let mut reason = None;

    if stored_app_version.as_deref() != Some(APP_VERSION) {
        reason = Some(match stored_app_version {
            Some(value) => format!("app version changed from {value} to {APP_VERSION}"),
            None => format!("missing app version metadata; expected {APP_VERSION}"),
        });
    }

    let expected_schema_version = APP_DB_SCHEMA_VERSION.to_string();
    if reason.is_none()
        && stored_schema_version.as_deref() != Some(expected_schema_version.as_str())
    {
        reason = Some(match stored_schema_version {
            Some(value) => {
                format!("schema version changed from {value} to {APP_DB_SCHEMA_VERSION}")
            }
            None => format!("missing schema version metadata; expected {APP_DB_SCHEMA_VERSION}"),
        });
    }

    let unique_table_names: std::collections::HashSet<&str> =
        TRANSFER_JOBS.iter().map(|j| j.table_name).collect();
    if reason.is_none() && existing_state.len() != unique_table_names.len() {
        reason = Some(format!(
            "sync state row count mismatch: expected {}, found {}",
            unique_table_names.len(),
            existing_state.len()
        ));
    }

    if reason.is_none() {
        let mut checked = std::collections::HashSet::new();
        for job in TRANSFER_JOBS {
            if !checked.insert(job.table_name) {
                continue;
            }
            let current_source_count = *source_counts.get(job.table_name).unwrap_or(&-1);
            let current_dest_count = *destination_counts.get(job.table_name).unwrap_or(&-1);

            if current_dest_count < 0 {
                reason = Some(format!("destination table {} is missing", job.table_name));
                break;
            }

            let state = match existing_state.get(job.table_name) {
                Some(state) => state,
                None => {
                    reason = Some(format!("sync state missing for {}", job.table_name));
                    break;
                }
            };

            if state.row_count != current_source_count {
                reason = Some(format!(
                    "{} row count changed: master={} synced={}",
                    job.table_name, current_source_count, state.row_count
                ));
                break;
            }

            if state.app_version != APP_VERSION {
                reason = Some(format!(
                    "{} app version changed from {} to {}",
                    job.table_name, state.app_version, APP_VERSION
                ));
                break;
            }

            if state.source_db_path != master_path.display().to_string() {
                reason = Some(format!("{} source db path changed", job.table_name));
                break;
            }

            if current_source_count > 0 && current_dest_count == 0 {
                reason = Some(format!("{} destination table is empty", job.table_name));
                break;
            }
        }
    }

    Ok(SyncNeed {
        needs_refresh: reason.is_some(),
        reason,
        current_counts: source_counts.clone(),
    })
}

fn delete_and_recreate_tables(tx: &Connection) -> Result<(), AppDbError> {
    for table in [
        "scenario_data",
        "trainee_stats_data",
        "trainee_skill",
        "schedule_race",
        "trophy_race",
        "support_card_data",
        "major_wins_data",
        "affinity_member",
        "affinity_groups",
        "race_data",
        "spark_data",
        "trainee_data",
        "skill_data",
        "character_data",
        "support_card_effect",
        "support_card_unique_effect_entry",
        "support_card_unique_effect",
        "support_card_has_skill_hint",
    ] {
        tx.execute(&format!("DROP TABLE IF EXISTS {table}"), [])?;
    }

    tx.execute_batch(APP_DB_SCHEMA_SQL)?;
    VeteranSchema::init(tx)?;
    Ok(())
}

fn sync_unique_effects(tx: &Connection, master_conn: &Connection) -> Result<i64, AppDbError> {
    fn effect_label(t: i64) -> &'static str {
        let e = shared::models::SupportCardEffectType::from_raw(t);
        if matches!(e, shared::models::SupportCardEffectType::None) {
            ""
        } else {
            e.label()
        }
    }

    let mut stmt = master_conn.prepare(
        "SELECT sue.id, td.text, sue.lv, sue.type_0, sue.value_0,
                sue.value_0_1, sue.value_0_2, sue.value_0_3, sue.value_0_4,
                sue.type_1, sue.value_1,
                sue.value_1_1, sue.value_1_2, sue.value_1_3, sue.value_1_4
         FROM support_card_unique_effect sue
         JOIN text_data td ON td.\"index\" = sue.id AND td.category = 155
         ORDER BY sue.id",
    )?;
    let rows = stmt.query_map([], |row| {
        Ok((
            row.get::<_, i64>(0)?,
            row.get::<_, String>(1)?,
            row.get::<_, i64>(2)?,
            row.get::<_, i64>(3)?,
            row.get::<_, i64>(4)?,
            row.get::<_, i64>(5)?,
            row.get::<_, i64>(6)?,
            row.get::<_, i64>(7)?,
            row.get::<_, i64>(8)?,
            row.get::<_, i64>(9)?,
            row.get::<_, i64>(10)?,
            row.get::<_, i64>(11)?,
            row.get::<_, i64>(12)?,
            row.get::<_, i64>(13)?,
            row.get::<_, i64>(14)?,
        ))
    })?;

    let mut ins_ue = tx.prepare(
        "INSERT INTO support_card_unique_effect (support_card_id, name, limit_break_level) VALUES (?1, ?2, ?3)",
    )?;
    let mut ins_entry = tx.prepare(
        "INSERT INTO support_card_unique_effect_entry (support_card_id, sort_order, effect_label, effect_value) VALUES (?1, ?2, ?3, ?4)",
    )?;

    let mut count: i64 = 0;

    for row in rows {
        let (id, name, lv, t0, v0, s01, s02, s03, s04, t1, v1, s11, s12, s13, s14) = row?;

        let entries: Vec<(&str, i64)> = if t0 == 101 {
            let mut v = Vec::new();
            if s01 > 0 {
                v.push((effect_label(s01), s02));
            }
            if s03 > 0 {
                v.push((effect_label(s03), s04));
            }
            if t1 > 0 {
                v.push((effect_label(t1), v1));
            }
            v
        } else if t0 == 102 {
            vec![("Training Effect", s01)]
        } else if t0 == 103 {
            vec![("Training Effect", s01)]
        } else if t0 == 104 {
            vec![("Training Effect", s01)]
        } else if t0 == 105 {
            vec![("Initial Stats", v0)]
        } else if t0 == 106 {
            vec![("Friendship bonus", v0 * s02)]
        } else if t0 == 107 {
            vec![("Friendship bonus", s02)]
        } else if t0 == 108 {
            vec![("Friendship bonus", s04)]
        } else if t0 == 110 {
            vec![("Friendship bonus", s01 * 5)]
        } else if t0 == 112 {
            vec![("Failure 0% Chance", v0)]
        } else if t0 >= 100 {
            Vec::new()
        } else {
            let mut v = Vec::new();
            if t0 > 0 {
                v.push((effect_label(t0), v0));
            }
            if t1 > 0 {
                v.push((effect_label(t1), v1));
            }
            v
        };

        ins_ue.execute(params![id, name, lv])?;
        for (order, (label, value)) in entries.iter().enumerate() {
            ins_entry.execute(params![id, order as i64, label, value])?;
        }
        count += 1;
    }

    Ok(count)
}

fn transfer_table(
    tx: &Connection,
    master_conn: &Connection,
    job: &TransferJob,
) -> Result<i64, AppDbError> {
    let mut select_stmt = master_conn.prepare(job.select_sql)?;
    let column_count = select_stmt.column_count();
    let mut rows = select_stmt.query([])?;
    let mut insert_stmt = tx.prepare(job.insert_sql)?;
    let mut transferred = 0_i64;

    while let Some(row) = rows.next()? {
        let values: Vec<Value> = (0..column_count)
            .map(|index| row.get(index))
            .collect::<Result<Vec<_>, _>>()?;
        insert_stmt.execute(rusqlite::params_from_iter(values.into_iter()))?;
        transferred += 1;
    }

    Ok(transferred)
}

fn write_sync_state(
    tx: &Connection,
    master_path: &Path,
    counts: &HashMap<&'static str, i64>,
) -> Result<Vec<AppDbTableSyncState>, AppDbError> {
    let now = Utc::now().to_rfc3339();
    let source_db_path = master_path.display().to_string();
    let mut states = Vec::new();

    for job in TRANSFER_JOBS {
        let row_count = *counts.get(job.table_name).unwrap_or(&0);
        let state = AppDbTableSyncState {
            table_name: job.table_name.to_string(),
            source_table: job.source_table.to_string(),
            row_count,
            app_version: APP_VERSION.to_string(),
            source_db_path: source_db_path.clone(),
            synced_at: now.clone(),
        };
        tx.execute(
            "INSERT INTO db_sync_state (table_name, source_table, row_count, app_version, source_db_path, synced_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)\n             ON CONFLICT(table_name) DO UPDATE SET source_table = excluded.source_table, row_count = excluded.row_count, app_version = excluded.app_version, source_db_path = excluded.source_db_path, synced_at = excluded.synced_at",
            params![
                state.table_name,
                state.source_table,
                state.row_count,
                state.app_version,
                state.source_db_path,
                state.synced_at,
            ],
        )?;
        states.push(state);
    }

    Ok(states)
}

fn finalize_metadata(tx: &Connection, master_path: &Path) -> Result<(), AppDbError> {
    let source_db_path = master_path.display().to_string();
    let now = Utc::now().to_rfc3339();

    for (key, value) in APP_METADATA_KEYS {
        upsert_metadata(tx, key, value)?;
    }

    upsert_metadata(tx, "source_db_path", &source_db_path)?;
    upsert_metadata(tx, "last_synced_at", &now)?;
    Ok(())
}

fn build_report(
    synced: bool,
    up_to_date: bool,
    master_path: Option<&Path>,
    refreshed_tables: Vec<String>,
    table_states: Vec<AppDbTableSyncState>,
    message: String,
) -> AppDbSyncReport {
    let db_size_bytes = app_db_path()
        .ok()
        .and_then(|p| std::fs::metadata(p).ok().map(|m| m.len()));
    AppDbSyncReport {
        synced,
        up_to_date,
        app_version: APP_VERSION.to_string(),
        source_db_path: master_path.map(|path| path.display().to_string()),
        refreshed_tables,
        table_states,
        message,
        checked_at: Utc::now().to_rfc3339(),
        db_size_bytes,
    }
}

pub fn sync_app_database_from_state(
    app: &AppHandle,
    master_state: &State<'_, MasterDbState>,
) -> Result<AppDbSyncReport, String> {
    let path = match current_master_db_path(master_state).map_err(|err| err.to_string())? {
        Some(path) => path,
        None => {
            let status =
                discover_master_db_path_impl(app, master_state).map_err(|err| err.to_string())?;
            match status.path {
                Some(path) => PathBuf::from(path),
                None => {
                    let report = build_report(
                        false,
                        false,
                        None,
                        Vec::new(),
                        Vec::new(),
                        "master.db path not available; sync skipped".to_string(),
                    );
                    let _ = app.emit("app-db-sync-status", report.clone());
                    return Ok(report);
                }
            }
        }
    };

    sync_app_database_with_master_path(app, &path)
}

fn migrate_skill_data_columns(
    app_conn: &Connection,
    master_conn: &Connection,
) -> Result<(), AppDbError> {
    let new_columns: &[&str] = &[
        "icon_id",
        "ability_type",
        "target_type",
        "ability_type_2",
        "ability_type_3",
        "target_type_2",
        "target_type_3",
        "effect_value_1",
        "effect_value_2",
        "effect_value_3",
        "target_value_1",
        "target_value_2",
        "target_value_3",
        "effect_duration",
        "effect_cooldown",
        "activate_lot",
        "skill_cost",
    ];
    for column in new_columns {
        let sql = format!("ALTER TABLE skill_data ADD COLUMN {column} INTEGER");
        let _ = app_conn.execute(&sql, []);
    }

    let needs_backfill: bool = app_conn
        .query_row(
            "SELECT COUNT(*) FROM skill_data WHERE icon_id IS NULL OR skill_cost IS NULL LIMIT 1",
            [],
            |r| r.get::<_, i64>(0),
        )
        .map(|c| c > 0)
        .unwrap_or(false);

    if needs_backfill {
        let mut stmt = app_conn.prepare(
            "UPDATE skill_data SET \
             icon_id = ?1, ability_type = ?2, target_type = ?3, \
             ability_type_2 = ?4, ability_type_3 = ?5, \
             target_type_2 = ?6, target_type_3 = ?7, \
             effect_value_1 = ?8, effect_value_2 = ?9, effect_value_3 = ?10, \
             target_value_1 = ?11, target_value_2 = ?12, target_value_3 = ?13, \
             effect_duration = ?14, effect_cooldown = ?15, \
             activate_lot = ?16, skill_cost = ?17 \
             WHERE id = ?18",
        )?;
        let mut master_stmt = master_conn.prepare(
            "SELECT \
             sk.icon_id, sk.ability_type_1_1, sk.target_type_1_1, \
             sk.ability_type_1_2, sk.ability_type_1_3, \
             sk.target_type_1_2, sk.target_type_1_3, \
             sk.float_ability_value_1_1, sk.float_ability_value_1_2, sk.float_ability_value_1_3, \
             sk.target_value_1_1, sk.target_value_1_2, sk.target_value_1_3, \
             sk.float_ability_time_1, sk.float_cooldown_time_1, \
             sk.activate_lot, smnp.need_skill_point \
             FROM skill_data sk \
             LEFT JOIN single_mode_skill_need_point smnp ON smnp.id = sk.id \
             WHERE sk.id = ?1",
        )?;
        let mut app_stmt = app_conn
            .prepare("SELECT id FROM skill_data WHERE icon_id IS NULL OR skill_cost IS NULL")?;

        let ids: Vec<i64> = app_stmt
            .query_map([], |r| r.get(0))?
            .filter_map(|r| r.ok())
            .collect();

        for id in ids {
            if let Ok(row) = master_stmt.query_row(params![id], |r| {
                Ok((
                    r.get::<_, Option<i64>>(0)?,
                    r.get::<_, Option<i64>>(1)?,
                    r.get::<_, Option<i64>>(2)?,
                    r.get::<_, Option<i64>>(3)?,
                    r.get::<_, Option<i64>>(4)?,
                    r.get::<_, Option<i64>>(5)?,
                    r.get::<_, Option<i64>>(6)?,
                    r.get::<_, Option<i64>>(7)?,
                    r.get::<_, Option<i64>>(8)?,
                    r.get::<_, Option<i64>>(9)?,
                    r.get::<_, Option<i64>>(10)?,
                    r.get::<_, Option<i64>>(11)?,
                    r.get::<_, Option<i64>>(12)?,
                    r.get::<_, Option<i64>>(13)?,
                    r.get::<_, Option<i64>>(14)?,
                    r.get::<_, Option<i64>>(15)?,
                    r.get::<_, Option<i64>>(16)?,
                ))
            }) {
                stmt.execute(params![
                    row.0, row.1, row.2, row.3, row.4, row.5, row.6, row.7, row.8, row.9, row.10,
                    row.11, row.12, row.13, row.14, row.15, row.16, id
                ])?;
            }
        }
    }

    Ok(())
}

pub fn sync_app_database_with_master_path(
    app: &AppHandle,
    master_db_path: &Path,
) -> Result<AppDbSyncReport, String> {
    if !master_db_path.exists() {
        let report = build_report(
            false,
            false,
            Some(master_db_path),
            Vec::new(),
            Vec::new(),
            format!("master.db does not exist: {}", master_db_path.display()),
        );
        let _ = app.emit("app-db-sync-status", report.clone());
        return Ok(report);
    }

    let mut app_conn = open_app_connection().map_err(|err| err.to_string())?;
    init_app_schema(&app_conn).map_err(|err| err.to_string())?;
    let master_conn = open_master_connection(master_db_path).map_err(|err| err.to_string())?;

    migrate_skill_data_columns(&app_conn, &master_conn).map_err(|err| err.to_string())?;

    let source_counts = collect_source_counts(&master_conn).map_err(|err| err.to_string())?;
    let need =
        should_refresh(&app_conn, master_db_path, &source_counts).map_err(|err| err.to_string())?;

    if !need.needs_refresh {
        let table_states = load_existing_sync_state(&app_conn)
            .map_err(|err| err.to_string())?
            .into_values()
            .collect::<Vec<_>>();
        let report = build_report(
            false,
            true,
            Some(master_db_path),
            Vec::new(),
            table_states,
            "application database is already synchronized".to_string(),
        );
        let _ = app.emit("app-db-sync-status", report.clone());
        return Ok(report);
    }

    let tx = app_conn.transaction().map_err(|err| err.to_string())?;

    delete_and_recreate_tables(&tx).map_err(|err| err.to_string())?;

    let mut refreshed_tables = Vec::new();
    for job in TRANSFER_JOBS {
        let transferred = transfer_table(&tx, &master_conn, job).map_err(|err| err.to_string())?;
        refreshed_tables.push(format!("{}:{}", job.table_name, transferred));
    }

    let ue_transferred = sync_unique_effects(&tx, &master_conn).map_err(|err| err.to_string())?;
    refreshed_tables.push(format!("support_card_unique_effect:{}", ue_transferred));

    let table_states = write_sync_state(&tx, master_db_path, &need.current_counts)
        .map_err(|err| err.to_string())?;
    finalize_metadata(&tx, master_db_path).map_err(|err| err.to_string())?;

    tx.commit().map_err(|err| err.to_string())?;

    if let Ok(conn) = open_app_connection() {
        if let Some(affinity) = app.try_state::<std::sync::Mutex<crate::storage::affinity::AffinityStorage>>() {
            let _ = affinity.lock().unwrap().load_all(&conn);
        }
        if let Some(store) = app.try_state::<std::sync::Mutex<crate::storage::veterans::VeteranStore>>() {
            let _ = store.lock().unwrap().load_all(&conn);
        }
    }

    let report = build_report(
        true,
        true,
        Some(master_db_path),
        refreshed_tables,
        table_states,
        need.reason
            .unwrap_or_else(|| "application database refreshed".to_string()),
    );
    let _ = app.emit("app-db-sync-status", report.clone());
    Ok(report)
}

pub fn ensure_app_db_schema() -> Result<(), String> {
    let conn = open_app_connection().map_err(|err| err.to_string())?;
    init_app_schema(&conn).map_err(|err| err.to_string())
}

pub fn open_app_database_connection() -> Result<Connection, String> {
    open_app_connection().map_err(|err| err.to_string())
}

#[tauri::command]
pub fn sync_app_database(
    app: AppHandle,
    master_state: State<'_, MasterDbState>,
    path: Option<String>,
) -> Result<AppDbSyncReport, String> {
    if let Some(path) = path {
        return sync_app_database_with_master_path(&app, Path::new(&path));
    }

    sync_app_database_from_state(&app, &master_state)
}

#[tauri::command]
pub fn get_app_db_status() -> Result<Option<AppDbSyncReport>, String> {
    let path = app_db_path().map_err(|e| e.to_string())?;
    if !path.exists() {
        return Ok(None);
    }

    let conn = Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| e.to_string())?;

    if !table_exists(&conn, "db_sync_state").map_err(|e| e.to_string())? {
        return Ok(None);
    }

    let table_states: Vec<AppDbTableSyncState> = load_existing_sync_state(&conn)
        .map_err(|e| e.to_string())?
        .into_values()
        .collect();

    let source_db_path =
        current_app_metadata(&conn, "source_db_path").map_err(|e| e.to_string())?;
    let last_synced_at =
        current_app_metadata(&conn, "last_synced_at").map_err(|e| e.to_string())?;
    let app_version = current_app_metadata(&conn, "app_version").map_err(|e| e.to_string())?;

    let synced = !table_states.is_empty();
    let checked_at = last_synced_at
        .clone()
        .unwrap_or_else(|| "never".to_string());

    let db_size_bytes = std::fs::metadata(&path).ok().map(|m| m.len());

    Ok(Some(AppDbSyncReport {
        synced,
        up_to_date: synced,
        app_version: app_version.unwrap_or_else(|| APP_VERSION.to_string()),
        source_db_path,
        refreshed_tables: Vec::new(),
        table_states,
        message: if synced {
            "application database is synchronized".to_string()
        } else {
            "application database not yet synchronized".to_string()
        },
        checked_at,
        db_size_bytes,
    }))
}
