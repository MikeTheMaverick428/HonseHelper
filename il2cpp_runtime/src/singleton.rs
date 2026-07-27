use crate::memory::ProcessMemory;
use anyhow::{anyhow, Result};

const IL2CPP_CLASS_STATIC_FIELDS_OFFSET: u64 = 184;
const IL2CPP_CLASS_PARENT_OFFSET: u64 = 88;
const IL2CPP_CLASS_FIELDS_PTR_OFFSET: u64 = 128;
const FIELD_INFO_NAME_PTR_OFFSET: u64 = 0;
const FIELD_INFO_OFFSET_OFFSET: u64 = 24;
const FIELD_INFO_STRIDE: u64 = 32;
const MAX_RUNTIME_FIELDS_TO_SCAN: usize = 256;

pub struct SingletonResolver;

impl SingletonResolver {
    pub fn resolve_singleton_instance<F>(
        memory: &mut ProcessMemory,
        singleton_class_ptr: u64,
        resolve_ptr: F,
    ) -> Result<u64>
    where
        F: Fn(&mut ProcessMemory, u64) -> Option<u64>,
    {
        Self::resolve_singleton_like_instance(memory, singleton_class_ptr, resolve_ptr).ok_or_else(
            || {
                anyhow!(
                    "Singleton _instance pointer is null/unreadable for class {:#x}",
                    singleton_class_ptr
                )
            },
        )
    }

    pub fn resolve_mono_singleton_instance<F>(
        memory: &mut ProcessMemory,
        mono_singleton_class_ptr: u64,
        resolve_ptr: F,
    ) -> Result<u64>
    where
        F: Fn(&mut ProcessMemory, u64) -> Option<u64>,
    {
        Self::resolve_singleton_like_instance(memory, mono_singleton_class_ptr, resolve_ptr)
            .ok_or_else(|| {
                anyhow!(
                    "MonoSingleton _instance pointer is null/unreadable for class {:#x}",
                    mono_singleton_class_ptr
                )
            })
    }

    fn resolve_singleton_like_instance<F>(
        memory: &mut ProcessMemory,
        singleton_class_ptr: u64,
        resolve_ptr: F,
    ) -> Option<u64>
    where
        F: Fn(&mut ProcessMemory, u64) -> Option<u64>,
    {
        let static_fields_ptr_val = memory
            .read_pointer(singleton_class_ptr.wrapping_add(IL2CPP_CLASS_STATIC_FIELDS_OFFSET))
            .ok()?;
        let static_fields_abs = resolve_ptr(memory, static_fields_ptr_val)?;

        if let Some(instance_field_offset) =
            Self::find_singleton_instance_static_field_offset(memory, singleton_class_ptr)
        {
            if let Some(instance_abs) = Self::try_read_valid_managed_object_ptr(
                memory,
                static_fields_abs + instance_field_offset,
                &resolve_ptr,
            ) {
                return Some(instance_abs);
            }
        }

        Self::try_read_valid_managed_object_ptr(memory, static_fields_abs, &resolve_ptr)
    }

    fn try_read_valid_managed_object_ptr<F>(
        memory: &mut ProcessMemory,
        ptr_addr: u64,
        resolve_ptr: &F,
    ) -> Option<u64>
    where
        F: Fn(&mut ProcessMemory, u64) -> Option<u64>,
    {
        let instance_ptr_val = memory.read_pointer(ptr_addr).ok()?;
        let instance_abs = resolve_ptr(memory, instance_ptr_val)?;
        if instance_abs < 0x10000 {
            return None;
        }

        let klass_ptr = memory.read_pointer(instance_abs).ok()?;
        if klass_ptr < 0x10000 {
            return None;
        }

        Some(instance_abs)
    }

    fn find_singleton_instance_static_field_offset(
        memory: &mut ProcessMemory,
        singleton_class_ptr: u64,
    ) -> Option<u64> {
        let mut current = singleton_class_ptr;
        let mut depth = 0usize;

        while current != 0 && depth < 16 {
            depth += 1;

            if let Ok(fields_ptr) = memory.read_pointer(current.wrapping_add(IL2CPP_CLASS_FIELDS_PTR_OFFSET)) {
                if fields_ptr != 0 {
                    let mut invalid_streak = 0usize;
                    for i in 0..MAX_RUNTIME_FIELDS_TO_SCAN {
                        let base = fields_ptr.wrapping_add((i as u64) * FIELD_INFO_STRIDE);

                        let name_ptr = match memory.read_pointer(base.wrapping_add(FIELD_INFO_NAME_PTR_OFFSET))
                        {
                            Ok(v) => v,
                            Err(_) => {
                                invalid_streak += 1;
                                if invalid_streak >= 8 {
                                    break;
                                }
                                continue;
                            }
                        };

                        let field_offset = match memory.read_i32(base.wrapping_add(FIELD_INFO_OFFSET_OFFSET)) {
                            Ok(v) => v,
                            Err(_) => {
                                invalid_streak += 1;
                                if invalid_streak >= 8 {
                                    break;
                                }
                                continue;
                            }
                        };

                        let Some(name) = Self::read_ascii_c_string(memory, name_ptr, 128) else {
                            invalid_streak += 1;
                            if invalid_streak >= 8 {
                                break;
                            }
                            continue;
                        };

                        invalid_streak = 0;
                        if field_offset >= 0 && Self::looks_like_singleton_instance_field(&name) {
                            return Some(field_offset as u64);
                        }
                    }
                }
            }

            let parent_ptr = match memory.read_pointer(current.wrapping_add(IL2CPP_CLASS_PARENT_OFFSET)) {
                Ok(v) => v,
                Err(_) => break,
            };
            if parent_ptr == 0 {
                break;
            }
            current = parent_ptr;
        }

        None
    }

    fn looks_like_singleton_instance_field(name: &str) -> bool {
        let normalized = name
            .trim()
            .trim_start_matches('_')
            .trim_start_matches('m')
            .trim_start_matches('_')
            .to_ascii_lowercase();

        normalized == "instance" || normalized == "s_instance"
    }

    fn read_ascii_c_string(memory: &mut ProcessMemory, ptr: u64, max_len: usize) -> Option<String> {
        if ptr < 0x10000 || max_len == 0 {
            return None;
        }

        let bytes = memory.read_bytes(ptr, max_len).ok()?;
        let end = bytes.iter().position(|&b| b == 0)?;
        if end == 0 {
            return None;
        }

        let slice = &bytes[..end];
        if !slice
            .iter()
            .all(|b| b.is_ascii_graphic() || matches!(*b, b' ' | b'_' | b'<' | b'>' | b'`'))
        {
            return None;
        }

        String::from_utf8(slice.to_vec()).ok()
    }
}
