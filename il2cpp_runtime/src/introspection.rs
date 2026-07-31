use crate::il2cpp::Il2CppMetadata;
use crate::memory::ProcessMemory;
#[cfg(unix)]
use crate::process::list_memory_regions;
use crate::readers::{
    describe_object, read_c_string, RuntimeField, RuntimeReaderContext,
    IL2CPP_CLASS_NAMESPACE_PTR_OFFSET, IL2CPP_CLASS_NAME_PTR_OFFSET,
};
use anyhow::Context;
use anyhow::{anyhow, Result};
use std::collections::{BTreeMap, HashMap, HashSet};

#[cfg(windows)]
use windows::Win32::System::Memory::{
    VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, MEM_PRIVATE, PAGE_EXECUTE,
    PAGE_EXECUTE_READ, PAGE_EXECUTE_READWRITE, PAGE_EXECUTE_WRITECOPY, PAGE_GUARD, PAGE_NOACCESS,
    PAGE_PROTECTION_FLAGS, PAGE_READONLY, PAGE_READWRITE, PAGE_WRITECOPY,
};

const IL2CPP_CLASS_PARENT_OFFSET: u64 = 88;
const IL2CPP_CLASS_FIELDS_PTR_OFFSET: u64 = 128;
const FIELD_INFO_NAME_PTR_OFFSET: u64 = 0;
const FIELD_INFO_TYPE_PTR_OFFSET: u64 = 8;
const FIELD_INFO_PARENT_PTR_OFFSET: u64 = 16;
const FIELD_INFO_OFFSET_OFFSET: u64 = 24;
const FIELD_INFO_STRIDE: u64 = 32;
const MAX_RUNTIME_FIELDS_TO_SCAN: usize = 128;

#[derive(Debug, Clone, Copy)]
struct ScanRegion {
    start: u64,
    end: u64,
}

pub struct RuntimeIntrospector {
    memory: ProcessMemory,
    il2cpp_metadata: Option<Il2CppMetadata>,
    runtime_field_cache_by_class: HashMap<u64, Vec<RuntimeField>>,
}

impl RuntimeIntrospector {
    pub fn new(memory: ProcessMemory) -> Self {
        Self {
            memory,
            il2cpp_metadata: None,
            runtime_field_cache_by_class: HashMap::new(),
        }
    }

    pub fn set_il2cpp_metadata(&mut self, metadata: Il2CppMetadata) {
        self.il2cpp_metadata = Some(metadata);
    }

    pub fn resolve_ptr(&mut self, ptr_val: u64) -> Option<u64> {
        let metadata = self.il2cpp_metadata.as_ref()?;
        metadata.resolve_ptr(&mut self.memory, ptr_val)
    }

    pub fn process_memory(&mut self) -> &mut ProcessMemory {
        &mut self.memory
    }

    pub fn pid(&self) -> u32 {
        self.memory.pid
    }

    pub fn runtime_fields_for_object_cached(&mut self, obj_ptr: u64) -> Result<Vec<RuntimeField>> {
        let class_ptr = self.memory.read_pointer(obj_ptr)?;
        if class_ptr == 0 {
            return Err(anyhow!("Object at {:#x} has null klass", obj_ptr));
        }

        if let Some(cached) = self.runtime_field_cache_by_class.get(&class_ptr) {
            return Ok(cached.clone());
        }

        let fields = self.runtime_field_map_for_class(class_ptr)?;
        self.runtime_field_cache_by_class
            .insert(class_ptr, fields.clone());
        Ok(fields)
    }

    pub fn resolve_runtime_offset_for_object(
        &mut self,
        obj_ptr: u64,
        candidates: &[&str],
    ) -> Result<u64> {
        let fields = self.runtime_fields_for_object_cached(obj_ptr)?;
        for candidate in candidates {
            if let Some(field) = fields
                .iter()
                .find(|f| Self::field_name_matches(&f.name, candidate))
            {
                if field.offset < 0 {
                    return Err(anyhow!(
                        "Runtime field '{}' resolved to negative offset {}",
                        field.name,
                        field.offset
                    ));
                }
                return Ok(field.offset as u64);
            }
        }

        Err(anyhow!(
            "Runtime field lookup failed for candidates [{}] on object {:#x}",
            candidates.join(", "),
            obj_ptr
        ))
    }

    pub fn class_name_for_object(&mut self, obj_ptr: u64) -> Result<String> {
        describe_object(self, obj_ptr)
    }

    pub fn class_name_for_class_ptr(&mut self, class_ptr: u64) -> Option<(String, String)> {
        if class_ptr < 0x10000 || class_ptr > 0x0001_0000_0000_0000 {
            return None;
        }

        let namespace_ptr = self
            .memory
            .read_pointer(class_ptr.wrapping_add(IL2CPP_CLASS_NAMESPACE_PTR_OFFSET))
            .ok()
            .unwrap_or(0);
        let name_ptr = self
            .memory
            .read_pointer(class_ptr.wrapping_add(IL2CPP_CLASS_NAME_PTR_OFFSET))
            .ok()
            .unwrap_or(0);

        let namespace = read_c_string(self, namespace_ptr, 128).unwrap_or_default();
        let name = read_c_string(self, name_ptr, 128)?;
        Some((namespace, name))
    }

    // -----------------------------------------------------------------------
    // Primitive readers (convenience wrappers used by RuntimeModelSpec)
    // -----------------------------------------------------------------------

    pub fn read_pointer_at(&mut self, addr: u64) -> Result<u64> {
        self.memory.read_pointer(addr)
    }

    pub fn read_i8_at(&mut self, addr: u64) -> Result<i8> {
        self.memory.read_i8(addr)
    }

    pub fn read_i32_at(&mut self, addr: u64) -> Result<i32> {
        self.memory.read_i32(addr)
    }

    pub fn read_i64_at(&mut self, addr: u64) -> Result<i64> {
        self.memory.read_i64(addr)
    }

    pub fn read_f32_at(&mut self, addr: u64) -> Result<f32> {
        self.memory.read_f32(addr)
    }

    pub fn read_f64_at(&mut self, addr: u64) -> Result<f64> {
        self.memory.read_f64(addr)
    }

    pub fn decode_obscured_int(&mut self, addr: u64) -> Result<i32> {
        crate::readers::decode_obscured_int(self, addr)
    }

    pub fn decode_obscured_long(&mut self, addr: u64) -> Result<i64> {
        crate::readers::decode_obscured_long(self, addr)
    }

    pub fn decode_obscured_bool(&mut self, addr: u64) -> Result<bool> {
        crate::readers::decode_obscured_bool(self, addr)
    }

    pub fn read_bytes_at(&mut self, addr: u64, size: usize) -> Result<Vec<u8>> {
        self.process_memory().read_bytes(addr, size)
    }

    pub fn read_managed_string_ptr(&mut self, addr: u64) -> Result<String> {
        crate::readers::read_managed_string_ptr(self, addr)
    }

    pub fn read_obscured_string_ptr(&mut self, addr: u64) -> Result<String> {
        crate::readers::read_obscured_string_ptr(self, addr)
    }

    pub fn read_obscured_int_array(&mut self, addr: u64) -> Result<Vec<i64>> {
        crate::readers::read_obscured_int_array(self, addr)
    }

    pub fn read_int32_array(&mut self, addr: u64) -> Result<Vec<i32>> {
        crate::readers::read_int32_array(self, addr)
    }

    pub fn read_int32_array_from_array_ptr(&mut self, array_ptr: u64) -> Result<Vec<i32>> {
        crate::readers::read_int32_array_from_array_ptr(self, array_ptr)
    }

    pub fn read_pointer_array(&mut self, addr: u64) -> Result<Vec<u64>> {
        crate::readers::read_pointer_array(self, addr)
    }

    pub fn read_pointer_array_from_array_ptr(&mut self, array_ptr: u64) -> Result<Vec<u64>> {
        crate::readers::read_pointer_array_from_array_ptr(self, array_ptr)
    }

    pub fn read_pointer_list(&mut self, addr: u64) -> Result<Vec<u64>> {
        crate::readers::read_pointer_list(self, addr)
    }

    pub fn read_int32_list(&mut self, addr: u64) -> Result<Vec<i32>> {
        crate::readers::read_int32_list(self, addr)
    }

    pub fn read_pointer_list_from_list_ptr(&mut self, list_ptr: u64) -> Result<Vec<u64>> {
        crate::readers::read_pointer_list_from_list_ptr(self, list_ptr)
    }

    /// Iterate value pointers from an IL2CPP `Dictionary<K,V>` object.
    /// Layout: +24 entries array ptr; each entry is 24 bytes: hashCode(i32 @0), value ptr(@16).
    pub fn iter_dictionary_value_ptrs(&mut self, dict_ptr: u64) -> Result<Vec<u64>> {
        const DICT_ENTRIES_PTR_OFFSET: u64 = 24;
        const ENTRY_SIZE: u64 = 24;
        const ENTRY_VALUE_PTR_OFFSET: u64 = 16;

        let entries_ptr = self
            .memory
            .read_pointer(dict_ptr + DICT_ENTRIES_PTR_OFFSET)?;
        if entries_ptr == 0 {
            return Ok(Vec::new());
        }

        let max_length = crate::readers::read_array_length(self, entries_ptr)?;
        let entries_base = entries_ptr + crate::readers::ARRAY_ITEMS_OFFSET;
        let mut result = Vec::new();

        for i in 0..max_length {
            let entry_addr = entries_base + i * ENTRY_SIZE;
            let hash_code = match self.memory.read_i32(entry_addr) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if hash_code < 0 {
                continue;
            }
            let value_ptr = match self
                .memory
                .read_pointer(entry_addr + ENTRY_VALUE_PTR_OFFSET)
            {
                Ok(v) => v,
                Err(_) => continue,
            };
            if value_ptr != 0 {
                result.push(value_ptr);
            }
        }

        Ok(result)
    }

    pub fn iter_dictionary_value_addrs(
        &mut self,
        dict_ptr: u64,
        entry_size: u64,
        value_offset: u64,
    ) -> Result<Vec<u64>> {
        const DICT_ENTRIES_PTR_OFFSET: u64 = 24;
        const DICT_COUNT_OFFSET: u64 = 32;

        let entries_ptr = self
            .memory
            .read_pointer(dict_ptr + DICT_ENTRIES_PTR_OFFSET)?;
        if entries_ptr == 0 {
            return Ok(Vec::new());
        }

        let _count = self.memory.read_i32(dict_ptr + DICT_COUNT_OFFSET)? as u64;
        let max_length = crate::readers::read_array_length(self, entries_ptr)?;
        let entries_base = entries_ptr + crate::readers::ARRAY_ITEMS_OFFSET;
        let mut result = Vec::new();

        for i in 0..max_length {
            let entry_addr = entries_base + i * entry_size;
            let hash_code = match self.memory.read_i32(entry_addr) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if hash_code < 0 {
                continue;
            }
            if result.len() as u64 >= _count {
                break;
            }
            result.push(entry_addr + value_offset);
        }

        Ok(result)
    }

    pub fn iter_dictionary_entries(
        &mut self,
        dict_ptr: u64,
        entry_size: u64,
        key_offset: u64,
        value_offset: u64,
    ) -> Result<Vec<(i32, u64)>> {
        const DICT_ENTRIES_PTR_OFFSET: u64 = 24;
        const DICT_COUNT_OFFSET: u64 = 32;

        let entries_ptr = self
            .memory
            .read_pointer(dict_ptr + DICT_ENTRIES_PTR_OFFSET)?;
        if entries_ptr == 0 {
            return Ok(Vec::new());
        }

        let count = self.memory.read_i32(dict_ptr + DICT_COUNT_OFFSET)?;
        let max_length = crate::readers::read_array_length(self, entries_ptr)?;
        let entries_base = entries_ptr + crate::readers::ARRAY_ITEMS_OFFSET;
        let mut result = Vec::new();

        for i in 0..max_length {
            let entry_addr = entries_base + i * entry_size;
            let hash_code = match self.memory.read_i32(entry_addr) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if hash_code < 0 {
                continue;
            }
            if result.len() as i32 >= count {
                break;
            }
            let key = self.memory.read_i32(entry_addr + key_offset)?;
            if key <= 0 {
                continue;
            }
            result.push((key, entry_addr + value_offset));
        }

        Ok(result)
    }

    pub fn find_first_live_object_by_class(
        &mut self,
        namespace: &str,
        class_name: &str,
        max_scan_bytes: usize,
    ) -> Result<Option<u64>> {
        let regions = self
            .candidate_live_object_regions()
            .context("Failed to enumerate candidate memory regions")?;
        let mut scanned = 0usize;
        let mut class_match_cache: HashMap<u64, bool> = HashMap::new();

        // Sort regions: largest first, so we hit the managed heap sooner
        let mut sorted_regions = regions;
        sorted_regions.sort_by(|a, b| (b.end - b.start).cmp(&(a.end - a.start)));

        for region in sorted_regions {
            if scanned >= max_scan_bytes {
                break;
            }

            let mut addr = region.start;
            let region_end = region.end;
            let mut remaining_region = (region_end - region.start) as usize;

            while addr < region_end && scanned < max_scan_bytes && remaining_region > 0 {
                let hard_remaining = max_scan_bytes - scanned;
                let chunk_size = remaining_region.min(hard_remaining).min(1024 * 1024);
                if chunk_size < 8 {
                    break;
                }

                let bytes = match self.memory.read_bytes(addr, chunk_size) {
                    Ok(v) => v,
                    Err(_) => {
                        // Skip problematic region and continue
                        addr += chunk_size as u64;
                        remaining_region = remaining_region.saturating_sub(chunk_size);
                        continue;
                    }
                };

                for offset in (0..=bytes.len() - 8).step_by(8) {
                    let candidate_obj = addr + offset as u64;
                    let class_ptr =
                        u64::from_le_bytes(bytes[offset..offset + 8].try_into().unwrap());
                    if class_ptr < 0x10000 {
                        continue;
                    }

                    let is_target = if let Some(v) = class_match_cache.get(&class_ptr) {
                        *v
                    } else {
                        let matched = self
                            .class_name_for_class_ptr(class_ptr)
                            .map(|(ns, name)| ns == namespace && name == class_name)
                            .unwrap_or(false);
                        class_match_cache.insert(class_ptr, matched);
                        matched
                    };

                    if !is_target {
                        continue;
                    }

                    if self.runtime_fields_for_object_cached(candidate_obj).is_ok() {
                        return Ok(Some(candidate_obj));
                    }
                }

                addr += chunk_size as u64;
                scanned += chunk_size;
                remaining_region = remaining_region.saturating_sub(chunk_size);
            }
        }

        Ok(None)
    }

    #[cfg(unix)]
    fn candidate_live_object_regions(&self) -> Result<Vec<ScanRegion>> {
        let regions = list_memory_regions(self.memory.pid)?;
        Ok(regions
            .into_iter()
            .filter(|r| {
                r.readable
                    && r.writable
                    && r.private
                    && !r.executable
                    && !matches!(r.pathname.as_deref(), Some(path) if path.starts_with('/'))
            })
            .map(|r| ScanRegion {
                start: r.start,
                end: r.end,
            })
            .collect())
    }

    #[cfg(not(any(unix, windows)))]
    fn candidate_live_object_regions(&self) -> Result<Vec<ScanRegion>> {
        anyhow::bail!("Memory scanning is not supported on this platform")
    }

    #[cfg(windows)]
    fn candidate_live_object_regions(&self) -> Result<Vec<ScanRegion>> {
        let mut regions = Vec::new();

        unsafe {
            let mut addr = 0usize;
            let mut mbi = MEMORY_BASIC_INFORMATION::default();

            while VirtualQueryEx(
                self.memory.process_handle(),
                Some(addr as *const _),
                &mut mbi,
                std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
            ) != 0
            {
                let base = mbi.BaseAddress as usize;
                let region_size = mbi.RegionSize;
                let end = base.saturating_add(region_size);

                let readable =
                    matches!(mbi.Protect, PAGE_READONLY | PAGE_READWRITE | PAGE_WRITECOPY);
                let writable = matches!(mbi.Protect, PAGE_READWRITE | PAGE_WRITECOPY);
                let executable = matches!(
                    mbi.Protect,
                    PAGE_EXECUTE
                        | PAGE_EXECUTE_READ
                        | PAGE_EXECUTE_READWRITE
                        | PAGE_EXECUTE_WRITECOPY
                );
                let guarded_or_inaccessible = (mbi.Protect & PAGE_GUARD)
                    != PAGE_PROTECTION_FLAGS(0)
                    || (mbi.Protect & PAGE_NOACCESS) != PAGE_PROTECTION_FLAGS(0);
                let private = mbi.Type == MEM_PRIVATE;

                if mbi.State == MEM_COMMIT
                    && !guarded_or_inaccessible
                    && readable
                    && writable
                    && private
                    && !executable
                    && end > base
                {
                    regions.push(ScanRegion {
                        start: base as u64,
                        end: end as u64,
                    });
                }

                if end <= addr {
                    break;
                }
                addr = end;
            }
        }

        Ok(regions)
    }

    pub fn runtime_field_map_for_class(&mut self, class_ptr: u64) -> Result<Vec<RuntimeField>> {
        let mut class_chain = Vec::new();
        let mut seen = HashSet::new();
        let mut current = class_ptr;
        while current != 0 && seen.insert(current) && class_chain.len() < 16 {
            class_chain.push(current);
            current = self
                .memory
                .read_pointer(current.wrapping_add(IL2CPP_CLASS_PARENT_OFFSET))
                .unwrap_or(0);
        }
        class_chain.reverse();

        let mut result = BTreeMap::<String, RuntimeField>::new();
        for class_ptr in class_chain {
            for field in self.scan_runtime_fields_for_class(class_ptr)? {
                result.insert(field.name.clone(), field);
            }
        }
        Ok(result.into_values().collect())
    }

    fn scan_runtime_fields_for_class(&mut self, class_ptr: u64) -> Result<Vec<RuntimeField>> {
        let fields_ptr = self
            .memory
            .read_pointer(class_ptr.wrapping_add(IL2CPP_CLASS_FIELDS_PTR_OFFSET))?;
        if fields_ptr == 0 {
            return Ok(Vec::new());
        }

        let mut fields = Vec::new();
        let mut invalid_streak = 0usize;
        for i in 0..MAX_RUNTIME_FIELDS_TO_SCAN {
            let base = fields_ptr.wrapping_add((i as u64) * FIELD_INFO_STRIDE);
            let name_ptr = match self.memory.read_pointer(base.wrapping_add(FIELD_INFO_NAME_PTR_OFFSET)) {
                Ok(ptr) => ptr,
                Err(_) => break,
            };
            let _type_ptr = self
                .memory
                .read_pointer(base.wrapping_add(FIELD_INFO_TYPE_PTR_OFFSET))
                .unwrap_or(0);
            let owner_class = self
                .memory
                .read_pointer(base.wrapping_add(FIELD_INFO_PARENT_PTR_OFFSET))
                .unwrap_or(0);
            let offset = self
                .memory
                .read_i32(base.wrapping_add(FIELD_INFO_OFFSET_OFFSET))
                .unwrap_or(i32::MIN);

            let Some(name) = read_c_string(self, name_ptr, 128) else {
                invalid_streak += 1;
                if invalid_streak >= 8 && !fields.is_empty() {
                    break;
                }
                continue;
            };

            if !Self::looks_like_runtime_field_name(&name) || offset < -1 || offset > 0x8000 {
                invalid_streak += 1;
                if invalid_streak >= 8 && !fields.is_empty() {
                    break;
                }
                continue;
            }

            if owner_class != 0 && owner_class != class_ptr {
                invalid_streak += 1;
                if invalid_streak >= 8 && !fields.is_empty() {
                    break;
                }
                continue;
            }

            fields.push(RuntimeField { name, offset });
            invalid_streak = 0;
        }

        Ok(fields)
    }

    pub fn field_name_matches(actual: &str, requested: &str) -> bool {
        let actual_lower = actual.to_ascii_lowercase();
        let requested_lower = requested.to_ascii_lowercase();
        actual_lower == requested_lower
            || Self::normalize_field_name(actual) == Self::normalize_field_name(requested)
    }

    pub fn normalize_field_name(name: &str) -> String {
        let mut value = name.trim();
        if let Some(stripped) = value
            .strip_prefix('<')
            .and_then(|s| s.strip_suffix(">k__BackingField"))
        {
            value = stripped;
        }
        value.trim_start_matches('_').to_ascii_lowercase()
    }

    pub fn looks_like_runtime_field_name(name: &str) -> bool {
        !name.is_empty()
            && name.len() <= 120
            && name
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'<' | b'>' | b'`' | b'.'))
            && name.bytes().any(|b| b.is_ascii_alphabetic())
    }
}

impl RuntimeReaderContext for RuntimeIntrospector {
    fn process_memory(&mut self) -> &mut ProcessMemory {
        &mut self.memory
    }

    fn runtime_fields_for_object(&mut self, obj_ptr: u64) -> Result<Vec<RuntimeField>> {
        self.runtime_fields_for_object_cached(obj_ptr)
    }

    fn field_name_matches(actual: &str, requested: &str) -> bool {
        Self::field_name_matches(actual, requested)
    }
}
