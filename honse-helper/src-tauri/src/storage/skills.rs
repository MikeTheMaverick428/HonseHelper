use std::sync::Mutex;

use nohash_hasher::IntMap;
use shared::SkillDataRow;

const SELECT_SKILL_SQL: &str = "\
SELECT id, name, description, \
  precondition1, condition1, precondition2, condition2, \
  skill_category, group_id, rarity, \
  icon_id, ability_type, target_type, \
  ability_type_2, ability_type_3, \
  target_type_2, target_type_3, \
  effect_value_1, effect_value_2, effect_value_3, \
  target_value_1, target_value_2, target_value_3, \
  effect_duration, effect_cooldown, \
  activate_lot, skill_cost \
FROM skill_data WHERE id = ?1";

pub struct SkillStorage {
    conn: rusqlite::Connection,
    cache: Mutex<IntMap<i64, SkillDataRow>>,
}

impl SkillStorage {
    pub fn new(conn: rusqlite::Connection) -> Self {
        Self {
            conn,
            cache: Mutex::new(IntMap::default()),
        }
    }

    pub fn get_by_id(&self, id: i64) -> Option<SkillDataRow> {
        {
            let cache = self.cache.lock().unwrap();
            if let Some(data) = cache.get(&id) {
                return Some(data.clone());
            }
        }

        if let Ok(row) = self.conn.query_row(SELECT_SKILL_SQL, [id], |r| {
            Ok(SkillDataRow {
                id: r.get(0)?,
                name: r.get(1)?,
                description: r.get(2)?,
                precondition1: r.get(3)?,
                condition1: r.get(4)?,
                precondition2: r.get(5)?,
                condition2: r.get(6)?,
                skill_category: r.get(7)?,
                group_id: r.get(8)?,
                rarity: r.get(9)?,
                icon_id: r.get(10)?,
                ability_type: r.get(11)?,
                target_type: r.get(12)?,
                ability_type_2: r.get(13)?,
                ability_type_3: r.get(14)?,
                target_type_2: r.get(15)?,
                target_type_3: r.get(16)?,
                effect_value_1: r.get(17)?,
                effect_value_2: r.get(18)?,
                effect_value_3: r.get(19)?,
                target_value_1: r.get(20)?,
                target_value_2: r.get(21)?,
                target_value_3: r.get(22)?,
                effect_duration: r.get(23)?,
                effect_cooldown: r.get(24)?,
                activate_lot: r.get(25)?,
                skill_cost: r.get(26)?,
            })
        }) {
            self.cache.lock().unwrap().insert(id, row.clone());
            return Some(row);
        }
        None
    }
}
