use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// Circle info attached to a friend entry. Best-effort: all fields optional.
pub struct CircleInfoAtFriendModel;

pub const KEY_CIRCLE_ID: &str = "circle_id";
pub const KEY_CIRCLE_NAME: &str = "circle_name";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for CircleInfoAtFriendModel {
    fn model_name() -> &'static str {
        "CircleInfoAtFriend"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_CIRCLE_ID,
                emit: true,
                required: false,
                candidates: &["circle_id", "<CircleId>k__BackingField", "CircleId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_CIRCLE_NAME,
                emit: true,
                required: false,
                candidates: &["circle_name", "<CircleName>k__BackingField", "CircleName"],
                reader: FieldReaderKind::ManagedString,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
