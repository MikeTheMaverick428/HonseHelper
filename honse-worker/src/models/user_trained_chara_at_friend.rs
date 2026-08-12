use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

/// Summary row for the friend's rental uma (companion to the full
/// `TrainedCharaData` in `RentalTrainedCharaArray`). Plain int32 fields.
pub struct UserTrainedCharaAtFriendModel;

pub const KEY_VIEWER_ID: &str = "viewer_id";
pub const KEY_TRAINED_CHARA_ID: &str = "trained_chara_id";
pub const KEY_CARD_ID: &str = "card_id";
pub const KEY_RANK_SCORE: &str = "rank_score";
pub const KEY_RANK: &str = "rank";
pub const KEY_PROPER_DISTANCE_SHORT: &str = "proper_distance_short";
pub const KEY_PROPER_DISTANCE_MILE: &str = "proper_distance_mile";
pub const KEY_PROPER_DISTANCE_MIDDLE: &str = "proper_distance_middle";
pub const KEY_PROPER_DISTANCE_LONG: &str = "proper_distance_long";
pub const KEY_PROPER_RUNNING_STYLE_NIGE: &str = "proper_running_style_nige";
pub const KEY_PROPER_RUNNING_STYLE_SENKO: &str = "proper_running_style_senko";
pub const KEY_PROPER_RUNNING_STYLE_SASHI: &str = "proper_running_style_sashi";
pub const KEY_PROPER_RUNNING_STYLE_OIKOMI: &str = "proper_running_style_oikomi";
pub const KEY_PROPER_GROUND_TURF: &str = "proper_ground_turf";
pub const KEY_PROPER_GROUND_DIRT: &str = "proper_ground_dirt";
pub const KEY_RARITY: &str = "rarity";
pub const KEY_TALENT_LEVEL: &str = "talent_level";
pub const KEY_REGISTER_TIME: &str = "register_time";
pub const KEY_SKILL_COUNT: &str = "skill_count";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for UserTrainedCharaAtFriendModel {
    fn model_name() -> &'static str {
        "UserTrainedCharaAtFriend"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_VIEWER_ID,
                emit: true,
                required: true,
                candidates: &["viewer_id", "<ViewerId>k__BackingField", "ViewerId"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_TRAINED_CHARA_ID,
                emit: true,
                required: true,
                candidates: &["trained_chara_id", "<TrainedCharaId>k__BackingField", "TrainedCharaId"],
                reader: FieldReaderKind::I64,
            },
            FieldSpec {
                key: KEY_CARD_ID,
                emit: true,
                required: true,
                candidates: &["card_id", "<CardId>k__BackingField", "CardId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RANK_SCORE,
                emit: true,
                required: true,
                candidates: &["rank_score", "<RankScore>k__BackingField", "RankScore"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RANK,
                emit: true,
                required: true,
                candidates: &["rank", "<Rank>k__BackingField", "Rank"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_SHORT,
                emit: true,
                required: true,
                candidates: &["proper_distance_short", "<ProperDistanceShort>k__BackingField", "ProperDistanceShort"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_MILE,
                emit: true,
                required: true,
                candidates: &["proper_distance_mile", "<ProperDistanceMile>k__BackingField", "ProperDistanceMile"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_MIDDLE,
                emit: true,
                required: true,
                candidates: &["proper_distance_middle", "<ProperDistanceMiddle>k__BackingField", "ProperDistanceMiddle"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_DISTANCE_LONG,
                emit: true,
                required: true,
                candidates: &["proper_distance_long", "<ProperDistanceLong>k__BackingField", "ProperDistanceLong"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_NIGE,
                emit: true,
                required: true,
                candidates: &["proper_running_style_nige", "<ProperRunningStyleNige>k__BackingField", "ProperRunningStyleNige"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_SENKO,
                emit: true,
                required: true,
                candidates: &["proper_running_style_senko", "<ProperRunningStyleSenko>k__BackingField", "ProperRunningStyleSenko"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_SASHI,
                emit: true,
                required: true,
                candidates: &["proper_running_style_sashi", "<ProperRunningStyleSashi>k__BackingField", "ProperRunningStyleSashi"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_RUNNING_STYLE_OIKOMI,
                emit: true,
                required: true,
                candidates: &["proper_running_style_oikomi", "<ProperRunningStyleOikomi>k__BackingField", "ProperRunningStyleOikomi"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_GROUND_TURF,
                emit: true,
                required: true,
                candidates: &["proper_ground_turf", "<ProperGroundTurf>k__BackingField", "ProperGroundTurf"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_PROPER_GROUND_DIRT,
                emit: true,
                required: true,
                candidates: &["proper_ground_dirt", "<ProperGroundDirt>k__BackingField", "ProperGroundDirt"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_RARITY,
                emit: true,
                required: true,
                candidates: &["rarity", "<Rarity>k__BackingField", "Rarity"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_TALENT_LEVEL,
                emit: true,
                required: true,
                candidates: &["talent_level", "<TalentLevel>k__BackingField", "TalentLevel"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: KEY_REGISTER_TIME,
                emit: true,
                required: false,
                candidates: &["register_time", "<RegisterTime>k__BackingField", "RegisterTime"],
                reader: FieldReaderKind::ManagedString,
            },
            FieldSpec {
                key: KEY_SKILL_COUNT,
                emit: true,
                required: false,
                candidates: &["skill_count", "<SkillCount>k__BackingField", "SkillCount"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
