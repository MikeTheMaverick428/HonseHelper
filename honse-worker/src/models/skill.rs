use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct SkillModel;

pub const KEY_SKILL_ID: &str = "skill_id";
pub const KEY_LEVEL: &str = "level";

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for SkillModel {
    fn model_name() -> &'static str {
        "Skill"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: KEY_SKILL_ID,
                emit: true,
                required: true,
                candidates: &[
                    "_masterId",
                    "masterId",
                    "MasterId",
                    "skill_id",
                    "SkillId",
                    "_skillId",
                ],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: KEY_LEVEL,
                emit: true,
                required: true,
                candidates: &["_level", "level", "Level"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
