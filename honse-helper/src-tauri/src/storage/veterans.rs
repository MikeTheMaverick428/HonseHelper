use std::collections::HashMap;

use nohash_hasher::{IntMap, IntSet};
use rusqlite::Connection;
use shared::{
    filters::{AptitudeType, Filter, SparkFilter},
    legacy_planner::{
        lookup_dtos::{PaginatedVeteranHash, SlimUma, SlimUmaGroup},
        LegacyPlannerSlot, LegacyPlannerState,
    },
    models::INDEPENDENT_LEARNER_NICKNAME,
    veteran_browser::{SortConfig, SparkGroupRow, TagRow, VeteranPageItem, VeteranRow},
};

use crate::storage::affinity::{
    calculate_browser_affinity, calculate_legacy_planner_affinity, AffinityStorage,
};

type SqlParam = Box<dyn rusqlite::types::ToSql>;

const BASE_COLS: &str = "v.hash, v.trainee_id, v.scenario, \
     v.favorite_icon_type, v.favorite_memo, v.created_at, \
     v.rank, v.rank_score, \
     v.stat_speed, v.stat_stamina, v.stat_power, v.stat_guts, v.stat_wit, \
     v.aptitude_turf, v.aptitude_dirt, \
     v.aptitude_sprint, v.aptitude_mile, v.aptitude_medium, v.aptitude_long, \
     v.aptitude_front, v.aptitude_pace_chaser, v.aptitude_late_surger, v.aptitude_end_closer, \
     v.owner_id, v.owned, \
     v.rarity, v.talent_level, \
     td.name AS trainee_name, \
     v.active, \
     v.min_hash";

pub(crate) fn veteran_select_cols() -> String {
    if crate::app_config::win_saddle_version() == 2 {
        format!(
            "{}, \
             (SELECT COUNT(*) FROM veteran_win_count vw \
              JOIN major_wins_data mwd ON mwd.id = vw.win_id \
              WHERE vw.veteran_hash = v.hash AND mwd.win_saddle_type = 3) AS major_wins_count, \
             (SELECT COUNT(*) FROM veteran_win_count vwc \
              JOIN major_wins_data mwd ON mwd.id = vwc.win_id \
              WHERE vwc.veteran_hash = v.hash AND vwc.on_veteran != 0 AND mwd.win_saddle_type = 3) AS major_wins_on_veteran_count, \
             (SELECT COUNT(DISTINCT vss.spark_group_id) FROM veteran_spark_summary vss \
              JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
              WHERE vss.veteran_hash = v.hash AND sd.spark_type IN (4,5)) AS white_spark_count, \
             (SELECT COUNT(DISTINCT vhs.spark_id / 100) FROM veteran_has_spark vhs \
              JOIN spark_data sd ON sd.group_id = vhs.spark_id / 100 \
              WHERE vhs.veteran_hash = v.hash AND sd.spark_type IN (4,5)) AS white_spark_on_veteran_count, \
             NULLIF(v.nickname_id, 0) AS nickname_id",
            BASE_COLS
        )
    } else {
        format!(
            "{}, \
             (SELECT COUNT(*) FROM veteran_win_count vw WHERE vw.veteran_hash = v.hash) AS major_wins_count, \
             (SELECT COUNT(*) FROM veteran_win_count vwc WHERE vwc.veteran_hash = v.hash AND vwc.on_veteran != 0) AS major_wins_on_veteran_count, \
             (SELECT COUNT(DISTINCT vss.spark_group_id) FROM veteran_spark_summary vss \
              JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
              WHERE vss.veteran_hash = v.hash AND sd.spark_type IN (4,5)) AS white_spark_count, \
             (SELECT COUNT(DISTINCT vhs.spark_id / 100) FROM veteran_has_spark vhs \
              JOIN spark_data sd ON sd.group_id = vhs.spark_id / 100 \
              WHERE vhs.veteran_hash = v.hash AND sd.spark_type IN (4,5)) AS white_spark_on_veteran_count, \
             NULLIF(v.nickname_id, 0) AS nickname_id",
            BASE_COLS
        )
    }
}

pub(crate) const VETERAN_FROM: &str =
    "FROM veterans v LEFT JOIN trainee_data td ON td.id = v.trainee_id";

pub(crate) fn make_veteran_row(row: &rusqlite::Row) -> rusqlite::Result<VeteranRow> {
    Ok(VeteranRow {
        hash: row.get(0)?,
        trainee_id: row.get(1)?,
        scenario: row.get(2)?,
        favorite_icon_type: row.get(3)?,
        favorite_memo: row.get(4)?,
        created_at: row.get(5)?,
        rank: row.get(6)?,
        rank_score: row.get(7)?,
        stat_speed: row.get(8)?,
        stat_stamina: row.get(9)?,
        stat_power: row.get(10)?,
        stat_guts: row.get(11)?,
        stat_wit: row.get(12)?,
        aptitude_turf: row.get(13)?,
        aptitude_dirt: row.get(14)?,
        aptitude_sprint: row.get(15)?,
        aptitude_mile: row.get(16)?,
        aptitude_medium: row.get(17)?,
        aptitude_long: row.get(18)?,
        aptitude_front: row.get(19)?,
        aptitude_pace_chaser: row.get(20)?,
        aptitude_late_surger: row.get(21)?,
        aptitude_end_closer: row.get(22)?,
        owner_id: row.get(23)?,
        owned: row.get(24)?,
        rarity: row.get(25)?,
        talent_level: row.get(26)?,
        trainee_name: row.get(27)?,
        active: row.get(28)?,
        min_hash: row.get(29)?,
        major_wins_count: row.get(30)?,
        major_wins_on_veteran_count: row.get(31)?,
        white_spark_count: row.get(32)?,
        white_spark_on_veteran_count: row.get(33)?,
        nickname_id: row.get(34)?,
        spark_groups: Vec::new(),
        affinity: None,
    })
}

pub struct VeteranStore {
    umas: IntMap<u64, SlimUmaGroup>,
    parents: IntMap<u64, SlimUma>,
    sorted_hashes: Vec<PaginatedVeteranHash>,
    current_filters: Vec<Filter>,
    current_sort: SortConfig,
    current_legacy_planner_state: Option<LegacyPlannerState>,
    current_legacy_planner_slot: Option<LegacyPlannerSlot>,
}

impl VeteranStore {
    pub fn new() -> Self {
        Self {
            umas: IntMap::default(),
            parents: IntMap::default(),
            sorted_hashes: Vec::new(),
            current_filters: Vec::new(),
            current_sort: SortConfig::default(),
            current_legacy_planner_state: None,
            current_legacy_planner_slot: None,
        }
    }

    /// Load all veterans + their parents into in-memory SlimUmaGroups.
    pub fn load_all(&mut self, conn: &Connection) -> Result<(), String> {
        // Query 1: all veterans with character_ids and parent refs
        let mut stmt = conn
            .prepare(
                "SELECT v.hash, COALESCE(td.character_id, 0), v.parent_a, v.parent_b
                 FROM veterans v
                 LEFT JOIN trainee_data td ON td.id = v.trainee_id
                 WHERE v.active = 1 AND v.is_browser = 1",
            )
            .map_err(|e| format!("prepare veterans query failed: {e}"))?;

        let rows: Vec<(i64, i64, Option<i64>, Option<i64>)> = stmt
            .query_map([], |row| {
                Ok((
                    row.get::<_, i64>(0)?,
                    row.get::<_, i64>(1)?,
                    row.get::<_, Option<i64>>(2)?,
                    row.get::<_, Option<i64>>(3)?,
                ))
            })
            .map_err(|e| format!("query veterans failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("collect veterans failed: {e}"))?;

        // Collect all referenced parent hashes
        let mut parent_hashes: Vec<i64> = Vec::new();
        let mut hash_to_row: HashMap<i64, (i64, Option<i64>, Option<i64>)> = HashMap::new();
        for (hash, char_id, pa, pb) in &rows {
            hash_to_row.insert(*hash, (*char_id, *pa, *pb));
            if let Some(h) = pa {
                parent_hashes.push(*h);
            }
            if let Some(h) = pb {
                parent_hashes.push(*h);
            }
        }
        parent_hashes.sort();
        parent_hashes.dedup();

        // Query 2: all veteran wins
        let mut veteran_wins: IntMap<u64, IntSet<u32>> = IntMap::default();
        {
            let win_sql = if crate::app_config::win_saddle_version() == 2 {
                "SELECT vwc.veteran_hash, vwc.win_id FROM veteran_win_count vwc \
                 JOIN major_wins_data mwd ON mwd.id = vwc.win_id \
                 WHERE mwd.win_saddle_type = 3"
            } else {
                "SELECT veteran_hash, win_id FROM veteran_win_count"
            };
            let mut win_stmt = conn
                .prepare(win_sql)
                .map_err(|e| format!("prepare wins query failed: {e}"))?;
            let win_rows: Vec<(i64, i32)> = win_stmt
                .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i32>(1)?)))
                .map_err(|e| format!("query wins failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("collect wins failed: {e}"))?;
            for (hash, win_id) in win_rows {
                veteran_wins
                    .entry(hash as u64)
                    .or_default()
                    .insert(win_id as u32);
            }
        }

        // Query 3: parent character_ids
        let mut parent_chars: IntMap<u64, i64> = IntMap::default();
        if !parent_hashes.is_empty() {
            let placeholders: Vec<String> = parent_hashes.iter().map(|_| "?".to_string()).collect();
            let sql = format!(
                "SELECT p.hash, COALESCE(td.character_id, 0)
                 FROM parents p
                 LEFT JOIN trainee_data td ON td.id = p.trainee_id
                 WHERE p.hash IN ({})",
                placeholders.join(",")
            );
            let mut parent_stmt = conn
                .prepare(&sql)
                .map_err(|e| format!("prepare parents query failed: {e}"))?;
            let parent_params: Vec<&dyn rusqlite::types::ToSql> = parent_hashes
                .iter()
                .map(|h| h as &dyn rusqlite::types::ToSql)
                .collect();
            let parent_rows: Vec<(i64, i64)> = parent_stmt
                .query_map(parent_params.as_slice(), |row| {
                    Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
                })
                .map_err(|e| format!("query parents failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("collect parents failed: {e}"))?;
            for (hash, char_id) in parent_rows {
                parent_chars.insert(hash as u64, char_id);
            }
        }

        // Query 4: all parent wins
        let mut parent_wins: IntMap<u64, IntSet<u32>> = IntMap::default();
        {
            let pwin_sql = if crate::app_config::win_saddle_version() == 2 {
                "SELECT phw.parent_hash, phw.win_id FROM parent_has_win phw \
                 JOIN major_wins_data mwd ON mwd.id = phw.win_id \
                 WHERE mwd.win_saddle_type = 3"
            } else {
                "SELECT parent_hash, win_id FROM parent_has_win"
            };
            let mut pwin_stmt = conn
                .prepare(pwin_sql)
                .map_err(|e| format!("prepare parent wins query failed: {e}"))?;
            let pwin_rows: Vec<(i64, i32)> = pwin_stmt
                .query_map([], |row| Ok((row.get::<_, i64>(0)?, row.get::<_, i32>(1)?)))
                .map_err(|e| format!("query parent wins failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("collect parent wins failed: {e}"))?;
            for (hash, win_id) in pwin_rows {
                parent_wins
                    .entry(hash as u64)
                    .or_default()
                    .insert(win_id as u32);
            }
        }

        // Assemble SlimUmaGroups
        self.umas.clear();
        self.parents.clear();
        for (hash, char_id, pa_opt, pb_opt) in rows {
            let hash = hash as u64;
            let pa_hash = pa_opt.unwrap_or(0) as u64;
            let pb_hash = pb_opt.unwrap_or(0) as u64;

            let veteran = SlimUma {
                hash,
                character_id: char_id,
                wins: veteran_wins.remove(&hash).unwrap_or_default(),
            };
            let parent_a = SlimUma {
                hash: pa_hash,
                character_id: parent_chars.get(&pa_hash).copied().unwrap_or(0),
                wins: parent_wins.get(&pa_hash).cloned().unwrap_or_default(),
            };
            let parent_b = SlimUma {
                hash: pb_hash,
                character_id: parent_chars.get(&pb_hash).copied().unwrap_or(0),
                wins: parent_wins.get(&pb_hash).cloned().unwrap_or_default(),
            };

            self.umas.insert(
                hash,
                SlimUmaGroup {
                    veteran,
                    parent_a: pa_hash,
                    parent_b: pb_hash,
                },
            );

            if pa_hash != 0 {
                self.parents.insert(pa_hash, parent_a);
            }
            if pb_hash != 0 {
                self.parents.insert(pb_hash, parent_b);
            }
        }

        Ok(())
    }

    /// Load veterans missing from the in-memory cache.
    fn ensure_loaded(
        &mut self,
        conn: &Connection,
        hashes: impl Iterator<Item = u64>,
    ) -> Result<(), String> {
        let missing: Vec<u64> = hashes.filter(|h| !self.umas.contains_key(h)).collect();
        if missing.is_empty() {
            return Ok(());
        }

        let placeholders: Vec<String> = missing.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT v.hash, COALESCE(td.character_id, 0), v.parent_a, v.parent_b
             FROM veterans v
             LEFT JOIN trainee_data td ON td.id = v.trainee_id
             WHERE v.hash IN ({})",
            placeholders.join(",")
        );
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("ensure_loaded prepare failed: {e}"))?;
        let hash_i64: Vec<i64> = missing.iter().map(|h| *h as i64).collect();
        let hash_params: Vec<&dyn rusqlite::types::ToSql> = hash_i64
            .iter()
            .map(|h| h as &dyn rusqlite::types::ToSql)
            .collect();
        let rows: Vec<(i64, i64, Option<i64>, Option<i64>)> = stmt
            .query_map(hash_params.as_slice(), |row| {
                Ok((row.get(0)?, row.get(1)?, row.get(2)?, row.get(3)?))
            })
            .map_err(|e| format!("ensure_loaded query failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("ensure_loaded collect failed: {e}"))?;

        let wins_sql = if crate::app_config::win_saddle_version() == 2 {
            format!(
                "SELECT vwc.veteran_hash, vwc.win_id FROM veteran_win_count vwc \
                 JOIN major_wins_data mwd ON mwd.id = vwc.win_id \
                 WHERE vwc.veteran_hash IN ({}) AND mwd.win_saddle_type = 3",
                placeholders.join(",")
            )
        } else {
            format!(
                "SELECT veteran_hash, win_id FROM veteran_win_count WHERE veteran_hash IN ({})",
                placeholders.join(",")
            )
        };
        let mut wins_stmt = conn
            .prepare(&wins_sql)
            .map_err(|e| format!("ensure_loaded wins prepare failed: {e}"))?;
        let wins_hash_params: Vec<&dyn rusqlite::types::ToSql> = hash_i64
            .iter()
            .map(|h| h as &dyn rusqlite::types::ToSql)
            .collect();
        let win_rows: Vec<(i64, i32)> = wins_stmt
            .query_map(wins_hash_params.as_slice(), |row| {
                Ok((row.get(0)?, row.get(1)?))
            })
            .map_err(|e| format!("ensure_loaded wins query failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("ensure_loaded wins collect failed: {e}"))?;

        let mut wins_by_hash: IntMap<u64, IntSet<u32>> = IntMap::default();
        for (hash, win_id) in &win_rows {
            wins_by_hash
                .entry(*hash as u64)
                .or_default()
                .insert(*win_id as u32);
        }

        for (hash, char_id, pa, pb) in &rows {
            let hash = *hash as u64;
            let pa_hash = pa.unwrap_or(0) as u64;
            let pb_hash = pb.unwrap_or(0) as u64;

            let veteran = SlimUma {
                hash,
                character_id: *char_id,
                wins: wins_by_hash.remove(&hash).unwrap_or_default(),
            };
            self.umas.insert(
                hash,
                SlimUmaGroup {
                    veteran,
                    parent_a: pa_hash,
                    parent_b: pb_hash,
                },
            );
        }

        let missing_parents: Vec<u64> = rows
            .iter()
            .flat_map(|(_, _, pa, pb)| [pa.unwrap_or(0) as u64, pb.unwrap_or(0) as u64])
            .filter(|h| *h != 0 && !self.parents.contains_key(h))
            .collect();

        if !missing_parents.is_empty() {
            let p_placeholders: Vec<String> =
                missing_parents.iter().map(|_| "?".to_string()).collect();
            let p_sql = format!(
                "SELECT p.hash, COALESCE(td.character_id, 0)
                 FROM parents p
                 LEFT JOIN trainee_data td ON td.id = p.trainee_id
                 WHERE p.hash IN ({})",
                p_placeholders.join(",")
            );
            let p_i64: Vec<i64> = missing_parents.iter().map(|h| *h as i64).collect();
            let p_params: Vec<&dyn rusqlite::types::ToSql> = p_i64
                .iter()
                .map(|h| h as &dyn rusqlite::types::ToSql)
                .collect();
            let mut p_stmt = conn
                .prepare(&p_sql)
                .map_err(|e| format!("ensure_loaded parents prepare failed: {e}"))?;
            let parent_rows: Vec<(i64, i64)> = p_stmt
                .query_map(p_params.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))
                .map_err(|e| format!("ensure_loaded parents query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("ensure_loaded parents collect failed: {e}"))?;

            let pwins_sql = if crate::app_config::win_saddle_version() == 2 {
                format!(
                    "SELECT phw.parent_hash, phw.win_id FROM parent_has_win phw \
                     JOIN major_wins_data mwd ON mwd.id = phw.win_id \
                     WHERE phw.parent_hash IN ({}) AND mwd.win_saddle_type = 3",
                    p_placeholders.join(",")
                )
            } else {
                format!(
                    "SELECT parent_hash, win_id FROM parent_has_win WHERE parent_hash IN ({})",
                    p_placeholders.join(",")
                )
            };
            let mut pwins_stmt = conn
                .prepare(&pwins_sql)
                .map_err(|e| format!("ensure_loaded parent wins prepare failed: {e}"))?;
            let pwin_rows: Vec<(i64, i32)> = pwins_stmt
                .query_map(p_params.as_slice(), |row| Ok((row.get(0)?, row.get(1)?)))
                .map_err(|e| format!("ensure_loaded parent wins query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("ensure_loaded parent wins collect failed: {e}"))?;

            let mut p_wins: IntMap<u64, IntSet<u32>> = IntMap::default();
            for (hash, win_id) in &pwin_rows {
                p_wins
                    .entry(*hash as u64)
                    .or_default()
                    .insert(*win_id as u32);
            }

            for (hash, char_id) in &parent_rows {
                let hash = *hash as u64;
                self.parents.insert(
                    hash,
                    SlimUma {
                        hash,
                        character_id: *char_id,
                        wins: p_wins.remove(&hash).unwrap_or_default(),
                    },
                );
            }
        }

        Ok(())
    }

    /// Check if cached sorted_hashes is still valid.
    fn needs_refresh(
        &self,
        filters: &[Filter],
        sort: &SortConfig,
        legacy_planner_state: &Option<LegacyPlannerState>,
        legacy_planner_slot: &Option<LegacyPlannerSlot>,
    ) -> bool {
        self.sorted_hashes.is_empty()
            || self.current_filters != filters
            || self.current_sort != *sort
            || self.current_legacy_planner_state != *legacy_planner_state
            || self.current_legacy_planner_slot != *legacy_planner_slot
    }

    pub fn get_parent_slim(&self, hash: u64) -> Option<&SlimUma> {
        self.parents.get(&hash)
    }

    pub fn get_veteran_slim(&self, hash: u64) -> Option<&SlimUma> {
        self.umas.get(&hash).map(|u| &u.veteran)
    }

    /// Two-phase filter + sort pipeline.
    ///
    /// 1. DB-side SQL for non-Affinity filters + non-Affinity sort
    /// 2. Rust-side Affinity filter + sort using in-memory SlimUmaGroups
    pub fn apply_filters(
        &mut self,
        conn: &Connection,
        affinity_store: &AffinityStorage,
        filters: &[Filter],
        sort: &SortConfig,
        legacy_planner_state: &Option<LegacyPlannerState>,
        legacy_planner_slot: &Option<LegacyPlannerSlot>,
    ) -> Result<(), String> {
        // Step 1: cache check
        if !self.needs_refresh(filters, sort, legacy_planner_state, legacy_planner_slot) {
            return Ok(());
        }

        // Step 2: split filters
        let db_filters: Vec<&Filter> = filters.iter().filter(|f| !f.is_rust_side()).collect();
        let affinity_min: Option<u32> = filters.iter().find_map(|f| {
            if let Filter::Affinity { min } = f {
                Some(*min)
            } else {
                None
            }
        });

        // Step 3: DB-side query
        let (where_clause, where_params) = Self::build_filter_where(&db_filters);
        let order_clause = if sort.key == "Affinity" {
            String::new()
        } else {
            Self::build_order_clause(sort)
        };

        let sql = if order_clause.is_empty() {
            format!(
                "SELECT v.hash FROM veterans v \
                 LEFT JOIN trainee_data td ON td.id = v.trainee_id \
                 WHERE v.active = 1 AND v.is_browser = 1 AND ({})",
                where_clause
            )
        } else {
            format!(
                "SELECT v.hash FROM veterans v \
                 LEFT JOIN trainee_data td ON td.id = v.trainee_id \
                 WHERE v.active = 1 AND v.is_browser = 1 AND ({}) ORDER BY {}",
                where_clause, order_clause
            )
        };

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("prepare filter query failed: {e}"))?;
        let param_refs: Vec<&dyn rusqlite::types::ToSql> =
            where_params.iter().map(|p| p.as_ref()).collect();

        let mut hashes: Vec<PaginatedVeteranHash> = stmt
            .query_map(param_refs.as_slice(), |row| {
                let h: i64 = row.get(0)?;
                Ok(h as u64)
            })
            .map_err(|e| format!("filter query failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map(|v| {
                v.into_iter()
                    .map(|h| PaginatedVeteranHash {
                        hash: h,
                        affinity: None,
                    })
                    .collect()
            })
            .map_err(|e| format!("collect filter results failed: {e}"))?;

        // Step 3.5: Load any veteran data missing from the in-memory cache
        self.ensure_loaded(conn, hashes.iter().map(|h| h.hash))?;

        // Step 4: Rust-side Affinity filter + sort
        let mut scored: Vec<PaginatedVeteranHash> =
            if let (Some(legacy_planner_state), Some(legacy_planner_slot)) =
                (legacy_planner_state, legacy_planner_slot)
            {
                hashes
                    .iter()
                    .map(|h| {
                        let result = calculate_legacy_planner_affinity(
                            *legacy_planner_slot,
                            legacy_planner_state,
                            self.umas
                                .get(&h.hash)
                                .expect("ensure_loaded should have loaded this hash"),
                            affinity_store,
                            self,
                        );
                        PaginatedVeteranHash {
                            hash: h.hash,
                            affinity: Some(result.total_result()),
                        }
                    })
                    .collect()
            } else {
                hashes
                    .iter()
                    .map(|h| {
                        let result = calculate_browser_affinity(
                            None,
                            affinity_store,
                            self,
                            self.umas
                                .get(&h.hash)
                                .expect("ensure_loaded should have loaded this hash"),
                        );
                        PaginatedVeteranHash {
                            hash: h.hash,
                            affinity: Some(result.total_result()),
                        }
                    })
                    .collect()
            };

        // Filter by min affinity
        if let Some(min_aff) = affinity_min {
            scored.retain(|h| h.affinity.map(|r| r.total()).unwrap_or(0) >= min_aff);
        }

        // Sort by affinity
        if sort.key == "Affinity" {
            match sort.direction.as_str() {
                "Asc" => scored.sort_by_key(|h| h.affinity.map(|r| r.total())),
                _ => scored.sort_by(|a, b| {
                    b.affinity
                        .map(|r| r.total())
                        .cmp(&a.affinity.map(|r| r.total()))
                }),
            }
        }

        hashes = scored;

        // Step 5: cache and return
        self.sorted_hashes = hashes;
        self.current_filters = filters.to_vec();
        self.current_sort = sort.clone();
        self.current_legacy_planner_state = legacy_planner_state.clone();
        self.current_legacy_planner_slot = *legacy_planner_slot;

        Ok(())
    }

    /// Total number of veterans in the filtered result.
    pub fn total_count(&self) -> usize {
        self.sorted_hashes.len()
    }

    pub fn invalidate_cache(&mut self) {
        self.sorted_hashes.clear();
    }

    /// Returns true if the store has been populated.
    pub fn is_populated(&self) -> bool {
        !self.umas.is_empty()
    }

    /// Get a page of full VeteranRow data from the cached sorted hashes.
    /// Page is 1-indexed.
    pub fn get_page(
        &self,
        conn: &Connection,
        page: usize,
        page_size: usize,
    ) -> Result<Vec<VeteranPageItem>, String> {
        let start = (page.saturating_sub(1)) * page_size;
        let page_hashes: &[_] = self
            .sorted_hashes
            .get(start..)
            .map(|s| &s[..page_size.min(s.len())])
            .unwrap_or(&[]);

        if page_hashes.is_empty() {
            return Ok(Vec::new());
        }

        // Query DB for full VeteranRows matching these hashes
        let placeholders: Vec<String> = page_hashes.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT {} {} WHERE v.active = 1 AND v.is_browser = 1 AND v.hash IN ({})",
            veteran_select_cols(),
            VETERAN_FROM,
            placeholders.join(",")
        );

        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("get_page prepare failed: {e}"))?;

        let hash_params: Vec<Box<dyn rusqlite::types::ToSql>> = page_hashes
            .iter()
            .map(|h| Box::new(h.hash as i64) as Box<dyn rusqlite::types::ToSql>)
            .collect();
        let hash_refs: Vec<&dyn rusqlite::types::ToSql> =
            hash_params.iter().map(|p| p.as_ref()).collect();

        let mut veterans: Vec<VeteranRow> = stmt
            .query_map(hash_refs.as_slice(), make_veteran_row)
            .map_err(|e| format!("get_page query failed: {e}"))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("get_page collect failed: {e}"))?;

        // Batch fetch spark groups
        if !veterans.is_empty() {
            let spark_placeholders: Vec<String> =
                veterans.iter().map(|_| "?".to_string()).collect();
            let spark_sql = format!(
                "SELECT vss.veteran_hash, vss.spark_group_id, vss.uma_count, vss.level_sum, \
                        COALESCE(sd.name, ''), COALESCE(sd.spark_type, 0), vss.veteran_level_sum \
                 FROM veteran_spark_summary vss \
                 LEFT JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
                 WHERE vss.veteran_hash IN ({}) \
                 GROUP BY vss.veteran_hash, vss.spark_group_id \
                 ORDER BY vss.veteran_hash, sd.spark_type, vss.level_sum DESC",
                spark_placeholders.join(",")
            );
            let mut spark_stmt = conn
                .prepare(&spark_sql)
                .map_err(|e| format!("get_page spark prepare failed: {e}"))?;
            let spark_hash_refs: Vec<Box<dyn rusqlite::types::ToSql>> = veterans
                .iter()
                .map(|v| Box::new(v.hash) as Box<dyn rusqlite::types::ToSql>)
                .collect();
            let spark_hash_params: Vec<&dyn rusqlite::types::ToSql> =
                spark_hash_refs.iter().map(|p| p.as_ref()).collect();
            let spark_rows = spark_stmt
                .query_map(spark_hash_params.as_slice(), |row| {
                    Ok(SparkGroupRow {
                        veteran_hash: row.get(0)?,
                        spark_group_id: row.get(1)?,
                        uma_count: row.get(2)?,
                        level_sum: row.get(3)?,
                        name: row.get(4)?,
                        spark_type: row.get(5)?,
                        veteran_level_sum: row.get(6)?,
                    })
                })
                .map_err(|e| format!("get_page spark query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("get_page spark collect failed: {e}"))?;

            let mut spark_map: HashMap<i64, Vec<SparkGroupRow>> = HashMap::new();
            for s in spark_rows {
                spark_map.entry(s.veteran_hash).or_default().push(s);
            }
            for v in &mut veterans {
                if let Some(sparks) = spark_map.remove(&v.hash) {
                    v.spark_groups = sparks;
                }
            }
        }

        // Batch fetch tags (collect hashes before consuming `veterans`)
        let vet_hashes: Vec<i64> = veterans.iter().map(|v| v.hash).collect();
        let mut tag_map: HashMap<i64, Vec<TagRow>> = HashMap::new();
        {
            let tag_placeholders: Vec<String> =
                vet_hashes.iter().map(|_| "?".to_string()).collect();
            let tag_sql = format!(
                "SELECT vht.veteran_hash, t.id, t.tag_value, t.create_date \
                 FROM veteran_has_tag vht \
                 JOIN tag t ON t.id = vht.tag_id \
                 WHERE vht.veteran_hash IN ({}) \
                 ORDER BY t.tag_value",
                tag_placeholders.join(",")
            );
            let mut tag_stmt = conn
                .prepare(&tag_sql)
                .map_err(|e| format!("get_page tag prepare failed: {e}"))?;
            let tag_hash_refs: Vec<Box<dyn rusqlite::types::ToSql>> = vet_hashes
                .iter()
                .map(|v| Box::new(*v) as Box<dyn rusqlite::types::ToSql>)
                .collect();
            let tag_hash_params: Vec<&dyn rusqlite::types::ToSql> =
                tag_hash_refs.iter().map(|p| p.as_ref()).collect();
            let tag_rows = tag_stmt
                .query_map(tag_hash_params.as_slice(), |row| {
                    let vet_hash: i64 = row.get(0)?;
                    let tag = TagRow {
                        id: row.get(1)?,
                        tag_value: row.get(2)?,
                        create_date: row.get(3)?,
                    };
                    Ok((vet_hash, tag))
                })
                .map_err(|e| format!("get_page tag query failed: {e}"))?
                .collect::<Result<Vec<_>, _>>()
                .map_err(|e| format!("get_page tag collect failed: {e}"))?;
            for (vet_hash, tag) in tag_rows {
                tag_map.entry(vet_hash).or_default().push(tag);
            }
        }

        // Build hashmap for O(1) lookup, then iterate in page_hashes order
        let vet_map: IntMap<i64, VeteranRow> = veterans.into_iter().map(|v| (v.hash, v)).collect();

        let items: Vec<VeteranPageItem> = page_hashes
            .iter()
            .filter_map(|ph| {
                let v = vet_map.get(&(ph.hash as i64))?.clone();
                let tags = tag_map.remove(&v.hash).unwrap_or_default();
                Some(VeteranPageItem {
                    veteran: v,
                    affinity: ph.affinity,
                    tags,
                })
            })
            .collect();

        Ok(items)
    }

    // ── Private SQL helpers ─────────────────────────────────────

    fn build_filter_where(filters: &[&Filter]) -> (String, Vec<SqlParam>) {
        if filters.is_empty() {
            return ("1=1".to_string(), Vec::new());
        }

        let mut clauses: Vec<String> = Vec::new();
        let mut params: Vec<SqlParam> = Vec::new();

        for filter in filters {
            match filter {
                Filter::TraineeHash(h) => {
                    clauses.push("v.hash = ?".to_string());
                    params.push(Box::new(h.as_i64()));
                }
                Filter::ParentHash(h) => {
                    clauses.push("v.min_hash = ?".to_string());
                    params.push(Box::new(h.as_i64()));
                }
                Filter::HasParent(h) => {
                    clauses.push("(v.parent_a = ? OR v.parent_b = ?)".to_string());
                    let h = h.as_i64();
                    params.push(Box::new(h));
                    params.push(Box::new(h));
                }
                Filter::Ranking { min, max } => {
                    let min_val = min.unwrap_or(0);
                    let max_val = max.unwrap_or(i64::MAX);
                    clauses.push("v.rank_score BETWEEN ? AND ?".to_string());
                    params.push(Box::new(min_val));
                    params.push(Box::new(max_val));
                }
                Filter::Trainee(id) => {
                    clauses.push("v.trainee_id = ?".to_string());
                    params.push(Box::new(*id));
                }
                Filter::Character(id) => {
                    clauses.push("td.character_id = ?".to_string());
                    params.push(Box::new(*id));
                }
                Filter::Scenario(s) => {
                    clauses.push("v.scenario = ?".to_string());
                    params.push(Box::new(i64::from(*s)));
                }
                Filter::Spark(sf) => {
                    Self::build_spark_clause(sf, &mut clauses, &mut params);
                }
                Filter::WhiteSparkCount { min, max } => {
                    let min_val = min.map(i64::from).unwrap_or(0);
                    let max_val = max.map(i64::from).unwrap_or(i64::MAX);
                    clauses.push(
                        "(SELECT COUNT(DISTINCT vss.spark_group_id) \
                          FROM veteran_spark_summary vss \
                          JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
                          WHERE vss.veteran_hash = v.hash AND sd.spark_type IN (4,5)) \
                         BETWEEN ? AND ?"
                            .to_string(),
                    );
                    params.push(Box::new(min_val));
                    params.push(Box::new(max_val));
                }
                Filter::G1Wins { min, max } => {
                    let min_val = min.map(i64::from).unwrap_or(0);
                    let max_val = max.map(i64::from).unwrap_or(i64::MAX);
                    clauses.push(
                        "(SELECT COUNT(*) FROM veteran_win_count vwc \
                          WHERE vwc.veteran_hash = v.hash) \
                         BETWEEN ? AND ?"
                            .to_string(),
                    );
                    params.push(Box::new(min_val));
                    params.push(Box::new(max_val));
                }
                Filter::Aptitude {
                    aptitude_type,
                    min_level,
                } => {
                    let col = match aptitude_type {
                        AptitudeType::Turf => "v.aptitude_turf",
                        AptitudeType::Dirt => "v.aptitude_dirt",
                        AptitudeType::Sprint => "v.aptitude_sprint",
                        AptitudeType::Mile => "v.aptitude_mile",
                        AptitudeType::Medium => "v.aptitude_medium",
                        AptitudeType::Long => "v.aptitude_long",
                        AptitudeType::Front => "v.aptitude_front",
                        AptitudeType::PaceChaser => "v.aptitude_pace_chaser",
                        AptitudeType::LateSurger => "v.aptitude_late_surger",
                        AptitudeType::EndCloser => "v.aptitude_end_closer",
                    };
                    clauses.push(format!("{} >= ?", col));
                    params.push(Box::new(i64::from(*min_level)));
                }
                Filter::MajorWinsCount { min, both } => {
                    let min_val = i64::from(min.unwrap_or(0));
                    let max_val = i64::MAX;
                    let subquery = if *both {
                        format!(
                            "(SELECT COUNT(*) FROM veteran_win_count vw WHERE vw.veteran_hash = v.hash) \
                             + COALESCE((SELECT COUNT(*) FROM parent_has_win phw WHERE phw.parent_hash = v.parent_a), 0) \
                             + COALESCE((SELECT COUNT(*) FROM parent_has_win phw WHERE phw.parent_hash = v.parent_b), 0)"
                        )
                    } else {
                        "(SELECT COUNT(*) FROM veteran_win_count vw WHERE vw.veteran_hash = v.hash)"
                            .to_string()
                    };
                    clauses.push(format!("{} BETWEEN ? AND ?", subquery));
                    params.push(Box::new(min_val));
                    params.push(Box::new(max_val));
                }
                Filter::SpecificMajorWin {
                    major_win_id,
                    shared_with_parent,
                } => {
                    match shared_with_parent {
                        Some(true) => {
                            clauses.push(
                                "EXISTS (SELECT 1 FROM veteran_win_count vwc \
                                  WHERE vwc.veteran_hash = v.hash AND vwc.win_id = ? AND vwc.win_count > 1)"
                                    .to_string(),
                            );
                        }
                        _ => {
                            clauses.push(
                                "EXISTS (SELECT 1 FROM veteran_has_win vw \
                                  WHERE vw.veteran_hash = v.hash AND vw.win_id = ?)"
                                    .to_string(),
                            );
                        }
                    }
                    params.push(Box::new(*major_win_id));
                }
                Filter::HasFavouriteMemo { search_text } => {
                    if let Some(text) = search_text {
                        if !text.is_empty() {
                            clauses.push(
                                "v.favorite_memo IS NOT NULL AND v.favorite_memo LIKE ?"
                                    .to_string(),
                            );
                            params.push(Box::new(format!("%{}%", text)));
                        } else {
                            clauses.push("v.favorite_memo IS NOT NULL".to_string());
                        }
                    } else {
                        clauses.push("v.favorite_memo IS NOT NULL".to_string());
                    }
                }
                Filter::HasFavouriteIcon { icon_type } => {
                    if let Some(icon) = icon_type {
                        clauses.push("v.favorite_icon_type = ?".to_string());
                        params.push(Box::new(i64::from(*icon)));
                    } else {
                        clauses.push("v.favorite_icon_type IS NOT NULL".to_string());
                    }
                }
                Filter::HasTag { tag_value } => {
                    clauses.push(
                        "EXISTS (SELECT 1 FROM veteran_has_tag vht \
                          JOIN tag t ON t.id = vht.tag_id \
                          WHERE vht.veteran_hash = v.hash AND t.tag_value = ?)"
                            .to_string(),
                    );
                    params.push(Box::new(tag_value.clone()));
                }
                Filter::Affinity { .. } => {
                    // Rust-side only — omitted from SQL
                }
                Filter::BorrowStatus { is_borrowed } => {
                    clauses.push(if *is_borrowed {
                        "v.owned = 0".to_string()
                    } else {
                        "v.owned = 1".to_string()
                    });
                }
                Filter::IsIndependentTrainer { is_independent } => {
                    clauses.push(if *is_independent {
                        format!("v.nickname_id = {INDEPENDENT_LEARNER_NICKNAME}")
                    } else {
                        format!("v.nickname_id != {INDEPENDENT_LEARNER_NICKNAME}")
                    });
                }
            }
        }

        if clauses.is_empty() {
            ("1=1".to_string(), params)
        } else {
            (clauses.join(" AND "), params)
        }
    }

    fn build_spark_clause(sf: &SparkFilter, clauses: &mut Vec<String>, params: &mut Vec<SqlParam>) {
        let mut sub_clauses: Vec<String> = Vec::new();

        if sf.group_id != 0 {
            sub_clauses.push("vss.spark_group_id = ?".to_string());
            params.push(Box::new(i64::from(sf.group_id)));
        }

        if sf.on_trainee {
            sub_clauses.push("vss.veteran_level_sum > 0".to_string());
        }

        let level_col = if sf.on_trainee {
            "vss.veteran_level_sum"
        } else {
            "vss.level_sum"
        };

        if let Some(min) = sf.min_stars {
            sub_clauses.push(format!("{} >= ?", level_col));
            params.push(Box::new(i64::from(min)));
        }

        if let Some(max) = sf.max_stars {
            sub_clauses.push(format!("{} <= ?", level_col));
            params.push(Box::new(i64::from(max)));
        }

        if let Some(shared) = sf.shared_count {
            sub_clauses.push("vss.uma_count >= ?".to_string());
            params.push(Box::new(i64::from(shared)));
        }

        if sub_clauses.is_empty() {
            return;
        }

        clauses.push(format!(
            "EXISTS (SELECT 1 FROM veteran_spark_summary vss \
             WHERE vss.veteran_hash = v.hash AND {})",
            sub_clauses.join(" AND ")
        ));
    }

    fn build_order_clause(sort: &SortConfig) -> String {
        let dir = match sort.direction.as_str() {
            "Asc" => "ASC",
            _ => "DESC",
        };
        let col = match sort.key.as_str() {
            "Rank" => "v.rank_score",
            "Name" => "td.name",
            "CreatedAt" => "v.created_at",
            "WhiteSparkCount" => {
                "(SELECT COUNT(DISTINCT vss.spark_group_id) \
                  FROM veteran_spark_summary vss \
                  JOIN spark_data sd ON sd.group_id = vss.spark_group_id \
                  WHERE vss.veteran_hash = v.hash AND sd.spark_type IN (4,5))"
            }
            "MajorWinCount" => {
                "(SELECT COUNT(*) FROM veteran_win_count vwc WHERE vwc.veteran_hash = v.hash)"
            }
            _ => "v.rank_score",
        };
        format!("{} {}", col, dir)
    }
}
