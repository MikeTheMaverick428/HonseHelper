use chrono::{DateTime, Utc};

pub struct FriendData {
    pub id: i64,
    pub friend_type: u8,
    pub trainer_id: i64,
    pub name: String,
    pub borrow_uma_hash: u64,
    pub support_card_id: i64,
    pub character_id: i64,
    pub card_type: u8,
    pub card_rarity: u8,
    pub limit_break_count: u8,
    pub created_at: DateTime<Utc>,
}
