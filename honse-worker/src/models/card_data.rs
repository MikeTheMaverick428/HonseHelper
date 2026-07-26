use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct WorkCardDataModel;

pub const KEY_CARD_DIC: &str = "card_dic";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for WorkCardDataModel {
    fn model_name() -> &'static str {
        "WorkCardData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_CARD_DIC,
            emit: true,
            required: true,
            candidates: &["_dataDic", "DataDic", "dataDic"],
            reader: FieldReaderKind::TypedDictionary(CardDataModel::read_model_value),
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}

pub struct CardDataModel;

pub const KEY_CARD_ID: &str = "card_id";
pub const KEY_TALENT_LEVEL: &str = "talent_level";
pub const KEY_RARITY: &str = "rarity";

static CARD_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for CardDataModel {
    fn model_name() -> &'static str {
        "CardData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_CARD_ID,
                emit: true,
                required: true,
                candidates: &["<CardId>k__BackingField", "CardId", "cardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_TALENT_LEVEL,
                emit: true,
                required: true,
                candidates: &["<TalentLevel>k__BackingField", "TalentLevel", "talentLevel"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RARITY,
                emit: true,
                required: true,
                candidates: &["<Rarity>k__BackingField", "Rarity", "rarity"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CARD_CACHE
    }
}
