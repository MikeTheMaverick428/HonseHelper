use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct OwnedSupportCardModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_SUPPORT_CARD_ID: &str = "support_card_id";
pub const KEY_EXP: &str = "exp";
pub const KEY_LIMIT_BREAK_COUNT: &str = "limit_break_count";
pub const KEY_FAVORITE_FLAG: &str = "favorite_flag";
pub const KEY_STOCK: &str = "stock";
pub const KEY_CREATE_TIME_UNIX: &str = "_create_time_unix";
pub const KEY_CREATE_TIME: &str = "create_time";
pub const KEY_POSSESS_TIME: &str = "possess_time";
pub const KEY_LEVEL: &str = "level";
pub const KEY_MAX_LEVEL: &str = "max_level";
pub const KEY_BEST_TRAINING: &str = "best_training";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for OwnedSupportCardModel {
    fn model_name() -> &'static str {
        "SupportCardData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_ID,
                emit: true,
                required: true,
                candidates: &["_supportCardId", "supportCardId", "SupportCardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_EXP,
                emit: true,
                required: true,
                candidates: &["_exp", "exp", "Exp"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_LIMIT_BREAK_COUNT,
                emit: true,
                required: true,
                candidates: &["_limitBreakCount", "limitBreakCount", "LimitBreakCount"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_FAVORITE_FLAG,
                emit: true,
                required: true,
                candidates: &[
                    "<IsFavoriteLock>k__BackingField",
                    "_isFavoriteLock",
                    "isFavoriteLock",
                    "IsFavoriteLock",
                ],
                reader: FieldReaderKind::ObscuredBoolAsI64,
            },
            FieldSpec {
                key: KEY_STOCK,
                emit: true,
                required: true,
                candidates: &["_stock", "stock", "Stock"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_CREATE_TIME_UNIX,
                emit: false,
                required: true,
                candidates: &["_createTime", "createTime", "CreateTime"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_CREATE_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::TimestampFrom {
                    string_source_key: "",
                    obscured_unix_source_key: KEY_CREATE_TIME_UNIX,
                },
            },
            FieldSpec {
                key: KEY_POSSESS_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_LEVEL,
                emit: true,
                required: true,
                candidates: &["_level", "level", "Level"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_MAX_LEVEL,
                emit: true,
                required: true,
                candidates: &["_maxLevel", "maxLevel", "MaxLevel"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_BEST_TRAINING,
                emit: true,
                required: true,
                candidates: &["_bestTraining", "bestTraining", "BestTraining"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
