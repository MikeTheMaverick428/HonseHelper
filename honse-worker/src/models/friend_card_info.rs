use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// Friend card header from the Single Mode start screen. Carries the
/// support card's real display level (`SupportCardLevel`).
pub struct FriendCardInfoModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_USER_NAME: &str = "user_name";
pub const KEY_SUPPORT_CARD_ID: &str = "support_card_id";
pub const KEY_SUPPORT_CARD_LEVEL: &str = "support_card_level";
pub const KEY_LIMIT_BREAK_COUNT: &str = "limit_break_count";
pub const KEY_FRIEND_STATE: &str = "friend_state";
pub const KEY_LAST_LOGIN_TIME: &str = "last_login_time";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for FriendCardInfoModel {
    fn model_name() -> &'static str {
        "FriendCardInfo"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["ViewerId", "viewerId", "<ViewerId>k__BackingField"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_USER_NAME,
                emit: true,
                required: false,
                candidates: &["UserName", "userName", "<UserName>k__BackingField"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_ID,
                emit: true,
                required: true,
                candidates: &["SupportCardId", "supportCardId", "<SupportCardId>k__BackingField"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_LEVEL,
                emit: true,
                required: true,
                candidates: &[
                    "SupportCardLevel",
                    "supportCardLevel",
                    "<SupportCardLevel>k__BackingField",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_LIMIT_BREAK_COUNT,
                emit: true,
                required: true,
                candidates: &["LimitBreakCount", "limitBreakCount", "<LimitBreakCount>k__BackingField"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_FRIEND_STATE,
                emit: true,
                required: true,
                candidates: &["FriendState", "friendState", "<FriendState>k__BackingField"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_LAST_LOGIN_TIME,
                emit: true,
                required: false,
                candidates: &["LastLoginTime", "lastLoginTime", "<LastLoginTime>k__BackingField"],
                reader: FieldReaderKind::I64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
