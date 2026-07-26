use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct FactorInfoModel;

pub const KEY_FACTOR_ID: &str = "factor_id";
pub const KEY_LEVEL: &str = "level";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for FactorInfoModel {
    fn model_name() -> &'static str {
        "FactorInfo"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_FACTOR_ID,
                emit: true,
                required: true,
                candidates: &["FactorId", "factorId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_LEVEL,
                emit: true,
                required: true,
                candidates: &["FactorLv", "factorLv"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
