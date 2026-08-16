use crate::models::entry_info::EntryInfoModel;
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// Root model for the Single Mode select screen. Holds the `EntryInfo`
/// object that carries the rental friend lists.
pub struct SingleModeStartFriendsModel;

pub const KEY_ENTRY: &str = "entry";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SingleModeStartFriendsModel {
    fn model_name() -> &'static str {
        "SingleModeStartViewController"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_ENTRY,
            emit: true,
            required: true,
            candidates: &["<Entry>k__BackingField", "_entry", "Entry"],
            reader: FieldReaderKind::Pointer(EntryInfoModel::read_model_value),
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
