use crate::models::factor_info::FactorInfoModel;
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct SuccessionCharaModel;

pub const KEY_POSITION_ID: &str = "position_id";
pub const KEY_CARD_ID: &str = "card_id";
pub const KEY_RANK: &str = "rank";
pub const KEY_RARITY: &str = "rarity";
pub const KEY_TALENT_LEVEL: &str = "talent_level";
pub const KEY_FACTOR_INFO_ARRAY: &str = "factor_info_array";
pub const KEY_FACTOR_ID_ARRAY: &str = "factor_id_array";
pub const KEY_WIN_SADDLE_ID_ARRAY: &str = "win_saddle_id_array";
pub const KEY_OWNER_VIEWER_ID: &str = "owner_viewer_id";
pub const KEY_RACE_RESULT_LIST: &str = "race_result_list";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SuccessionCharaModel {
    fn model_name() -> &'static str {
        "SuccessionChara"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_POSITION_ID,
                emit: true,
                required: true,
                candidates: &["_positionId", "positionId", "PositionId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_CARD_ID,
                emit: true,
                required: true,
                candidates: &["<CardId>k__BackingField", "CardId", "cardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RANK,
                emit: true,
                required: true,
                candidates: &["_rank", "rank", "Rank"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RARITY,
                emit: true,
                required: true,
                candidates: &["<Rarity>k__BackingField", "Rarity", "rarity"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_TALENT_LEVEL,
                emit: true,
                required: true,
                candidates: &["<Level>k__BackingField", "Level", "level"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_FACTOR_INFO_ARRAY,
                emit: true,
                required: true,
                candidates: &[
                    "<FactorDataArray>k__BackingField",
                    "FactorDataArray",
                    "factorDataArray",
                ],
                reader: FieldReaderKind::PointerArray(FactorInfoModel::read_model_value),
            },
            FieldSpec {
                key: KEY_FACTOR_ID_ARRAY,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::FactorIdsFrom {
                    source_key: KEY_FACTOR_INFO_ARRAY,
                },
            },
            FieldSpec {
                key: KEY_WIN_SADDLE_ID_ARRAY,
                emit: true,
                required: true,
                candidates: &["_winSaddleIdArray", "winSaddleIdArray", "WinSaddleIdArray"],
                reader: FieldReaderKind::ObscuredIntArray,
            },
            FieldSpec {
                key: KEY_OWNER_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["_ownerViewerId", "ownerViewerId", "OwnerViewerId"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_RACE_RESULT_LIST,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantEmptyArray,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
