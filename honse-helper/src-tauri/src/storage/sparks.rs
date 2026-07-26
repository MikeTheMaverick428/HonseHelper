use nohash_hasher::IntMap;
use rusqlite::Connection;

pub struct SparkGroupStorage {
    names: IntMap<i64, String>,
    types: IntMap<i64, i64>,
}

impl SparkGroupStorage {
    pub fn new(conn: &Connection, ids: &[i64]) -> Self {
        let mut names = IntMap::default();
        let mut types = IntMap::default();

        if ids.is_empty() {
            return Self { names, types };
        }

        let placeholders: Vec<String> = ids.iter().map(|_| "?".to_string()).collect();
        let sql = format!(
            "SELECT group_id, name, spark_type FROM spark_data WHERE group_id IN ({})",
            placeholders.join(",")
        );

        if let Ok(mut stmt) = conn.prepare(&sql) {
            let _ = stmt
                .query_map(rusqlite::params_from_iter(ids.iter().copied()), |row| {
                    let group_id: i64 = row.get(0)?;
                    let name: String = row.get(1)?;
                    let spark_type: i64 = row.get(2)?;
                    Ok((group_id, name, spark_type))
                })
                .map(|rows| {
                    for r in rows.flatten() {
                        names.insert(r.0, r.1);
                        types.insert(r.0, r.2);
                    }
                });
        }

        Self { names, types }
    }

    pub fn name(&self, id: i64) -> &str {
        self.names.get(&id).map(|s| s.as_str()).unwrap_or("")
    }

    pub fn spark_type(&self, id: i64) -> i64 {
        self.types.get(&id).copied().unwrap_or(0)
    }
}

impl Default for SparkGroupStorage {
    fn default() -> Self {
        Self {
            names: IntMap::default(),
            types: IntMap::default(),
        }
    }
}
