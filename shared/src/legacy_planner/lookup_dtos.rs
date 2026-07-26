use nohash_hasher::IntSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlimUma {
    pub hash: u64,
    pub character_id: i64,
    pub wins: IntSet<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SlimUmaGroup {
    pub veteran: SlimUma,
    pub parent_a: u64,
    pub parent_b: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AffinityResult {
    pub base: u32,
    pub bonus: u32,
}

impl AffinityResult {
    pub fn total(&self) -> u32 {
        self.base + self.bonus
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct PaginatedVeteranHash {
    pub hash: u64,
    pub affinity: Option<AffinityResult>,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct AffinityResultTree {
    pub parent_a: Option<u32>,
    pub parent_b: Option<u32>,
    pub interparent: Option<AffinityResult>,
    pub grandparent_aa: Option<AffinityResult>,
    pub grandparent_ab: Option<AffinityResult>,
    pub grandparent_ba: Option<AffinityResult>,
    pub grandparent_bb: Option<AffinityResult>,
}

impl Default for AffinityResultTree {
    fn default() -> Self {
        Self {
            parent_a: None,
            parent_b: None,
            interparent: None,
            grandparent_aa: None,
            grandparent_ab: None,
            grandparent_ba: None,
            grandparent_bb: None,
        }
    }
}

impl AffinityResultTree {
    pub fn total_result(&self) -> AffinityResult {
        AffinityResult {
            base: self.parent_a.unwrap_or(0)
                + self.parent_b.unwrap_or(0)
                + self.interparent.map(|r| r.base).unwrap_or(0)
                + self.grandparent_aa.map(|r| r.base).unwrap_or(0)
                + self.grandparent_ab.map(|r| r.base).unwrap_or(0)
                + self.grandparent_ba.map(|r| r.base).unwrap_or(0)
                + self.grandparent_bb.map(|r| r.base).unwrap_or(0),
            bonus: self.interparent.map(|r| r.bonus).unwrap_or(0)
                + self.grandparent_aa.map(|r| r.bonus).unwrap_or(0)
                + self.grandparent_ab.map(|r| r.bonus).unwrap_or(0)
                + self.grandparent_ba.map(|r| r.bonus).unwrap_or(0)
                + self.grandparent_bb.map(|r| r.bonus).unwrap_or(0),
        }
    }

    pub fn total(&self) -> u32 {
        self.parent_a.unwrap_or(0)
            + self.parent_b.unwrap_or(0)
            + self.interparent.map(|r| r.total()).unwrap_or(0)
            + self.grandparent_aa.map(|r| r.total()).unwrap_or(0)
            + self.grandparent_ab.map(|r| r.total()).unwrap_or(0)
            + self.grandparent_ba.map(|r| r.total()).unwrap_or(0)
            + self.grandparent_bb.map(|r| r.total()).unwrap_or(0)
    }
}
