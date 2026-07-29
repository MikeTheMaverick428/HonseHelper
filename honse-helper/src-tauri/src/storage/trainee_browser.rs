use shared::trainee_browser::TraineeFilter;

type SqlParam = Box<dyn rusqlite::types::ToSql>;

pub fn build_filter_where(filters: &[TraineeFilter]) -> (String, Vec<SqlParam>, bool) {
    let mut clauses: Vec<String> = Vec::new();
    let mut params: Vec<SqlParam> = Vec::new();
    let mut needs_stats_join = false;

    for f in filters {
        match f {
            TraineeFilter::Owned { owned } => {
                if *owned {
                    clauses.push("tor.trainee_id IS NOT NULL".into());
                } else {
                    clauses.push("tor.trainee_id IS NULL".into());
                }
            }
            TraineeFilter::GrowthBonus { stat, min_value } => {
                let col = stat.growth_column_name();
                if let Some(min) = min_value {
                    clauses.push(format!("COALESCE(td.{col}, 0) >= ?"));
                    params.push(Box::new(*min));
                } else {
                    clauses.push(format!("COALESCE(td.{col}, 0) > 0"));
                }
            }
            TraineeFilter::MinAptitude {
                category,
                min_level,
            } => {
                needs_stats_join = true;
                let col = category.column_name();
                clauses.push(format!("COALESCE(tsdf.{col}, 0) >= ?"));
                params.push(Box::new(*min_level));
            }
            TraineeFilter::MaxAAptitudes { max_count } => {
                needs_stats_join = true;
                clauses.push(format!(
                    "(CASE WHEN COALESCE(tsdf.aptitude_ground_turf,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_ground_dirt,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_sprint,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_mile,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_medium,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_dist_long,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_front,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_pace_chaser,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_late_surger,0) >= 7 THEN 1 ELSE 0 END \
                     + CASE WHEN COALESCE(tsdf.aptitude_style_end_closer,0) >= 7 THEN 1 ELSE 0 END) <= ?"
                ));
                params.push(Box::new(*max_count));
            }
            TraineeFilter::Character { character_id } => {
                clauses.push("td.character_id = ?".to_string());
                params.push(Box::new(*character_id));
            }
            TraineeFilter::HasSkill { group_id, exact_skill_id, sources } => {
                let mut skill_clauses: Vec<String> = Vec::new();
                let (match_col, match_val): (String, i64) = if let Some(sid) = exact_skill_id {
                    ("sd.id".into(), *sid)
                } else {
                    ("sd.group_id".into(), *group_id)
                };

                if sources.innate {
                    skill_clauses.push(format!(
                        "EXISTS (SELECT 1 FROM trainee_skill ts \
                         JOIN skill_data sd ON sd.id = ts.skill_id \
                         WHERE ts.trainee_id = td.id AND {}= ?)", match_col
                    ));
                    params.push(Box::new(match_val));
                }

                if sources.event || sources.secret {
                    let cat_filter = if sources.event && !sources.secret {
                        "AND se.category != 'secret'"
                    } else if !sources.event && sources.secret {
                        "AND se.category = 'secret'"
                    } else {
                        ""
                    };
                    skill_clauses.push(format!(
                        "EXISTS (SELECT 1 FROM support_event_reward ser \
                         JOIN support_event_choice sec ON sec.id = ser.choice_id \
                         JOIN support_event se ON se.story_id = sec.story_id \
                         JOIN skill_data sd ON sd.id = ser.skill_id \
                         WHERE (se.trainee_id = td.id OR se.character_id = td.character_id) \
                           AND ser.reward_type IN (11, 12) AND {}= ? {})",
                        match_col, cat_filter
                    ));
                    params.push(Box::new(match_val));
                }

                if !skill_clauses.is_empty() {
                    clauses.push(format!("({})", skill_clauses.join(" OR ")));
                }
            }
        }
    }

    if clauses.is_empty() {
        ("1=1".to_string(), params, needs_stats_join)
    } else {
        (clauses.join(" AND "), params, needs_stats_join)
    }
}
