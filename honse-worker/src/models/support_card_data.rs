use crate::models::owned_support_card::OwnedSupportCardModel;
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct SupportCardDataModel;

pub const KEY_SUPPORT_CARDS: &str = "support_cards";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SupportCardDataModel {
    fn model_name() -> &'static str {
        "WorkSupportCardData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_SUPPORT_CARDS,
            emit: true,
            required: true,
            candidates: &["_dataDic", "dataDic", "DataDic"],
            reader: FieldReaderKind::TypedDictionary(OwnedSupportCardModel::read_model_value),
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
