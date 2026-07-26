mod friend;
mod race;
mod race_dump;
mod trophy;
mod types;
mod veteran;

pub use friend::*;
pub use race::*;
pub use race_dump::*;
pub use trophy::*;
pub use types::*;
pub use veteran::*;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CharacterOption {
    pub character_id: i64,
    pub name: String,
}

#[derive(Debug, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginationRequest<T> {
    pub query: T,
    pub page: u32,
    pub page_size: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PaginationResponse<T> {
    pub results: Vec<T>,
    pub total: u32,
    pub page: u32,
    pub page_size: u32,
}

impl<T> PaginationResponse<T> {
    /// 1-indexed item number of the first entry on this page.
    /// Returns 0 if the page is empty.
    pub fn item_no_first_on_page(&self) -> u32 {
        if self.results.is_empty() {
            0
        } else {
            (self.page.saturating_sub(1)) * self.page_size + 1
        }
    }

    /// 1-indexed item number of the last entry on this page.
    /// Returns 0 if the page is empty.
    pub fn item_no_last_on_page(&self) -> u32 {
        if self.results.is_empty() {
            0
        } else {
            self.total.min(self.page * self.page_size)
        }
    }
}
