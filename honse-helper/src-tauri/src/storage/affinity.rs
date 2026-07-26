use std::collections::HashMap;
use std::sync::Mutex;

use nohash_hasher::{IntMap, IntSet};
use rusqlite::Connection;
use shared::legacy_planner::lookup_dtos::{
    AffinityResult, AffinityResultTree, SlimUma, SlimUmaGroup,
};
use shared::legacy_planner::{LegacyPlannerSlot, LegacyPlannerState, LegacySlotValue};
use shared::{AffinityGroupRow, AffinityMemberRow};

use crate::storage::veterans::VeteranStore;

/// Storage for character affinities.
///
/// Populated from SQL storage and then kept in memory for quicker access, keeping internal
/// cache calculated affinities for pairs and trios.
#[derive(Default)]
pub struct AffinityStorage {
    groups_by_char: IntMap<u32, IntSet<u32>>,
    group_points: IntMap<u32, u8>,
    pair_cache: Mutex<HashMap<(u32, u32), u32>>,
    trio_cache: Mutex<HashMap<(u32, u32, u32), u32>>,
}

impl AffinityStorage {
    pub fn load_all(&mut self, conn: &Connection) -> rusqlite::Result<()> {
        let mut member_stmt =
            conn.prepare("SELECT id, affinity_group, chara_id FROM affinity_member")?;
        let members = member_stmt.query_map([], |row| {
            Ok(AffinityMemberRow {
                id: row.get(0)?,
                affinity_group: row.get(1)?,
                chara_id: row.get(2)?,
            })
        })?;
        for member in members {
            self.load_member(member?);
        }

        let mut group_stmt =
            conn.prepare("SELECT affinity_group, affinity_point FROM affinity_groups")?;
        let groups = group_stmt.query_map([], |row| {
            Ok(AffinityGroupRow {
                affinity_group: row.get(0)?,
                affinity_point: row.get(1)?,
            })
        })?;
        for group in groups {
            self.load_group(group?);
        }

        Ok(())
    }

    pub fn load_member(&mut self, member: AffinityMemberRow) {
        self.groups_by_char
            .entry(member.chara_id as u32)
            .or_default()
            .insert(member.affinity_group as u32);
    }

    pub fn load_group(&mut self, group: AffinityGroupRow) {
        self.group_points
            .insert(group.affinity_group as u32, group.affinity_point as u8);
    }

    pub fn get_for_two(&self, a: u32, b: u32) -> u32 {
        if a == b {
            return 0;
        }
        let mut pair = [a, b];
        pair.sort();
        if let Some(points) = self.pair_cache.lock().unwrap().get(&(pair[0], pair[1])) {
            return *points;
        }

        let groups_a = match self.groups_by_char.get(&pair[0]) {
            Some(g) => g,
            None => return 0,
        };
        let groups_b = match self.groups_by_char.get(&pair[1]) {
            Some(g) => g,
            None => return 0,
        };

        let common = groups_a.intersection(&groups_b);

        let mut sum = 0;
        for affinity_group in common {
            if let Some(points) = self.group_points.get(affinity_group) {
                sum += (*points) as u32;
            }
        }

        self.pair_cache
            .lock()
            .unwrap()
            .insert((pair[0], pair[1]), sum);
        sum
    }

    pub fn get_for_three(&self, a: u32, b: u32, c: u32) -> u32 {
        if a == b || a == c || b == c {
            return 0;
        }
        let mut trio = [a, b, c];
        trio.sort();
        if let Some(points) = self
            .trio_cache
            .lock()
            .unwrap()
            .get(&(trio[0], trio[1], trio[2]))
        {
            return *points;
        }

        let groups_a = match self.groups_by_char.get(&trio[0]) {
            Some(g) => g,
            None => return 0,
        };
        let groups_b = match self.groups_by_char.get(&trio[1]) {
            Some(g) => g,
            None => return 0,
        };
        let groups_c = match self.groups_by_char.get(&trio[2]) {
            Some(g) => g,
            None => return 0,
        };

        let (smallest, other1, other2) =
            if groups_a.len() <= groups_b.len() && groups_a.len() <= groups_c.len() {
                (groups_a, groups_b, groups_c)
            } else if groups_b.len() <= groups_c.len() {
                (groups_b, groups_a, groups_c)
            } else {
                (groups_c, groups_a, groups_b)
            };

        let mut sum = 0;
        for g in smallest
            .iter()
            .filter(|g| other1.contains(*g) && other2.contains(*g))
        {
            if let Some(points) = self.group_points.get(g) {
                sum += (*points) as u32;
            }
        }

        self.trio_cache
            .lock()
            .unwrap()
            .insert((trio[0], trio[1], trio[2]), sum);
        sum
    }

    /// Base pair affinity between two characters.
    pub fn pair_base(&self, a: i64, b: i64) -> u32 {
        self.get_for_two(a as u32, b as u32)
    }

    /// Base trio affinity among three characters.
    pub fn trio_base(&self, a: i64, b: i64, c: i64) -> u32 {
        self.get_for_three(a as u32, b as u32, c as u32)
    }

    /// Bonus affinity from shared wins — 1 point per shared win_id (×3 when V2).
    pub fn shared_wins_bonus(a: &IntSet<u32>, b: &IntSet<u32>) -> u32 {
        let count = a.iter().filter(|w| b.contains(w)).count() as u32;
        if crate::app_config::win_saddle_version() == 2 {
            count * 3
        } else {
            count
        }
    }
}

/// Total affinity for a single-veteran browsing context.
///
/// Computes `AffinityResultTree` for a veteran candidate given a chosen
/// character and optional partner (second parent).
///
/// Uses `store` to resolve parent `u64` hash refs to `SlimUma` data.
pub fn calculate_legacy_planner_affinity(
    slot: LegacyPlannerSlot,
    state: &LegacyPlannerState,
    candidate: &SlimUmaGroup,
    affinity: &AffinityStorage,
    store: &VeteranStore,
) -> AffinityResultTree {
    let state = state.clone();

    let (pa, pb, gpaa, gpab, gpba, gpbb) = match slot {
        LegacyPlannerSlot::ParentA => (
            Some(candidate.veteran.clone()),
            state.parent_b.map(|u| Into::<SlimUma>::into(u)),
            Some(store.get_parent_slim(candidate.parent_a).cloned().unwrap()),
            Some(store.get_parent_slim(candidate.parent_b).cloned().unwrap()),
            state.grandparent_ba.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_bb.map(|u| Into::<SlimUma>::into(u)),
        ),
        LegacyPlannerSlot::ParentB => (
            state.parent_a.map(|u| Into::<SlimUma>::into(u)),
            Some(candidate.veteran.clone()),
            state.grandparent_ab.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_aa.map(|u| Into::<SlimUma>::into(u)),
            Some(store.get_parent_slim(candidate.parent_a).cloned().unwrap()),
            Some(store.get_parent_slim(candidate.parent_b).cloned().unwrap()),
        ),
        LegacyPlannerSlot::GrandparentAA => (
            state.parent_a.map(|u| Into::<SlimUma>::into(u)),
            state.parent_b.map(|u| Into::<SlimUma>::into(u)),
            Some(candidate.veteran.clone()),
            state.grandparent_ab.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_ba.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_bb.map(|u| Into::<SlimUma>::into(u)),
        ),
        LegacyPlannerSlot::GrandparentAB => (
            state.parent_a.map(|u| Into::<SlimUma>::into(u)),
            state.parent_b.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_aa.map(|u| Into::<SlimUma>::into(u)),
            Some(candidate.veteran.clone()),
            state.grandparent_ba.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_bb.map(|u| Into::<SlimUma>::into(u)),
        ),
        LegacyPlannerSlot::GrandparentBA => (
            state.parent_a.map(|u| Into::<SlimUma>::into(u)),
            state.parent_b.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_aa.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_ab.map(|u| Into::<SlimUma>::into(u)),
            Some(candidate.veteran.clone()),
            state.grandparent_bb.map(|u| Into::<SlimUma>::into(u)),
        ),
        LegacyPlannerSlot::GrandparentBB => (
            state.parent_a.map(|u| Into::<SlimUma>::into(u)),
            state.parent_b.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_aa.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_ab.map(|u| Into::<SlimUma>::into(u)),
            state.grandparent_ba.map(|u| Into::<SlimUma>::into(u)),
            Some(candidate.veteran.clone()),
        ),
    };
    let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id);

    let mut result = AffinityResultTree::default();

    if let (Some(chosen_id), Some(pa)) = (chosen_id, &pa) {
        result.parent_a = Some(affinity.pair_base(chosen_id, pa.character_id));
    }
    if let (Some(chosen_id), Some(pb)) = (chosen_id, &pb) {
        result.parent_b = Some(affinity.pair_base(chosen_id, pb.character_id));
    }
    if let (Some(chosen_id), Some(pa), Some(gpaa)) = (chosen_id, &pa, &gpaa) {
        result.grandparent_aa = Some(AffinityResult {
            base: affinity.trio_base(chosen_id, pa.character_id, gpaa.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&pa.wins, &gpaa.wins),
        });
    }
    if let (Some(chosen_id), Some(pa), Some(gpab)) = (chosen_id, &pa, &gpab) {
        result.grandparent_ab = Some(AffinityResult {
            base: affinity.trio_base(chosen_id, pa.character_id, gpab.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&pa.wins, &gpab.wins),
        });
    }
    if let (Some(chosen_id), Some(pb), Some(gpba)) = (chosen_id, &pb, &gpba) {
        result.grandparent_ba = Some(AffinityResult {
            base: affinity.trio_base(chosen_id, pb.character_id, gpba.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&pb.wins, &gpba.wins),
        });
    }
    if let (Some(chosen_id), Some(pb), Some(gpbb)) = (chosen_id, &pb, &gpbb) {
        result.grandparent_bb = Some(AffinityResult {
            base: affinity.trio_base(chosen_id, pb.character_id, gpbb.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&pb.wins, &gpbb.wins),
        });
    }
    if let (Some(pa), Some(pb)) = (pa, pb) {
        result.interparent = Some(AffinityResult {
            base: affinity.pair_base(pa.character_id, pb.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&pa.wins, &pb.wins),
        });
    }

    result
}

/// Total affinity for a candidate in a specific planner slot.
///
/// When `slot` is `None`, computes the full affinity as if `chosen_char` were placed in
/// the chosen trainee slot (existing behavior).
///
/// When `slot` is a parent/grandparent slot, computes only the edges that directly
/// involve the candidate — pair affinities for parent slots, trio affinities for
/// grandparent slots. Other tree edges are unaffected by the choice of candidate.
pub fn compute_chosen_slot_affinity(
    chosen_char: i64,
    state: &LegacyPlannerState,
    affinity: &AffinityStorage,
    slot: Option<LegacyPlannerSlot>,
) -> AffinityResult {
    // No slot specified — candidate is the chosen character (original behavior)
    let slot = match slot {
        Some(s) => s,
        None => return compute_chosen_trainee_slot_affinity(chosen_char, state, affinity),
    };

    let chosen_id = state.chosen_character.as_ref().map(|c| c.character_id).unwrap_or(0);

    let orig_pa = state.parent_a.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));
    let orig_pb = state.parent_b.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));
    let orig_gpaa = state.grandparent_aa.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));
    let orig_gpab = state.grandparent_ab.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));
    let orig_gpba = state.grandparent_ba.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));
    let orig_gpbb = state.grandparent_bb.as_ref().map(|v| Into::<SlimUma>::into(v.clone()));

    // Grandparents locked to a parent are cleared when that parent is replaced.
    let gpaa_locked = matches!(&state.grandparent_aa, Some(LegacySlotValue::ParentUma(_)));
    let gpab_locked = matches!(&state.grandparent_ab, Some(LegacySlotValue::ParentUma(_)));
    let gpba_locked = matches!(&state.grandparent_ba, Some(LegacySlotValue::ParentUma(_)));
    let gpbb_locked = matches!(&state.grandparent_bb, Some(LegacySlotValue::ParentUma(_)));

    let candidate = SlimUma { hash: 0, character_id: chosen_char, wins: Default::default() };

    // Build the substituted tree: candidate fills the target slot, everything else stays.
    // Grandparents locked to the replaced parent become None; independent ones stay.
    let (pa, pb, gpaa, gpab, gpba, gpbb) = match slot {
        LegacyPlannerSlot::ParentA => (
            Some(candidate), orig_pb,
            if gpaa_locked { None } else { orig_gpaa },
            if gpab_locked { None } else { orig_gpab },
            orig_gpba, orig_gpbb,
        ),
        LegacyPlannerSlot::ParentB => (
            orig_pa, Some(candidate),
            orig_gpaa, orig_gpab,
            if gpba_locked { None } else { orig_gpba },
            if gpbb_locked { None } else { orig_gpbb },
        ),
        LegacyPlannerSlot::GrandparentAA => (orig_pa, orig_pb, Some(candidate), orig_gpab, orig_gpba, orig_gpbb),
        LegacyPlannerSlot::GrandparentAB => (orig_pa, orig_pb, orig_gpaa, Some(candidate), orig_gpba, orig_gpbb),
        LegacyPlannerSlot::GrandparentBA => (orig_pa, orig_pb, orig_gpaa, orig_gpab, Some(candidate), orig_gpbb),
        LegacyPlannerSlot::GrandparentBB => (orig_pa, orig_pb, orig_gpaa, orig_gpab, orig_gpba, Some(candidate)),
    };

    // Compute the full tree total: all 7 edges with the substituted values.
    let mut base: u32 = 0;
    let mut bonus: u32 = 0;

    // chosen ↔ parent_a (pair)
    if let Some(p) = &pa {
        base += affinity.pair_base(chosen_id, p.character_id);
    }
    // chosen ↔ parent_b (pair)
    if let Some(p) = &pb {
        base += affinity.pair_base(chosen_id, p.character_id);
    }
    // parent_a ↔ parent_b (interparent pair + shared wins)
    if let (Some(a), Some(b)) = (&pa, &pb) {
        base += affinity.pair_base(a.character_id, b.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&a.wins, &b.wins);
    }
    // parent_a ↔ grandparent trios
    if let (Some(p), Some(g)) = (&pa, &gpaa) {
        base += affinity.trio_base(chosen_id, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    if let (Some(p), Some(g)) = (&pa, &gpab) {
        base += affinity.trio_base(chosen_id, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    // parent_b ↔ grandparent trios
    if let (Some(p), Some(g)) = (&pb, &gpba) {
        base += affinity.trio_base(chosen_id, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    if let (Some(p), Some(g)) = (&pb, &gpbb) {
        base += affinity.trio_base(chosen_id, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }

    AffinityResult { base, bonus }
}

fn compute_chosen_trainee_slot_affinity(
    chosen_char: i64,
    state: &LegacyPlannerState,
    affinity: &AffinityStorage,
) -> AffinityResult {
    let pa = state
        .parent_a
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let pb = state
        .parent_b
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpaa = state
        .grandparent_aa
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpab = state
        .grandparent_ab
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpba = state
        .grandparent_ba
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));
    let gpbb = state
        .grandparent_bb
        .as_ref()
        .map(|v| Into::<SlimUma>::into(v.clone()));

    let mut base: u32 = 0;
    let mut bonus: u32 = 0;

    // trainee ↔ parent_a
    if let Some(p) = &pa {
        base += affinity.pair_base(chosen_char, p.character_id);
    }
    // trainee ↔ parent_b
    if let Some(p) = &pb {
        base += affinity.pair_base(chosen_char, p.character_id);
    }
    // parent_a ↔ parent_b
    if let (Some(a), Some(b)) = (&pa, &pb) {
        base += affinity.pair_base(a.character_id, b.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&a.wins, &b.wins);
    }
    // parent_a ↔ grandparent trios
    if let (Some(p), Some(g)) = (&pa, &gpaa) {
        base += affinity.trio_base(chosen_char, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    if let (Some(p), Some(g)) = (&pa, &gpab) {
        base += affinity.trio_base(chosen_char, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    // parent_b ↔ grandparent trios
    if let (Some(p), Some(g)) = (&pb, &gpba) {
        base += affinity.trio_base(chosen_char, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }
    if let (Some(p), Some(g)) = (&pb, &gpbb) {
        base += affinity.trio_base(chosen_char, p.character_id, g.character_id);
        bonus += AffinityStorage::shared_wins_bonus(&p.wins, &g.wins);
    }

    AffinityResult { base, bonus }
}

pub fn calculate_browser_affinity(
    chosen_id: Option<i64>,
    affinity: &AffinityStorage,
    store: &VeteranStore,
    candidate: &SlimUmaGroup,
) -> AffinityResultTree {
    let mut result = AffinityResultTree::default();

    let parent_a = store
        .get_parent_slim(candidate.parent_a)
        .cloned()
        .unwrap_or_else(|| SlimUma {
            hash: 0,
            character_id: 0,
            wins: IntSet::default(),
        });
    let parent_b = store
        .get_parent_slim(candidate.parent_b)
        .cloned()
        .unwrap_or_else(|| SlimUma {
            hash: 0,
            character_id: 0,
            wins: IntSet::default(),
        });

    if let Some(chosen_id) = chosen_id {
        result.parent_a = Some(affinity.pair_base(chosen_id, candidate.veteran.character_id));

        result.grandparent_aa = Some(AffinityResult {
            base: affinity.trio_base(
                chosen_id,
                candidate.veteran.character_id,
                parent_a.character_id,
            ),
            bonus: AffinityStorage::shared_wins_bonus(&candidate.veteran.wins, &parent_a.wins),
        });

        result.grandparent_ab = Some(AffinityResult {
            base: affinity.trio_base(
                chosen_id,
                candidate.veteran.character_id,
                parent_b.character_id,
            ),
            bonus: AffinityStorage::shared_wins_bonus(&candidate.veteran.wins, &parent_b.wins),
        });
    } else {
        result.grandparent_aa = Some(AffinityResult {
            base: affinity.pair_base(candidate.veteran.character_id, parent_a.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&candidate.veteran.wins, &parent_a.wins),
        });

        result.grandparent_ab = Some(AffinityResult {
            base: affinity.pair_base(candidate.veteran.character_id, parent_b.character_id),
            bonus: AffinityStorage::shared_wins_bonus(&candidate.veteran.wins, &parent_b.wins),
        });
    }

    result
}
