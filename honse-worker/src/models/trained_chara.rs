use crate::models::{
    factor_info::FactorInfoModel, race_result::RaceResultModel, skill::SkillModel,
    succession_chara::SuccessionCharaModel, succession_history::SuccessionHistoryModel,
    support_card::SupportCardModel,
};
use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct TrainedCharaModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_TRAINED_CHARA_ID: &str = "trained_chara_id";
pub const KEY_OWNER_VIEWER_ID: &str = "owner_viewer_id";
pub const KEY_OWNER_TRAINED_CHARA_ID: &str = "owner_trained_chara_id";
pub const KEY_SINGLE_MODE_CHARA_ID: &str = "single_mode_chara_id";
pub const KEY_CHARA_SEED: &str = "chara_seed";
pub const KEY_CARD_ID: &str = "card_id";
pub const KEY_SUCCESSION_TRAINED_CHARA_ID_1: &str = "succession_trained_chara_id_1";
pub const KEY_SUCCESSION_TRAINED_CHARA_ID_2: &str = "succession_trained_chara_id_2";
pub const KEY_USE_TYPE: &str = "use_type";
pub const KEY_SPEED: &str = "speed";
pub const KEY_STAMINA: &str = "stamina";
pub const KEY_POWER: &str = "power";
pub const KEY_WIZ: &str = "wiz";
pub const KEY_GUTS: &str = "guts";
pub const KEY_FANS: &str = "fans";
pub const KEY_RANK_SCORE: &str = "rank_score";
pub const KEY_RANK: &str = "rank";
pub const KEY_SCENARIO_ID: &str = "scenario_id";
pub const KEY_ROUTE_ID: &str = "route_id";
pub const KEY_ARRIVE_ROUTE_RACE_ID: &str = "arrive_route_race_id";
pub const KEY_PROPER_GROUND_TURF: &str = "proper_ground_turf";
pub const KEY_PROPER_GROUND_DIRT: &str = "proper_ground_dirt";
pub const KEY_PROPER_RUNNING_STYLE_NIGE: &str = "proper_running_style_nige";
pub const KEY_PROPER_RUNNING_STYLE_SENKO: &str = "proper_running_style_senko";
pub const KEY_PROPER_RUNNING_STYLE_SASHI: &str = "proper_running_style_sashi";
pub const KEY_PROPER_RUNNING_STYLE_OIKOMI: &str = "proper_running_style_oikomi";
pub const KEY_PROPER_DISTANCE_SHORT: &str = "proper_distance_short";
pub const KEY_PROPER_DISTANCE_MILE: &str = "proper_distance_mile";
pub const KEY_PROPER_DISTANCE_MIDDLE: &str = "proper_distance_middle";
pub const KEY_PROPER_DISTANCE_LONG: &str = "proper_distance_long";
pub const KEY_SUCCESSION_NUM: &str = "succession_num";
pub const KEY_RARITY: &str = "rarity";
pub const KEY_IS_SAVED: &str = "is_saved";
pub const KEY_IS_LOCKED: &str = "is_locked";
pub const KEY_TALENT_LEVEL: &str = "talent_level";
pub const KEY_RACE_CLOTH_ID: &str = "race_cloth_id";
pub const KEY_CHARA_GRADE: &str = "chara_grade";
pub const KEY_RUNNING_STYLE: &str = "running_style";
pub const KEY_NICKNAME_ID: &str = "nickname_id";
pub const KEY_WINS: &str = "wins";
pub const KEY_CREATE_TIME_SOURCE: &str = "_create_time_source";
pub const KEY_CACHED_CREATE_TS_SOURCE: &str = "_cached_create_ts_source";
pub const KEY_CREATE_TIME: &str = "create_time";
pub const KEY_REGISTER_TIME: &str = "register_time";
pub const KEY_SKILL_ARRAY: &str = "skill_array";
pub const KEY_SUPPORT_CARD_LIST: &str = "support_card_list";
pub const KEY_RACE_RESULT_LIST: &str = "race_result_list";
pub const KEY_WIN_SADDLE_ID_ARRAY: &str = "win_saddle_id_array";
pub const KEY_NICKNAME_ID_ARRAY: &str = "nickname_id_array";
pub const KEY_FACTOR_INFO_ARRAY: &str = "factor_info_array";
pub const KEY_FACTOR_ID_ARRAY: &str = "factor_id_array";
pub const KEY_SUCCESSION_CHARA_ARRAY: &str = "succession_chara_array";
pub const KEY_SUCCESSION_HISTORY_ARRAY: &str = "succession_history_array";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for TrainedCharaModel {
    fn model_name() -> &'static str {
        "TrainedChara"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["viewerId"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_ID,
                emit: true,
                required: true,
                candidates: &["id"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_OWNER_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["ownerViewerId"],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_OWNER_TRAINED_CHARA_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_SINGLE_MODE_CHARA_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_CHARA_SEED,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_CARD_ID,
                emit: true,
                required: true,
                candidates: &["cardId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SUCCESSION_TRAINED_CHARA_ID_1,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_SUCCESSION_TRAINED_CHARA_ID_2,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_USE_TYPE,
                emit: true,
                required: true,
                candidates: &["useType"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_SPEED,
                emit: true,
                required: true,
                candidates: &["speed"],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_STAMINA,
                emit: true,
                required: true,
                candidates: &["stamina"],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_POWER,
                emit: true,
                required: true,
                candidates: &["power"],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_WIZ,
                emit: true,
                required: true,
                candidates: &["wiz"],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_GUTS,
                emit: true,
                required: true,
                candidates: &["guts"],
                reader: FieldReaderKind::ObscuredInt,
            },
            FieldSpec {
                key: KEY_FANS,
                emit: true,
                required: true,
                candidates: &["fans"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RANK_SCORE,
                emit: true,
                required: true,
                candidates: &["rankScore"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RANK,
                emit: true,
                required: true,
                candidates: &["rank"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SCENARIO_ID,
                emit: true,
                required: true,
                candidates: &["ScenarioId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_ROUTE_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_ARRIVE_ROUTE_RACE_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_PROPER_GROUND_TURF,
                emit: true,
                required: true,
                candidates: &["properGroundTurf"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_GROUND_DIRT,
                emit: true,
                required: true,
                candidates: &["properGroundDirt"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_NIGE,
                emit: true,
                required: true,
                candidates: &["properRunningStyleNige"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_SENKO,
                emit: true,
                required: true,
                candidates: &["properRunningStyleSenko"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_SASHI,
                emit: true,
                required: true,
                candidates: &["properRunningStyleSashi"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_OIKOMI,
                emit: true,
                required: true,
                candidates: &["properRunningStyleOikomi"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_SHORT,
                emit: true,
                required: true,
                candidates: &["properDistanceShort"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_MILE,
                emit: true,
                required: true,
                candidates: &["properDistanceMile"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_MIDDLE,
                emit: true,
                required: true,
                candidates: &["properDistanceMiddle"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_LONG,
                emit: true,
                required: true,
                candidates: &["properDistanceLong"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_SUCCESSION_NUM,
                emit: true,
                required: true,
                candidates: &["SuccessionCount"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RARITY,
                emit: true,
                required: true,
                candidates: &["Rarity"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_IS_SAVED,
                emit: true,
                required: true,
                candidates: &["isSaved"],
                reader: FieldReaderKind::ObscuredBoolAsI64,
            },
            FieldSpec {
                key: KEY_IS_LOCKED,
                emit: true,
                required: true,
                candidates: &["IsLock"],
                reader: FieldReaderKind::ObscuredBoolAsI64,
            },
            FieldSpec {
                key: KEY_TALENT_LEVEL,
                emit: true,
                required: true,
                candidates: &["TalentLevel"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RACE_CLOTH_ID,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_CHARA_GRADE,
                emit: true,
                required: true,
                candidates: &["CharaGrade"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RUNNING_STYLE,
                emit: true,
                required: true,
                candidates: &["runningStyle"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_NICKNAME_ID,
                emit: true,
                required: true,
                candidates: &["nickNameId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_WINS,
                emit: true,
                required: true,
                candidates: &["singleWinNum"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_CREATE_TIME_SOURCE,
                emit: false,
                required: false,
                candidates: &[
                    "<CreateTime>k__BackingField",
                    "_createTime",
                    "createTime",
                    "CreateTime",
                ],
                reader: FieldReaderKind::ObscuredString,
            },
            FieldSpec {
                key: KEY_CACHED_CREATE_TS_SOURCE,
                emit: false,
                required: false,
                candidates: &[
                    "<CachedCreateTimeTimeStamp>k__BackingField",
                    "_cachedCreateTimeTimeStamp",
                    "cachedCreateTimeTimeStamp",
                    "CachedCreateTimeTimeStamp",
                ],
                reader: FieldReaderKind::ObscuredLongAsI64,
            },
            FieldSpec {
                key: KEY_CREATE_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::TimestampFrom {
                    string_source_key: KEY_CREATE_TIME_SOURCE,
                    obscured_unix_source_key: KEY_CACHED_CREATE_TS_SOURCE,
                },
            },
            FieldSpec {
                key: KEY_REGISTER_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::Alias {
                    source_key: KEY_CREATE_TIME,
                },
            },
            FieldSpec {
                key: KEY_SKILL_ARRAY,
                emit: true,
                required: true,
                candidates: &["AcquiredSkillArray"],
                reader: FieldReaderKind::PointerArray(SkillModel::read_model_value),
            },
            FieldSpec {
                key: KEY_SUPPORT_CARD_LIST,
                emit: true,
                required: true,
                candidates: &["SupportCardArray"],
                reader: FieldReaderKind::PointerArray(SupportCardModel::read_model_value),
            },
            FieldSpec {
                key: KEY_RACE_RESULT_LIST,
                emit: true,
                required: true,
                candidates: &["SingleModeRaceResultArray"],
                reader: FieldReaderKind::PointerArray(RaceResultModel::read_model_value),
            },
            FieldSpec {
                key: KEY_WIN_SADDLE_ID_ARRAY,
                emit: true,
                required: true,
                candidates: &["winSaddleIdArray"],
                reader: FieldReaderKind::ObscuredIntArray,
            },
            FieldSpec {
                key: KEY_NICKNAME_ID_ARRAY,
                emit: true,
                required: true,
                candidates: &["nickNameIdArray"],
                reader: FieldReaderKind::ObscuredIntArray,
            },
            FieldSpec {
                key: KEY_FACTOR_INFO_ARRAY,
                emit: true,
                required: true,
                candidates: &["FactorDataArray"],
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
                key: KEY_SUCCESSION_CHARA_ARRAY,
                emit: true,
                required: true,
                candidates: &["SuccessionCharaList"],
                reader: FieldReaderKind::PointerList(SuccessionCharaModel::read_model_value),
            },
            FieldSpec {
                key: KEY_SUCCESSION_HISTORY_ARRAY,
                emit: true,
                required: true,
                candidates: &["SuccessionHistoryList"],
                reader: FieldReaderKind::PointerList(SuccessionHistoryModel::read_model_value),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
