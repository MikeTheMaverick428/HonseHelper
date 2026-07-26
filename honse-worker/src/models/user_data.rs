use std::sync::LazyLock;

use anyhow::Result;
use il2cpp_runtime::{
    FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeIntrospector, RuntimeModelSpec,
};
use serde::{Deserialize, Serialize};

static USER_DATA_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static CARROT_DATA_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

pub const KEY_TRAINER_ID: &str = "trainer_id";
pub const KEY_TRAINER_NAME: &str = "trainer_name";
pub const KEY_CARRAT_STONE: &str = "carrats";

pub const KEY_PAID_CARRATS: &str = "paid";
pub const KEY_FREE_CARRATS: &str = "free";

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserData {
    pub trainer_id: i64,
    pub trainer_name: String,
    pub free_carrats_count: u32,
    pub paid_carrats_count: u32,
}

impl UserData {
    pub fn extract_from_data_manager(
        ctx: &mut RuntimeIntrospector,
        entry_ptr: u64,
    ) -> Result<UserData> {
        let user_data_off = ctx.resolve_runtime_offset_for_object(
            entry_ptr,
            &[
                "<UserData>k__BackingField",
                "UserData",
                "userData",
                "_userData",
            ],
        )?;
        let user_data_ptr = ctx.read_pointer_at(entry_ptr + user_data_off)?;

        let value = UserDataModel::read_model_value(ctx, user_data_ptr)?;

        let map = value
            .as_map()
            .ok_or_else(|| anyhow::anyhow!("UserData value is not a map"))?;

        let get_i64 = |key: &str| -> Option<i64> {
            map.iter().find_map(|(k, v)| {
                if k.as_str() == Some(key) {
                    v.as_i64()
                } else {
                    None
                }
            })
        };

        let get_str = |key: &str| -> Option<String> {
            map.iter().find_map(|(k, v)| {
                if k.as_str() == Some(key) {
                    v.as_str().map(String::from)
                } else {
                    None
                }
            })
        };

        let trainer_id =
            get_i64(KEY_TRAINER_ID).ok_or_else(|| anyhow::anyhow!("missing {}", KEY_TRAINER_ID))?;

        let trainer_name = get_str(KEY_TRAINER_NAME)
            .ok_or_else(|| anyhow::anyhow!("missing {}", KEY_TRAINER_NAME))?;

        let carrats_map = map
            .iter()
            .find_map(|(k, v)| {
                if k.as_str() == Some(KEY_CARRAT_STONE) {
                    v.as_map()
                } else {
                    None
                }
            })
            .ok_or_else(|| anyhow::anyhow!("missing {}", KEY_CARRAT_STONE))?;

        let get_carrat = |key: &str| -> Option<u32> {
            carrats_map.iter().find_map(|(k, v)| {
                if k.as_str() == Some(key) {
                    v.as_i64().map(|n| n as u32)
                } else {
                    None
                }
            })
        };

        let free_carrats_count = get_carrat(KEY_FREE_CARRATS)
            .ok_or_else(|| anyhow::anyhow!("missing {}", KEY_FREE_CARRATS))?;

        let paid_carrats_count = get_carrat(KEY_PAID_CARRATS)
            .ok_or_else(|| anyhow::anyhow!("missing {}", KEY_PAID_CARRATS))?;

        Ok(UserData {
            trainer_id,
            trainer_name,
            free_carrats_count,
            paid_carrats_count,
        })
    }
}

pub struct UserDataModel;

impl RuntimeModelSpec for UserDataModel {
    fn model_name() -> &'static str {
        "UserData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &USER_DATA_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_TRAINER_ID,
                emit: true,
                required: true,
                candidates: &[
                    "<ViewerId>k__BackingField",
                    "viewerId",
                    "ViewerId",
                    "_viewerId",
                ],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_TRAINER_NAME,
                emit: true,
                required: true,
                candidates: &[
                    "<UserName>k__BackingField",
                    "userName",
                    "UserName",
                    "_userName",
                ],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_CARRAT_STONE,
                emit: true,
                required: true,
                candidates: &[
                    "<CarrotStone>k__BackingField",
                    "CarrotStone",
                    "carrotStone",
                    "_carrotStone",
                ],
                reader: FieldReaderKind::Pointer(CarrotDataModel::read_model_value),
            },
        ]
    }
}

pub struct CarrotDataModel;

impl RuntimeModelSpec for CarrotDataModel {
    fn model_name() -> &'static str {
        "CarrotData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &CARROT_DATA_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_PAID_CARRATS,
                emit: true,
                required: true,
                candidates: &[
                    "<ChargeCoin>k__BackingField",
                    "ChargeCoin",
                    "chargeCoin",
                    "_chargeCoin",
                ],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_FREE_CARRATS,
                emit: true,
                required: true,
                candidates: &[
                    "<FreeCoin>k__BackingField",
                    "FreeCoin",
                    "freeCoin",
                    "_freeCoin",
                ],
                reader: FieldReaderKind::ObscuredInt,
            },
        ]
    }
}
