use rusqlite::Connection;
use shared::{
    models::{PaginationResponse, SupportCardEffectType},
    support_card_browser::{
        SupportCardBrowserQuery, SupportCardFilter, SupportCardPageItem, SupportCardSortConfig,
    },
};

type SqlParam = Box<dyn rusqlite::types::ToSql>;

const BASE_COLS: &str = "\
    scd.id AS support_card_id, \
    COALESCE(scd.name, '') AS name, \
    COALESCE(scd.rarity, 0) AS rarity, \
    COALESCE(scd.card_type, 0) AS card_type, \
    COALESCE(sco.level, 0) AS level, \
    COALESCE(sco.max_level, 0) AS max_level, \
    COALESCE(sco.limit_break_count, 0) AS limit_break_count, \
    COALESCE(sco.exp, 0) AS exp, \
    COALESCE(sco.favorite_flag, 0) AS favorite_flag, \
    COALESCE(sco.stock, 0) AS stock, \
    COALESCE(scd.character_id, 0) AS character_id, \
    CASE WHEN sco.support_card_id IS NOT NULL THEN 1 ELSE 0 END AS owned";

const FROM_CLAUSE: &str = "\
    FROM support_card_data scd \
    LEFT JOIN support_card_owned sco ON sco.support_card_id = scd.id";

fn make_page_item(row: &rusqlite::Row) -> rusqlite::Result<SupportCardPageItem> {
    Ok(SupportCardPageItem {
        support_card_id: row.get(0)?,
        name: row.get(1)?,
        rarity: row.get(2)?,
        card_type: row.get(3)?,
        level: row.get(4)?,
        max_level: row.get(5)?,
        limit_break_count: row.get(6)?,
        exp: row.get(7)?,
        favorite_flag: row.get::<_, i64>(8)? != 0,
        stock: row.get(9)?,
        character_id: row.get(10)?,
        owned: row.get::<_, i64>(11)? != 0,
    })
}

fn build_filter_where(filters: &[SupportCardFilter]) -> (String, Vec<SqlParam>) {
    let mut clauses: Vec<String> = Vec::new();
    let mut params: Vec<SqlParam> = Vec::new();

    for f in filters {
        match f {
            SupportCardFilter::Owned { owned } => {
                if *owned {
                    clauses.push("sco.support_card_id IS NOT NULL".into());
                } else {
                    clauses.push("sco.support_card_id IS NULL".into());
                }
            }
            SupportCardFilter::NameSearch { search_text } => {
                if !search_text.is_empty() {
                    clauses.push("scd.name LIKE ?".to_string());
                    params.push(Box::new(format!("%{}%", search_text)));
                }
            }
            SupportCardFilter::Rarity { rarity } => {
                clauses.push("scd.rarity = ?".to_string());
                params.push(Box::new(*rarity));
            }
            SupportCardFilter::CardType { card_type } => {
                clauses.push("scd.card_type = ?".to_string());
                params.push(Box::new(*card_type));
            }
            SupportCardFilter::LimitBreak { min, max } => {
                clauses.push("sco.limit_break_count >= ?".to_string());
                params.push(Box::new(*min));
                clauses.push("sco.limit_break_count <= ?".to_string());
                params.push(Box::new(*max));
            }
            SupportCardFilter::Character { character_id } => {
                clauses.push("scd.character_id = ?".to_string());
                params.push(Box::new(*character_id));
            }
            SupportCardFilter::HasSkill { group_id, exact_skill_id, sources } => {
                let mut skill_clauses: Vec<String> = Vec::new();
                let (match_col, match_val): (String, i64) = if let Some(sid) = exact_skill_id {
                    ("sd.id".into(), *sid)
                } else {
                    ("sd.group_id".into(), *group_id)
                };

                if sources.hint {
                    skill_clauses.push(format!(
                        "EXISTS (SELECT 1 FROM support_card_has_skill_hint schsh \
                         JOIN skill_data sd ON sd.id = schsh.skill_id \
                         WHERE schsh.support_card_id = scd.id AND {}= ?)", match_col
                    ));
                    params.push(Box::new(match_val));
                }

                if sources.chain_event || sources.random_event {
                    let cat_filter = match (sources.chain_event, sources.random_event) {
                        (true, false) => "AND se.category = 'arrows'",
                        (false, true) => "AND se.category = 'random'",
                        _ => "",
                    };
                    skill_clauses.push(format!(
                        "EXISTS (SELECT 1 FROM support_event_reward ser \
                         JOIN support_event_choice sec ON sec.id = ser.choice_id \
                         JOIN support_event se ON se.story_id = sec.story_id \
                         JOIN skill_data sd ON sd.id = ser.skill_id \
                         WHERE se.support_card_id = scd.id \
                           AND ser.reward_type = 11 AND {}= ? {})",
                        match_col, cat_filter
                    ));
                    params.push(Box::new(match_val));
                }

                if !skill_clauses.is_empty() {
                    clauses.push(format!("({})", skill_clauses.join(" OR ")));
                }
            }
            SupportCardFilter::HasEffect { effect_type } => {
                let label = SupportCardEffectType::from_raw(*effect_type).label();
                clauses.push(
                    "(EXISTS (SELECT 1 FROM support_card_effect sce WHERE sce.support_card_id = scd.id AND sce.effect_type = ?) \
                      OR EXISTS (SELECT 1 FROM support_card_unique_effect_entry scue WHERE scue.support_card_id = scd.id AND scue.effect_label LIKE ?))"
                        .to_string(),
                );
                params.push(Box::new(*effect_type));
                params.push(Box::new(format!("%{}%", label)));
            }
        }
    }

    if clauses.is_empty() {
        ("1=1".to_string(), params)
    } else {
        (clauses.join(" AND "), params)
    }
}

fn build_order_clause(sort: &SupportCardSortConfig) -> String {
    let dir = match sort.direction.as_str() {
        "Asc" => "ASC",
        _ => "DESC",
    };
    let col = match sort.key.as_str() {
        "Name" => "scd.name",
        "Rarity" => "scd.rarity",
        "CardType" => "scd.card_type",
        "Level" => "COALESCE(sco.level, 0)",
        _ => "scd.name",
    };
    format!("{} {}", col, dir)
}

pub fn query_support_card_page(
    conn: &Connection,
    query: &SupportCardBrowserQuery,
) -> Result<PaginationResponse<SupportCardPageItem>, String> {
    let (where_clause, where_params) = build_filter_where(&query.filters);
    let order_clause = build_order_clause(&query.sort);

    let count_sql = format!("SELECT COUNT(*) {} WHERE {}", FROM_CLAUSE, where_clause);
    let data_sql = format!(
        "SELECT {} {} WHERE {} ORDER BY {} LIMIT ? OFFSET ?",
        BASE_COLS, FROM_CLAUSE, where_clause, order_clause
    );

    let total: u32 = {
        let mut stmt = conn
            .prepare(&count_sql)
            .map_err(|e| format!("count prepare failed: {e}"))?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            where_params.iter().map(|p| p.as_ref()).collect();
        stmt.query_row(param_refs.as_slice(), |row| row.get::<_, u32>(0))
            .map_err(|e| format!("count query failed: {e}"))?
    };

    let offset = query.page.saturating_sub(1) * query.page_size;
    let mut all_params: Vec<SqlParam> = where_params;
    all_params.push(Box::new(query.page_size));
    all_params.push(Box::new(offset));

    let param_refs: Vec<&dyn rusqlite::types::ToSql> =
        all_params.iter().map(|p| p.as_ref()).collect();

    let mut stmt = conn
        .prepare(&data_sql)
        .map_err(|e| format!("data prepare failed: {e}"))?;
    let rows = stmt
        .query_map(param_refs.as_slice(), make_page_item)
        .map_err(|e| format!("data query failed: {e}"))?;
    let results = rows
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("data collect failed: {e}"))?;

    Ok(PaginationResponse {
        results,
        total,
        page: query.page,
        page_size: query.page_size,
    })
}
