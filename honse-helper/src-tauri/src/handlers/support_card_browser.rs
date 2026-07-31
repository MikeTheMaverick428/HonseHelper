use crate::db::app_db;
use rusqlite::params;
use shared::{
    models::PaginationResponse,
    support_card_browser::{
        SupportCardBrowserQuery, SupportCardDetail, SupportCardEventBranch,
        SupportCardEventChoiceDetail, SupportCardEventDetail, SupportCardEventRewardDetail,
        SupportCardFilterOptions, SupportCardPageItem, SupportCardSkillDetail, BROWSER_TYPE,
    },
    SkillDataRow, SkillType, SupportCardUniqueEffectDetail, UniqueEffectEntry,
};
use std::collections::HashMap;
use std::sync::Mutex;
use tauri::{AppHandle, Manager, State, WebviewUrl, WebviewWindowBuilder};

pub struct SupportCardBrowserConfig {
    pub modes: Mutex<HashMap<String, String>>,
}

const LABEL: &str = "support-card-browser";

#[tauri::command]
pub async fn open_support_card_browser(
    app: AppHandle,
    config: State<'_, SupportCardBrowserConfig>,
    mode: Option<String>,
) -> Result<(), String> {
    {
        let mut modes = config.modes.lock().map_err(|e| e.to_string())?;
        if let Some(m) = &mode {
            modes.insert(LABEL.to_string(), m.clone());
        } else {
            modes.remove(LABEL);
        }
    }

    if let Some(win) = app.get_webview_window(LABEL) {
        let _ = win.set_focus();
        let _ = win.eval("window.location.reload()");
        return Ok(());
    }

    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(LABEL);
    WebviewWindowBuilder::new(&app, LABEL, WebviewUrl::App("index.html".into()))
        .title("Support Card Browser")
        .inner_size(1200.0, 800.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
pub fn get_support_card_browser_mode(
    config: State<'_, SupportCardBrowserConfig>,
    window_label: String,
) -> Result<Option<String>, String> {
    Ok(config
        .modes
        .lock()
        .map_err(|e| e.to_string())?
        .get(&window_label)
        .cloned())
}

#[tauri::command]
pub fn query_support_card_store_page(
    query: SupportCardBrowserQuery,
) -> Result<PaginationResponse<SupportCardPageItem>, String> {
    let conn = app_db::open_app_database_connection()?;
    crate::storage::support_cards::query_support_card_page(&conn, &query)
}

#[tauri::command]
pub fn get_support_card_filter_options() -> Result<SupportCardFilterOptions, String> {
    let conn = app_db::open_app_database_connection()?;

    let rarities = {
        let mut stmt = conn
            .prepare(
                "SELECT DISTINCT rarity FROM support_card_data WHERE rarity > 0 ORDER BY rarity",
            )
            .map_err(|e| format!("rarities prepare failed: {e}"))?;
        let mapped = stmt
            .query_map([], |row| {
                let r: i64 = row.get(0)?;
                let label = match r {
                    1 => "R",
                    2 => "SR",
                    3 => "SSR",
                    _ => "?",
                };
                Ok((r, label.to_string()))
            })
            .map_err(|e| format!("rarities query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("rarities collect failed: {e}"))?
    };

    let card_types = {
        let mut stmt = conn
            .prepare("SELECT DISTINCT card_type FROM support_card_data WHERE card_type > 0 ORDER BY card_type")
            .map_err(|e| format!("card_types prepare failed: {e}"))?;
        let mapped = stmt
            .query_map([], |row| {
                let ct: i64 = row.get(0)?;
                let label = match ct {
                    1 => "Speed",
                    2 => "Stamina",
                    3 => "Power",
                    4 => "Guts",
                    5 => "Wisdom",
                    6 => "Friend",
                    7 => "Group",
                    _ => "Unknown",
                };
                Ok((ct, label.to_string()))
            })
            .map_err(|e| format!("card_types query failed: {e}"))?;
        mapped
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("card_types collect failed: {e}"))?
    };

    let effect_types: Vec<(i64, String)> = (1..=31)
        .filter_map(|id| {
            let etype = shared::models::SupportCardEffectType::from_raw(id);
            if matches!(etype, shared::models::SupportCardEffectType::None) {
                None
            } else {
                Some((id, etype.label().to_string()))
            }
        })
        .collect();

    let characters =
        crate::handlers::veteran_browser::get_id_name_pairs(&conn, "character_data", None)?;

    let skills = {
        let mut stmt = conn
            .prepare("SELECT sd.group_id, sd.id, sd.name, sd.rarity FROM skill_data sd WHERE sd.group_id IS NOT NULL AND (EXISTS (SELECT 1 FROM support_card_has_skill_hint WHERE skill_id = sd.id) OR EXISTS (SELECT 1 FROM support_event_reward WHERE skill_id = sd.id AND is_support_event = 1)) ORDER BY sd.group_id")
            .map_err(|e| format!("skills prepare: {e}"))?;
        let rows = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, i64>(3)?,
                ))
            })
            .map_err(|e| format!("skills query: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("skills collect: {e}"))?;

        let mut groups: std::collections::HashMap<i64, Vec<(i64, String, i64)>> =
            std::collections::HashMap::new();
        for (gid, sid, name, rarity) in rows {
            groups.entry(gid).or_default().push((sid, name, rarity));
        }

        let mut result: Vec<(i64, String)> = Vec::new();
        for (gid, members) in &groups {
            let min_rarity = members.iter().map(|(_, _, r)| *r).min().unwrap_or(0);
            let group_name = members
                .iter()
                .filter(|(_, _, r)| *r == min_rarity)
                .min_by_key(|(id, _, _)| *id)
                .map(|(_, n, _)| n.clone())
                .unwrap_or_default();
            result.push((*gid, group_name));
            if members.len() > 1 {
                for (sid, name, rarity) in members {
                    if *rarity > min_rarity {
                        result.push((-sid, name.clone()));
                    }
                }
            }
        }
        result
    };

    Ok(SupportCardFilterOptions {
        rarities,
        card_types,
        effect_types,
        characters,
        skills,
    })
}

// ── Preset delegation ────────────────────────────────────────────
// The existing veteran_browser preset commands already accept browser_type,
// so the frontend can call them directly with browser_type = "support_card".

#[tauri::command]
pub fn save_support_card_preset(name: String, filters: Option<String>, sort: Option<String>) -> Result<(), String> {
    crate::handlers::veteran_browser::save_preset(name, filters, sort, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn load_support_card_preset_active(
) -> Result<Option<shared::veteran_browser::PresetData>, String> {
    crate::handlers::veteran_browser::load_preset_active(Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn load_support_card_preset(
    name: String,
) -> Result<Option<shared::veteran_browser::PresetData>, String> {
    crate::handlers::veteran_browser::load_preset(name, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn delete_support_card_preset(name: String) -> Result<(), String> {
    crate::handlers::veteran_browser::delete_preset(name, Some(BROWSER_TYPE.into()))
}

#[tauri::command]
pub fn list_support_card_presets(preset_type: Option<String>) -> Result<Vec<String>, String> {
    crate::handlers::veteran_browser::list_presets(Some(BROWSER_TYPE.into()), preset_type)
}

#[tauri::command]
pub fn get_support_card_detail(support_card_id: i64) -> Result<SupportCardDetail, String> {
    let conn = app_db::open_app_database_connection()?;

    let effects = {
        let mut stmt = conn
            .prepare(
                "SELECT support_card_id, effect_type, init_value, lv5, lv10, lv15, lv20, lv25, lv30, lv35, lv40, lv45, lv50 FROM support_card_effect WHERE support_card_id = ?1 ORDER BY effect_type",
            )
            .map_err(|e| format!("effects prepare: {e}"))?;
        let rows = stmt
            .query_map([support_card_id], |row| {
                Ok(shared::SupportCardEffectRow {
                    support_card_id: row.get(0)?,
                    effect_type: row.get(1)?,
                    init_value: row.get(2)?,
                    lv5: row.get(3)?,
                    lv10: row.get(4)?,
                    lv15: row.get(5)?,
                    lv20: row.get(6)?,
                    lv25: row.get(7)?,
                    lv30: row.get(8)?,
                    lv35: row.get(9)?,
                    lv40: row.get(10)?,
                    lv45: row.get(11)?,
                    lv50: row.get(12)?,
                })
            })
            .map_err(|e| format!("effects query: {e}"))?;
        rows.collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("effects collect: {e}"))?
    };

    let unique_effect = {
        let mut stmt = conn
            .prepare(
                "SELECT support_card_id, name, limit_break_level FROM support_card_unique_effect WHERE support_card_id = ?1",
            )
            .map_err(|e| format!("unique prepare: {e}"))?;
        let mut rows = stmt
            .query_map([support_card_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, i64>(2)?,
                ))
            })
            .map_err(|e| format!("unique query: {e}"))?;

        match rows
            .next()
            .transpose()
            .map_err(|e| format!("unique next: {e}"))?
        {
            Some((_, name, lv)) => {
                let mut entry_stmt = conn
                    .prepare("SELECT effect_label, effect_value FROM support_card_unique_effect_entry WHERE support_card_id = ?1 ORDER BY sort_order")
                    .map_err(|e| format!("entries prepare: {e}"))?;
                let entries = entry_stmt
                    .query_map([support_card_id], |row| {
                        Ok(UniqueEffectEntry {
                            effect_label: row.get::<_, String>(0)?,
                            effect_value: row.get::<_, i64>(1)?,
                        })
                    })
                    .map_err(|e| format!("entries query: {e}"))?
                    .collect::<Result<Vec<_>, _>>()
                    .map_err(|e| format!("entries collect: {e}"))?;

                Some(SupportCardUniqueEffectDetail {
                    name,
                    limit_break_level: lv,
                    entries,
                })
            }
            None => None,
        }
    };

    let chara_id: i64 = conn
        .query_row(
            "SELECT character_id FROM support_card_data WHERE id = ?1",
            [support_card_id],
            |row| row.get(0),
        )
        .map_err(|e| format!("chara_id query: {e}"))?;

    let skill_hints = {
        let mut skills = Vec::new();

        let mut stmt = conn
            .prepare(
                "SELECT schsh.skill_id, COALESCE(sd.name, ''), schsh.skill_level, \
                 sd.icon_id, sd.ability_type, sd.target_type, sd.rarity
                 FROM support_card_has_skill_hint schsh
                 LEFT JOIN skill_data sd ON sd.id = schsh.skill_id
                 WHERE schsh.support_card_id = ?1
                 ORDER BY schsh.skill_id",
            )
            .map_err(|e| format!("hints prepare: {e}"))?;
        let rows = stmt
            .query_map([support_card_id], |row| {
                let skill_type = SkillType::from(&SkillDataRow {
                    icon_id: row.get(3)?,
                    ability_type: row.get(4)?,
                    target_type: row.get(5)?,
                    ..Default::default()
                });
                Ok(SupportCardSkillDetail {
                    skill_id: row.get(0)?,
                    skill_name: row.get(1)?,
                    skill_level: row.get(2)?,
                    source: "hint".into(),
                    source_name: String::new(),
                    skill_type: skill_type.label().to_string(),
                    rarity: row.get(6).unwrap_or(1),
                })
            })
            .map_err(|e| format!("hints query: {e}"))?;
        for r in rows {
            skills.push(r.map_err(|e| format!("hint row: {e}"))?);
        }

        let mut evt_stmt = conn
            .prepare(
                "SELECT ser.skill_id, COALESCE(sd.name, ''), ser.size, se.event_name, se.category, \
                 sd.icon_id, sd.ability_type, sd.target_type, sd.rarity
                 FROM support_event_reward ser
                 JOIN support_event_choice sec ON sec.id = ser.choice_id
                 JOIN support_event se ON se.story_id = sec.story_id
                 INNER JOIN support_card_data scd ON scd.id = se.support_card_id
                 LEFT JOIN skill_data sd ON sd.id = ser.skill_id
                 WHERE ser.reward_type = 11 AND ser.skill_id IS NOT NULL
                   AND (se.support_card_id = ?1
                        OR (se.category = 'random' AND scd.character_id = ?2))
                 ORDER BY se.category = 'arrows' DESC, se.story_id",
            )
            .map_err(|e| format!("event skills prepare: {e}"))?;
        let evt_rows = evt_stmt
            .query_map(params![support_card_id, chara_id], |row| {
                let category: String = row.get(4)?;
                let skill_type = SkillType::from(&SkillDataRow {
                    icon_id: row.get(5)?,
                    ability_type: row.get(6)?,
                    target_type: row.get(7)?,
                    ..Default::default()
                });
                Ok(SupportCardSkillDetail {
                    skill_id: row.get(0)?,
                    skill_name: row.get(1)?,
                    skill_level: row.get(2).unwrap_or(0),
                    source: if category == "arrows" {
                        "chain_event".into()
                    } else {
                        "random_event".into()
                    },
                    source_name: row.get(3)?,
                    skill_type: skill_type.label().to_string(),
                    rarity: row.get(8).unwrap_or(1),
                })
            })
            .map_err(|e| format!("event skills query: {e}"))?;
        for r in evt_rows {
            skills.push(r.map_err(|e| format!("event skill row: {e}"))?);
        }

        skills.retain(|s| s.skill_id >= 10000);
        skills
    };

    let events = {
        let mut stmt = conn
            .prepare(
                "SELECT se.story_id, se.event_name, se.category, se.conditions
                 FROM support_event se
                 INNER JOIN support_card_data scd ON scd.id = se.support_card_id
                 WHERE se.support_card_id = ?1
                    OR (se.category = 'random' AND scd.character_id = ?2)
                 ORDER BY se.category = 'arrows' DESC, se.story_id",
            )
            .map_err(|e| format!("events prepare: {e}"))?;
        let mut events = Vec::new();
        let rows = stmt
            .query_map(params![support_card_id, chara_id], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, String>(1)?,
                    row.get::<_, String>(2)?,
                    row.get::<_, Option<String>>(3)?,
                ))
            })
            .map_err(|e| format!("events query: {e}"))?;
        for row in rows {
            let (story_id, event_name, category, conditions) =
                row.map_err(|e| format!("event row: {e}"))?;

            let mut choice_stmt = conn
                .prepare("SELECT id, choice_index FROM support_event_choice WHERE story_id = ?1 ORDER BY choice_index")
                .map_err(|e| format!("choices prepare: {e}"))?;
            let choice_rows = choice_stmt
                .query_map([story_id], |row| {
                    Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
                })
                .map_err(|e| format!("choices query: {e}"))?;

            let mut choices = Vec::new();
            for cr in choice_rows {
                let (choice_id, choice_index) = cr.map_err(|e| format!("choice row: {e}"))?;

                let mut br_stmt = conn
                    .prepare("SELECT id, probability FROM support_event_branch WHERE choice_id = ?1 ORDER BY branch_index")
                    .map_err(|e| format!("branch prepare: {e}"))?;
                let br_rows = br_stmt
                    .query_map([choice_id], |row| {
                        Ok((row.get::<_, i64>(0)?, row.get::<_, Option<String>>(1)?))
                    })
                    .map_err(|e| format!("branch query: {e}"))?;

                let mut branches = Vec::new();
                for br in br_rows {
                    let (branch_id, probability) = br.map_err(|e| format!("branch row: {e}"))?;

                    let mut rew_stmt = conn
                        .prepare("SELECT reward_type, size, skill_id, negative, alternatives, effect_id FROM support_event_reward WHERE branch_id = ?1")
                        .map_err(|e| format!("rewards prepare: {e}"))?;
                    let reward_rows = rew_stmt
                        .query_map([branch_id], |row| {
                            let reward_type: i64 = row.get(0)?;
                            let skill_id: Option<i64> = row.get(2)?;
                            let skill_name: Option<String> = skill_id.and_then(|sid| {
                                conn.query_row(
                                    "SELECT name FROM skill_data WHERE id = ?1",
                                    [sid],
                                    |r| r.get(0),
                                )
                                .ok()
                            });
                            let size: Option<i64> = row.get(1)?;
                            let alternatives_raw: Option<String> = row.get(4)?;
                            let alternatives = alternatives_raw
                                .and_then(|s| serde_json::from_str::<Vec<i64>>(&s).ok());
                            let effect_id: Option<i64> = row.get(5)?;
                            let mut negative: bool = row.get(3)?;
                            let effect_label: Option<String> = effect_id
                                .and_then(|id| shared::models::ScenarioStatus::from_id(id))
                                .map(|s| {
                                    if s.negative() {
                                        negative = true;
                                    }
                                    s.label().to_string()
                                });
                            let reward_label = effect_label.clone().unwrap_or_else(|| {
                                shared::models::RewardType::from_raw(reward_type)
                                    .label()
                                    .to_string()
                            });
                            Ok(SupportCardEventRewardDetail {
                                reward_type,
                                reward_label,
                                size,
                                skill_id,
                                skill_name,
                                negative,
                                alternatives,
                                effect_label,
                            })
                        })
                        .map_err(|e| format!("rewards query: {e}"))?;

                    let rewards: Vec<_> = reward_rows
                        .collect::<Result<_, _>>()
                        .map_err(|e| format!("rewards collect: {e}"))?;

                    branches.push(SupportCardEventBranch {
                        probability,
                        rewards,
                    });
                }

                choices.push(SupportCardEventChoiceDetail {
                    choice_index,
                    branches,
                });
            }

            events.push(SupportCardEventDetail {
                story_id,
                event_name,
                category,
                choices,
                conditions,
            });
        }
        events
    };

    Ok(SupportCardDetail {
        effects,
        unique_effect,
        skill_hints,
        events,
    })
}
