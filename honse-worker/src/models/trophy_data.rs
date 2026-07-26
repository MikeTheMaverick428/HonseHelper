use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct WorkTrophyDataModel;

pub const KEY_TROPHY_DIC: &str = "trophy_dic";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for WorkTrophyDataModel {
    fn model_name() -> &'static str {
        "WorkTrophyData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_TROPHY_DIC,
            emit: true,
            required: true,
            candidates: &["_dataDic", "DataDic", "dataDic"],
            reader: FieldReaderKind::TypedDictionary(TrophyDataModel::read_model_value),
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}

pub struct TrophyDataModel;

pub const KEY_TROPHY_ID: &str = "trophy_id";
pub const KEY_CHARA_ID_LIST: &str = "chara_id_list";

static TROPHY_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for TrophyDataModel {
    fn model_name() -> &'static str {
        "TrophyData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_TROPHY_ID,
                emit: true,
                required: true,
                candidates: &["_trophyId", "trophyId", "TrophyId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_CHARA_ID_LIST,
                emit: true,
                required: true,
                candidates: &["_charaIdList", "charaIdList", "CharaIdList"],
                reader: FieldReaderKind::Int32List,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &TROPHY_CACHE
    }
}
