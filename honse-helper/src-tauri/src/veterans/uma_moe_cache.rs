use std::collections::HashMap;
use std::sync::Mutex;

use shared::db_models::veteran_data::{Parent, UmaGroup};

pub struct UmaMoeCache {
    groups: Mutex<HashMap<u64, UmaGroup>>,
}

impl UmaMoeCache {
    pub fn new() -> Self {
        Self {
            groups: Mutex::new(HashMap::new()),
        }
    }

    pub fn store(&self, hash: u64, group: UmaGroup) {
        if let Ok(mut guard) = self.groups.lock() {
            guard.insert(hash, group);
        }
    }

    pub fn get(&self, hash: u64) -> Option<UmaGroup> {
        self.groups.lock().ok()?.get(&hash).cloned()
    }

    pub fn store_batch(&self, groups: Vec<(u64, UmaGroup)>) {
        if let Ok(mut guard) = self.groups.lock() {
            for (hash, group) in groups {
                guard.insert(hash, group);
            }
        }
    }

    pub fn remove(&self, hash: u64) {
        if let Ok(mut guard) = self.groups.lock() {
            guard.remove(&hash);
        }
    }

    pub fn find_parent(&self, parent_hash: u64) -> Option<Parent> {
        let guard = self.groups.lock().ok()?;
        for group in guard.values() {
            if group.parent_a.hash.as_u64() == parent_hash {
                return Some(group.parent_a.clone());
            }
            if group.parent_b.hash.as_u64() == parent_hash {
                return Some(group.parent_b.clone());
            }
            if let Some(ref g) = group.grandparent_aa {
                if g.hash.as_u64() == parent_hash {
                    return Some(g.clone());
                }
            }
            if let Some(ref g) = group.grandparent_ab {
                if g.hash.as_u64() == parent_hash {
                    return Some(g.clone());
                }
            }
            if let Some(ref g) = group.grandparent_ba {
                if g.hash.as_u64() == parent_hash {
                    return Some(g.clone());
                }
            }
            if let Some(ref g) = group.grandparent_bb {
                if g.hash.as_u64() == parent_hash {
                    return Some(g.clone());
                }
            }
        }
        None
    }
}
