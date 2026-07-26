use il2cpp_runtime::{
    FieldReaderKind, FieldSpec, ModelOffsetCache, RuntimeIntrospector, RuntimeModelSpec,
    RuntimeValue,
};
use rmpv::Value;
use std::sync::LazyLock;

pub struct PieceModel;

pub const KEY_SHARD_COUNT: &str = "shard_count";

static PIECE_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for PieceModel {
    fn model_name() -> &'static str {
        "Piece"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_SHARD_COUNT,
            emit: true,
            required: true,
            candidates: &["_num"],
            reader: FieldReaderKind::ObscuredInt,
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &PIECE_CACHE
    }
}

fn read_piece_entry(
    ctx: &mut RuntimeIntrospector,
    value_addr: u64,
) -> anyhow::Result<RuntimeValue> {
    let piece_ptr = ctx.read_pointer_at(value_addr)?;
    if piece_ptr == 0 {
        Ok(Value::Nil)
    } else {
        PieceModel::read_model_value(ctx, piece_ptr)
    }
}

pub struct WorkPieceDataModel;

pub const KEY_PIECES: &str = "pieces";

static WORK_PIECE_DATA_CACHE: LazyLock<ModelOffsetCache> = LazyLock::new(ModelOffsetCache::default);

impl RuntimeModelSpec for WorkPieceDataModel {
    fn model_name() -> &'static str {
        "WorkPieceData"
    }

    fn fields() -> &'static [FieldSpec] {
        &[FieldSpec {
            key: KEY_PIECES,
            emit: true,
            required: true,
            candidates: &["_pieceDic", "pieceDic", "PieceDic"],
            reader: FieldReaderKind::TypedDictionaryInline {
                entry_size: 40,
                value_offset: 32,
                key_offset: Some(0),
                decoder: read_piece_entry,
            },
        }]
    }

    fn cache() -> &'static ModelOffsetCache {
        &WORK_PIECE_DATA_CACHE
    }
}
