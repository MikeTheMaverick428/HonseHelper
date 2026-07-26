use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct FavouriteCharaItemModel;

pub const KEY_ICON_TYPE: &str = "icon_type";
pub const KEY_MEMO: &str = "memo";
pub const KEY_TRAINED_CHARA_ID: &str = "trained_chara_id";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for FavouriteCharaItemModel {
    fn model_name() -> &'static str {
        "FavouriteCharaItem"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_ICON_TYPE,
                emit: true,
                required: true,
                candidates: &["Type", "type"],
                reader: FieldReaderKind::I32,
            },
            FieldSpec {
                key: KEY_MEMO,
                emit: true,
                required: true,
                candidates: &["Memo", "memo"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_ID,
                emit: true,
                required: true,
                candidates: &["TrainedCharaId", "trainedCharaId"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
