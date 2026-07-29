use crate::models::race_horse_skill::RaceHorseSkillModel;
use crate::models::trained_chara_data::TrainedCharaDataModel;
use anyhow::{anyhow, Result};
use il2cpp_runtime::{
    FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeIntrospector, RuntimeModelSpec,
    RuntimeValue,
};
use rmpv::Value;
use std::collections::HashMap;
use std::sync::LazyLock;

// ---------------------------------------------------------------------------
// Constants
// ---------------------------------------------------------------------------

const KEY_HORSE_INDEX: &str = "horse_index";
const KEY_TRAINED_CHARA_DATA: &str = "trained_chara_data";
const KEY_POST_NUMBER: &str = "post_number";
const KEY_CHARA_ID: &str = "chara_id";
const KEY_CHARA_NAME: &str = "chara_name";
const KEY_RESPONSE_HORSE_DATA: &str = "response_horse_data";
const KEY_VIEWER_ID: &str = "viewer_id";
const KEY_OWNER_VIEWER_ID: &str = "owner_viewer_id";
const KEY_TRAINER_NAME: &str = "trainer_name";
const KEY_SINGLE_MODE_CHARA_ID: &str = "single_mode_chara_id";
const KEY_CARD_ID: &str = "card_id";
const KEY_NPC_TYPE: &str = "npc_type";
const KEY_SPEED: &str = "speed";
const KEY_STAMINA: &str = "stamina";
const KEY_POW: &str = "pow";
const KEY_GUTS: &str = "guts";
const KEY_WIZ: &str = "wiz";
const KEY_FINISH_ORDER: &str = "finish_order";
const KEY_FINISH_TIME_RAW: &str = "finish_time_raw";
const KEY_FINISH_TIME_SCALED: &str = "finish_time_scaled";
const KEY_FINISH_DIFF_TIME: &str = "finish_diff_time";
const KEY_POPULARITY: &str = "popularity";
const KEY_POPULARITY_RANK_LEFT: &str = "popularity_rank_left";
const KEY_POPULARITY_RANK_CENTER: &str = "popularity_rank_center";
const KEY_POPULARITY_RANK_RIGHT: &str = "popularity_rank_right";
const KEY_RARITY: &str = "rarity";
const KEY_IS_GHOST: &str = "is_ghost";
const KEY_DEFEAT: &str = "defeat";
const KEY_RACE_DRESS_ID: &str = "race_dress_id";
const KEY_RUNNING_TYPE: &str = "running_type";
const KEY_ACTIVE_PROPER_DISTANCE: &str = "active_proper_distance";
const KEY_ACTIVE_PROPER_GROUND_TYPE: &str = "active_proper_ground_type";
const KEY_MOB_ID: &str = "mob_id";
const KEY_FINISH_ORDER_RAW_SCORE: &str = "finish_order_raw_score";

const KEY_RUNNING_STYLE: &str = "running_style";
const KEY_FINISH_TIME: &str = "finish_time";
const KEY_START_DELAY_TIME: &str = "start_delay_time";
const KEY_LAST_SPURT_START_DISTANCE: &str = "last_spurt_start_distance";

const KEY_DISTANCE: &str = "distance";
const KEY_LANE_POSITION: &str = "lane_position";
const KEY_HP: &str = "hp";
const KEY_TEMPTATION_MODE: &str = "temptationMode";
const KEY_BLOCK_FRONT_HORSE_INDEX: &str = "blockFrontHorseIndex";

const KEY_TIME: &str = "time";
const KEY_HORSE_DATA_ARRAY: &str = "horseDataArray";

const KEY_FRAME_TIME: &str = "frame_time";
const KEY_EVENT_TYPE: &str = "type";
const KEY_PARAM: &str = "param";

static RESULT_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

static EVENT_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static HORSE_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static HORSE_FRAME_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static RACE_FRAME_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static RACE_INFO_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static COURSE_SET_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);
static RACE_INSTANCE_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

pub enum TemptationMode {
    None = 0,
    PositionSashi = 1,
    PositionSenko = 2,
    PositionNige = 3,
    Boost = 4,
}

impl TemptationMode {
    pub fn hakuraku_value(&self) -> &'static str {
        match self {
            TemptationMode::None => "NONE",
            TemptationMode::PositionSashi => "POSITION_SASHI",
            TemptationMode::PositionSenko => "POSITION_SENKO",
            TemptationMode::PositionNige => "POSITION_NIGE",
            TemptationMode::Boost => "BOOST",
        }
    }
}

pub enum EventType {
    Score = 0,
    ChallengeMatchPoint = 1,
    NoUse2 = 2,
    Skill = 3,
    CompeteTop = 4,
    CompeteFight = 5,
}

impl EventType {
    pub fn hakuraku_value(&self) -> &'static str {
        match self {
            EventType::Score => "SCORE",
            EventType::ChallengeMatchPoint => "CHALLENGE_MATCH_POINT",
            EventType::NoUse2 => "NOUSE_2",
            EventType::Skill => "SKILL",
            EventType::CompeteTop => "COMPETE_TOP",
            EventType::CompeteFight => "COMPETE_FIGHT",
        }
    }
}

// ---------------------------------------------------------------------------
// HorseDataModel — Gallop::HorseData (horse identity)
// ---------------------------------------------------------------------------

pub struct HorseDataModel;

impl RuntimeModelSpec for HorseDataModel {
    fn model_name() -> &'static str {
        "HorseData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &HORSE_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_HORSE_INDEX,
                emit: true,
                required: true,
                candidates: &["horseIndex", "_horseIndex", "HorseIndex"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_POST_NUMBER,
                emit: true,
                required: true,
                candidates: &["postNumber", "_postNumber", "PostNumber"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_CHARA_ID,
                emit: true,
                required: true,
                candidates: &["charaId", "_charaId", "CharaId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_CHARA_NAME,
                emit: true,
                required: false,
                candidates: &[
                    "<charaName>k__BackingField",
                    "charaName",
                    "CharaName",
                    "_charaName",
                ],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_FINISH_ORDER,
                emit: true,
                required: false,
                candidates: &["FinishOrder", "_finishOrder", "finishOrder"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_FINISH_TIME_RAW,
                emit: true,
                required: false,
                candidates: &["FinishTimeRaw", "finishTimeRaw", "_finishTimeRaw"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_FINISH_TIME_SCALED,
                emit: true,
                required: false,
                candidates: &["FinishTimeScaled", "finishTimeScaled", "_finishTimeScaled"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_FINISH_DIFF_TIME,
                emit: true,
                required: false,
                candidates: &[
                    "FinishDiffTimeFromPrev",
                    "finishDiffTimeFromPrev",
                    "_finishDiffTimeFromPrev",
                    "FinishDiffTimeFromPrev",
                ],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_POPULARITY,
                emit: true,
                required: false,
                candidates: &[
                    "<Popularity>k__BackingField",
                    "Popularity",
                    "_popularity",
                    "popularity",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_POPULARITY_RANK_LEFT,
                emit: true,
                required: false,
                candidates: &[
                    "<PopularityRankLeft>k__BackingField",
                    "PopularityRankLeft",
                    "_popularityRankLeft",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_POPULARITY_RANK_CENTER,
                emit: true,
                required: false,
                candidates: &[
                    "<PopularityRankCenter>k__BackingField",
                    "PopularityRankCenter",
                    "_popularityRankCenter",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_POPULARITY_RANK_RIGHT,
                emit: true,
                required: false,
                candidates: &[
                    "<PopularityRankRight>k__BackingField",
                    "PopularityRankRight",
                    "_popularityRankRight",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RARITY,
                emit: true,
                required: false,
                candidates: &["<Rarity>k__BackingField", "Rarity", "_rarity", "rarity"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_TRAINER_NAME,
                emit: true,
                required: false,
                candidates: &[
                    "<TrainerName>k__BackingField",
                    "TrainerName",
                    "_trainerName",
                    "trainerName",
                ],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_IS_GHOST,
                emit: true,
                required: false,
                candidates: &["IsGhost", "_isGhost", "isGhost"],
                reader: FieldReaderKind::Bool,
            },
            FieldSpec {
                key: KEY_DEFEAT,
                emit: true,
                required: false,
                candidates: &["<Defeat>k__BackingField", "Defeat", "_defeat", "defeat"],
                reader: FieldReaderKind::Bool,
            },
            FieldSpec {
                key: KEY_RACE_DRESS_ID,
                emit: true,
                required: false,
                candidates: &[
                    "<RaceDressId>k__BackingField",
                    "RaceDressId",
                    "_raceDressId",
                    "raceDressId",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RUNNING_TYPE,
                emit: true,
                required: false,
                candidates: &[
                    "<RunningType>k__BackingField",
                    "RunningType",
                    "_runningType",
                    "runningType",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_ACTIVE_PROPER_DISTANCE,
                emit: true,
                required: false,
                candidates: &[
                    "<ActiveProperDistance>k__BackingField",
                    "ActiveProperDistance",
                    "_activeProperDistance",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_ACTIVE_PROPER_GROUND_TYPE,
                emit: true,
                required: false,
                candidates: &[
                    "<ActiveProperGroundType>k__BackingField",
                    "ActiveProperGroundType",
                    "_activeProperGroundType",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_MOB_ID,
                emit: true,
                required: false,
                candidates: &["<MobId>k__BackingField", "MobId", "_mobId", "mobId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_FINISH_ORDER_RAW_SCORE,
                emit: true,
                required: false,
                candidates: &[
                    "<FinishOrderRawScore>k__BackingField",
                    "FinishOrderRawScore",
                    "_finishOrderRawScore",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_DATA,
                emit: true,
                required: false,
                candidates: &["<TrainedCharaData>k__BackingField"],
                reader: FieldReaderKind::Pointer(TrainedCharaDataModel::read_model_value),
            },
            FieldSpec {
                key: KEY_RESPONSE_HORSE_DATA,
                emit: true,
                required: false,
                candidates: &[
                    "_responseHorseData",
                    "responseHorseData",
                    "ResponseHorseData",
                ],
                reader: FieldReaderKind::Pointer(ResponseHorseDataModel::read_model_value),
            },
        ]
    }
}

// ---------------------------------------------------------------------------
// ResponseHorseDataModel — Gallop::RaceHorseData (network response data)
// ---------------------------------------------------------------------------

pub struct ResponseHorseDataModel;

impl RuntimeModelSpec for ResponseHorseDataModel {
    fn model_name() -> &'static str {
        "RaceHorseData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &HORSE_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: false,
                candidates: &["viewer_id", "ViewerId", "_viewerId"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_OWNER_VIEWER_ID,
                emit: true,
                required: false,
                candidates: &["owner_viewer_id", "OwnerViewerId", "_ownerViewerId"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_TRAINER_NAME,
                emit: true,
                required: false,
                candidates: &["trainer_name", "TrainerName", "_trainerName"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_SINGLE_MODE_CHARA_ID,
                emit: true,
                required: false,
                candidates: &[
                    "single_mode_chara_id",
                    "SingleModeCharaId",
                    "_singleModeCharaId",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_CARD_ID,
                emit: true,
                required: false,
                candidates: &["card_id", "CardId", "_cardId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_NPC_TYPE,
                emit: true,
                required: false,
                candidates: &["npc_type", "NpcType", "_npcType"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_SPEED,
                emit: true,
                required: false,
                candidates: &["speed", "Speed", "_speed"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_STAMINA,
                emit: true,
                required: false,
                candidates: &["stamina", "Stamina", "_stamina"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_POW,
                emit: true,
                required: false,
                candidates: &["pow", "Pow", "_pow"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_GUTS,
                emit: true,
                required: false,
                candidates: &["guts", "Guts", "_guts"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_WIZ,
                emit: true,
                required: false,
                candidates: &["wiz", "Wiz", "_wiz"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_distance_short",
                emit: true,
                required: false,
                candidates: &[
                    "proper_distance_short",
                    "ProperDistanceShort",
                    "_properDistanceShort",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_distance_mile",
                emit: true,
                required: false,
                candidates: &[
                    "proper_distance_mile",
                    "ProperDistanceMile",
                    "_properDistanceMile",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_distance_middle",
                emit: true,
                required: false,
                candidates: &[
                    "proper_distance_middle",
                    "ProperDistanceMiddle",
                    "_properDistanceMiddle",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_distance_long",
                emit: true,
                required: false,
                candidates: &[
                    "proper_distance_long",
                    "ProperDistanceLong",
                    "_properDistanceLong",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_running_style_nige",
                emit: true,
                required: false,
                candidates: &[
                    "proper_running_style_nige",
                    "ProperRunningStyleNige",
                    "_properRunningStyleNige",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_running_style_senko",
                emit: true,
                required: false,
                candidates: &[
                    "proper_running_style_senko",
                    "ProperRunningStyleSenko",
                    "_properRunningStyleSenko",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_running_style_sashi",
                emit: true,
                required: false,
                candidates: &[
                    "proper_running_style_sashi",
                    "ProperRunningStyleSashi",
                    "_properRunningStyleSashi",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_running_style_oikomi",
                emit: true,
                required: false,
                candidates: &[
                    "proper_running_style_oikomi",
                    "ProperRunningStyleOikomi",
                    "_properRunningStyleOikomi",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_ground_turf",
                emit: true,
                required: false,
                candidates: &[
                    "proper_ground_turf",
                    "ProperGroundTurf",
                    "_properGroundTurf",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "proper_ground_dirt",
                emit: true,
                required: false,
                candidates: &[
                    "proper_ground_dirt",
                    "ProperGroundDirt",
                    "_properGroundDirt",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "skill_array",
                emit: true,
                required: false,
                candidates: &["skill_array", "SkillArray", "_skillArray"],
                reader: FieldReaderKind::PointerArray(RaceHorseSkillModel::read_model_value),
            },
        ]
    }
}

// ---------------------------------------------------------------------------
// HorseResultDataModel — Gallop::RaceSimulateHorseResultData
// ---------------------------------------------------------------------------

pub struct HorseResultDataModel;

impl RuntimeModelSpec for HorseResultDataModel {
    fn model_name() -> &'static str {
        "RaceSimulateHorseResultData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &RESULT_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_FINISH_ORDER,
                emit: true,
                required: true,
                candidates: &["finishOrder", "_finishOrder", "FinishOrder"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RUNNING_STYLE,
                emit: true,
                required: false,
                candidates: &["runningStyle", "_runningStyle", "RunningStyle"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_FINISH_TIME,
                emit: true,
                required: false,
                candidates: &["finishTime", "_finishTime", "FinishTime"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_FINISH_TIME_RAW,
                emit: true,
                required: false,
                candidates: &["finishTimeRaw", "_finishTimeRaw", "FinishTimeRaw"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_FINISH_DIFF_TIME,
                emit: true,
                required: false,
                candidates: &["finishDiffTime", "_finishDiffTime", "FinishDiffTime"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_START_DELAY_TIME,
                emit: true,
                required: false,
                candidates: &["startDelayTime", "_startDelayTime", "StartDelayTime"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_LAST_SPURT_START_DISTANCE,
                emit: true,
                required: false,
                candidates: &[
                    "lastSpurtStartDistance",
                    "_lastSpurtStartDistance",
                    "LastSpurtStartDistance",
                ],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_DEFEAT,
                emit: true,
                required: false,
                candidates: &["defeat", "_defeat", "Defeat"],
                reader: FieldReaderKind::Bool,
            },
        ]
    }
}

// ---------------------------------------------------------------------------
// RaceSimulateEventDataModel — Gallop::RaceSimulateEventData
// ---------------------------------------------------------------------------

pub struct RaceSimulateEventDataModel;

impl RuntimeModelSpec for RaceSimulateEventDataModel {
    fn model_name() -> &'static str {
        "RaceSimulateEventData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &EVENT_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_FRAME_TIME,
                emit: true,
                required: true,
                candidates: &["frameTime", "FrameTime", "_frameTime"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_EVENT_TYPE,
                emit: true,
                required: true,
                candidates: &["type", "Type", "_type"],
                reader: FieldReaderKind::I32,
            },
            FieldSpec {
                key: KEY_PARAM,
                emit: true,
                required: false,
                candidates: &["param", "Param", "_param"],
                reader: FieldReaderKind::Int32Array,
            },
        ]
    }
}

fn read_event_data(ctx: &mut RuntimeIntrospector, event_ptr: u64) -> Result<Value> {
    RaceSimulateEventDataModel::read_model_value(ctx, event_ptr)
}

// ---------------------------------------------------------------------------
// RaceInfoModel — Gallop::RaceInfo (race metadata)
// ---------------------------------------------------------------------------

pub struct RaceInfoModel;

impl RuntimeModelSpec for RaceInfoModel {
    fn model_name() -> &'static str {
        "RaceInfo"
    }
    fn cache() -> &'static ModelOffsetCache {
        &RACE_INFO_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: "race_type",
                emit: true,
                required: false,
                candidates: &["<RaceType>k__BackingField", "RaceType", "_raceType"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "weather",
                emit: true,
                required: false,
                candidates: &["<Weather>k__BackingField", "Weather", "_weather"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "season",
                emit: true,
                required: false,
                candidates: &["<Season>k__BackingField", "Season", "_season"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "ground_condition",
                emit: true,
                required: false,
                candidates: &[
                    "<GroundCondition>k__BackingField",
                    "GroundCondition",
                    "_groundCondition",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "course_furlong_num",
                emit: true,
                required: false,
                candidates: &[
                    "<CourseFurlongNum>k__BackingField",
                    "CourseFurlongNum",
                    "_courseFurlongNum",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "race_course_set",
                emit: true,
                required: false,
                candidates: &[
                    "<RaceCourseSet>k__BackingField",
                    "RaceCourseSet",
                    "_raceCourseSet",
                ],
                reader: FieldReaderKind::Pointer(RaceCourseSetModel::read_model_value),
            },
            FieldSpec {
                key: "player_horse_index",
                emit: true,
                required: false,
                candidates: &[
                    "_playerHorseIndex",
                    "PlayerHorseIndex",
                    "<PlayerHorseIndex>k__BackingField",
                ],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "sim_data_base64",
                emit: true,
                required: false,
                candidates: &[
                    "<SimDataBase64>k__BackingField",
                    "SimDataBase64",
                    "_simDataBase64",
                ],
                reader: FieldReaderKind::ManagedString,
            },
        ]
    }
}

// ---------------------------------------------------------------------------
// RaceCourseSetModel — course configuration
// ---------------------------------------------------------------------------

pub struct RaceCourseSetModel;

impl RuntimeModelSpec for RaceCourseSetModel {
    fn model_name() -> &'static str {
        "RaceCourseSet"
    }
    fn cache() -> &'static ModelOffsetCache {
        &COURSE_SET_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: "race_course_set_id",
                emit: true,
                required: false,
                candidates: &["<Id>k__BackingField", "Id", "_id"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "track_id",
                emit: true,
                required: false,
                candidates: &["RaceTrackId", "raceTrackId", "_raceTrackId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "distance",
                emit: true,
                required: false,
                candidates: &["Distance", "distance", "_distance"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "ground_type",
                emit: true,
                required: false,
                candidates: &["Ground", "ground", "_ground"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "turn",
                emit: true,
                required: false,
                candidates: &["Turn", "turn", "_turn"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "inout",
                emit: true,
                required: false,
                candidates: &["Inout", "inout", "_inout"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "float_lane_max",
                emit: true,
                required: false,
                candidates: &["FloatLaneMax", "floatLaneMax", "_floatLaneMax"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }
}

// ---------------------------------------------------------------------------
// RaceInstanceModel — Gallop::RaceInstance (race_instance_id, race_id)
// ---------------------------------------------------------------------------

pub struct RaceInstanceModel;

impl RuntimeModelSpec for RaceInstanceModel {
    fn model_name() -> &'static str {
        "RaceInstance"
    }
    fn cache() -> &'static ModelOffsetCache {
        &RACE_INSTANCE_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: "race_instance_id",
                emit: true,
                required: false,
                candidates: &["Id", "id", "_id", "<Id>k__BackingField"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "race_id",
                emit: true,
                required: false,
                candidates: &["RaceId", "raceId", "_raceId", "<RaceId>k__BackingField"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }
}

// Read race_instance_id and race_id from RaceInfo._raceInstanceMaster (pointer → RaceInstance)
fn extract_race_instance_ids(
    ctx: &mut RuntimeIntrospector,
    race_info_ptr: u64,
) -> Option<(i64, i64)> {
    let instance_ptr = resolve_field_ptr(
        ctx,
        race_info_ptr,
        &[
            "_raceInstanceMaster",
            "RaceInstanceMaster",
            "<RaceInstanceMaster>k__BackingField",
        ],
    )
    .ok()?;
    let meta = RaceInstanceModel::read_model_value(ctx, instance_ptr).ok()?;
    let id = meta
        .as_map()
        .and_then(|m| {
            m.iter()
                .find(|(k, _)| k.as_str() == Some("race_instance_id"))
        })
        .and_then(|(_, v)| v.as_i64())?;
    let race_id = meta
        .as_map()
        .and_then(|m| m.iter().find(|(k, _)| k.as_str() == Some("race_id")))
        .and_then(|(_, v)| v.as_i64())?;
    Some((id, race_id))
}

fn flatten_course_set(meta: &mut Value) {
    if let Value::Map(ref mut entries) = meta {
        if let Some(cs) = entries
            .iter()
            .position(|(k, _)| k.as_str() == Some("race_course_set"))
        {
            let flat: Vec<(Value, Value)> = entries[cs]
                .1
                .as_map()
                .map(|m| m.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
                .unwrap_or_default();
            entries.remove(cs);
            entries.extend(flat);
        }
        if let Some(pos) = entries
            .iter()
            .position(|(k, _)| k.as_str() == Some("course_furlong_num"))
        {
            entries.remove(pos);
        }
    }
}

pub fn detect_race_type(ctx: &mut RuntimeIntrospector, entry_ptr: u64) -> i64 {
    resolve_field_ptr(
        ctx,
        entry_ptr,
        &[
            "<BgmController>k__BackingField",
            "BgmController",
            "_bgmController",
        ],
    )
    .ok()
    .and_then(|bgm| {
        resolve_field_ptr(
            ctx,
            bgm,
            &["_horseAccessor", "horseAccessor", "HorseAccessor"],
        )
        .ok()
    })
    .and_then(|race_mgr| resolve_field_ptr(ctx, race_mgr, &["_jikkyo", "jikkyo", "Jikkyo"]).ok())
    .and_then(|jikkyo| resolve_field_ptr(ctx, jikkyo, &["_raceInfo", "raceInfo", "RaceInfo"]).ok())
    .and_then(|ri| {
        ctx.resolve_runtime_offset_for_object(
            ri,
            &["<RaceType>k__BackingField", "RaceType", "_raceType"],
        )
        .ok()
        .and_then(|off| ctx.read_i32_at(ri + off).ok().map(|v| v as i64))
    })
    .unwrap_or(0)
}

// ---------------------------------------------------------------------------
// Frame data
// ---------------------------------------------------------------------------

pub struct HorseFrameDataModel;

impl RuntimeModelSpec for HorseFrameDataModel {
    fn model_name() -> &'static str {
        "HorseFrameData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &HORSE_FRAME_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_DISTANCE,
                emit: true,
                required: true,
                candidates: &["distance", "_distance", "Distance"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_LANE_POSITION,
                emit: true,
                required: true,
                candidates: &["lanePosition", "_lanePosition", "LanePosition"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_SPEED,
                emit: true,
                required: true,
                candidates: &["speed", "_speed", "Speed"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_HP,
                emit: true,
                required: true,
                candidates: &["hp", "_hp", "HP"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_TEMPTATION_MODE,
                emit: true,
                required: false,
                candidates: &["temptationMode", "_temptationMode", "TemptationMode"],
                reader: FieldReaderKind::I8,
            },
            FieldSpec {
                key: KEY_BLOCK_FRONT_HORSE_INDEX,
                emit: true,
                required: false,
                candidates: &[
                    "blockFrontHorseIndex",
                    "_blockFrontHorseIndex",
                    "BlockFrontHorseIndex",
                ],
                reader: FieldReaderKind::I32,
            },
        ]
    }
}

pub struct RaceFrameDataModel;

impl RuntimeModelSpec for RaceFrameDataModel {
    fn model_name() -> &'static str {
        "RaceFrameData"
    }
    fn cache() -> &'static ModelOffsetCache {
        &RACE_FRAME_CACHE
    }
    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_TIME,
                emit: true,
                required: true,
                candidates: &["Time", "time", "_time"],
                reader: FieldReaderKind::F32,
            },
            FieldSpec {
                key: KEY_HORSE_DATA_ARRAY,
                emit: true,
                required: true,
                candidates: &["HorseDataArray", "horseDataArray", "_horseDataArray"],
                reader: FieldReaderKind::PointerArray(HorseFrameDataModel::read_model_value),
            },
        ]
    }
}

fn read_frame_data(ctx: &mut RuntimeIntrospector, frame_ptr: u64) -> Result<Value> {
    RaceFrameDataModel::read_model_value(ctx, frame_ptr)
}

// ---------------------------------------------------------------------------
// Navigation helpers
// ---------------------------------------------------------------------------

fn resolve_field_ptr(
    ctx: &mut RuntimeIntrospector,
    obj_ptr: u64,
    candidates: &[&str],
) -> Result<u64> {
    let off = ctx.resolve_runtime_offset_for_object(obj_ptr, candidates)?;
    ctx.read_pointer_at(obj_ptr + off)
}

// ---------------------------------------------------------------------------
// Top-level extraction
// ---------------------------------------------------------------------------

pub fn extract_race_team_data(
    ctx: &mut RuntimeIntrospector,
    entry_ptr: u64,
    race_type: i64,
) -> Result<RuntimeValue> {
    // --- 1. Resolve BgmController ---
    let bgm_ptr = resolve_field_ptr(
        ctx,
        entry_ptr,
        &[
            "<BgmController>k__BackingField",
            "BgmController",
            "bgmController",
            "_bgmController",
        ],
    )
    .or_else(|_: anyhow::Error| {
        ctx.resolve_runtime_offset_for_object(entry_ptr, &["_simReader", "simReader", "SimReader"])
            .map(|_| entry_ptr)
    })?;

    // --- 2. RaceSimulateData (results + frames + events) ---
    let sim_reader_ptr =
        resolve_field_ptr(ctx, bgm_ptr, &["_simReader", "simReader", "SimReader"])?;

    let sim_data_ptr = resolve_field_ptr(ctx, sim_reader_ptr, &["_simData", "simData", "SimData"])?;

    let horse_result_arr_ptr = resolve_field_ptr(
        ctx,
        sim_data_ptr,
        &[
            "_horseResultDataArray",
            "horseResultDataArray",
            "HorseResultDataArray",
        ],
    )?;

    let horse_result_ptrs = ctx
        .read_pointer_array_from_array_ptr(horse_result_arr_ptr)
        .unwrap_or_default();
    let horse_results: Vec<Value> = horse_result_ptrs
        .into_iter()
        .filter_map(|p| HorseResultDataModel::read_model_value(ctx, p).ok())
        .collect();

    let frame_list_ptr = resolve_field_ptr(
        ctx,
        sim_data_ptr,
        &["_frameDataList", "frameDataList", "FrameDataList"],
    )?;
    let frame_ptrs = ctx.read_pointer_list_from_list_ptr(frame_list_ptr)?;
    let frames: Vec<Value> = frame_ptrs
        .into_iter()
        .filter_map(|ptr| read_frame_data(ctx, ptr).ok())
        .collect();

    let ev_list_ptr = resolve_field_ptr(
        ctx,
        sim_data_ptr,
        &["_simEvDataList", "simEvDataList", "SimEvDataList"],
    )?;
    let ev_ptrs = ctx.read_pointer_list_from_list_ptr(ev_list_ptr)?;
    let events: Vec<Value> = ev_ptrs
        .into_iter()
        .filter_map(|ptr| read_event_data(ctx, ptr).ok())
        .collect();

    let horse_num_off = ctx
        .resolve_runtime_offset_for_object(sim_data_ptr, &["_horseNum", "horseNum", "HorseNum"])?;
    let horse_num = ctx
        .process_memory()
        .read_i32(sim_data_ptr + horse_num_off)?;

    // --- 3. Resolve horse manager ---
    let horse_accessor_ptr = resolve_field_ptr(
        ctx,
        bgm_ptr,
        &["_horseAccessor", "horseAccessor", "HorseAccessor"],
    )?;

    let horse_mgr_ptr = resolve_field_ptr(
        ctx,
        horse_accessor_ptr,
        &["_horseManager", "horseManager", "HorseManager"],
    )?;

    // --- 4. Team membership (TeamStadium only) ---
    // Walk _teamInfoList to build horse_index -> (team_id, is_player) map
    let mut horse_team_map: HashMap<i64, (i64, i64)> = HashMap::new();
    if race_type == 14 {
        if let Ok(team_list_ptr) = resolve_field_ptr(
            ctx,
            horse_mgr_ptr,
            &["_teamInfoList", "teamInfoList", "TeamInfoList"],
        ) {
            if let Ok(team_ptrs) = ctx.read_pointer_list_from_list_ptr(team_list_ptr) {
                for team_ptr in team_ptrs {
                    let team_id_val = ctx
                        .resolve_runtime_offset_for_object(
                            team_ptr,
                            &["<TeamId>k__BackingField", "TeamId", "_teamId"],
                        )
                        .ok()
                        .and_then(|off| ctx.read_i32_at(team_ptr + off).ok())
                        .unwrap_or(0);
                    let is_player = if team_id_val == 1 { 1i64 } else { 0i64 };

                    if let Ok(member_arr_ptr) = resolve_field_ptr(
                        ctx,
                        team_ptr,
                        &[
                            "<TeamMemberArray>k__BackingField",
                            "TeamMemberArray",
                            "_teamMemberArray",
                        ],
                    ) {
                        if let Ok(member_ptrs) =
                            ctx.read_pointer_array_from_array_ptr(member_arr_ptr)
                        {
                            for member_ptr in member_ptrs {
                                let horse_idx = resolve_field_ptr(
                                    ctx,
                                    member_ptr,
                                    &["_horseData", "horseData", "HorseData"],
                                )
                                .ok()
                                .and_then(|hd| {
                                    ctx.resolve_runtime_offset_for_object(
                                        hd,
                                        &["horseIndex", "_horseIndex", "HorseIndex"],
                                    )
                                    .ok()
                                    .and_then(|off| {
                                        ctx.read_i32_at(hd + off).ok().map(|v| v as i64)
                                    })
                                });
                                if let Some(idx) = horse_idx {
                                    horse_team_map.insert(idx, (team_id_val as i64, is_player));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // --- 5. Horse identity data ---
    let horse_infos_arr_ptr = resolve_field_ptr(
        ctx,
        horse_mgr_ptr,
        &["_horseRaceInfos", "horseRaceInfos", "HorseRaceInfos"],
    )?;

    let horse_info_ptrs = ctx.read_pointer_array_from_array_ptr(horse_infos_arr_ptr)?;

    let horses: Vec<Value> = horse_info_ptrs
        .into_iter()
        .filter_map(|info_ptr| {
            let horse_data_ptr =
                resolve_field_ptr(ctx, info_ptr, &["_horseData", "horseData", "HorseData"]).ok()?;
            let data = HorseDataModel::read_model_value(ctx, horse_data_ptr).ok()?;
            let horse_idx = data.as_map().and_then(|m| {
                m.iter().find_map(|(k, v)| {
                    if k.as_str() == Some("horse_index") {
                        v.as_i64()
                    } else {
                        None
                    }
                })
            });
            let result = horse_idx
                .filter(|&idx| idx >= 0 && (idx as usize) < horse_results.len())
                .and_then(|idx| Some(horse_results[idx as usize].clone()));

            let mut entries = Vec::new();
            if let Some(map) = data.as_map() {
                entries.reserve(map.len() + 3);
                entries.extend(map.iter().cloned());
            }
            if let Some(r) = result {
                entries.push((Value::from("result"), r));
            }
            if let Some(idx) = horse_idx {
                if let Some(&(team_id, is_player)) = horse_team_map.get(&idx) {
                    entries.push((Value::from("team_id"), Value::from(team_id)));
                    entries.push((Value::from("is_player"), Value::from(is_player)));
                }
            }
            Some(Value::Map(entries))
        })
        .collect();

    // --- 6. Metadata ---
    let metadata =
        extract_race_metadata(ctx, bgm_ptr, race_type, entry_ptr).unwrap_or(Value::Map(Vec::new()));

    Ok(Value::Map(vec![
        (Value::from("race_type"), Value::from(race_type)),
        (Value::from("horse_count"), Value::from(horse_num)),
        (Value::from("horses"), Value::Array(horses)),
        (Value::from("frames"), Value::Array(frames)),
        (Value::from("events"), Value::Array(events)),
        (Value::from("metadata"), metadata),
    ]))
}

fn extract_race_metadata(
    ctx: &mut RuntimeIntrospector,
    bgm_ptr: u64,
    race_type: i64,
    entry_ptr: u64,
) -> Result<Value> {
    match race_type {
        5 => {
            let mut meta = extract_std_metadata(ctx, bgm_ptr)?;
            if let Ok(champs) = extract_champions_metadata(ctx, entry_ptr) {
                if let Value::Map(ref mut entries) = meta {
                    if let Value::Map(champs_entries) = champs {
                        entries.extend(champs_entries);
                    }
                }
            }
            Ok(meta)
        }
        6 => extract_single_metadata(ctx, bgm_ptr),
        8 => extract_room_match_metadata(ctx, bgm_ptr),
        14 => extract_team_stadium_metadata(ctx, bgm_ptr),
        _ => extract_std_metadata(ctx, bgm_ptr),
    }
}

fn resolve_race_info_ptr(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Option<u64> {
    resolve_field_ptr(
        ctx,
        bgm_ptr,
        &["_horseAccessor", "horseAccessor", "HorseAccessor"],
    )
    .ok()
    .and_then(|race_mgr| resolve_field_ptr(ctx, race_mgr, &["_jikkyo", "jikkyo", "Jikkyo"]).ok())
    .and_then(|jikkyo| resolve_field_ptr(ctx, jikkyo, &["_raceInfo", "raceInfo", "RaceInfo"]).ok())
}

fn extract_single_metadata(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Result<Value> {
    if let Some(ri) = resolve_race_info_ptr(ctx, bgm_ptr) {
        let mut meta = RaceInfoModel::read_model_value(ctx, ri)?;
        flatten_course_set(&mut meta);

        if let Some((race_instance_id, race_id)) = extract_race_instance_ids(ctx, ri) {
            if let Value::Map(ref mut entries) = meta {
                entries.push((
                    Value::from("race_instance_id"),
                    Value::from(race_instance_id),
                ));
                entries.push((Value::from("race_id"), Value::from(race_id)));
            }
        }

        return Ok(meta);
    }

    // Fallback: RaceConditionInfo on BgmController (legacy)
    extract_race_condition_fallback(ctx, bgm_ptr)
}

fn extract_std_metadata(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Result<Value> {
    if let Some(ri) = resolve_race_info_ptr(ctx, bgm_ptr) {
        let mut meta = RaceInfoModel::read_model_value(ctx, ri)?;
        flatten_course_set(&mut meta);

        if let Some((race_instance_id, race_id)) = extract_race_instance_ids(ctx, ri) {
            if let Value::Map(ref mut entries) = meta {
                entries.push((
                    Value::from("race_instance_id"),
                    Value::from(race_instance_id),
                ));
                entries.push((Value::from("race_id"), Value::from(race_id)));
            }
        }

        return Ok(meta);
    }

    extract_race_condition_fallback(ctx, bgm_ptr)
}

fn extract_room_match_metadata(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Result<Value> {
    extract_std_metadata(ctx, bgm_ptr)
}

fn extract_team_stadium_metadata(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Result<Value> {
    if let Some(ri) = resolve_race_info_ptr(ctx, bgm_ptr) {
        let mut meta = RaceInfoModel::read_model_value(ctx, ri)?;

        // Weather fallback: if weather is 0, try RaceManagerReplay.<RaceInfo>k__BackingField
        let has_weather = meta
            .as_map()
            .and_then(|m| m.iter().find(|(k, _)| k.as_str() == Some("weather")))
            .and_then(|(_, v)| v.as_i64())
            .filter(|&w| w != 0)
            .is_some();

        if !has_weather {
            if let Ok(race_mgr) = resolve_field_ptr(
                ctx,
                bgm_ptr,
                &["_horseAccessor", "horseAccessor", "HorseAccessor"],
            ) {
                if let Ok(off) = ctx.resolve_runtime_offset_for_object(
                    race_mgr,
                    &["<RaceInfo>k__BackingField", "RaceInfo", "_raceInfo"],
                ) {
                    if let Ok(ri2) = ctx.read_pointer_at(race_mgr + off) {
                        if ri2 != 0 {
                            if let Ok(alt_meta) = RaceInfoModel::read_model_value(ctx, ri2) {
                                if let Some(w) = alt_meta
                                    .as_map()
                                    .and_then(|m| {
                                        m.iter().find(|(k, _)| k.as_str() == Some("weather"))
                                    })
                                    .and_then(|(_, v)| v.as_i64())
                                    .filter(|&w| w != 0)
                                {
                                    if let Value::Map(ref mut entries) = meta {
                                        if let Some(pos) = entries
                                            .iter()
                                            .position(|(ek, _)| ek.as_str() == Some("weather"))
                                        {
                                            entries[pos] = (Value::from("weather"), Value::from(w));
                                        } else {
                                            entries.push((Value::from("weather"), Value::from(w)));
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }

        flatten_course_set(&mut meta);

        if let Some((race_instance_id, race_id)) = extract_race_instance_ids(ctx, ri) {
            if let Value::Map(ref mut entries) = meta {
                entries.push((
                    Value::from("race_instance_id"),
                    Value::from(race_instance_id),
                ));
                entries.push((Value::from("race_id"), Value::from(race_id)));
            }
        }

        return Ok(meta);
    }

    extract_race_condition_fallback(ctx, bgm_ptr)
}

fn extract_race_condition_fallback(ctx: &mut RuntimeIntrospector, bgm_ptr: u64) -> Result<Value> {
    if let Ok(rcp) = resolve_field_ptr(
        ctx,
        bgm_ptr,
        &[
            "_raceConditionInfo",
            "raceConditionInfo",
            "RaceConditionInfo",
            "_raceCondition",
            "raceCondition",
            "RaceCondition",
        ],
    ) {
        struct RaceConditionInfoModel;
        static RACE_CONDITION_CACHE: LazyLock<ModelOffsetCache> =
            LazyLock::new(ModelOffsetCache::default);
        impl RuntimeModelSpec for RaceConditionInfoModel {
            fn model_name() -> &'static str {
                "RaceConditionInfo"
            }
            fn cache() -> &'static ModelOffsetCache {
                &RACE_CONDITION_CACHE
            }
            fn fields() -> &'static [FieldSpec] {
                &[
                    FieldSpec {
                        key: "race_instance_id",
                        emit: true,
                        required: false,
                        candidates: &["race_instance_id", "RaceInstanceId", "_raceInstanceId"],
                        reader: FieldReaderKind::I32AsI64,
                    },
                    FieldSpec {
                        key: "season",
                        emit: true,
                        required: false,
                        candidates: &["season", "Season", "_season"],
                        reader: FieldReaderKind::I32AsI64,
                    },
                    FieldSpec {
                        key: "weather",
                        emit: true,
                        required: false,
                        candidates: &["weather", "Weather", "_weather"],
                        reader: FieldReaderKind::I32AsI64,
                    },
                    FieldSpec {
                        key: "ground_condition",
                        emit: true,
                        required: false,
                        candidates: &[
                            "ground_condition",
                            "groundCondition",
                            "GroundCondition",
                            "_groundCondition",
                        ],
                        reader: FieldReaderKind::I32AsI64,
                    },
                ]
            }
        }
        return RaceConditionInfoModel::read_model_value(ctx, rcp);
    }

    Err(anyhow!("no race metadata path available"))
}

// ---------------------------------------------------------------------------
// extract_champions_metadata — Gallop::RaceMainViewController → ChampionsRoundDetail
// ---------------------------------------------------------------------------

fn extract_champions_metadata(ctx: &mut RuntimeIntrospector, race_mvc_ptr: u64) -> Result<Value> {
    let round_detail_ptr = resolve_field_ptr(ctx, race_mvc_ptr, &["_championsRoundDetail"])?;

    fn read_i32_field(ctx: &mut RuntimeIntrospector, base: u64, offset: u64) -> Result<i64> {
        Ok(ctx.read_i32_at(base + offset)? as i64)
    }

    let champions_id = read_i32_field(ctx, round_detail_ptr, 0x14)?;
    let league_type = read_i32_field(ctx, round_detail_ptr, 0x18)?;
    let round = read_i32_field(ctx, round_detail_ptr, 0x20)?;

    Ok(Value::Map(vec![
        (Value::from("champions_id"), Value::from(champions_id)),
        (Value::from("league_type"), Value::from(league_type)),
        (Value::from("round"), Value::from(round)),
    ]))
}
