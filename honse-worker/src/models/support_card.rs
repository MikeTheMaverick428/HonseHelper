use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct SupportCardModel;

pub const KEY_POSITION: &str = "position";
pub const KEY_SUPPORT_CARD_ID: &str = "support_card_id";
pub const KEY_EXP: &str = "exp";
pub const KEY_LIMIT_BREAK_COUNT: &str = "limit_break_count";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SupportCardModel {
    fn model_name() -> &'static str {
        "SupportCard"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_POSITION,
                emit: true,
                required: true,
                candidates: &["<Position>k__BackingField", "Position", "position"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_ID,
                emit: true,
                required: true,
                candidates: &[
                    "<SupportCardId>k__BackingField",
                    "SupportCardId",
                    "supportCardId",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_EXP,
                emit: true,
                required: true,
                candidates: &["<Exp>k__BackingField", "Exp", "exp"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_LIMIT_BREAK_COUNT,
                emit: true,
                required: true,
                candidates: &[
                    "<LimitBreakCount>k__BackingField",
                    "LimitBreakCount",
                    "limitBreakCount",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
