use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// Honor data attached to a friend entry. Best-effort: all fields optional.
pub struct HonorDataModel;

pub const KEY_HONOR_ID: &str = "honor_id";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for HonorDataModel {
    fn model_name() -> &'static str {
        "HonorData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_HONOR_ID,
            emit: true,
            required: false,
            candidates: &["honor_id", "<HonorId>k__BackingField", "HonorId"],
            reader: FieldReaderKind::ObscuredIntAsI64,
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
