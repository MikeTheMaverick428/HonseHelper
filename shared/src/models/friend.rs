use serde::{Deserialize, Serialize};

use super::{SupportCardRarity, SupportCardType};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FriendData {
    pub trainer_id: i64,
    pub name: String,
    pub borrow_uma_hash: u64,
    pub support_card: SupportCard,
}

impl FriendData {
    // pub fn from_mssgpack_and_db(
    //     mssgpack: &MssgPackSummaryUserInfo,
    //     borrow_uma_hash: u64,
    //     support_card_storage: &dyn crate::storage::ItemStorage<
    //         i64,
    //         crate::db::support::SupportCardDbResult,
    //     >,
    // ) -> Self {
    //     Self {
    //         trainer_id: mssgpack.viewer_id,
    //         name: mssgpack.name.clone(),
    //         borrow_uma_hash,
    //         support_card: SupportCard::from_friend_mssgpack_and_db(
    //             &mssgpack.user_support_card,
    //             support_card_storage,
    //         ),
    //     }
    // }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportCard {
    pub id: i64,
    pub name: String,
    pub character_id: i64,
    pub card_type: SupportCardType,
    pub card_rarity: SupportCardRarity,
    pub limit_break_count: u8,
}
