use crate::introspection::RuntimeIntrospector;
use anyhow::{anyhow, Result};
use chrono::{DateTime, Utc};
use rmpv::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub type RuntimeValue = Value;

/// Function pointer type for decoding a single IL2CPP object.
/// Used by FieldReaderKind for sub-model dispatch without string indirection.
pub type ModelDecoder = fn(&mut RuntimeIntrospector, u64) -> Result<RuntimeValue>;

/// Primitive field reader operations used by `FieldSpec`.
#[derive(Debug, Clone, Copy)]
pub enum FieldReaderKind {
    Pointer(ModelDecoder),
    I8,
    I32,
    I64,
    F32,
    Bool,
    I32AsI64,
    ObscuredInt,
    ObscuredIntAsI64,
    ObscuredLongAsI64,
    ObscuredBoolAsI64,
    ObscuredString,
    ManagedString,
    ObscuredIntArray,
    Int32Array,
    PointerArray(ModelDecoder),
    PointerList(ModelDecoder),
    Int32List,
    TypedDictionary(ModelDecoder),
    TypedDictionaryInline {
        entry_size: u64,
        value_offset: u64,
        key_offset: Option<u64>,
        decoder: ModelDecoder,
    },
    ConstantEmptyArray,
    ConstantI64(i64),
    ConstantString(&'static str),
    Alias {
        source_key: &'static str,
    },
    TimestampFrom {
        string_source_key: &'static str,
        obscured_unix_source_key: &'static str,
    },
    FactorIdsFrom {
        source_key: &'static str,
    },
}

impl FieldReaderKind {
    pub fn requires_address(&self) -> bool {
        !matches!(
            self,
            FieldReaderKind::ConstantEmptyArray
                | FieldReaderKind::ConstantI64(_)
                | FieldReaderKind::ConstantString(_)
                | FieldReaderKind::Alias { .. }
                | FieldReaderKind::TimestampFrom { .. }
                | FieldReaderKind::FactorIdsFrom { .. }
        )
    }
}

/// Declarative definition of one model field.
#[derive(Debug, Clone, Copy)]
pub struct FieldSpec {
    pub key: &'static str,
    pub emit: bool,
    pub required: bool,
    pub candidates: &'static [&'static str],
    pub reader: FieldReaderKind,
}

/// Resolved field offsets for one runtime class pointer.
#[derive(Debug, Default)]
pub struct ResolvedOffsets {
    map: HashMap<&'static str, u64>,
}

impl ResolvedOffsets {
    pub fn insert(&mut self, key: &'static str, offset: u64) {
        self.map.insert(key, offset);
    }

    pub fn get(&self, key: &'static str) -> Result<u64> {
        self.map
            .get(key)
            .copied()
            .ok_or_else(|| anyhow!("Missing resolved offset for key '{}'", key))
    }

    pub fn contains_key(&self, key: &'static str) -> bool {
        self.map.contains_key(key)
    }
}

/// Per-model cache of resolved offsets keyed by runtime class pointer.
#[derive(Debug, Default)]
pub struct ModelOffsetCache {
    by_class: RwLock<HashMap<u64, Arc<ResolvedOffsets>>>,
}

impl ModelOffsetCache {
    pub fn get(&self, class_ptr: u64) -> Option<Arc<ResolvedOffsets>> {
        self.by_class.read().ok()?.get(&class_ptr).cloned()
    }

    pub fn put(&self, class_ptr: u64, offsets: Arc<ResolvedOffsets>) {
        if let Ok(mut guard) = self.by_class.write() {
            guard.insert(class_ptr, offsets);
        }
    }
}

/// Base trait for declarative runtime model definitions.
pub trait RuntimeModelSpec {
    fn model_name() -> &'static str;
    fn fields() -> &'static [FieldSpec];
    fn cache() -> &'static ModelOffsetCache;

    fn field_spec(key: &str) -> Result<&'static FieldSpec> {
        Self::fields()
            .iter()
            .find(|f| f.key == key)
            .ok_or_else(|| anyhow!("{}: unknown field key '{}'", Self::model_name(), key))
    }

    fn resolved_offsets_for_object(
        ctx: &mut RuntimeIntrospector,
        obj_ptr: u64,
    ) -> Result<Arc<ResolvedOffsets>> {
        let class_ptr = ctx.read_pointer_at(obj_ptr)?;
        Self::resolve_for_class(ctx, obj_ptr, class_ptr)
    }

    fn resolve_for_class(
        ctx: &mut RuntimeIntrospector,
        obj_ptr: u64,
        class_ptr: u64,
    ) -> Result<Arc<ResolvedOffsets>> {
        if let Some(cached) = Self::cache().get(class_ptr) {
            return Ok(cached);
        }

        let mut resolved = ResolvedOffsets::default();
        for field in Self::fields() {
            if field.candidates.is_empty() {
                continue;
            }
            match ctx.resolve_runtime_offset_for_object(obj_ptr, field.candidates) {
                Ok(offset) => resolved.insert(field.key, offset),
                Err(_) if !field.required => {}
                Err(e) => {
                    return Err(anyhow!(
                        "{}: required field '{}' failed to resolve from {:?} on object {:#x}: {}",
                        Self::model_name(),
                        field.key,
                        field.candidates,
                        obj_ptr,
                        e
                    ));
                }
            }
        }

        let resolved = Arc::new(resolved);
        Self::cache().put(class_ptr, resolved.clone());
        Ok(resolved)
    }

    fn read_model_value(ctx: &mut RuntimeIntrospector, obj_ptr: u64) -> Result<RuntimeValue>
    where
        Self: Sized,
    {
        let offsets = Self::resolved_offsets_for_object(ctx, obj_ptr)?;

        let mut all_values = HashMap::<String, RuntimeValue>::new();
        let mut emitted_values = Vec::<(RuntimeValue, RuntimeValue)>::new();

        for field in Self::fields() {
            let offset_missing = !offsets.contains_key(field.key);
            if offset_missing && field.reader.requires_address() {
                continue;
            }
            let value = read_field_value(ctx, obj_ptr, &offsets, &all_values, field)?;
            all_values.insert(field.key.to_string(), value.clone());
            if field.emit {
                emitted_values.push((Value::from(field.key), value));
            }
        }

        Ok(Value::Map(emitted_values))
    }
}

fn read_field_value(
    ctx: &mut RuntimeIntrospector,
    obj_ptr: u64,
    offsets: &ResolvedOffsets,
    all_values: &HashMap<String, RuntimeValue>,
    field: &FieldSpec,
) -> Result<RuntimeValue> {
    let addr = |key: &'static str| -> Result<u64> { Ok(obj_ptr + offsets.get(key)?) };

    match field.reader {
        FieldReaderKind::Pointer(decoder) => {
            let ptr = ctx.read_pointer_at(addr(field.key)?)?;
            if ptr == 0 {
                Ok(Value::Nil)
            } else {
                decoder(ctx, ptr)
            }
        }
        FieldReaderKind::I8 => Ok(Value::from(ctx.read_i8_at(addr(field.key)?)?)),
        FieldReaderKind::I32 => Ok(Value::from(ctx.read_i32_at(addr(field.key)?)?)),
        FieldReaderKind::I64 => Ok(Value::from(ctx.read_i64_at(addr(field.key)?)?)),
        FieldReaderKind::F32 => Ok(Value::from(ctx.read_f32_at(addr(field.key)?)?)),
        FieldReaderKind::Bool => Ok(Value::from(ctx.read_i32_at(addr(field.key)?)? != 0)),
        FieldReaderKind::I32AsI64 => Ok(Value::from(ctx.read_i32_at(addr(field.key)?)? as i64)),
        FieldReaderKind::ObscuredInt => Ok(Value::from(ctx.decode_obscured_int(addr(field.key)?)?)),
        FieldReaderKind::ObscuredIntAsI64 => Ok(Value::from(
            ctx.decode_obscured_int(addr(field.key)?)? as i64,
        )),
        FieldReaderKind::ObscuredLongAsI64 => {
            Ok(Value::from(ctx.decode_obscured_long(addr(field.key)?)?))
        }
        FieldReaderKind::ObscuredBoolAsI64 => Ok(Value::from(
            ctx.decode_obscured_bool(addr(field.key)?)? as i64,
        )),
        FieldReaderKind::ObscuredString => {
            Ok(Value::from(ctx.read_obscured_string_ptr(addr(field.key)?)?))
        }
        FieldReaderKind::ManagedString => {
            Ok(Value::from(ctx.read_managed_string_ptr(addr(field.key)?)?))
        }
        FieldReaderKind::ObscuredIntArray => {
            let values = ctx
                .read_obscured_int_array(addr(field.key)?)?
                .into_iter()
                .map(Value::from)
                .collect::<Vec<_>>();
            Ok(Value::Array(values))
        }
        FieldReaderKind::Int32Array => {
            let values = ctx
                .read_int32_array(addr(field.key)?)?
                .into_iter()
                .map(Value::from)
                .collect::<Vec<_>>();
            Ok(Value::Array(values))
        }
        FieldReaderKind::PointerArray(decoder) => {
            let ptrs = ctx.read_pointer_array(addr(field.key)?)?;
            let values: Vec<Value> = ptrs
                .into_iter()
                .filter_map(|ptr| {
                    if ptr == 0 {
                        None
                    } else {
                        decoder(ctx, ptr).ok()
                    }
                })
                .collect();
            Ok(Value::Array(values))
        }
        FieldReaderKind::PointerList(decoder) => {
            let ptrs = ctx.read_pointer_list(addr(field.key)?)?;
            let values: Vec<Value> = ptrs
                .into_iter()
                .filter_map(|ptr| {
                    if ptr == 0 {
                        None
                    } else {
                        decoder(ctx, ptr).ok()
                    }
                })
                .collect();
            Ok(Value::Array(values))
        }
        FieldReaderKind::Int32List => {
            let values = ctx
                .read_int32_list(addr(field.key)?)?
                .into_iter()
                .map(|v| Value::from(v as i64))
                .collect::<Vec<_>>();
            Ok(Value::Array(values))
        }
        FieldReaderKind::TypedDictionary(decoder) => {
            let dict_ptr = ctx.read_pointer_at(addr(field.key)?)?;
            if dict_ptr == 0 {
                return Ok(Value::Array(Vec::new()));
            }
            let ptrs = ctx.iter_dictionary_value_ptrs(dict_ptr)?;
            let values: Vec<Value> = ptrs
                .into_iter()
                .filter_map(|ptr| decoder(ctx, ptr).ok())
                .collect();
            Ok(Value::Array(values))
        }
        FieldReaderKind::TypedDictionaryInline {
            entry_size,
            value_offset,
            key_offset,
            decoder,
        } => {
            let dict_ptr = ctx.read_pointer_at(addr(field.key)?)?;
            if dict_ptr == 0 {
                return Ok(Value::Array(Vec::new()));
            }
            let values: Vec<Value> = if let Some(ko) = key_offset {
                let entries =
                    ctx.iter_dictionary_entries(dict_ptr, entry_size, ko, value_offset)?;
                entries
                    .into_iter()
                    .filter_map(|(key, addr)| {
                        decoder(ctx, addr).ok().map(|val| {
                            let mut map = Vec::with_capacity(2);
                            map.push((Value::from("_key"), Value::from(key)));
                            if let Value::Map(inner) = val {
                                map.extend(inner);
                            } else {
                                map.push((Value::from("_value"), val));
                            }
                            Value::Map(map)
                        })
                    })
                    .collect()
            } else {
                let addrs = ctx.iter_dictionary_value_addrs(dict_ptr, entry_size, value_offset)?;
                addrs
                    .into_iter()
                    .filter_map(|addr| decoder(ctx, addr).ok())
                    .collect()
            };
            Ok(Value::Array(values))
        }
        FieldReaderKind::ConstantEmptyArray => Ok(Value::Array(Vec::new())),
        FieldReaderKind::ConstantI64(v) => Ok(Value::from(v)),
        FieldReaderKind::ConstantString(v) => Ok(Value::from(v)),
        FieldReaderKind::Alias { source_key } => all_values
            .get(source_key)
            .cloned()
            .ok_or_else(|| anyhow!("Missing aliased source value '{}'", source_key)),
        FieldReaderKind::TimestampFrom {
            string_source_key,
            obscured_unix_source_key,
        } => {
            let string_value = all_values
                .get(string_source_key)
                .and_then(|v| v.as_str())
                .unwrap_or_default()
                .to_string();
            if !string_value.is_empty() {
                if let Some(normalized) = normalize_datetime_string(&string_value) {
                    return Ok(Value::from(normalized));
                }
            }
            let unix_ts = all_values
                .get(obscured_unix_source_key)
                .and_then(|v| v.as_i64())
                .unwrap_or(0);
            if let Some(normalized) = normalize_unix_timestamp(unix_ts) {
                return Ok(Value::from(normalized));
            }
            Ok(Value::from(String::new()))
        }
        FieldReaderKind::FactorIdsFrom { source_key } => {
            let ids = all_values
                .get(source_key)
                .and_then(|v| v.as_array())
                .map(|items| {
                    items
                        .iter()
                        .filter_map(|item| {
                            value_get_key(item, "factor_id").and_then(|v| v.as_i64())
                        })
                        .collect::<Vec<i64>>()
                })
                .unwrap_or_default();
            Ok(Value::Array(ids.into_iter().map(Value::from).collect()))
        }
    }
}

fn value_get_key<'a>(value: &'a RuntimeValue, key: &str) -> Option<&'a RuntimeValue> {
    let entries = value.as_map()?;
    entries
        .iter()
        .find_map(|(k, v)| (k.as_str() == Some(key)).then_some(v))
}

fn normalize_unix_timestamp(unix_ts: i64) -> Option<String> {
    if !(946_684_800..=4_102_444_800).contains(&unix_ts) {
        return None;
    }
    DateTime::<Utc>::from_timestamp(unix_ts, 0)
        .map(|d| d.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string())
}

fn normalize_datetime_string(raw: &str) -> Option<String> {
    if raw.trim().is_empty() {
        return None;
    }
    if let Ok(dt) = chrono::NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S") {
        return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
    }
    if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(raw) {
        return Some(dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string());
    }
    None
}
