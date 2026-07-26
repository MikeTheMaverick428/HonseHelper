use il2cpp_runtime::{FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeModelSpec};
use std::sync::LazyLock;

pub struct RaceHorseSkillModel;

static CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for RaceHorseSkillModel {
    fn model_name() -> &'static str {
        "RaceHorseSkill"
    }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: "skill_id",
                emit: true,
                required: true,
                candidates: &["skill_id", "SkillId", "_skillId"],
                reader: FieldReaderKind::I32AsI64,
            },
            FieldSpec {
                key: "level",
                emit: true,
                required: true,
                candidates: &["level", "Level", "_level"],
                reader: FieldReaderKind::I32AsI64,
            },
        ]
    }

    fn cache() -> &'static ModelOffsetCache {
        &CACHE
    }
}
