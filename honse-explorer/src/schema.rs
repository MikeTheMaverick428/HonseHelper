use anyhow::{Result, anyhow};
use chrono::{DateTime, NaiveDateTime, Utc};
use il2cpp_runtime::{Il2CppMetadata, RuntimeField, RuntimeIntrospector, SingletonCandidate};
use serde::Serialize;
use std::{
    collections::{BTreeSet, HashSet, VecDeque},
    fs,
    io::{self, Write},
};

// ---------------------------------------------------------------------------
// Constants (IL2CPP runtime layout offsets)
// ---------------------------------------------------------------------------

const IL2CPP_OBJECT_HEADER_SIZE: u64 = 16;
const ARRAY_ITEMS_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 16;
const LIST_ITEMS_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE;
const _LIST_SIZE_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 8;
const DICT_ENTRIES_PTR_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 8;
const SUPPORT_CARD_ENTRY_SIZE: u64 = 24;
const SUPPORT_CARD_ENTRY_HASH_OFFSET: u64 = 0;
const SUPPORT_CARD_ENTRY_VALUE_PTR_OFFSET: u64 = 16;

// ---------------------------------------------------------------------------
// Output types (JSON-serializable)
// ---------------------------------------------------------------------------

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeSchemaField {
    pub name: String,
    pub offset: i32,
    pub offset_hex: String,
    pub class_name: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct RuntimeSchemaDump {
    pub object_class: String,
    pub object_address: String,
    pub follow_path: Vec<String>,
    pub total_fields: usize,
    pub fields: Vec<RuntimeSchemaField>,
    pub missing_fields: Vec<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct PeekedField {
    pub field: String,
    pub decoder: String,
    pub value: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct BatchPeekOutput {
    pub object_class: String,
    pub object_address: String,
    pub follow_path: Vec<String>,
    pub total_peeks: usize,
    pub success_count: usize,
    pub error_count: usize,
    pub peeks: Vec<PeekedField>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AllTypesSchemaField {
    pub name: String,
    pub token: String,
    pub type_index: i32,
    pub type_name: Option<String>,
    pub runtime_offset: Option<i32>,
    pub runtime_offset_hex: Option<String>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AllTypesSchemaType {
    pub name: String,
    pub namespace: String,
    pub full_name: String,
    pub image_name: Option<String>,
    pub declaring_type: Option<String>,
    pub parent_type: Option<String>,
    pub byval_type_index: i32,
    pub flags: String,
    pub bitfield: String,
    pub token: String,
    pub is_value_type: bool,
    pub is_enum: bool,
    pub field_start: i32,
    pub field_count: usize,
    pub fields: Vec<AllTypesSchemaField>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AllTypesSchemaDump {
    pub metadata_path: String,
    pub metadata_version: i32,
    pub total_types: usize,
    pub total_fields: usize,
    pub types_with_fields: usize,
    pub types: Vec<AllTypesSchemaType>,
}

// ---------------------------------------------------------------------------
// Private helper structs
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
struct CollectionContext {
    collection_ptr: u64,
    collection_type: String,
    current_index: usize,
    total_count: usize,
    item_depth: usize,
}

#[derive(Clone, Debug)]
struct FieldSearchHit {
    object_ptr: u64,
    object_class: String,
    object_path: Vec<String>,
    field: RuntimeField,
}

#[derive(Clone, Debug)]
struct ClassSearchHit {
    root_index: usize,
    root_name: String,
    object_ptr: u64,
    object_class: String,
    object_path: Vec<String>,
}

// ---------------------------------------------------------------------------
// SchemaEngine
// ---------------------------------------------------------------------------

pub struct SchemaEngine {
    pub introspector: RuntimeIntrospector,
}

impl SchemaEngine {
    pub fn new(introspector: RuntimeIntrospector) -> Self {
        Self { introspector }
    }

    // -----------------------------------------------------------------------
    // Adapter methods (il2cpp-extractor self.xxx → RuntimeIntrospector calls)
    // -----------------------------------------------------------------------

    fn runtime_field_map_for_object(&mut self, obj_ptr: u64) -> Result<Vec<RuntimeField>> {
        let class_ptr = self.introspector.process_memory().read_pointer(obj_ptr)?;
        if class_ptr == 0 {
            return Err(anyhow!("Object at {:#x} has null klass", obj_ptr));
        }
        self.introspector.runtime_field_map_for_class(class_ptr)
    }

    fn describe_object(&mut self, ptr: u64) -> Result<String> {
        il2cpp_runtime::readers::describe_object(&mut self.introspector, ptr)
    }

    fn read_string_from_pointer(&mut self, ptr: u64) -> Result<String> {
        il2cpp_runtime::readers::read_string_from_pointer(&mut self.introspector, ptr)
    }

    fn read_array_length(&mut self, array_ptr: u64) -> Result<u64> {
        il2cpp_runtime::readers::read_array_length(&mut self.introspector, array_ptr)
    }

    // -----------------------------------------------------------------------
    // Public API: dump_all_types_schema
    // -----------------------------------------------------------------------

    pub fn dump_all_types_schema(&mut self) -> Result<AllTypesSchemaDump> {
        let pid = self.introspector.pid();

        println!("Finding Il2Cpp metadata...");
        let il2cpp_metadata =
            Il2CppMetadata::find_in_process(self.introspector.process_memory(), pid)?;
        println!("  Found at {:#x}\n", il2cpp_metadata.registration_addr);

        let metadata_path = il2cpp_metadata.metadata_path.as_ref().ok_or_else(|| {
            anyhow!("Metadata path unavailable (global-metadata.dat not found earlier)")
        })?;
        println!("Parsing {}...", metadata_path.display());

        let parsed = il2cpp_runtime::parse_full_metadata(metadata_path)?;
        let byval_name_map: std::collections::HashMap<i32, String> = parsed
            .type_defs
            .iter()
            .enumerate()
            .filter_map(|(idx, type_def)| {
                if type_def.byval_type_index < 0 {
                    return None;
                }
                parsed
                    .type_full_name(idx)
                    .map(|full_name| (type_def.byval_type_index, full_name))
            })
            .collect();

        let mut types = Vec::with_capacity(parsed.type_defs.len());
        let mut types_with_fields = 0usize;

        for (type_index, type_def) in parsed.type_defs.iter().enumerate() {
            let class_name = parsed
                .resolve_string(type_def.name_index)
                .unwrap_or("<unknown>")
                .to_string();
            let namespace = parsed
                .resolve_string(type_def.namespace_index)
                .unwrap_or_default()
                .to_string();
            let full_name = parsed
                .type_full_name(type_index)
                .unwrap_or_else(|| class_name.clone());
            let declaring_type = if type_def.declaring_type_index >= 0 {
                parsed.type_full_name(type_def.declaring_type_index as usize)
            } else {
                None
            };
            let parent_type = if type_def.parent_index >= 0 {
                parsed.type_full_name(type_def.parent_index as usize)
            } else {
                None
            };
            let image_name = parsed.find_image_for_type(type_index).and_then(|image| {
                parsed
                    .resolve_string_u32(image.name_index)
                    .map(|name| name.to_string())
            });

            let field_start = type_def.field_start.max(0) as usize;
            let field_count = type_def.field_count as usize;
            let runtime_offsets = il2cpp_metadata
                .get_field_offsets_for_type(
                    self.introspector.process_memory(),
                    type_index,
                    field_start,
                    field_count,
                )
                .unwrap_or_else(|_| vec![None; field_count]);

            let mut fields = Vec::with_capacity(field_count);
            for field_idx_in_type in 0..field_count {
                let global_field_index = field_start + field_idx_in_type;
                let Some(field_def) = parsed.field_defs.get(global_field_index) else {
                    break;
                };

                let field_name = parsed
                    .resolve_string_u32(field_def.name_index)
                    .unwrap_or("<unknown>")
                    .to_string();
                let runtime_offset = runtime_offsets.get(field_idx_in_type).cloned().flatten();
                let runtime_offset_hex = runtime_offset.map(|offset| format!("{:#x}", offset));
                let type_name = byval_name_map.get(&field_def.type_index).cloned();

                fields.push(AllTypesSchemaField {
                    name: field_name,
                    token: format!("{:#x}", field_def.token),
                    type_index: field_def.type_index,
                    type_name,
                    runtime_offset,
                    runtime_offset_hex,
                });
            }

            if !fields.is_empty() {
                types_with_fields += 1;
            }

            types.push(AllTypesSchemaType {
                name: class_name,
                namespace,
                full_name,
                image_name,
                declaring_type,
                parent_type,
                byval_type_index: type_def.byval_type_index,
                flags: format!("{:#x}", type_def.flags),
                bitfield: format!("{:#x}", type_def.bitfield),
                token: format!("{:#x}", type_def.token),
                is_value_type: type_def.is_value_type(),
                is_enum: type_def.is_enum(),
                field_start: type_def.field_start,
                field_count: fields.len(),
                fields,
            });
        }

        Ok(AllTypesSchemaDump {
            metadata_path: metadata_path.display().to_string(),
            metadata_version: parsed.header.version,
            total_types: parsed.type_defs.len(),
            total_fields: parsed.field_defs.len(),
            types_with_fields,
            types,
        })
    }

    // -----------------------------------------------------------------------
    // Public API: build_schema_dump_for_object
    // -----------------------------------------------------------------------

    pub fn build_schema_dump_for_object(
        &mut self,
        object_ptr: u64,
        resolved_follow_path: Vec<String>,
        requested_fields: &[String],
    ) -> Result<RuntimeSchemaDump> {
        let object_class = self.describe_object(object_ptr)?;
        let mut all_fields = self.runtime_field_map_for_object(object_ptr)?;
        all_fields.sort_by_key(|field| field.offset);

        let mut missing_fields = Vec::new();
        let filtered_fields = if requested_fields.is_empty() {
            all_fields.clone()
        } else {
            let mut matched = Vec::new();
            let mut seen = BTreeSet::new();
            for requested in requested_fields {
                let mut found = false;
                for field in &all_fields {
                    if RuntimeIntrospector::field_name_matches(&field.name, requested) {
                        found = true;
                        if seen.insert(field.name.clone()) {
                            matched.push(field.clone());
                        }
                    }
                }
                if !found {
                    missing_fields.push(requested.clone());
                }
            }
            matched.sort_by_key(|field| field.offset);
            matched
        };

        Ok(RuntimeSchemaDump {
            object_class,
            object_address: format!("{:#x}", object_ptr),
            follow_path: resolved_follow_path,
            total_fields: all_fields.len(),
            fields: filtered_fields
                .into_iter()
                .map(|field| RuntimeSchemaField {
                    offset_hex: format!("{:#x}", field.offset),
                    class_name: self.try_resolve_field_class_name(object_ptr, &field),
                    name: field.name,
                    offset: field.offset,
                })
                .collect(),
            missing_fields,
        })
    }

    fn try_resolve_field_class_name(
        &mut self,
        object_ptr: u64,
        field: &RuntimeField,
    ) -> Option<String> {
        if field.offset < 0 {
            return None;
        }

        let field_addr = object_ptr + field.offset as u64;
        let ptr = self
            .introspector
            .process_memory()
            .read_pointer(field_addr)
            .ok()?;
        if ptr == 0 {
            return None;
        }

        if self.runtime_field_map_for_object(ptr).is_err() {
            return None;
        }

        self.describe_object(ptr).ok()
    }

    // -----------------------------------------------------------------------
    // Public API: follow_step / resolve_follow_operator
    // -----------------------------------------------------------------------

    pub fn follow_step(&mut self, object_ptr: u64, step: &str) -> Result<(u64, String)> {
        let (op, index) = Self::parse_follow_step(step);
        if op.starts_with('@') {
            return self.resolve_follow_operator(object_ptr, op, index);
        }

        let fields = self.runtime_field_map_for_object(object_ptr)?;
        let matched = fields
            .iter()
            .find(|f| RuntimeIntrospector::field_name_matches(&f.name, step));
        match matched {
            Some(field) => {
                if field.offset < 0 {
                    return Err(anyhow!(
                        "Follow step '{}' resolved to negative offset {}",
                        field.name,
                        field.offset
                    ));
                }

                let child_ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(object_ptr + field.offset as u64)?;
                if child_ptr == 0 {
                    return Err(anyhow!(
                        "Follow step '{}' (field '{}' at offset {:#x}) points to null",
                        step,
                        field.name,
                        field.offset
                    ));
                }

                Ok((child_ptr, field.name.clone()))
            }
            None => Err(anyhow!(
                "Follow step '{}': no matching field found on object at {:#x}",
                step,
                object_ptr
            )),
        }
    }

    fn resolve_follow_operator(
        &mut self,
        object_ptr: u64,
        step: &str,
        index: Option<usize>,
    ) -> Result<(u64, String)> {
        let idx = index.unwrap_or(0);
        match step.to_ascii_lowercase().as_str() {
            "@dict-value" => {
                let values = self.iter_dictionary_value_ptrs(object_ptr)?;
                let ptr = values.get(idx).copied().ok_or_else(|| {
                    anyhow!(
                        "@dict-value: index {} out of range (0..{})",
                        idx,
                        values.len()
                    )
                })?;
                let resolved = if index.is_some() {
                    format!("@dict-value[{}]", idx)
                } else {
                    "@dict-value".to_string()
                };
                Ok((ptr, resolved))
            }
            "@list-item" => {
                let items = self.read_pointer_list_from_list_ptr(object_ptr)?;
                let ptr = items.get(idx).copied().ok_or_else(|| {
                    anyhow!(
                        "@list-item: index {} out of range (0..{})",
                        idx,
                        items.len()
                    )
                })?;
                let resolved = if index.is_some() {
                    format!("@list-item[{}]", idx)
                } else {
                    "@list-item".to_string()
                };
                Ok((ptr, resolved))
            }
            "@array-item" => {
                let items = self.read_pointer_array_from_array_ptr(object_ptr)?;
                let ptr = items.get(idx).copied().ok_or_else(|| {
                    anyhow!(
                        "@array-item: index {} out of range (0..{})",
                        idx,
                        items.len()
                    )
                })?;
                let resolved = if index.is_some() {
                    format!("@array-item[{}]", idx)
                } else {
                    "@array-item".to_string()
                };
                Ok((ptr, resolved))
            }
            _ => Err(anyhow!(
                "Unknown follow step '{}'. Supported: @dict-value, @list-item, @array-item",
                step
            )),
        }
    }

    // -----------------------------------------------------------------------
    // Public API: batch_peek / peek_single_field / read_with_decoder
    // -----------------------------------------------------------------------

    pub fn batch_peek(
        &mut self,
        object_ptr: u64,
        peeks: &[(String, Option<String>)],
        resolved_follow_path: &[String],
    ) -> Result<BatchPeekOutput> {
        let object_class = self.describe_object(object_ptr)?;
        let fields = self.runtime_field_map_for_object(object_ptr)?;

        let mut peek_results = Vec::new();
        let mut success_count = 0usize;
        let mut error_count = 0usize;

        println!("\n=== Peek Results ===");

        for (field_name, decoder) in peeks {
            let decoder_str = decoder.as_deref().unwrap_or("auto");

            let result = self.peek_single_field(&fields, object_ptr, field_name, decoder_str);
            match &result {
                Ok(pf) => {
                    success_count += 1;
                    println!("  {} [{}] → {}", pf.field, pf.decoder, pf.value);
                }
                Err(err) => {
                    error_count += 1;
                    println!("  {}: <error: {}>", field_name, err);
                }
            }
            peek_results.push(result.unwrap_or_else(|e| PeekedField {
                field: field_name.clone(),
                decoder: decoder_str.to_string(),
                value: serde_json::Value::Null,
                error: Some(e.to_string()),
            }));
        }

        println!("  ({} / {} succeeded)", success_count, peek_results.len());

        Ok(BatchPeekOutput {
            object_class,
            object_address: format!("{:#x}", object_ptr),
            follow_path: resolved_follow_path.to_vec(),
            total_peeks: peek_results.len(),
            success_count,
            error_count,
            peeks: peek_results,
        })
    }

    fn peek_single_field(
        &mut self,
        fields: &[RuntimeField],
        object_ptr: u64,
        field_name: &str,
        decoder: &str,
    ) -> Result<PeekedField> {
        let field = Self::resolve_field_selector(fields, field_name)
            .ok_or_else(|| anyhow!("No field matches '{}'", field_name))?;

        if field.offset < 0 {
            return Err(anyhow!(
                "Field '{}' has negative offset {}",
                field.name,
                field.offset
            ));
        }

        let field_addr = object_ptr + field.offset as u64;
        let decoder = decoder.to_ascii_lowercase();

        let value = self.read_with_decoder(field_addr, &decoder)?;

        Ok(PeekedField {
            field: field.name.clone(),
            decoder,
            value,
            error: None,
        })
    }

    fn read_with_decoder(&mut self, field_addr: u64, decoder: &str) -> Result<serde_json::Value> {
        match decoder {
            "raw" => {
                let bytes = self
                    .introspector
                    .read_bytes_at(field_addr, 16)
                    .unwrap_or_default();
                Ok(serde_json::Value::String(
                    bytes
                        .iter()
                        .map(|b| format!("{:02x}", b))
                        .collect::<Vec<_>>()
                        .join(" "),
                ))
            }
            "i32" => Ok(serde_json::json!(
                self.introspector.read_i32_at(field_addr)?
            )),
            "u32" => Ok(serde_json::json!(
                self.introspector.read_i32_at(field_addr)? as u32
            )),
            "i64" => Ok(serde_json::json!(
                self.introspector.read_i64_at(field_addr)?
            )),
            "u64" => Ok(serde_json::json!(
                self.introspector.read_i64_at(field_addr)? as u64
            )),
            "f32" => Ok(serde_json::json!(
                self.introspector.read_f32_at(field_addr)?
            )),
            "f64" => Ok(serde_json::json!(
                self.introspector.read_f64_at(field_addr)?
            )),
            "bool" => {
                let byte = self.introspector.read_bytes_at(field_addr, 1)?[0];
                Ok(serde_json::json!(byte != 0))
            }
            "ptr" => {
                let ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                let mut obj = serde_json::Map::new();
                obj.insert(
                    "address".to_string(),
                    serde_json::json!(format!("{:#x}", ptr)),
                );
                if ptr != 0 {
                    if let Ok(class_name) = self.describe_object(ptr) {
                        obj.insert("class".to_string(), serde_json::json!(class_name));
                    }
                }
                Ok(serde_json::Value::Object(obj))
            }
            "string" | "managed-string" => {
                let value = self.introspector.read_managed_string_ptr(field_addr)?;
                Ok(serde_json::json!(value))
            }
            "obscured-int" => {
                let value = self.introspector.decode_obscured_int(field_addr)?;
                Ok(serde_json::json!(value))
            }
            "obscured-long" => {
                let value = self.introspector.decode_obscured_long(field_addr)?;
                Ok(serde_json::json!(value))
            }
            "obscured-bool" => {
                let value = self.introspector.decode_obscured_bool(field_addr)?;
                Ok(serde_json::json!(value))
            }
            "obscured-string" => {
                let value = self.introspector.read_obscured_string_ptr(field_addr)?;
                Ok(serde_json::json!(value))
            }
            "datetime" | "date-time" | "timestamp" => {
                let mut results = serde_json::Map::new();
                if let Ok(s) = self.introspector.read_managed_string_ptr(field_addr) {
                    if !s.is_empty() {
                        results.insert("managed-string".to_string(), serde_json::json!(s));
                    }
                }
                if let Ok(v) = self.introspector.decode_obscured_int(field_addr) {
                    results.insert("obscured-int".to_string(), serde_json::json!(v));
                }
                if let Ok(v) = self.introspector.read_i64_at(field_addr) {
                    results.insert("raw-i64".to_string(), serde_json::json!(v));
                }
                Ok(serde_json::Value::Object(results))
            }
            "int32-array" | "i32-array" => {
                let array_ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                if array_ptr == 0 {
                    return Ok(serde_json::Value::Null);
                }
                let arr = self
                    .introspector
                    .read_int32_array_from_array_ptr(array_ptr)?;
                Ok(serde_json::json!(arr))
            }
            "int32-list" | "i32-list" => {
                let list_ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                if list_ptr == 0 {
                    return Ok(serde_json::Value::Null);
                }
                let arr = self.introspector.read_int32_list(list_ptr)?;
                Ok(serde_json::json!(arr))
            }
            "auto" => {
                let mut results = serde_json::Map::new();
                if let Ok(v) = self.introspector.read_i32_at(field_addr) {
                    results.insert("i32".to_string(), serde_json::json!(v));
                }
                if let Ok(v) = self.introspector.read_i64_at(field_addr) {
                    results.insert("i64".to_string(), serde_json::json!(v));
                }
                if let Ok(v) = self.introspector.read_managed_string_ptr(field_addr) {
                    if !v.is_empty() {
                        results.insert("managed-string".to_string(), serde_json::json!(v));
                    }
                }
                if let Ok(ptr) = self.introspector.process_memory().read_pointer(field_addr) {
                    results.insert("ptr".to_string(), serde_json::json!(format!("{:#x}", ptr)));
                    if ptr != 0 {
                        if let Ok(cls) = self.describe_object(ptr) {
                            results.insert("ptr-class".to_string(), serde_json::json!(cls));
                        }
                    }
                }
                Ok(serde_json::Value::Object(results))
            }
            _ => Err(anyhow!("Unknown decoder '{}'", decoder)),
        }
    }

    // -----------------------------------------------------------------------
    // Public API: interactive_schema
    // -----------------------------------------------------------------------

    pub fn interactive_schema(&mut self, initial_follow: &[String]) -> Result<()> {
        println!("Interactive schema explorer");
        println!("Type 'help' to list commands.\n");

        let pid = self.introspector.pid();
        println!("Finding Il2Cpp metadata...");
        let il2cpp_metadata =
            Il2CppMetadata::find_in_process(self.introspector.process_memory(), pid)?;
        println!("  Found at {:#x}\n", il2cpp_metadata.registration_addr);

        println!("Discovering Singleton`1<T> roots...");
        let mut singleton_candidates =
            il2cpp_metadata.discover_singleton_candidates(self.introspector.process_memory())?;

        println!("Discovering MonoSingleton`1<T> roots...");
        match il2cpp_metadata.discover_mono_singleton_candidates(self.introspector.process_memory())
        {
            Ok(mono_candidates) => {
                println!(
                    "  Found {} MonoSingleton`1<T> candidates",
                    mono_candidates.len()
                );
                for candidate in mono_candidates {
                    let key = candidate.full_name.to_ascii_lowercase();
                    if !singleton_candidates
                        .iter()
                        .any(|c| c.full_name.to_ascii_lowercase() == key)
                    {
                        singleton_candidates.push(candidate);
                    }
                }
            }
            Err(err) => {
                println!("  MonoSingleton`1<T> discovery skipped: {}", err);
            }
        }

        Self::discover_supplemental_roots_from_singletons(
            &mut self.introspector,
            &mut singleton_candidates,
        )?;

        let live_root_count = singleton_candidates
            .iter()
            .filter(|candidate| candidate.instance_ptr.is_some())
            .count();
        if live_root_count == 0 {
            return Err(anyhow!(
                "No live singleton instances found (all _instance pointers are null/unreadable)"
            ));
        }

        println!(
            "  Found {} root candidates ({} with live instances)",
            singleton_candidates.len(),
            live_root_count
        );

        let mut active_root_idx = singleton_candidates
            .iter()
            .position(|candidate| {
                candidate.instance_ptr.is_some()
                    && candidate.namespace == "Gallop"
                    && candidate.class_name == "WorkDataManager"
            })
            .or_else(|| {
                singleton_candidates
                    .iter()
                    .position(|candidate| candidate.instance_ptr.is_some())
            })
            .ok_or_else(|| anyhow!("No suitable singleton root found"))?;

        Self::print_singleton_roots(&singleton_candidates, active_root_idx);

        let mut root_ptr = singleton_candidates[active_root_idx]
            .instance_ptr
            .ok_or_else(|| anyhow!("Selected singleton root has null instance pointer"))?;

        println!(
            "\nUsing root singleton: {} @ {:#x}\n",
            singleton_candidates[active_root_idx].full_name, root_ptr
        );

        let mut stack: Vec<(u64, Option<String>)> = vec![(root_ptr, None)];
        let mut object_ptr = root_ptr;
        let mut show_fields = true;
        let mut collection_contexts: Vec<CollectionContext> = Vec::new();

        for step in initial_follow {
            let (child_ptr, resolved_step) = self.follow_step(object_ptr, step)?;
            println!("  -> Followed '{}' → {:#x}", resolved_step, child_ptr);
            stack.push((child_ptr, Some(resolved_step)));
            object_ptr = child_ptr;
            if step.starts_with('@') {
                let parent_ptr = stack
                    .get(stack.len().saturating_sub(2))
                    .map(|(ptr, _)| *ptr);
                if let Some(parent) = parent_ptr {
                    let item_depth = stack.len().saturating_sub(1);
                    if let Ok(ctx) =
                        self.build_collection_context(parent, step, child_ptr, item_depth)
                    {
                        Self::upsert_collection_context(&mut collection_contexts, ctx);
                    }
                }
            }
        }

        loop {
            let object_class = match self.describe_object(object_ptr) {
                Ok(name) => name,
                Err(err) => {
                    println!(
                        "\nCurrent object at {:#x} is unreadable ({})",
                        object_ptr, err
                    );
                    if stack.len() > 1 {
                        println!("Returning to previous object...");
                        stack.pop();
                        Self::prune_collection_contexts(
                            &mut collection_contexts,
                            stack.len().saturating_sub(1),
                        );
                        object_ptr = stack.last().map(|(ptr, _)| *ptr).unwrap_or(root_ptr);
                        continue;
                    }
                    println!(
                        "Root object at {:#x} is unreadable. Use 'use-root <index>' to switch roots or 'singletons' to list them.",
                        object_ptr
                    );
                    if Self::recovery_prompt_use_root(
                        &singleton_candidates,
                        &mut active_root_idx,
                        &mut root_ptr,
                        &mut stack,
                        &mut object_ptr,
                        &mut collection_contexts,
                        &mut show_fields,
                    )? {
                        continue;
                    }
                    return Ok(());
                }
            };

            let mut fields = match self.runtime_field_map_for_object(object_ptr) {
                Ok(fields) => fields,
                Err(err) => {
                    println!(
                        "\nFailed to read fields for object at {:#x}: {}",
                        object_ptr, err
                    );
                    if stack.len() > 1 {
                        println!("Returning to previous object...");
                        stack.pop();
                        Self::prune_collection_contexts(
                            &mut collection_contexts,
                            stack.len().saturating_sub(1),
                        );
                        object_ptr = stack.last().map(|(ptr, _)| *ptr).unwrap_or(root_ptr);
                        continue;
                    }
                    println!(
                        "Root object at {:#x} has unreadable fields. Use 'use-root <index>' to switch roots or 'singletons' to list them.",
                        object_ptr
                    );
                    if Self::recovery_prompt_use_root(
                        &singleton_candidates,
                        &mut active_root_idx,
                        &mut root_ptr,
                        &mut stack,
                        &mut object_ptr,
                        &mut collection_contexts,
                        &mut show_fields,
                    )? {
                        continue;
                    }
                    return Ok(());
                }
            };
            fields.sort_by_key(|field| field.offset);

            if show_fields {
                println!("\nObject: {} @ {:#x}", object_class, object_ptr);
                println!(
                    "Path: {}",
                    Self::format_path_with_collection_contexts(&stack, &collection_contexts, " → ")
                );
                println!("Fields ({})", fields.len());
                for (idx, field) in fields.iter().enumerate() {
                    let offset = if field.offset >= 0 {
                        format!("{:#06x}", field.offset)
                    } else {
                        field.offset.to_string()
                    };
                    println!("  [{:>3}] {:>8} {}", idx, offset, field.name);
                }
                show_fields = false;
            }

            print!("\nschema> ");
            io::stdout().flush()?;

            let mut line = String::new();
            if io::stdin().read_line(&mut line)? == 0 {
                println!();
                break;
            }

            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }

            let mut parts = trimmed.split_whitespace();
            let command = parts.next().unwrap_or_default().to_ascii_lowercase();

            match command.as_str() {
                "h" | "help" | "?" => {
                    println!(
                        "Commands:\n  help                     Show this help\n  fields | ls              Show fields for current object\n  follow <field|index|@op> Follow pointer-like field or special operator\n  open <field|index|@op>   Alias for follow\n  next | >                 Move to next item in current collection\n  prev | <                 Move to previous item in current collection\n  peek <field|index> [as] [decoder]  Show typed value preview\n  peek help                Show detailed peek decoders/help\n  find-field <name> [depth] [limit]  Search reachable objects for matching field names\n  find-class <name> [depth] [limit]  Search reachable objects by class name across roots\n  jump <addr>              Jump directly to object address (hex or decimal)\n  back                     Move to previous object\n  root                     Jump back to current root singleton\n  singletons | roots       List discovered singleton roots\n  use-root <index|type>    Switch root singleton and reset navigation\n  path | pwd               Print current follow path\n  save [file]              Save current object schema JSON (default: schema.json)\n  quit | exit              Leave interactive mode\n\nSpecial operators: @dict-value, @list-item, @array-item"
                    );
                    show_fields = false;
                }
                "fields" | "ls" => {
                    show_fields = true;
                }
                "follow" | "open" => {
                    let Some(selector) = parts.next() else {
                        println!("Missing selector. Usage: follow <field|index|@op>");
                        continue;
                    };

                    let step = if selector.starts_with('@') {
                        selector.to_string()
                    } else {
                        let Some(field) = Self::resolve_field_selector(&fields, selector) else {
                            println!("No field matches selector '{}'", selector);
                            continue;
                        };
                        field.name.clone()
                    };

                    match self.follow_step(object_ptr, &step) {
                        Ok((child_ptr, resolved_step)) => {
                            match self.runtime_field_map_for_object(child_ptr) {
                                Ok(_) => {
                                    println!(
                                        "  -> Followed '{}' → {:#x}",
                                        resolved_step, child_ptr
                                    );
                                    stack.push((child_ptr, Some(resolved_step.clone())));
                                    object_ptr = child_ptr;

                                    if step.starts_with('@') {
                                        let parent_ptr = stack
                                            .get(stack.len().saturating_sub(2))
                                            .map(|(ptr, _)| *ptr);
                                        if let Some(parent) = parent_ptr {
                                            let item_depth = stack.len().saturating_sub(1);
                                            if let Ok(ctx) = self.build_collection_context(
                                                parent, &step, child_ptr, item_depth,
                                            ) {
                                                Self::upsert_collection_context(
                                                    &mut collection_contexts,
                                                    ctx,
                                                );
                                            }
                                        }
                                    }

                                    Self::prune_collection_contexts(
                                        &mut collection_contexts,
                                        stack.len().saturating_sub(1),
                                    );

                                    show_fields = true;
                                }
                                Err(err) => {
                                    println!(
                                        "Follow target {:#x} is not a readable object: {}",
                                        child_ptr, err
                                    );
                                    println!(
                                        "Hint: this field may be a primitive value, not a pointer. Try 'peek {}'",
                                        selector
                                    );
                                    show_fields = false;
                                }
                            }
                        }
                        Err(err) => {
                            println!("Follow failed: {}", err);
                            show_fields = false;
                        }
                    }
                }
                "next" | ">" => {
                    let current_depth = stack.len().saturating_sub(1);
                    let active_idx = collection_contexts
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, ctx)| ctx.item_depth <= current_depth)
                        .map(|(idx, _)| idx);

                    if let Some(active_idx) = active_idx {
                        let mut ctx = collection_contexts[active_idx].clone();
                        if ctx.current_index + 1 >= ctx.total_count {
                            println!(
                                "Already at last item ({}/{})",
                                ctx.current_index + 1,
                                ctx.total_count
                            );
                            collection_contexts[active_idx] = ctx;
                        } else {
                            ctx.current_index += 1;
                            match self.get_collection_item_at_index(&ctx) {
                                Ok(next_ptr) => {
                                    println!(
                                        "  -> Next item {} → {:#x}",
                                        ctx.current_index + 1,
                                        next_ptr
                                    );
                                    object_ptr = next_ptr;
                                    if stack.len() > ctx.item_depth {
                                        stack.truncate(ctx.item_depth + 1);
                                        if let Some((ptr, _)) = stack.get_mut(ctx.item_depth) {
                                            *ptr = next_ptr;
                                        }
                                    }
                                    collection_contexts[active_idx] = ctx;
                                    Self::prune_collection_contexts(
                                        &mut collection_contexts,
                                        stack.len().saturating_sub(1),
                                    );
                                    show_fields = false;
                                }
                                Err(err) => {
                                    println!("Failed to read next item: {}", err);
                                    collection_contexts[active_idx] = ctx;
                                    show_fields = false;
                                }
                            }
                        }
                    } else {
                        println!(
                            "Not in a collection (use 'follow @list-item', '@dict-value', or '@array-item')"
                        );
                    }
                }
                "prev" | "<" => {
                    let current_depth = stack.len().saturating_sub(1);
                    let active_idx = collection_contexts
                        .iter()
                        .enumerate()
                        .rev()
                        .find(|(_, ctx)| ctx.item_depth <= current_depth)
                        .map(|(idx, _)| idx);

                    if let Some(active_idx) = active_idx {
                        let mut ctx = collection_contexts[active_idx].clone();
                        if ctx.current_index == 0 {
                            println!("Already at first item (1/{})", ctx.total_count);
                            collection_contexts[active_idx] = ctx;
                        } else {
                            ctx.current_index -= 1;
                            match self.get_collection_item_at_index(&ctx) {
                                Ok(prev_ptr) => {
                                    println!(
                                        "  -> Prev item {} → {:#x}",
                                        ctx.current_index + 1,
                                        prev_ptr
                                    );
                                    object_ptr = prev_ptr;
                                    if stack.len() > ctx.item_depth {
                                        stack.truncate(ctx.item_depth + 1);
                                        if let Some((ptr, _)) = stack.get_mut(ctx.item_depth) {
                                            *ptr = prev_ptr;
                                        }
                                    }
                                    collection_contexts[active_idx] = ctx;
                                    Self::prune_collection_contexts(
                                        &mut collection_contexts,
                                        stack.len().saturating_sub(1),
                                    );
                                    show_fields = false;
                                }
                                Err(err) => {
                                    println!("Failed to read previous item: {}", err);
                                    collection_contexts[active_idx] = ctx;
                                    show_fields = false;
                                }
                            }
                        }
                    } else {
                        println!(
                            "Not in a collection (use 'follow @list-item', '@dict-value', or '@array-item')"
                        );
                    }
                }
                "peek" => {
                    let Some(selector) = parts.next() else {
                        println!("Missing selector. Usage: peek <field|index> [as] [decoder]");
                        continue;
                    };

                    if matches!(selector, "help" | "h" | "?") {
                        Self::print_peek_help();
                        continue;
                    }

                    let mut decoder = parts.next();
                    if matches!(decoder, Some("as")) {
                        decoder = parts.next();
                    }

                    let Some(field) = Self::resolve_field_selector(&fields, selector) else {
                        println!("No field matches selector '{}'", selector);
                        continue;
                    };

                    if let Err(err) = self.print_field_preview(object_ptr, field, decoder) {
                        println!("Peek failed: {}", err);
                    }
                    show_fields = false;
                }
                "find-field" | "ff" => {
                    let Some(needle) = parts.next() else {
                        println!("Missing field name. Usage: find-field <name> [depth] [limit]");
                        show_fields = false;
                        continue;
                    };

                    let max_depth = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(4)
                        .min(10);
                    let max_results = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(20)
                        .min(100);

                    match self.find_reachable_fields(object_ptr, needle, max_depth, max_results) {
                        Ok(hits) => {
                            if hits.is_empty() {
                                println!(
                                    "No reachable fields matching '{}' (depth <= {}, limit {}).",
                                    needle, max_depth, max_results
                                );
                            } else {
                                println!(
                                    "Found {} match(es) for '{}' (depth <= {}):",
                                    hits.len(),
                                    needle,
                                    max_depth
                                );

                                for (idx, hit) in hits.iter().enumerate() {
                                    let object_path = if hit.object_path.is_empty() {
                                        "<current>".to_string()
                                    } else {
                                        hit.object_path.join(" -> ")
                                    };
                                    println!(
                                        "  [{:>2}] {} | {} @ {:#x} | field '{}'",
                                        idx,
                                        object_path,
                                        hit.object_class,
                                        hit.object_ptr,
                                        hit.field.name
                                    );

                                    if hit.field.offset >= 0 {
                                        let field_addr = hit.object_ptr + hit.field.offset as u64;
                                        if let Ok(value) =
                                            self.introspector.read_managed_string_ptr(field_addr)
                                        {
                                            if !value.is_empty() {
                                                println!("       value (string): {:?}", value);
                                                continue;
                                            }
                                        }
                                        if let Ok(ptr) = self
                                            .introspector
                                            .process_memory()
                                            .read_pointer(field_addr)
                                        {
                                            if ptr != 0 {
                                                if let Ok(class_name) = self.describe_object(ptr) {
                                                    println!(
                                                        "       value ptr: {:#x} ({})",
                                                        ptr, class_name
                                                    );
                                                } else {
                                                    println!("       value ptr: {:#x}", ptr);
                                                }
                                            }
                                        }
                                    }

                                    if hit.object_path.is_empty() {
                                        println!("       hint: peek {} string", hit.field.name);
                                    } else {
                                        let chain = hit
                                            .object_path
                                            .iter()
                                            .map(|step| format!("follow {}", step))
                                            .collect::<Vec<_>>()
                                            .join(" ; ");
                                        println!(
                                            "       hint: {} ; peek {} string",
                                            chain, hit.field.name
                                        );
                                    }
                                }
                            }
                        }
                        Err(err) => println!("find-field failed: {}", err),
                    }

                    show_fields = false;
                }
                "find-class" | "fc" => {
                    let Some(needle) = parts.next() else {
                        println!("Missing class query. Usage: find-class <name> [depth] [limit]");
                        show_fields = false;
                        continue;
                    };

                    let max_depth = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(5)
                        .min(12);
                    let max_results = parts
                        .next()
                        .and_then(|v| v.parse::<usize>().ok())
                        .unwrap_or(30)
                        .min(200);

                    match self.find_reachable_classes_from_roots(
                        &singleton_candidates,
                        needle,
                        max_depth,
                        max_results,
                    ) {
                        Ok(hits) => {
                            if hits.is_empty() {
                                println!(
                                    "No reachable classes matching '{}' (depth <= {}, limit {}).",
                                    needle, max_depth, max_results
                                );
                            } else {
                                println!(
                                    "Found {} class match(es) for '{}' (depth <= {}):",
                                    hits.len(),
                                    needle,
                                    max_depth
                                );
                                Self::print_class_hits(&hits, active_root_idx);
                            }
                        }
                        Err(err) => println!("find-class failed: {}", err),
                    }

                    show_fields = false;
                }
                "jump" | "j" => {
                    let Some(raw_addr) = parts.next() else {
                        println!("Missing address. Usage: jump <addr>");
                        show_fields = false;
                        continue;
                    };

                    let parsed_addr = if let Some(hex) = raw_addr
                        .strip_prefix("0x")
                        .or_else(|| raw_addr.strip_prefix("0X"))
                    {
                        u64::from_str_radix(hex, 16).ok()
                    } else {
                        raw_addr.parse::<u64>().ok()
                    };

                    let Some(addr) = parsed_addr else {
                        println!(
                            "Invalid address '{}'. Use hex (0x...) or decimal.",
                            raw_addr
                        );
                        show_fields = false;
                        continue;
                    };

                    match self.describe_object(addr) {
                        Ok(class_name) => {
                            if self.runtime_field_map_for_object(addr).is_err() {
                                println!(
                                    "Address {:#x} resolves to '{}' but object fields are unreadable.",
                                    addr, class_name
                                );
                                show_fields = false;
                                continue;
                            }

                            stack.truncate(1);
                            stack.push((addr, Some(format!("@jump({:#x})", addr))));
                            object_ptr = addr;
                            collection_contexts.clear();
                            println!("Jumped to {} @ {:#x}", class_name, addr);
                            show_fields = true;
                        }
                        Err(err) => {
                            println!("Jump failed at {:#x}: {}", addr, err);
                            show_fields = false;
                        }
                    }
                }
                "back" => {
                    if stack.len() <= 1 {
                        println!("Already at root");
                    } else {
                        stack.pop();
                        object_ptr = stack.last().map(|(ptr, _)| *ptr).unwrap_or(root_ptr);
                        Self::prune_collection_contexts(
                            &mut collection_contexts,
                            stack.len().saturating_sub(1),
                        );
                    }
                    show_fields = true;
                }
                "root" => {
                    stack.truncate(1);
                    object_ptr = root_ptr;
                    collection_contexts.clear();
                    show_fields = false;
                }
                "singletons" | "roots" => {
                    Self::print_singleton_roots(&singleton_candidates, active_root_idx);
                    show_fields = false;
                }
                "use-root" => {
                    let Some(selector) = parts.next() else {
                        println!("Missing selector. Usage: use-root <index|type>");
                        show_fields = false;
                        continue;
                    };

                    let Some(next_root_idx) =
                        Self::resolve_singleton_selector(&singleton_candidates, selector)
                    else {
                        println!("No singleton matches selector '{}'", selector);
                        show_fields = false;
                        continue;
                    };

                    let Some(next_root_ptr) = singleton_candidates[next_root_idx].instance_ptr
                    else {
                        println!(
                            "Singleton '{}' currently has null/unreadable _instance",
                            singleton_candidates[next_root_idx].full_name
                        );
                        show_fields = false;
                        continue;
                    };

                    active_root_idx = next_root_idx;
                    root_ptr = next_root_ptr;
                    stack.clear();
                    stack.push((root_ptr, None));
                    object_ptr = root_ptr;
                    collection_contexts.clear();

                    println!(
                        "Switched root to {} @ {:#x}",
                        singleton_candidates[active_root_idx].full_name, root_ptr
                    );
                    show_fields = true;
                }
                "path" | "pwd" => {
                    println!(
                        "{}",
                        Self::format_path_with_collection_contexts(
                            &stack,
                            &collection_contexts,
                            " -> "
                        )
                    );
                    show_fields = false;
                }
                "save" => {
                    let output = parts.next().unwrap_or("schema.json");
                    let schema = self.build_schema_dump_for_object(
                        object_ptr,
                        Self::stack_follow_path(&stack),
                        &[],
                    )?;
                    let json = serde_json::to_string_pretty(&schema)?;
                    fs::write(output, json)?;
                    println!("Saved {}", output);
                    show_fields = false;
                }
                "q" | "quit" | "exit" => break,
                _ => {
                    println!("Unknown command '{}'. Type 'help'.", command);
                    show_fields = false;
                }
            }
        }

        Ok(())
    }

    // -----------------------------------------------------------------------
    // BFS search: find_reachable_fields
    // -----------------------------------------------------------------------

    fn find_reachable_fields(
        &mut self,
        start_object_ptr: u64,
        needle: &str,
        max_depth: usize,
        max_results: usize,
    ) -> Result<Vec<FieldSearchHit>> {
        if needle.trim().is_empty() {
            return Err(anyhow!("Field name query cannot be empty"));
        }

        let normalized_query = RuntimeIntrospector::normalize_field_name(needle);
        let mut hits = Vec::new();
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();

        const MAX_NODES_TO_SCAN: usize = 3000;
        let mut scanned_nodes = 0usize;

        visited.insert(start_object_ptr);
        queue.push_back((start_object_ptr, 0usize, Vec::<String>::new()));

        while let Some((obj_ptr, depth, path)) = queue.pop_front() {
            if scanned_nodes >= MAX_NODES_TO_SCAN || hits.len() >= max_results {
                break;
            }
            scanned_nodes += 1;

            let object_class = self
                .describe_object(obj_ptr)
                .unwrap_or_else(|_| "<unresolved>".to_string());
            let fields = match self.introspector.runtime_fields_for_object_cached(obj_ptr) {
                Ok(f) => f,
                Err(_) => continue,
            };

            for field in &fields {
                let field_norm = RuntimeIntrospector::normalize_field_name(&field.name);
                if field_norm.contains(&normalized_query) {
                    hits.push(FieldSearchHit {
                        object_ptr: obj_ptr,
                        object_class: object_class.clone(),
                        object_path: path.clone(),
                        field: field.clone(),
                    });

                    if hits.len() >= max_results {
                        break;
                    }
                }
            }

            if depth >= max_depth {
                continue;
            }

            for field in &fields {
                if field.offset < 0 {
                    continue;
                }

                let child_ptr = match self
                    .introspector
                    .process_memory()
                    .read_pointer(obj_ptr + field.offset as u64)
                {
                    Ok(ptr) => ptr,
                    Err(_) => continue,
                };
                if child_ptr == 0 || !visited.insert(child_ptr) {
                    continue;
                }

                if self
                    .introspector
                    .runtime_fields_for_object_cached(child_ptr)
                    .is_err()
                {
                    continue;
                }

                let mut child_path = path.clone();
                child_path.push(field.name.clone());
                queue.push_back((child_ptr, depth + 1, child_path));
            }
        }

        Ok(hits)
    }

    // -----------------------------------------------------------------------
    // BFS search: find_reachable_classes_from_roots
    // -----------------------------------------------------------------------

    fn find_reachable_classes_from_roots(
        &mut self,
        candidates: &[SingletonCandidate],
        needle: &str,
        max_depth: usize,
        max_results: usize,
    ) -> Result<Vec<ClassSearchHit>> {
        if needle.trim().is_empty() {
            return Err(anyhow!("Class query cannot be empty"));
        }

        let query = needle.to_ascii_lowercase();
        let mut hits = Vec::new();
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();

        const MAX_NODES_TO_SCAN: usize = 20_000;
        let mut scanned_nodes = 0usize;

        for (root_index, candidate) in candidates.iter().enumerate() {
            let Some(root_ptr) = candidate.instance_ptr else {
                continue;
            };
            if visited.insert(root_ptr) {
                queue.push_back((
                    root_index,
                    candidate.full_name.clone(),
                    root_ptr,
                    0usize,
                    Vec::<String>::new(),
                ));
            }
        }

        while let Some((root_index, root_name, obj_ptr, depth, path)) = queue.pop_front() {
            if scanned_nodes >= MAX_NODES_TO_SCAN || hits.len() >= max_results {
                break;
            }
            scanned_nodes += 1;

            let class_name = match self.describe_object(obj_ptr) {
                Ok(name) => name,
                Err(_) => continue,
            };

            if class_name.to_ascii_lowercase().contains(&query) {
                hits.push(ClassSearchHit {
                    root_index,
                    root_name: root_name.clone(),
                    object_ptr: obj_ptr,
                    object_class: class_name.clone(),
                    object_path: path.clone(),
                });
                if hits.len() >= max_results {
                    break;
                }
            }

            if depth >= max_depth {
                continue;
            }

            let fields = match self.introspector.runtime_fields_for_object_cached(obj_ptr) {
                Ok(fields) => fields,
                Err(_) => continue,
            };

            for field in fields {
                if field.offset < 0 {
                    continue;
                }

                let child_ptr = match self
                    .introspector
                    .process_memory()
                    .read_pointer(obj_ptr + field.offset as u64)
                {
                    Ok(ptr) => ptr,
                    Err(_) => continue,
                };
                if child_ptr == 0 || !visited.insert(child_ptr) {
                    continue;
                }

                if self
                    .introspector
                    .runtime_fields_for_object_cached(child_ptr)
                    .is_err()
                {
                    continue;
                }

                let mut child_path = path.clone();
                child_path.push(field.name);

                queue.push_back((
                    root_index,
                    root_name.clone(),
                    child_ptr,
                    depth + 1,
                    child_path,
                ));
            }
        }

        Ok(hits)
    }

    // -----------------------------------------------------------------------
    // Supplemental root discovery
    // -----------------------------------------------------------------------

    fn discover_supplemental_roots_from_singletons(
        introspector: &mut RuntimeIntrospector,
        candidates: &mut Vec<SingletonCandidate>,
    ) -> Result<()> {
        const MAX_NODES_TO_SCAN: usize = 15_000;
        const MAX_DEPTH: usize = 4;
        const MAX_EXTRA_ROOTS: usize = 256;

        let mut seen_root_names: HashSet<String> = candidates
            .iter()
            .map(|candidate| candidate.full_name.to_ascii_lowercase())
            .collect();
        let mut visited: HashSet<u64> = HashSet::new();
        let mut queue: VecDeque<(u64, usize)> = VecDeque::new();

        for candidate in candidates.iter() {
            if let Some(ptr) = candidate.instance_ptr {
                if visited.insert(ptr) {
                    queue.push_back((ptr, 0));
                }
            }
        }

        let mut scanned_nodes = 0usize;
        let mut extra_roots: Vec<SingletonCandidate> = Vec::new();

        while let Some((obj_ptr, depth)) = queue.pop_front() {
            if scanned_nodes >= MAX_NODES_TO_SCAN || extra_roots.len() >= MAX_EXTRA_ROOTS {
                break;
            }
            scanned_nodes += 1;

            let class_name = introspector
                .class_name_for_object(obj_ptr)
                .unwrap_or_default();
            let class_name_lower = class_name.to_ascii_lowercase();

            if Self::looks_like_root_candidate_class(&class_name)
                && !seen_root_names.contains(&class_name_lower)
            {
                seen_root_names.insert(class_name_lower);
                extra_roots.push(SingletonCandidate {
                    namespace: String::new(),
                    class_name: class_name.clone(),
                    full_name: class_name,
                    generic_class_index: 0,
                    singleton_class_ptr: 0,
                    instance_ptr: Some(obj_ptr),
                });
            }

            if depth >= MAX_DEPTH {
                continue;
            }

            let fields = match introspector.runtime_fields_for_object_cached(obj_ptr) {
                Ok(f) => f,
                Err(_) => continue,
            };

            for field in &fields {
                if field.offset < 0 {
                    continue;
                }

                let child_ptr = match introspector
                    .process_memory()
                    .read_pointer(obj_ptr + field.offset as u64)
                {
                    Ok(ptr) => ptr,
                    Err(_) => continue,
                };

                if child_ptr == 0 || !visited.insert(child_ptr) {
                    continue;
                }

                if introspector
                    .runtime_fields_for_object_cached(child_ptr)
                    .is_err()
                {
                    continue;
                }

                queue.push_back((child_ptr, depth + 1));
            }
        }

        candidates.extend(extra_roots);
        Ok(())
    }

    // -----------------------------------------------------------------------
    // Print helpers
    // -----------------------------------------------------------------------

    fn print_field_preview(
        &mut self,
        object_ptr: u64,
        field: &RuntimeField,
        decoder: Option<&str>,
    ) -> Result<()> {
        if field.offset < 0 {
            println!(
                "Field '{}' has unsupported negative offset {}",
                field.name, field.offset
            );
            return Ok(());
        }

        let field_addr = object_ptr + field.offset as u64;
        println!("Field '{}' at {:#x}", field.name, field_addr);

        let decoder = decoder.unwrap_or("auto").to_ascii_lowercase();
        if decoder == "auto" {
            self.print_raw_16(field_addr);

            if let Ok(value) = self.introspector.read_i32_at(field_addr) {
                println!("  i32: {} ({:#x})", value, value);
                println!("  u32: {} ({:#x})", value as u32, value as u32);
            }
            if let Ok(value) = self.introspector.read_i64_at(field_addr) {
                println!("  i64: {} ({:#x})", value, value);
                println!("  u64: {} ({:#x})", value as u64, value as u64);
            }

            if let Ok(bytes) = self.introspector.read_bytes_at(field_addr, 1) {
                if bytes[0] <= 1 {
                    println!("  bool(u8): {}", bytes[0] == 1);
                }
            }

            if let Ok(ptr) = self.introspector.process_memory().read_pointer(field_addr) {
                println!("  ptr: {:#x}", ptr);
                if ptr != 0 {
                    let mut ptr_class_hint = String::new();
                    if self
                        .introspector
                        .runtime_fields_for_object_cached(ptr)
                        .is_ok()
                    {
                        match self.describe_object(ptr) {
                            Ok(class_name) => {
                                ptr_class_hint = class_name.clone();
                                println!("  ptr class: {}", class_name);
                            }
                            Err(_) => println!("  ptr class: <unresolved>"),
                        }
                    } else {
                        println!("  ptr class: <not a readable managed object>");
                    }

                    if !ptr_class_hint.is_empty() {
                        let hint_lower = ptr_class_hint.to_ascii_lowercase();
                        if hint_lower.contains("obscuredstring") {
                            if let Ok(value) =
                                self.introspector.read_obscured_string_ptr(field_addr)
                            {
                                if !value.is_empty() {
                                    println!("  → decoded: {:?}", value);
                                }
                            }
                        } else if hint_lower.contains("obscuredint") {
                            if let Ok(value) = self.introspector.decode_obscured_int(field_addr) {
                                println!("  → decoded: {} ({:#x})", value, value);
                            }
                        } else if hint_lower.contains("obscuredlong") {
                            if let Ok(value) = self.introspector.decode_obscured_long(field_addr) {
                                println!("  → decoded: {} ({:#x})", value, value);
                            }
                        } else if hint_lower.contains("obscuredbool") {
                            if let Ok(value) = self.introspector.decode_obscured_bool(field_addr) {
                                println!("  → decoded: {}", value);
                            }
                        } else if hint_lower.contains("int32[]") {
                            if let Ok(arr) = self.introspector.read_int32_array_from_array_ptr(ptr)
                            {
                                let show = if arr.len() > 200 {
                                    format!("{} values (showing first 200)", arr.len())
                                } else {
                                    format!("{:?}", arr)
                                };
                                println!("  Int32[{}]: {}", arr.len(), show);
                            }
                        } else if hint_lower.contains("list`1") || hint_lower.contains("list") {
                            if let Ok(items_ptr) = self
                                .introspector
                                .process_memory()
                                .read_pointer(ptr + LIST_ITEMS_OFFSET)
                            {
                                if items_ptr != 0 {
                                    if let Ok(items_class) = self.describe_object(items_ptr) {
                                        let items_lower = items_class.to_ascii_lowercase();
                                        if items_lower.contains("int32[]") {
                                            if let Ok(arr) = self.introspector.read_int32_list(ptr)
                                            {
                                                let show = if arr.len() > 200 {
                                                    format!(
                                                        "{} values (showing first 200)",
                                                        arr.len()
                                                    )
                                                } else {
                                                    format!("{:?}", arr)
                                                };
                                                println!("  List<Int32>[{}]: {}", arr.len(), show);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if let Ok(value) = self.read_string_from_pointer(ptr) {
                        if !value.is_empty() {
                            println!("  ptr as string: {:?}", value);
                        }
                    }
                }
            }

            if let Ok(value) = self.introspector.read_managed_string_ptr(field_addr) {
                if !value.is_empty() {
                    println!("  managed-string: {:?}", value);
                }
            }

            let name_hint = field.name.to_ascii_lowercase();
            let normalized_name = RuntimeIntrospector::normalize_field_name(&field.name);
            let likely_id_name = normalized_name.ends_with("id");
            let likely_obscured = name_hint.contains("obscured")
                || name_hint.contains("crypto")
                || name_hint.contains("hidden")
                || name_hint.contains("xor");

            if likely_obscured || likely_id_name {
                if let Ok(value) = self.introspector.decode_obscured_int(field_addr) {
                    println!("  obscured-int: {} ({:#x})", value, value);
                }
            }

            if likely_obscured {
                if let Ok(value) = self.introspector.decode_obscured_long(field_addr) {
                    println!("  obscured-long: {} ({:#x})", value, value);
                }
                if let Ok(value) = self.introspector.decode_obscured_bool(field_addr) {
                    println!("  obscured-bool: {}", value);
                }
                if let Ok(value) = self.introspector.read_obscured_string_ptr(field_addr) {
                    if !value.is_empty() {
                        println!("  obscured-string: {:?}", value);
                    }
                }
            }

            return Ok(());
        }

        self.print_with_decoder(field_addr, &decoder)
    }

    fn print_raw_16(&mut self, field_addr: u64) {
        match self.introspector.read_bytes_at(field_addr, 16) {
            Ok(bytes) => {
                let hex = bytes
                    .iter()
                    .map(|b| format!("{:02x}", b))
                    .collect::<Vec<_>>()
                    .join(" ");
                println!("  raw[16]: {}", hex);
            }
            Err(err) => println!("  raw[16]: <unreadable: {}>", err),
        }
    }

    fn print_with_decoder(&mut self, field_addr: u64, decoder: &str) -> Result<()> {
        match decoder {
            "raw" => self.print_raw_16(field_addr),
            "i32" => println!("  i32: {}", self.introspector.read_i32_at(field_addr)?),
            "u32" => println!(
                "  u32: {}",
                self.introspector.read_i32_at(field_addr)? as u32
            ),
            "i64" => println!("  i64: {}", self.introspector.read_i64_at(field_addr)?),
            "u64" => println!(
                "  u64: {}",
                self.introspector.read_i64_at(field_addr)? as u64
            ),
            "f32" => println!("  f32: {}", self.introspector.read_f32_at(field_addr)?),
            "f64" => println!("  f64: {}", self.introspector.read_f64_at(field_addr)?),
            "bool" => {
                let byte = self.introspector.read_bytes_at(field_addr, 1)?[0];
                if byte > 1 {
                    println!(
                        "  bool: non-canonical byte {} (treating non-zero as true)",
                        byte
                    );
                }
                println!("  bool: {}", byte != 0);
            }
            "ptr" => {
                let ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                println!("  ptr: {:#x}", ptr);
                if ptr != 0 {
                    match self.describe_object(ptr) {
                        Ok(class_name) => println!("  ptr class: {}", class_name),
                        Err(_) => println!("  ptr class: <unresolved>"),
                    }
                }
            }
            "string" | "managed-string" => {
                let value = self.introspector.read_managed_string_ptr(field_addr)?;
                println!("  managed-string: {:?}", value);
            }
            "datetime" | "date-time" | "timestamp" => {
                self.print_datetime_preview(field_addr)?;
            }
            "obscured-int" => {
                let value = self.introspector.decode_obscured_int(field_addr)?;
                println!("  obscured-int: {} ({:#x})", value, value);
            }
            "obscured-long" => {
                let value = self.introspector.decode_obscured_long(field_addr)?;
                println!("  obscured-long: {} ({:#x})", value, value);
            }
            "obscured-bool" => {
                let value = self.introspector.decode_obscured_bool(field_addr)?;
                println!("  obscured-bool: {}", value);
            }
            "obscured-string" => {
                let value = self.introspector.read_obscured_string_ptr(field_addr)?;
                println!("  obscured-string: {:?}", value);
            }
            "int32-array" | "i32-array" => {
                let array_ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                if array_ptr == 0 {
                    println!("  (null)");
                } else {
                    let arr = self
                        .introspector
                        .read_int32_array_from_array_ptr(array_ptr)?;
                    let show = if arr.len() > 200 {
                        format!("{} values (showing first 200)", arr.len())
                    } else {
                        format!("{:?}", arr)
                    };
                    println!("  Int32[{}]: {}", arr.len(), show);
                }
            }
            "int32-list" | "i32-list" => {
                let list_ptr = self
                    .introspector
                    .process_memory()
                    .read_pointer(field_addr)?;
                if list_ptr == 0 {
                    println!("  (null)");
                } else {
                    let arr = self.introspector.read_int32_list(list_ptr)?;
                    let show = if arr.len() > 200 {
                        format!("{} values (showing first 200)", arr.len())
                    } else {
                        format!("{:?}", arr)
                    };
                    println!("  List<Int32>[{}]: {}", arr.len(), show);
                }
            }
            _ => println!("  Unknown decoder '{}'", decoder),
        }
        Ok(())
    }

    fn print_datetime_preview(&mut self, field_addr: u64) -> Result<()> {
        let mut printed_any = false;

        if let Ok(raw_string) = self.introspector.read_managed_string_ptr(field_addr) {
            if let Some(normalized) = Self::normalize_datetime_string_for_peek(&raw_string) {
                println!("  datetime (managed-string): {}", normalized);
                printed_any = true;
            }
        }

        if let Ok(obscured_value) = self.introspector.decode_obscured_int(field_addr) {
            println!("  obscured-int: {}", obscured_value);
            printed_any = true;
            if let Some(compact_date) = Self::format_compact_calendar_date(obscured_value as i64) {
                println!("  datetime (obscured YYYYMMDD): {}", compact_date);
            }
        }

        if let Ok(raw_i32) = self.introspector.read_i32_at(field_addr) {
            if let Some(compact_date) = Self::format_compact_calendar_date(raw_i32 as i64) {
                println!("  datetime (i32 YYYYMMDD): {}", compact_date);
                printed_any = true;
            }
        }

        if let Ok(raw_i64) = self.introspector.read_i64_at(field_addr) {
            for candidate in Self::decode_datetime_candidates(raw_i64) {
                println!("  {}", candidate);
                printed_any = true;
            }
        }

        if !printed_any {
            println!(
                "  datetime: no recognizable value (tried managed string, unix timestamp, and .NET ticks)"
            );
        }

        Ok(())
    }

    fn print_class_hits(hits: &[ClassSearchHit], active_root_idx: usize) {
        for (idx, hit) in hits.iter().enumerate() {
            let path = if hit.object_path.is_empty() {
                "<root>".to_string()
            } else {
                hit.object_path.join(" -> ")
            };
            println!(
                "  [{:>2}] {} @ {:#x}",
                idx, hit.object_class, hit.object_ptr
            );
            if hit.root_index == usize::MAX {
                println!("       root: {}", hit.root_name);
            } else {
                println!("       root[{}]: {}", hit.root_index, hit.root_name);
            }
            println!("       path: {}", path);

            if hit.root_index == usize::MAX {
                println!("       hint: jump {:#x}", hit.object_ptr);
                continue;
            }

            if hit.root_index == active_root_idx {
                if hit.object_path.is_empty() {
                    println!("       hint: already at root object");
                } else {
                    let chain = hit
                        .object_path
                        .iter()
                        .map(|step| format!("follow {}", step))
                        .collect::<Vec<_>>()
                        .join(" ; ");
                    println!("       hint: {}", chain);
                }
            } else {
                let chain = hit
                    .object_path
                    .iter()
                    .map(|step| format!("follow {}", step))
                    .collect::<Vec<_>>()
                    .join(" ; ");
                if chain.is_empty() {
                    println!("       hint: use-root {}", hit.root_index);
                } else {
                    println!("       hint: use-root {} ; {}", hit.root_index, chain);
                }
            }
        }
    }

    fn print_singleton_roots(candidates: &[SingletonCandidate], active_root_idx: usize) {
        println!("Root candidates:");
        for (idx, candidate) in candidates.iter().enumerate() {
            let selected_marker = if idx == active_root_idx { "*" } else { " " };
            if let Some(instance_ptr) = candidate.instance_ptr {
                println!(
                    "  {} [{:>3}] {:<45} instance={:#x} class={:#x} idx={}",
                    selected_marker,
                    idx,
                    candidate.full_name,
                    instance_ptr,
                    candidate.singleton_class_ptr,
                    candidate.generic_class_index
                );
            } else {
                println!(
                    "  {} [{:>3}] {:<45} instance=<null> class={:#x} idx={}",
                    selected_marker,
                    idx,
                    candidate.full_name,
                    candidate.singleton_class_ptr,
                    candidate.generic_class_index
                );
            }
        }
    }

    fn print_peek_help() {
        println!(
            "peek usage:\n  peek <field|index>\n  peek <field|index> as <decoder>\n  peek <field|index> <decoder>\n\nDecoders:\n  auto            Try useful non-noisy interpretations (default)\n  raw             Show first 16 raw bytes\n  i32 | u32       Interpret as 32-bit integer\n  i64 | u64       Interpret as 64-bit integer\n  f32 | f64       Interpret as floating point\n  bool            Interpret first byte as bool (0/1)\n  ptr             Interpret as managed object pointer\n  string          Interpret as managed string pointer\n  datetime        Decode common datetime encodings (string/unix/.NET ticks/YYYYMMDD)\n  obscured-int    Decode ObscuredInt at field address\n  obscured-long   Decode ObscuredLong at field address\n  obscured-bool   Decode ObscuredBool at field address\n  obscured-string Decode ObscuredString pointer at field address\n  int32-array     Decode as Int32[] at pointer\n  int32-list      Decode as List<Int32> at pointer\n\nExamples:\n  peek 12\n  peek dataVersion i32\n  peek createTime datetime\n  peek _id as obscured-int\n  peek _name string"
        );
    }

    // -----------------------------------------------------------------------
    // Collection navigation helpers
    // -----------------------------------------------------------------------

    fn build_collection_context(
        &mut self,
        collection_ptr: u64,
        collection_type: &str,
        current_item_ptr: u64,
        item_depth: usize,
    ) -> Result<CollectionContext> {
        let collection_type_str = collection_type.to_string();

        match collection_type.to_ascii_lowercase().as_str() {
            "@list-item" => {
                let items = self.read_pointer_list_from_list_ptr(collection_ptr)?;
                if items.is_empty() {
                    return Err(anyhow!("List is empty"));
                }
                let current_index = items
                    .iter()
                    .position(|ptr| *ptr == current_item_ptr)
                    .unwrap_or(0);
                Ok(CollectionContext {
                    collection_ptr,
                    collection_type: collection_type_str,
                    current_index,
                    total_count: items.len(),
                    item_depth,
                })
            }
            "@array-item" => {
                let items = self.read_pointer_array_from_array_ptr(collection_ptr)?;
                if items.is_empty() {
                    return Err(anyhow!("Array is empty"));
                }
                let current_index = items
                    .iter()
                    .position(|ptr| *ptr == current_item_ptr)
                    .unwrap_or(0);
                Ok(CollectionContext {
                    collection_ptr,
                    collection_type: collection_type_str,
                    current_index,
                    total_count: items.len(),
                    item_depth,
                })
            }
            "@dict-value" => {
                let items = self.iter_dictionary_value_ptrs(collection_ptr)?;
                if items.is_empty() {
                    return Err(anyhow!("Dictionary is empty"));
                }
                let current_index = items
                    .iter()
                    .position(|ptr| *ptr == current_item_ptr)
                    .unwrap_or(0);
                Ok(CollectionContext {
                    collection_ptr,
                    collection_type: collection_type_str,
                    current_index,
                    total_count: items.len(),
                    item_depth,
                })
            }
            _ => Err(anyhow!("Unknown collection type: {}", collection_type)),
        }
    }

    fn get_collection_item_at_index(&mut self, ctx: &CollectionContext) -> Result<u64> {
        match ctx.collection_type.to_ascii_lowercase().as_str() {
            "@list-item" => {
                let items = self.read_pointer_list_from_list_ptr(ctx.collection_ptr)?;
                items
                    .get(ctx.current_index)
                    .copied()
                    .ok_or_else(|| anyhow!("Index {} out of range", ctx.current_index))
            }
            "@array-item" => {
                let items = self.read_pointer_array_from_array_ptr(ctx.collection_ptr)?;
                items
                    .get(ctx.current_index)
                    .copied()
                    .ok_or_else(|| anyhow!("Index {} out of range", ctx.current_index))
            }
            "@dict-value" => {
                let items = self.iter_dictionary_value_ptrs(ctx.collection_ptr)?;
                items
                    .get(ctx.current_index)
                    .copied()
                    .ok_or_else(|| anyhow!("Index {} out of range", ctx.current_index))
            }
            _ => Err(anyhow!("Unknown collection type: {}", ctx.collection_type)),
        }
    }

    fn iter_dictionary_value_ptrs(&mut self, dict_ptr: u64) -> Result<Vec<u64>> {
        let entries_array_ptr = self
            .introspector
            .process_memory()
            .read_pointer(dict_ptr + DICT_ENTRIES_PTR_OFFSET)
            .map_err(|e| anyhow!("Failed reading dictionary.entries pointer: {}", e))?;
        if entries_array_ptr == 0 {
            return Ok(Vec::new());
        }

        let max_length = self.read_array_length(entries_array_ptr)?;
        let entries_base = entries_array_ptr + ARRAY_ITEMS_OFFSET;
        let mut result = Vec::new();

        for i in 0..max_length {
            let entry_addr = entries_base + i * SUPPORT_CARD_ENTRY_SIZE;
            let hash_code = match self
                .introspector
                .process_memory()
                .read_i32(entry_addr + SUPPORT_CARD_ENTRY_HASH_OFFSET)
            {
                Ok(v) => v,
                Err(_) => continue,
            };
            if hash_code < 0 {
                continue;
            }

            let value_ptr = match self
                .introspector
                .process_memory()
                .read_pointer(entry_addr + SUPPORT_CARD_ENTRY_VALUE_PTR_OFFSET)
            {
                Ok(v) => v,
                Err(_) => continue,
            };
            if value_ptr == 0 {
                continue;
            }

            result.push(value_ptr);
        }

        Ok(result)
    }

    // Delegate reader methods to introspector
    fn read_pointer_list_from_list_ptr(&mut self, list_ptr: u64) -> Result<Vec<u64>> {
        self.introspector.read_pointer_list_from_list_ptr(list_ptr)
    }

    fn read_pointer_array_from_array_ptr(&mut self, array_ptr: u64) -> Result<Vec<u64>> {
        self.introspector
            .read_pointer_array_from_array_ptr(array_ptr)
    }

    // -----------------------------------------------------------------------
    // Static helpers
    // -----------------------------------------------------------------------

    fn recovery_prompt_use_root(
        singleton_candidates: &[SingletonCandidate],
        active_root_idx: &mut usize,
        root_ptr: &mut u64,
        stack: &mut Vec<(u64, Option<String>)>,
        object_ptr: &mut u64,
        collection_contexts: &mut Vec<CollectionContext>,
        show_fields: &mut bool,
    ) -> Result<bool> {
        println!("Available commands: singletons, use-root <index>, quit");
        loop {
            print!("\nschema[recovery]> ");
            io::stdout().flush()?;
            let mut line = String::new();
            if io::stdin().read_line(&mut line)? == 0 {
                println!();
                return Ok(false);
            }
            let trimmed = line.trim();
            if trimmed.is_empty() {
                continue;
            }
            let mut parts = trimmed.split_whitespace();
            let cmd = parts.next().unwrap_or_default().to_ascii_lowercase();
            match cmd.as_str() {
                "singletons" | "roots" => {
                    Self::print_singleton_roots(singleton_candidates, *active_root_idx);
                }
                "use-root" => {
                    let Some(selector) = parts.next() else {
                        println!("Missing selector. Usage: use-root <index|type>");
                        continue;
                    };
                    let Some(next_root_idx) =
                        Self::resolve_singleton_selector(singleton_candidates, selector)
                    else {
                        println!("No singleton matches selector '{}'", selector);
                        continue;
                    };
                    let Some(next_root_ptr) = singleton_candidates[next_root_idx].instance_ptr
                    else {
                        println!(
                            "Singleton '{}' currently has null/unreadable _instance",
                            singleton_candidates[next_root_idx].full_name
                        );
                        continue;
                    };
                    *active_root_idx = next_root_idx;
                    *root_ptr = next_root_ptr;
                    stack.clear();
                    stack.push((*root_ptr, None));
                    *object_ptr = *root_ptr;
                    collection_contexts.clear();
                    println!(
                        "Switched root to {} @ {:#x}",
                        singleton_candidates[*active_root_idx].full_name, root_ptr
                    );
                    *show_fields = true;
                    return Ok(true);
                }
                "q" | "quit" | "exit" => return Ok(false),
                _ => {
                    println!("Available commands: singletons, use-root <index>, quit");
                }
            }
        }
    }

    fn parse_follow_step(step: &str) -> (&str, Option<usize>) {
        if step.starts_with('@') {
            if let Some((op, idx_str)) = step.split_once(':') {
                if let Ok(idx) = idx_str.parse::<usize>() {
                    return (op, Some(idx));
                }
            }
        }
        (step, None)
    }

    fn resolve_field_selector<'a>(
        fields: &'a [RuntimeField],
        selector: &str,
    ) -> Option<&'a RuntimeField> {
        if let Ok(index) = selector.parse::<usize>() {
            return fields.get(index);
        }

        fields
            .iter()
            .find(|field| RuntimeIntrospector::field_name_matches(&field.name, selector))
    }

    fn resolve_singleton_selector(
        candidates: &[SingletonCandidate],
        selector: &str,
    ) -> Option<usize> {
        if let Ok(index) = selector.parse::<usize>() {
            return candidates.get(index).map(|_| index);
        }

        let selector_lower = selector.to_ascii_lowercase();

        candidates.iter().position(|candidate| {
            candidate.full_name.to_ascii_lowercase() == selector_lower
                || candidate.class_name.to_ascii_lowercase() == selector_lower
        })
    }

    fn looks_like_root_candidate_class(class_name: &str) -> bool {
        let lower = class_name.to_ascii_lowercase();

        if lower.contains("<") || lower.contains("d__") || lower.contains("anonstorey") {
            return false;
        }

        if lower.starts_with("system.collections.generic::") {
            return false;
        }

        if lower.contains("gamesystem") {
            return true;
        }

        let contains_signal = lower.contains("manager")
            || lower.contains("system")
            || lower.contains("controller")
            || lower.contains("gamemain")
            || lower.contains("scene")
            || lower.contains("state")
            || lower.contains("flow")
            || lower.contains("navigator");

        if !contains_signal {
            return false;
        }

        if lower.starts_with("unityengine::") || lower.starts_with("system::") {
            return false;
        }

        lower.starts_with("gallop::")
    }

    fn stack_follow_path(stack: &[(u64, Option<String>)]) -> Vec<String> {
        stack
            .iter()
            .skip(1)
            .filter_map(|(_, step)| step.clone())
            .collect()
    }

    fn upsert_collection_context(contexts: &mut Vec<CollectionContext>, ctx: CollectionContext) {
        if let Some(existing_idx) = contexts.iter().position(|c| c.item_depth == ctx.item_depth) {
            contexts[existing_idx] = ctx;
        } else {
            contexts.push(ctx);
            contexts.sort_by_key(|c| c.item_depth);
        }
    }

    fn prune_collection_contexts(contexts: &mut Vec<CollectionContext>, current_depth: usize) {
        contexts.retain(|ctx| ctx.item_depth <= current_depth);
    }

    fn format_path_with_collection_contexts(
        stack: &[(u64, Option<String>)],
        contexts: &[CollectionContext],
        arrow: &str,
    ) -> String {
        if stack.len() <= 1 {
            return "<root>".to_string();
        }

        let mut parts = Vec::new();
        for (depth, (_, step)) in stack.iter().enumerate().skip(1) {
            if let Some(ctx) = contexts.iter().find(|ctx| ctx.item_depth == depth) {
                parts.push(format!(
                    "{}[{}/{}]",
                    ctx.collection_type,
                    ctx.current_index + 1,
                    ctx.total_count
                ));
            } else if let Some(step) = step {
                parts.push(step.clone());
            }
        }

        if parts.is_empty() {
            "<root>".to_string()
        } else {
            parts.join(arrow)
        }
    }

    // -----------------------------------------------------------------------
    // Datetime helpers
    // -----------------------------------------------------------------------

    fn decode_datetime_candidates(raw_i64: i64) -> Vec<String> {
        const DOTNET_TICKS_MASK: u64 = 0x3fff_ffff_ffff_ffff;

        let mut out = Vec::new();
        let mut seen = BTreeSet::new();

        if let Some(formatted) = Self::format_unix_seconds(raw_i64) {
            let line = format!("datetime (unix-seconds): {}", formatted);
            if seen.insert(line.clone()) {
                out.push(line);
            }
        }

        if let Some(formatted) = Self::format_unix_millis(raw_i64) {
            let line = format!("datetime (unix-millis): {}", formatted);
            if seen.insert(line.clone()) {
                out.push(line);
            }
        }

        if let Some(formatted) = Self::format_compact_calendar_date(raw_i64) {
            let line = format!("datetime (YYYYMMDD): {}", formatted);
            if seen.insert(line.clone()) {
                out.push(line);
            }
        }

        let raw_u64 = raw_i64 as u64;
        if let Some(formatted) = Self::format_dotnet_ticks(raw_u64) {
            let line = format!("datetime (.NET ticks): {}", formatted);
            if seen.insert(line.clone()) {
                out.push(line);
            }
        }

        let masked_ticks = raw_u64 & DOTNET_TICKS_MASK;
        if masked_ticks != raw_u64 {
            if let Some(formatted) = Self::format_dotnet_ticks(masked_ticks) {
                let line = format!("datetime (.NET ticks, kind-masked): {}", formatted);
                if seen.insert(line.clone()) {
                    out.push(line);
                }
            }
        }

        out
    }

    fn format_compact_calendar_date(raw: i64) -> Option<String> {
        if !(19000101..=29991231).contains(&raw) {
            return None;
        }

        let year = (raw / 10000) as i32;
        let month = ((raw / 100) % 100) as u32;
        let day = (raw % 100) as u32;

        chrono::NaiveDate::from_ymd_opt(year, month, day).map(|d| d.format("%Y-%m-%d").to_string())
    }

    fn format_unix_seconds(seconds: i64) -> Option<String> {
        if !(946_684_800..=4_102_444_800).contains(&seconds) {
            return None;
        }
        DateTime::<Utc>::from_timestamp(seconds, 0)
            .map(|dt| dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn format_unix_millis(millis: i64) -> Option<String> {
        if !(946_684_800_000..=4_102_444_800_000).contains(&millis) {
            return None;
        }
        DateTime::<Utc>::from_timestamp_millis(millis)
            .map(|dt| dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn format_dotnet_ticks(ticks: u64) -> Option<String> {
        const TICKS_PER_SECOND: i128 = 10_000_000;
        const UNIX_EPOCH_TICKS: i128 = 621_355_968_000_000_000;

        let unix_ticks = ticks as i128 - UNIX_EPOCH_TICKS;
        let secs = unix_ticks / TICKS_PER_SECOND;
        let rem_ticks = unix_ticks % TICKS_PER_SECOND;
        if secs < i64::MIN as i128 || secs > i64::MAX as i128 {
            return None;
        }
        let nanos_i128 = rem_ticks * 100;
        if nanos_i128 < 0 || nanos_i128 > u32::MAX as i128 {
            return None;
        }

        DateTime::<Utc>::from_timestamp(secs as i64, nanos_i128 as u32)
            .map(|dt| dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string())
    }

    fn normalize_datetime_string_for_peek(raw: &str) -> Option<String> {
        if raw.trim().is_empty() {
            return None;
        }

        if let Ok(dt) = NaiveDateTime::parse_from_str(raw, "%Y-%m-%d %H:%M:%S") {
            return Some(dt.format("%Y-%m-%d %H:%M:%S").to_string());
        }

        if let Ok(dt) = chrono::DateTime::parse_from_rfc3339(raw) {
            return Some(dt.naive_utc().format("%Y-%m-%d %H:%M:%S").to_string());
        }

        None
    }

    // -----------------------------------------------------------------------
    // Public API: resolve_singleton_root / resolve_work_data_manager_instance
    // -----------------------------------------------------------------------

    pub fn resolve_singleton_root(&mut self, selector: &str) -> Result<u64> {
        let pid = self.introspector.pid();
        let il2cpp_metadata =
            Il2CppMetadata::find_in_process(self.introspector.process_memory(), pid)?;

        println!("Discovering singleton roots...");
        let mut candidates =
            il2cpp_metadata.discover_singleton_candidates(self.introspector.process_memory())?;

        if let Ok(mono_candidates) =
            il2cpp_metadata.discover_mono_singleton_candidates(self.introspector.process_memory())
        {
            for candidate in mono_candidates {
                let key = candidate.full_name.to_ascii_lowercase();
                if !candidates
                    .iter()
                    .any(|c| c.full_name.to_ascii_lowercase() == key)
                {
                    candidates.push(candidate);
                }
            }
        }

        Self::discover_supplemental_roots_from_singletons(&mut self.introspector, &mut candidates)?;

        let live_count = candidates
            .iter()
            .filter(|c| c.instance_ptr.is_some())
            .count();

        println!(
            "  Found {} root candidates ({} with live instances)",
            candidates.len(),
            live_count
        );

        let idx = Self::resolve_singleton_selector(&candidates, selector).ok_or_else(|| {
            let mut msg = format!(
                "No singleton root matches '{}'. Available roots:\n",
                selector
            );
            for (i, c) in candidates.iter().enumerate() {
                match c.instance_ptr {
                    Some(ptr) => {
                        msg.push_str(&format!("  [{}] {} @ {:#x}\n", i, c.full_name, ptr));
                    }
                    None => {
                        msg.push_str(&format!("  [{}] {} <null>\n", i, c.full_name));
                    }
                }
            }
            anyhow!(msg)
        })?;

        candidates[idx].instance_ptr.ok_or_else(|| {
            anyhow!(
                "Singleton '{}' has null instance pointer",
                candidates[idx].full_name
            )
        })
    }

    pub fn resolve_work_data_manager_instance(&mut self) -> Result<u64> {
        let pid = self.introspector.pid();
        let il2cpp_metadata =
            Il2CppMetadata::find_in_process(self.introspector.process_memory(), pid)?;

        println!("Finding WorkDataManager singleton...");

        let mut candidates =
            il2cpp_metadata.discover_singleton_candidates(self.introspector.process_memory())?;
        if let Ok(mono_candidates) =
            il2cpp_metadata.discover_mono_singleton_candidates(self.introspector.process_memory())
        {
            for mc in mono_candidates {
                let key = mc.full_name.to_ascii_lowercase();
                if !candidates
                    .iter()
                    .any(|c| c.full_name.to_ascii_lowercase() == key)
                {
                    candidates.push(mc);
                }
            }
        }

        let matched = candidates.iter().find(|c| {
            c.class_name.eq_ignore_ascii_case("WorkDataManager")
                && c.namespace.eq_ignore_ascii_case("Gallop")
        });

        let wdm_ptr = match matched {
            Some(c) => c
                .instance_ptr
                .ok_or_else(|| anyhow!("WorkDataManager singleton instance pointer is null"))?,
            None => {
                println!("  Not found via Singleton<T> enumeration, trying metadata path...");
                il2cpp_metadata.resolve_singleton_by_class_name(
                    self.introspector.process_memory(),
                    "Gallop",
                    "WorkDataManager",
                )?
            }
        };

        println!("  Instance at {:#x}\n", wdm_ptr);
        Ok(wdm_ptr)
    }
}
