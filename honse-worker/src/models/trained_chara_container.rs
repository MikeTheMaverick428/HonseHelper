use crate::models::{
    favourite_chara_item::FavouriteCharaItemModel, trained_chara::TrainedCharaModel,
};
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct TrainedCharaContainerModel;

pub const KEY_TRAINED_CHARA: &str = "trained_chara";
pub const KEY_TRAINED_CHARA_FAVORITE_ARRAY: &str = "trained_chara_favorite_array";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for TrainedCharaContainerModel {
    fn model_name() -> &'static str {
        "WorkTrainedCharaData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_TRAINED_CHARA,
                emit: true,
                required: true,
                candidates: &["_dataDic", "<DataDic>k__BackingField", "dataDic", "DataDic"],
                reader: FieldReaderKind::TypedDictionary(TrainedCharaModel::read_model_value),
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_FAVORITE_ARRAY,
                emit: true,
                required: true,
                candidates: &[
                    "_favoriteDataDict",
                    "<FavoriteDataDict>k__BackingField",
                    "favoriteDataDict",
                    "FavoriteDataDict",
                ],
                reader: FieldReaderKind::TypedDictionary(FavouriteCharaItemModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
