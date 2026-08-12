use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// The friend's borrow support card. Exposes id/exp/limit-break; the display
/// level is taken from `FriendCardInfo.SupportCardLevel` on the Tauri side.
/// Best-effort: all fields optional.
pub struct UserSupportCardAtFriendModel;

pub const KEY_SUPPORT_CARD_ID: &str = "support_card_id";
pub const KEY_EXP: &str = "exp";
pub const KEY_LIMIT_BREAK_COUNT: &str = "limit_break_count";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for UserSupportCardAtFriendModel {
    fn model_name() -> &'static str {
        "UserSupportCardAtFriend"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_SUPPORT_CARD_ID,
                emit: true,
                required: false,
                candidates: &["support_card_id", "<SupportCardId>k__BackingField", "SupportCardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_EXP,
                emit: false,
                required: false,
                candidates: &["exp", "<Exp>k__BackingField", "Exp"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_LIMIT_BREAK_COUNT,
                emit: true,
                required: false,
                candidates: &[
                    "limit_break_count",
                    "<LimitBreakCount>k__BackingField",
                    "LimitBreakCount",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
