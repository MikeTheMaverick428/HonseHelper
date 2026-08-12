use crate::models::{
    circle_info_at_friend::CircleInfoAtFriendModel, honor_data::HonorDataModel,
    user_support_card_at_friend::UserSupportCardAtFriendModel,
    user_trained_chara_at_friend::UserTrainedCharaAtFriendModel,
};
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// A single friend entry in the Single Mode start rental list.
///
/// `viewer_id` is a plain (non-obscured) i64 in this class; reading it as
/// obscured corrupts the value. It matches `TrainedCharaData._ownerViewerId`
/// (obscured long) of the friend's rental uma.
pub struct UserInfoAtFriendModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_NAME: &str = "name";
pub const KEY_HONOR_DATA: &str = "honor_data";
pub const KEY_LAST_LOGIN_TIME: &str = "last_login_time";
pub const KEY_LEADER_CHARA_ID: &str = "leader_chara_id";
pub const KEY_SUPPORT_CARD_ID: &str = "support_card_id";
pub const KEY_COMMENT: &str = "comment";
pub const KEY_FAN: &str = "fan";
pub const KEY_FRIEND_STATE: &str = "friend_state";
pub const KEY_CIRCLE_INFO: &str = "circle_info";
pub const KEY_USER_SUPPORT_CARD: &str = "user_support_card";
pub const KEY_USER_TRAINED_CHARA: &str = "user_trained_chara";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for UserInfoAtFriendModel {
    fn model_name() -> &'static str {
        "UserInfoAtFriend"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["viewer_id", "<ViewerId>k__BackingField", "ViewerId"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_NAME,
                emit: true,
                required: true,
                candidates: &["name", "<Name>k__BackingField", "Name"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_HONOR_DATA,
                emit: true,
                required: false,
                candidates: &["honor_data", "<HonorData>k__BackingField", "HonorData"],
                reader: FieldReaderKind::Pointer(HonorDataModel::read_model_value),
            },
            FieldSpec {
                key: KEY_LAST_LOGIN_TIME,
                emit: true,
                required: false,
                candidates: &["last_login_time", "<LastLoginTime>k__BackingField", "LastLoginTime"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_LEADER_CHARA_ID,
                emit: true,
                required: false,
                candidates: &["leader_chara_id", "<LeaderCharaId>k__BackingField", "LeaderCharaId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_ID,
                emit: true,
                required: false,
                candidates: &["support_card_id", "<SupportCardId>k__BackingField", "SupportCardId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_COMMENT,
                emit: true,
                required: true,
                candidates: &["comment", "<Comment>k__BackingField", "Comment"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_FAN,
                emit: true,
                required: true,
                candidates: &["fan", "<Fan>k__BackingField", "Fan"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_FRIEND_STATE,
                emit: true,
                required: true,
                candidates: &["friend_state", "friendState", "FriendState"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_CIRCLE_INFO,
                emit: true,
                required: false,
                candidates: &["circle_info", "<CircleInfo>k__BackingField", "CircleInfo"],
                reader: FieldReaderKind::Pointer(CircleInfoAtFriendModel::read_model_value),
            },
            FieldSpec {
                key: KEY_USER_SUPPORT_CARD,
                emit: true,
                required: false,
                candidates: &["user_support_card", "<UserSupportCard>k__BackingField", "UserSupportCard"],
                reader: FieldReaderKind::Pointer(UserSupportCardAtFriendModel::read_model_value),
            },
            FieldSpec {
                key: KEY_USER_TRAINED_CHARA,
                emit: true,
                required: false,
                candidates: &["user_trained_chara", "<UserTrainedChara>k__BackingField", "UserTrainedChara"],
                reader: FieldReaderKind::Pointer(UserTrainedCharaAtFriendModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
