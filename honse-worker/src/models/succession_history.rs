use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct SuccessionHistoryModel;

pub const KEY_ID: &str = "id";
pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_TRAINED_CHARA_ID: &str = "trained_chara_id";
pub const KEY_HISTORY_TYPE: &str = "history_type";
pub const KEY_SUCCESSION_CARD_ID: &str = "succession_card_id";
pub const KEY_DATE: &str = "date";
pub const KEY_USER_NAME: &str = "user_name";
pub const KEY_CIRCLE_NAME: &str = "circle_name";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SuccessionHistoryModel {
    fn model_name() -> &'static str {
        "SuccessionHistory"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_ID,
                emit: true,
                required: true,
                candidates: &["id", "Id"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["viewer_id", "viewerId"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_ID,
                emit: true,
                required: true,
                candidates: &["trained_chara_id", "trainedCharaId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_HISTORY_TYPE,
                emit: true,
                required: true,
                candidates: &["hisotry_type", "history_type", "historyType"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SUCCESSION_CARD_ID,
                emit: true,
                required: true,
                candidates: &["succession_card_id", "successionCardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_DATE,
                emit: true,
                required: true,
                candidates: &["date", "Date"],
                reader: FieldReaderKind::I32,
            },
            FieldSpec {
                key: KEY_USER_NAME,
                emit: true,
                required: true,
                candidates: &["user_name", "userName"],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_CIRCLE_NAME,
                emit: true,
                required: true,
                candidates: &["circle_name", "circleName"],
                reader: FieldReaderKind::ObscuredString,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
