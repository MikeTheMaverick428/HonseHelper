use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct RaceResultModel;

pub const KEY_TURN: &str = "turn";
pub const KEY_PROGRAM_ID: &str = "program_id";
pub const KEY_WEATHER: &str = "weather";
pub const KEY_GROUND_CONDITION: &str = "ground_condition";
pub const KEY_RUNNING_STYLE: &str = "running_style";
pub const KEY_POPULARITY: &str = "popularity";
pub const KEY_RESULT_RANK: &str = "result_rank";
pub const KEY_RESULT_TIME: &str = "result_time";
pub const KEY_PRIZE_MONEY: &str = "prize_money";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for RaceResultModel {
    fn model_name() -> &'static str {
        "RaceResult"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_TURN,
                emit: true,
                required: true,
                candidates: &["_turn", "turn", "Turn"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_PROGRAM_ID,
                emit: true,
                required: true,
                candidates: &["_programId", "programId", "ProgramId"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_WEATHER,
                emit: true,
                required: true,
                candidates: &["_weather", "weather", "Weather"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_GROUND_CONDITION,
                emit: true,
                required: true,
                candidates: &["_groundCondition", "groundCondition", "GroundCondition"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RUNNING_STYLE,
                emit: true,
                required: true,
                candidates: &["_runningStyle", "runningStyle", "RunningStyle"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_POPULARITY,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_RESULT_RANK,
                emit: true,
                required: true,
                candidates: &["_resultRank", "resultRank", "ResultRank"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_RESULT_TIME,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
            FieldSpec {
                key: KEY_PRIZE_MONEY,
                emit: true,
                required: false,
                candidates: &[],
                reader: FieldReaderKind::ConstantI64(0),
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
