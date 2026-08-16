use crate::models::{
    friend_card_info::FriendCardInfoModel, trained_chara::TrainedCharaModel,
    user_info_at_friend::UserInfoAtFriendModel,
};
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// EntryInfo object attached to the Single Mode start view controller.
/// Carries the parallel rental friend arrays used by the Trainer Browser.
pub struct EntryInfoModel;

pub const KEY_RENTAL_USER_INFO_ARRAY: &str = "rental_user_info_array";
pub const KEY_RENTAL_TRAINED_CHARA_ARRAY: &str = "rental_trained_chara_array";
pub const KEY_FRIEND_CARD_INFO_LIST: &str = "friend_card_info_list";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for EntryInfoModel {
    fn model_name() -> &'static str {
        "EntryInfo"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_RENTAL_USER_INFO_ARRAY,
                emit: true,
                required: true,
                candidates: &["RentalUserInfoArray", "_rentalUserInfoArray"],
                reader: FieldReaderKind::PointerArray(UserInfoAtFriendModel::read_model_value),
            },
            FieldSpec {
                key: KEY_RENTAL_TRAINED_CHARA_ARRAY,
                emit: true,
                required: true,
                candidates: &["RentalTrainedCharaArray", "_rentalTrainedCharaArray"],
                reader: FieldReaderKind::PointerArray(TrainedCharaModel::read_model_value),
            },
            FieldSpec {
                key: KEY_FRIEND_CARD_INFO_LIST,
                emit: true,
                required: false,
                candidates: &["FriendCardInfoList", "_friendCardInfoList"],
                reader: FieldReaderKind::PointerList(FriendCardInfoModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
