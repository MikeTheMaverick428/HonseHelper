use crate::models::{owned_support_card::OwnedSupportCardModel, trained_chara::TrainedCharaModel};
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct FriendEntryModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_NAME: &str = "name";
pub const KEY_FRIEND_STATE: &str = "friend_state";
pub const KEY_HONOR_ID: &str = "honor_id";
pub const KEY_LAST_LOGIN_STR: &str = "_last_login_str";
pub const KEY_LAST_LOGIN_UNIX: &str = "_last_login_unix";
pub const KEY_LAST_LOGIN_TIME: &str = "last_login_time";
pub const KEY_COMMENT: &str = "comment";
pub const KEY_FAN: &str = "fan";
pub const KEY_CIRCLE_NAME: &str = "circle_name";
pub const KEY_CIRCLE_ID: &str = "circle_id";
pub const KEY_SC_ID: &str = "sc_id";
pub const KEY_SC_LIMIT_BREAK: &str = "sc_limit_break";
pub const KEY_SC_EXP: &str = "sc_exp";
pub const KEY_USER_SUPPORT_CARD: &str = "user_support_card";
pub const KEY_USER_TRAINED_CHARA: &str = "user_trained_chara";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for FriendEntryModel {
    fn model_name() -> &'static str {
        "FriendData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["<ViewerId>k__BackingField", "viewerId", "ViewerId"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_NAME,
                emit: true,
                required: true,
                candidates: &["<Name>k__BackingField", "name", "Name"],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_FRIEND_STATE,
                emit: true,
                required: true,
                candidates: &["_friendState", "friendState", "FriendState"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_HONOR_ID,
                emit: true,
                required: true,
                candidates: &["<HonorId>k__BackingField", "honorId", "HonorId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_LAST_LOGIN_STR,
                emit: false,
                required: false,
                candidates: &[
                    "<LastLoginTime>k__BackingField",
                    "lastLoginTime",
                    "LastLoginTime",
                ],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_LAST_LOGIN_UNIX,
                emit: false,
                required: true,
                candidates: &[
                    "<LastLoginUnixTime>k__BackingField",
                    "lastLoginUnixTime",
                    "LastLoginUnixTime",
                ],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_LAST_LOGIN_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::TimestampFrom {
                    string_source_key: KEY_LAST_LOGIN_STR,
                    obscured_unix_source_key: KEY_LAST_LOGIN_UNIX,
                },
            },
            FieldSpec {
                key: KEY_COMMENT,
                emit: true,
                required: true,
                candidates: &["<Comment>k__BackingField", "comment", "Comment"],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_FAN,
                emit: true,
                required: true,
                candidates: &["<Fan>k__BackingField", "fan", "Fan"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_CIRCLE_NAME,
                emit: true,
                required: false,
                candidates: &["<CircleName>k__BackingField", "circleName", "CircleName"],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_CIRCLE_ID,
                emit: true,
                required: false,
                candidates: &["<CircleId>k__BackingField", "circleId", "CircleId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SC_ID,
                emit: true,
                required: false,
                candidates: &[
                    "<SupportCardId>k__BackingField",
                    "supportCardId",
                    "SupportCardId",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SC_LIMIT_BREAK,
                emit: true,
                required: false,
                candidates: &[
                    "<SupportCardLimitBreakCount>k__BackingField",
                    "supportCardLimitBreakCount",
                    "SupportCardLimitBreakCount",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SC_EXP,
                emit: true,
                required: false,
                candidates: &[
                    "<SupportCardExp>k__BackingField",
                    "supportCardExp",
                    "SupportCardExp",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_USER_SUPPORT_CARD,
                emit: true,
                required: false,
                candidates: &[
                    "<VirtualSupportCardData>k__BackingField",
                    "virtualSupportCardData",
                    "VirtualSupportCardData",
                ],
                reader: FieldReaderKind::Pointer(OwnedSupportCardModel::read_model_value),
            },
            FieldSpec {
                key: KEY_USER_TRAINED_CHARA,
                emit: true,
                required: false,
                candidates: &[
                    "<VirtualTrainedCharaData>k__BackingField",
                    "virtualTrainedCharaData",
                    "VirtualTrainedCharaData",
                ],
                reader: FieldReaderKind::Pointer(TrainedCharaModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
