use crate::memory::ProcessMemory;
use crate::singleton::SingletonResolver;
use anyhow::{anyhow, Result};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

// ---------------------------------------------------------------------------
// IL2CPP Metadata Registration (runtime ABI struct)
// ---------------------------------------------------------------------------

#[repr(C)]
#[derive(Debug, Clone, Copy)]
#[allow(non_snake_case)]
struct Il2CppMetadataRegistration {
    pub genericClassesCount: i32,
    pub genericClasses: u64,
    pub genericInstsCount: i32,
    pub genericInsts: u64,
    pub genericMethodTableCount: i32,
    pub genericMethodTable: u64,
    pub typesCount: i32,
    pub types: u64,
    pub methodSpecsCount: i32,
    pub methodSpecs: u64,
    pub fieldOffsetsCount: i32,
    pub fieldOffsets: u64,
    pub typeDefinitionsSizesCount: i32,
    pub typeDefinitionsSizes: u64,
}

// ---------------------------------------------------------------------------
// global-metadata.dat parsing (minimal — type/field names only)
// ---------------------------------------------------------------------------

const TYPE_DEFINITION_SIZE: usize = 88;

const HEADER_STRING_OFFSET_OFFSET: usize = 24;
const HEADER_STRING_SIZE_OFFSET: usize = 28;
const HEADER_FIELDS_OFFSET_OFFSET: usize = 96;
const HEADER_FIELDS_SIZE_OFFSET: usize = 100;
const HEADER_TYPE_DEFS_OFFSET_OFFSET: usize = 160;
const HEADER_TYPE_DEFS_SIZE_OFFSET: usize = 164;

#[derive(Debug, Clone)]
struct GlobalMetadataHeaderLite {
    pub string_offset: usize,
    pub string_size: usize,
    pub _fields_offset: usize,
    pub _fields_size: usize,
    pub type_definitions_offset: usize,
    pub type_definitions_size: usize,
}

#[derive(Debug, Clone)]
struct TypeDefinitionLite {
    pub name_index: i32,
    pub namespace_index: i32,
    pub byval_type_index: i32,
}

#[derive(Debug, Clone)]
struct MinimalMetadata {
    pub type_defs: Vec<TypeDefinitionLite>,
    pub strings: HashMap<i32, String>,
}

impl MinimalMetadata {
    fn resolve_string(&self, idx: i32) -> Option<&str> {
        self.strings.get(&idx).map(|s| s.as_str())
    }

    fn find_type_def_index(&self, namespace: &str, class_name: &str) -> Option<usize> {
        self.type_defs.iter().position(|td| {
            self.resolve_string(td.namespace_index) == Some(namespace)
                && self.resolve_string(td.name_index) == Some(class_name)
        })
    }
}

fn parse_header(data: &[u8]) -> Result<GlobalMetadataHeaderLite> {
    if data.len() < HEADER_TYPE_DEFS_SIZE_OFFSET + 4 {
        return Err(anyhow!("Metadata file too small"));
    }
    let sanity = u32::from_le_bytes(data[0..4].try_into().unwrap());
    if sanity != 0xFAB11BAF {
        return Err(anyhow!("Invalid metadata header sanity"));
    }
    let version = i32::from_le_bytes(data[4..8].try_into().unwrap());
    if !(16..=31).contains(&version) {
        return Err(anyhow!("Unsupported metadata version: {}", version));
    }
    let read_usize = |off: usize| -> usize {
        i32::from_le_bytes(data[off..off + 4].try_into().unwrap()) as usize
    };
    Ok(GlobalMetadataHeaderLite {
        string_offset: read_usize(HEADER_STRING_OFFSET_OFFSET),
        string_size: read_usize(HEADER_STRING_SIZE_OFFSET),
        _fields_offset: read_usize(HEADER_FIELDS_OFFSET_OFFSET),
        _fields_size: read_usize(HEADER_FIELDS_SIZE_OFFSET),
        type_definitions_offset: read_usize(HEADER_TYPE_DEFS_OFFSET_OFFSET),
        type_definitions_size: read_usize(HEADER_TYPE_DEFS_SIZE_OFFSET),
    })
}

fn parse_strings(data: &[u8], header: &GlobalMetadataHeaderLite) -> Result<HashMap<i32, String>> {
    if header.string_offset.saturating_add(header.string_size) > data.len() {
        return Err(anyhow!("String table out of bounds"));
    }
    let mut strings = HashMap::new();
    let blob = &data[header.string_offset..(header.string_offset + header.string_size)];
    let mut i = 0usize;
    while i < blob.len() {
        let end = blob[i..]
            .iter()
            .position(|&b| b == 0)
            .map(|p| i + p)
            .unwrap_or(blob.len());
        let s = String::from_utf8_lossy(&blob[i..end]).to_string();
        strings.insert(i as i32, s);
        i = end.saturating_add(1);
    }
    Ok(strings)
}

fn parse_minimal_metadata(metadata_path: &Path) -> Result<MinimalMetadata> {
    let data = std::fs::read(metadata_path)?;
    let header = parse_header(&data)?;
    if header.type_definitions_size == 0 || header.type_definitions_size % TYPE_DEFINITION_SIZE != 0
    {
        return Err(anyhow!(
            "Invalid typeDefinitionsSize: {}",
            header.type_definitions_size
        ));
    }
    if header
        .type_definitions_offset
        .saturating_add(header.type_definitions_size)
        > data.len()
    {
        return Err(anyhow!("Type definitions table out of bounds"));
    }
    let strings = parse_strings(&data, &header)?;
    let count = header.type_definitions_size / TYPE_DEFINITION_SIZE;
    let mut type_defs = Vec::with_capacity(count);
    for n in 0..count {
        let base = header.type_definitions_offset + n * TYPE_DEFINITION_SIZE;
        let read_i32 =
            |off: usize| i32::from_le_bytes(data[base + off..base + off + 4].try_into().unwrap());
        type_defs.push(TypeDefinitionLite {
            name_index: read_i32(0),
            namespace_index: read_i32(4),
            byval_type_index: read_i32(8),
        });
    }
    Ok(MinimalMetadata { type_defs, strings })
}

fn get_type_definition_count(metadata_path: &Path) -> Result<i32> {
    let data = std::fs::read(metadata_path)?;
    let header = parse_header(&data)?;
    if header.type_definitions_size == 0 || header.type_definitions_size % TYPE_DEFINITION_SIZE != 0
    {
        return Err(anyhow!(
            "Invalid typeDefinitionsSize: {}",
            header.type_definitions_size
        ));
    }
    let count = (header.type_definitions_size / TYPE_DEFINITION_SIZE) as i32;
    if count < 100 || count > 100_000 {
        return Err(anyhow!("Type definition count out of range: {}", count));
    }
    Ok(count)
}

fn derive_metadata_path(game_assembly_path: &str) -> Option<PathBuf> {
    let ga = Path::new(game_assembly_path);
    let parent = ga.parent()?;
    let dir_name = parent.file_name()?.to_string_lossy();
    let data_dir = format!("{}_Data/il2cpp_data/Metadata/global-metadata.dat", dir_name);
    Some(parent.join(data_dir))
}

// ---------------------------------------------------------------------------
// Public types
// ---------------------------------------------------------------------------

pub struct Il2CppMetadata {
    pub registration_addr: u64,
    registration: Il2CppMetadataRegistration,
    module_base: u64,
    module_size: u64,
    pub metadata_path: Option<PathBuf>,
}

#[derive(Debug, Clone)]
pub struct SingletonCandidate {
    pub namespace: String,
    pub class_name: String,
    pub full_name: String,
    pub generic_class_index: i32,
    pub singleton_class_ptr: u64,
    pub instance_ptr: Option<u64>,
}

// ---------------------------------------------------------------------------
// Full metadata types (for --all-types dump)
// ---------------------------------------------------------------------------

pub const FIELD_DEFINITION_SIZE: usize = 12;
pub const IMAGE_DEFINITION_SIZE: usize = 40;

const HEADER_IMAGES_OFFSET_OFFSET: usize = 168;
const HEADER_IMAGES_SIZE_OFFSET: usize = 172;

#[derive(Debug, Clone)]
pub struct GlobalMetadataHeader {
    pub version: i32,
    pub string_offset: usize,
    pub string_size: usize,
    pub fields_offset: usize,
    pub fields_size: usize,
    pub type_definitions_offset: usize,
    pub type_definitions_size: usize,
    pub images_offset: usize,
    pub images_size: usize,
}

#[derive(Debug, Clone)]
pub struct TypeDefinition {
    pub name_index: i32,
    pub namespace_index: i32,
    pub byval_type_index: i32,
    pub byref_type_index: i32,
    pub declaring_type_index: i32,
    pub parent_index: i32,
    pub element_type_index: i32,
    pub flags: u32,
    pub field_start: i32,
    pub method_start: i32,
    pub event_start: i32,
    pub property_start: i32,
    pub nested_types_start: i32,
    pub interfaces_start: i32,
    pub vtable_start: i32,
    pub interface_offsets_start: i32,
    pub method_count: u16,
    pub property_count: u16,
    pub field_count: u16,
    pub event_count: u16,
    pub nested_type_count: u16,
    pub vtable_count: u16,
    pub interfaces_count: u16,
    pub interface_offsets_count: u16,
    pub bitfield: u32,
    pub token: u32,
}

impl TypeDefinition {
    pub fn is_value_type(&self) -> bool {
        (self.bitfield & 0x1) == 1
    }

    pub fn is_enum(&self) -> bool {
        ((self.bitfield >> 1) & 0x1) == 1
    }
}

#[derive(Debug, Clone)]
pub struct FieldDefinition {
    pub name_index: u32,
    pub type_index: i32,
    pub token: u32,
}

#[derive(Debug, Clone)]
pub struct ImageDefinition {
    pub name_index: u32,
    pub assembly_index: i32,
    pub type_start: i32,
    pub type_count: u32,
    pub exported_type_start: i32,
    pub exported_type_count: u32,
    pub entry_point_index: i32,
    pub token: u32,
    pub custom_attribute_start: i32,
    pub custom_attribute_count: u32,
}

#[derive(Debug, Clone)]
pub struct FullMetadata {
    pub header: GlobalMetadataHeader,
    pub type_defs: Vec<TypeDefinition>,
    pub field_defs: Vec<FieldDefinition>,
    pub image_defs: Vec<ImageDefinition>,
    pub strings: HashMap<i32, String>,
}

impl FullMetadata {
    pub fn resolve_string(&self, idx: i32) -> Option<&str> {
        self.strings.get(&idx).map(|s| s.as_str())
    }

    pub fn resolve_string_u32(&self, idx: u32) -> Option<&str> {
        self.resolve_string(idx as i32)
    }

    pub fn type_full_name(&self, type_index: usize) -> Option<String> {
        let type_def = self.type_defs.get(type_index)?;
        let class_name = self.resolve_string(type_def.name_index)?;
        let namespace = self
            .resolve_string(type_def.namespace_index)
            .unwrap_or_default();

        if type_def.declaring_type_index >= 0 {
            let declaring_name = self.type_full_name(type_def.declaring_type_index as usize)?;
            return Some(format!("{}+{}", declaring_name, class_name));
        }

        if namespace.is_empty() {
            Some(class_name.to_string())
        } else {
            Some(format!("{}::{}", namespace, class_name))
        }
    }

    pub fn find_image_for_type(&self, type_index: usize) -> Option<&ImageDefinition> {
        self.image_defs.iter().find(|image| {
            if image.type_start < 0 {
                return false;
            }
            let start = image.type_start as usize;
            let end = start.saturating_add(image.type_count as usize);
            type_index >= start && type_index < end
        })
    }
}

// ---------------------------------------------------------------------------
// Il2CppMetadata — platform implementations of find_in_process
// ---------------------------------------------------------------------------

impl Il2CppMetadata {
    #[cfg(unix)]
    pub fn find_in_process(memory: &mut ProcessMemory, pid: u32) -> Result<Self> {
        use crate::process::list_memory_regions;

        let maps = list_memory_regions(pid)?;

        let mut module_base = 0u64;
        let mut module_end = 0u64;
        let mut all_regions: Vec<(u64, u64, &str)> = Vec::new();
        let mut game_assembly_path: Option<String> = None;

        for region in &maps {
            if let Some(ref pathname) = region.pathname {
                if !pathname.contains("GameAssembly.dll") {
                    continue;
                }
                let start = region.start;
                let size = region.end - region.start;
                let end = region.end;

                if module_base == 0 || start < module_base {
                    module_base = start;
                }
                if end > module_end {
                    module_end = end;
                }
                if game_assembly_path.is_none() {
                    game_assembly_path = Some(pathname.clone());
                }

                let kind = if region.executable {
                    "exec"
                } else if region.writable {
                    "data"
                } else {
                    "rdata"
                };
                all_regions.push((start, size, kind));
            }
        }

        if module_base == 0 {
            return Err(anyhow!("Could not find GameAssembly.dll in process maps"));
        }

        let metadata_path = game_assembly_path
            .as_ref()
            .and_then(|p| derive_metadata_path(p));
        let exact_type_count = metadata_path.as_ref().and_then(|mp| {
            if mp.exists() {
                get_type_definition_count(mp).ok()
            } else {
                None
            }
        });
        let module_size = module_end - module_base;

        for (start, size, _kind) in &all_regions {
            if let Ok(meta) = Self::scan_region(
                memory,
                *start,
                *size,
                module_base,
                module_end,
                module_size,
                exact_type_count,
                metadata_path.clone(),
            ) {
                return Ok(meta);
            }
        }

        Err(anyhow!(
            "Il2CppMetadataRegistration not found in GameAssembly.dll"
        ))
    }

    #[cfg(windows)]
    pub fn find_in_process(memory: &mut ProcessMemory, pid: u32) -> Result<Self> {
        use windows::Win32::System::Diagnostics::ToolHelp::{
            CreateToolhelp32Snapshot, Module32FirstW, Module32NextW, MODULEENTRY32W,
            TH32CS_SNAPMODULE,
        };
        use windows::Win32::System::Memory::{
            VirtualQueryEx, MEMORY_BASIC_INFORMATION, MEM_COMMIT, PAGE_EXECUTE_READ,
            PAGE_EXECUTE_READWRITE, PAGE_READONLY, PAGE_READWRITE,
        };

        let mut metadata_path: Option<PathBuf> = None;
        let mut exact_type_count: Option<i32> = None;
        let mut game_assembly_bounds: Option<(u64, u64)> = None;

        unsafe {
            let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid)?;
            let mut entry = MODULEENTRY32W::default();
            entry.dwSize = std::mem::size_of::<MODULEENTRY32W>() as u32;

            if Module32FirstW(snapshot, &mut entry).is_ok() {
                loop {
                    let module_name = String::from_utf16_lossy(
                        &entry
                            .szModule
                            .iter()
                            .take_while(|&&c| c != 0)
                            .copied()
                            .collect::<Vec<_>>(),
                    );
                    if module_name.eq_ignore_ascii_case("GameAssembly.dll") {
                        let module_base = entry.modBaseAddr as u64;
                        let module_size = entry.modBaseSize as u64;
                        let module_end = module_base.saturating_add(module_size);
                        let module_path = String::from_utf16_lossy(
                            &entry
                                .szExePath
                                .iter()
                                .take_while(|&&c| c != 0)
                                .copied()
                                .collect::<Vec<_>>(),
                        );
                        game_assembly_bounds = Some((module_base, module_end));
                        metadata_path = derive_metadata_path(&module_path);
                        if let Some(ref mp) = metadata_path {
                            if mp.exists() {
                                exact_type_count = get_type_definition_count(mp).ok();
                            }
                        }
                        break;
                    }
                    if Module32NextW(snapshot, &mut entry).is_err() {
                        break;
                    }
                }
            }
        }

        let (module_base, module_end) = game_assembly_bounds
            .ok_or_else(|| anyhow!("Could not find GameAssembly.dll in process modules"))?;

        let process_handle = memory.get_process_handle();
        let regions = unsafe {
            let mut address: u64 = 0;
            let mut regions: Vec<(u64, u64, &'static str)> = Vec::new();
            loop {
                let mut mbi = MEMORY_BASIC_INFORMATION::default();
                let result = VirtualQueryEx(
                    process_handle,
                    Some(address as *const _),
                    &mut mbi,
                    std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
                );
                if result == 0 {
                    break;
                }
                let region_start = mbi.BaseAddress as u64;
                let region_size = mbi.RegionSize as u64;
                let region_end = region_start.saturating_add(region_size);
                let overlaps = region_end > module_base && region_start < module_end;
                if mbi.State == MEM_COMMIT
                    && overlaps
                    && (mbi.Protect == PAGE_READWRITE
                        || mbi.Protect == PAGE_READONLY
                        || mbi.Protect == PAGE_EXECUTE_READWRITE
                        || mbi.Protect == PAGE_EXECUTE_READ)
                {
                    let kind = if mbi.Protect == PAGE_EXECUTE_READ
                        || mbi.Protect == PAGE_EXECUTE_READWRITE
                    {
                        "exec"
                    } else if mbi.Protect == PAGE_READWRITE {
                        "data"
                    } else {
                        "rdata"
                    };
                    regions.push((region_start, region_size, kind));
                }
                address = region_start + region_size;
            }
            regions.sort_by_key(|(s, _, _)| *s);
            regions
        };

        let module_size = module_end.saturating_sub(module_base);
        for (start, size, _kind) in &regions {
            if let Ok(meta) = Self::scan_region(
                memory,
                *start,
                *size,
                module_base,
                module_end,
                module_size,
                exact_type_count,
                metadata_path.clone(),
            ) {
                return Ok(meta);
            }
        }

        Err(anyhow!(
            "Il2CppMetadataRegistration not found. Run as Administrator."
        ))
    }

    // -----------------------------------------------------------------------
    // Singleton discovery
    // -----------------------------------------------------------------------

    /// Discover all `Singleton`1<T>` root candidates (Gallop namespace).
    pub fn discover_singleton_candidates(
        &self,
        memory: &mut ProcessMemory,
    ) -> Result<Vec<SingletonCandidate>> {
        let metadata_path = self
            .metadata_path
            .as_ref()
            .ok_or_else(|| anyhow!("Metadata path unavailable"))?;
        let parsed = parse_minimal_metadata(metadata_path)?;
        let typedef = parsed
            .find_type_def_index("Gallop", "Singleton`1")
            .ok_or_else(|| anyhow!("Gallop::Singleton`1 not found"))?;
        self.discover_candidates_for_singleton_typedef(&parsed, memory, typedef)
    }

    /// Discover all `MonoSingleton`1<T>` root candidates (any namespace).
    pub fn discover_mono_singleton_candidates(
        &self,
        memory: &mut ProcessMemory,
    ) -> Result<Vec<SingletonCandidate>> {
        let metadata_path = self
            .metadata_path
            .as_ref()
            .ok_or_else(|| anyhow!("Metadata path unavailable"))?;
        let parsed = parse_minimal_metadata(metadata_path)?;
        let typedef = parsed
            .type_defs
            .iter()
            .position(|td| parsed.resolve_string(td.name_index) == Some("MonoSingleton`1"))
            .ok_or_else(|| anyhow!("MonoSingleton`1 not found"))?;
        self.discover_candidates_for_singleton_typedef(&parsed, memory, typedef)
    }

    /// Get the field offsets for a type definition by index.
    pub fn get_field_offsets_for_type(
        &self,
        memory: &mut ProcessMemory,
        type_def_index: usize,
        field_start: usize,
        field_count: usize,
    ) -> Result<Vec<Option<i32>>> {
        if field_count == 0 {
            return Ok(Vec::new());
        }

        let table_abs = self.field_offsets_table_abs(memory)?;
        let pointer_mode = self.field_offsets_are_pointers(memory)?;
        let mut result = Vec::with_capacity(field_count);

        if pointer_mode {
            if type_def_index >= self.registration.fieldOffsetsCount as usize {
                return Ok(vec![None; field_count]);
            }

            let ptr_val = memory.read_pointer(table_abs + (type_def_index as u64) * 8)?;
            if ptr_val == 0 {
                return Ok(vec![None; field_count]);
            }

            let ptr_abs = match self.resolve_ptr(memory, ptr_val) {
                Some(ptr) => ptr,
                None => return Ok(vec![None; field_count]),
            };

            for idx in 0..field_count {
                let offset = memory.read_i32(ptr_abs + (idx as u64) * 4).ok();
                result.push(offset);
            }
            return Ok(result);
        }

        for idx in 0..field_count {
            let global_field_index = field_start.saturating_add(idx);
            let offset = memory
                .read_i32(table_abs + (global_field_index as u64) * 4)
                .ok();
            result.push(offset);
        }

        Ok(result)
    }

    // -----------------------------------------------------------------------
    // Private helpers
    // -----------------------------------------------------------------------

    fn discover_candidates_for_singleton_typedef(
        &self,
        parsed: &MinimalMetadata,
        memory: &mut ProcessMemory,
        singleton_typedef: usize,
    ) -> Result<Vec<SingletonCandidate>> {
        let types_count = self.registration.typesCount;
        if types_count <= 0 || types_count > 500_000 {
            return Err(anyhow!("Invalid typesCount: {}", types_count));
        }

        let types_abs = self
            .resolve_ptr(memory, self.registration.types)
            .ok_or_else(|| anyhow!("types pointer not readable"))?;

        let singleton_type_ptr = self.read_runtime_type_ptr(
            memory,
            parsed.type_defs[singleton_typedef].byval_type_index,
            types_abs,
            types_count,
        )?;

        if self.registration.genericClassesCount <= 0
            || self.registration.genericClassesCount > 1_000_000
        {
            return Err(anyhow!(
                "Invalid genericClassesCount: {}",
                self.registration.genericClassesCount
            ));
        }

        let generic_classes_abs = self
            .resolve_ptr(memory, self.registration.genericClasses)
            .ok_or_else(|| anyhow!("genericClasses pointer not readable"))?;

        // Build name lookup by runtime type pointer
        let mut type_name_by_runtime_ptr: HashMap<u64, (String, String, String)> = HashMap::new();
        for type_def in &parsed.type_defs {
            if type_def.byval_type_index < 0 || type_def.byval_type_index >= types_count {
                continue;
            }
            let runtime_type_ptr = match self.read_runtime_type_ptr(
                memory,
                type_def.byval_type_index,
                types_abs,
                types_count,
            ) {
                Ok(ptr) => ptr,
                Err(_) => continue,
            };
            let class_name = parsed
                .resolve_string(type_def.name_index)
                .unwrap_or("<unknown>")
                .to_string();
            let namespace = parsed
                .resolve_string(type_def.namespace_index)
                .unwrap_or("")
                .to_string();
            let full_name = if namespace.is_empty() {
                class_name.clone()
            } else {
                format!("{}::{}", namespace, class_name)
            };
            type_name_by_runtime_ptr
                .entry(runtime_type_ptr)
                .or_insert((namespace, class_name, full_name));
        }

        let mut unique: HashMap<String, SingletonCandidate> = HashMap::new();

        for seq in 0..self.registration.genericClassesCount {
            let gc_ptr_val = match memory.read_pointer(generic_classes_abs + (seq as u64) * 8) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if gc_ptr_val == 0 {
                continue;
            }

            let gc_abs = match self.resolve_ptr(memory, gc_ptr_val) {
                Some(p) => p,
                None => continue,
            };

            // RuntimeIl2CppGenericClass: +0 type*, +8 context.class_inst*, +24 cached_class*
            let gc_type_ptr_val = match memory.read_pointer(gc_abs) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let gc_type_abs = match self.resolve_ptr(memory, gc_type_ptr_val) {
                Some(p) => p,
                None => continue,
            };
            if gc_type_abs != singleton_type_ptr {
                continue;
            }

            let class_inst_ptr_val = match memory.read_pointer(gc_abs + 8) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if class_inst_ptr_val == 0 {
                continue;
            }
            let class_inst_abs = match self.resolve_ptr(memory, class_inst_ptr_val) {
                Some(p) => p,
                None => continue,
            };

            // RuntimeIl2CppGenericInst: +0 type_argc(u32), +8 type_argv**
            let type_argc = match memory.read_i32(class_inst_abs) {
                Ok(v) => v,
                Err(_) => continue,
            };
            if type_argc < 1 {
                continue;
            }

            let type_argv_ptr_val = match memory.read_pointer(class_inst_abs + 8) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let type_argv_abs = match self.resolve_ptr(memory, type_argv_ptr_val) {
                Some(p) => p,
                None => continue,
            };
            let arg0_type_ptr_val = match memory.read_pointer(type_argv_abs) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let arg0_type_abs = match self.resolve_ptr(memory, arg0_type_ptr_val) {
                Some(p) => p,
                None => continue,
            };

            let (namespace, class_name, full_name) = type_name_by_runtime_ptr
                .get(&arg0_type_abs)
                .cloned()
                .unwrap_or_else(|| {
                    let fallback = format!("<unknown@{:#x}>", arg0_type_abs);
                    (String::new(), fallback.clone(), fallback)
                });

            let cached_class_ptr_val = match memory.read_pointer(gc_abs + 24) {
                Ok(v) => v,
                Err(_) => continue,
            };
            let cached_class_abs = match self.resolve_ptr(memory, cached_class_ptr_val) {
                Some(p) => p,
                None => continue,
            };

            let instance_ptr = self.try_resolve_singleton_instance(memory, cached_class_abs);

            let candidate = SingletonCandidate {
                namespace,
                class_name,
                full_name: full_name.clone(),
                generic_class_index: seq,
                singleton_class_ptr: cached_class_abs,
                instance_ptr,
            };

            match unique.get_mut(&full_name) {
                Some(existing) => {
                    if existing.instance_ptr.is_none() && candidate.instance_ptr.is_some() {
                        *existing = candidate;
                    }
                }
                None => {
                    unique.insert(full_name, candidate);
                }
            }
        }

        let mut result: Vec<_> = unique.into_values().collect();
        result.sort_by(|a, b| a.full_name.cmp(&b.full_name));
        Ok(result)
    }

    pub fn resolve_singleton_by_class_name(
        &self,
        memory: &mut ProcessMemory,
        namespace: &str,
        class_name: &str,
    ) -> Result<u64> {
        let metadata_path = self
            .metadata_path
            .as_ref()
            .ok_or_else(|| anyhow!("Metadata path unavailable"))?;
        let parsed = parse_minimal_metadata(metadata_path)?;

        let type_idx = parsed
            .find_type_def_index(namespace, class_name)
            .ok_or_else(|| {
                anyhow!(
                    "{}::{} not found in global-metadata.dat",
                    namespace,
                    class_name
                )
            })?;

        let types_count = self.registration.typesCount;
        if types_count <= 0 || types_count > 500_000 {
            return Err(anyhow!("Invalid typesCount: {}", types_count));
        }

        let types_abs = self
            .resolve_ptr(memory, self.registration.types)
            .ok_or_else(|| anyhow!("types pointer not readable"))?;

        let runtime_type_ptr = self.read_runtime_type_ptr(
            memory,
            parsed.type_defs[type_idx].byval_type_index,
            types_abs,
            types_count,
        )?;

        // Il2CppType on 64-bit: +0 = data/klass union (8 bytes)
        let class_ptr_val = memory.read_pointer(runtime_type_ptr)?;
        let class_abs = self
            .resolve_ptr(memory, class_ptr_val)
            .ok_or_else(|| anyhow!("Class pointer {:#x} not readable", class_ptr_val))?;

        SingletonResolver::resolve_singleton_instance(memory, class_abs, |mem, ptr| {
            self.resolve_ptr(mem, ptr)
        })
    }

    fn read_runtime_type_ptr(
        &self,
        memory: &mut ProcessMemory,
        byval_type_index: i32,
        types_abs: u64,
        types_count: i32,
    ) -> Result<u64> {
        if byval_type_index < 0 || byval_type_index >= types_count {
            return Err(anyhow!("Invalid byvalTypeIndex {}", byval_type_index));
        }
        let ptr_val = memory.read_pointer(types_abs + (byval_type_index as u64) * 8)?;
        self.resolve_ptr(memory, ptr_val)
            .ok_or_else(|| anyhow!("Runtime type pointer {:#x} not readable", ptr_val))
    }

    fn try_resolve_singleton_instance(
        &self,
        memory: &mut ProcessMemory,
        singleton_class_ptr: u64,
    ) -> Option<u64> {
        const IL2CPP_CLASS_STATIC_FIELDS_OFFSET: u64 = 184;
        const IL2CPP_CLASS_PARENT_OFFSET: u64 = 88;
        const IL2CPP_CLASS_FIELDS_PTR_OFFSET: u64 = 128;
        const FIELD_INFO_NAME_PTR_OFFSET: u64 = 0;
        const FIELD_INFO_OFFSET_OFFSET: u64 = 24;
        const FIELD_INFO_STRIDE: u64 = 32;

        let static_fields_ptr = memory
            .read_pointer(singleton_class_ptr.wrapping_add(IL2CPP_CLASS_STATIC_FIELDS_OFFSET))
            .ok()?;
        let static_fields_abs = self.resolve_ptr(memory, static_fields_ptr)?;

        // Try to find _instance field offset
        let instance_field_offset = {
            let mut found_offset: Option<u64> = None;
            let mut current = singleton_class_ptr;
            let mut depth = 0usize;
            'outer: while current != 0 && depth < 16 {
                depth += 1;
                if let Ok(fields_ptr) =
                    memory.read_pointer(current.wrapping_add(IL2CPP_CLASS_FIELDS_PTR_OFFSET))
                {
                    if fields_ptr != 0 {
                        let mut invalid_streak = 0usize;
                        for i in 0usize..256 {
                            let base = fields_ptr.wrapping_add((i as u64) * FIELD_INFO_STRIDE);
                            let name_ptr =
                                match memory.read_pointer(base.wrapping_add(FIELD_INFO_NAME_PTR_OFFSET)) {
                                    Ok(v) => v,
                                    Err(_) => {
                                        invalid_streak += 1;
                                        if invalid_streak >= 8 {
                                            break;
                                        }
                                        continue;
                                    }
                                };
                            let field_offset =
                                match memory.read_i32(base.wrapping_add(FIELD_INFO_OFFSET_OFFSET)) {
                                    Ok(v) => v,
                                    Err(_) => {
                                        invalid_streak += 1;
                                        if invalid_streak >= 8 {
                                            break;
                                        }
                                        continue;
                                    }
                                };
                            let Some(name) = Self::read_ascii_c_string(memory, name_ptr, 128)
                            else {
                                invalid_streak += 1;
                                if invalid_streak >= 8 {
                                    break;
                                }
                                continue;
                            };
                            invalid_streak = 0;
                            if field_offset >= 0 && Self::looks_like_singleton_instance_field(&name)
                            {
                                found_offset = Some(field_offset as u64);
                                break 'outer;
                            }
                        }
                    }
                }
                match memory.read_pointer(current.wrapping_add(IL2CPP_CLASS_PARENT_OFFSET)) {
                    Ok(0) | Err(_) => break,
                    Ok(parent) => {
                        current = self.resolve_ptr(memory, parent).unwrap_or(parent);
                    }
                }
            }
            found_offset
        };

        let try_read_valid = |memory: &mut ProcessMemory, ptr_addr: u64| -> Option<u64> {
            let instance = memory.read_pointer(ptr_addr).ok()?;
            let instance_abs = self.resolve_ptr(memory, instance)?;
            let klass = memory.read_pointer(instance_abs).ok()?;
            if klass == 0 {
                return None;
            }
            let module_end = self.module_base + self.module_size;
            if !Self::in_module(klass, self.module_base, module_end)
                && !Self::is_valid_pointer(memory, klass)
            {
                return None;
            }
            Some(instance_abs)
        };

        if let Some(offset) = instance_field_offset {
            if let Some(ptr) = try_read_valid(memory, static_fields_abs + offset) {
                return Some(ptr);
            }
        }
        try_read_valid(memory, static_fields_abs)
    }

    fn looks_like_singleton_instance_field(name: &str) -> bool {
        let n = name
            .trim()
            .trim_start_matches('_')
            .trim_start_matches('m')
            .trim_start_matches('_')
            .to_ascii_lowercase();
        n == "instance" || n == "s_instance"
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

    fn scan_region(
        memory: &mut ProcessMemory,
        region_start: u64,
        region_size: u64,
        module_base: u64,
        module_end: u64,
        module_size: u64,
        exact_type_count: Option<i32>,
        metadata_path: Option<PathBuf>,
    ) -> Result<Self> {
        let chunk_size = 16 * 1024 * 1024usize;
        let mut offset = 0u64;

        while offset < region_size {
            let chunk_len = (chunk_size as u64).min(region_size - offset) as usize;
            let chunk = match memory.read_bytes(region_start + offset, chunk_len) {
                Ok(c) => c,
                Err(_) => break,
            };

            for i in 0..(chunk_len.saturating_sub(32)) {
                let count1 = i32::from_le_bytes(chunk[i..i + 4].try_into().unwrap());
                let count2 = i32::from_le_bytes(chunk[i + 16..i + 20].try_into().unwrap());

                if let Some(exact) = exact_type_count {
                    if count1 != exact || count2 != exact {
                        continue;
                    }
                } else {
                    if count1 != count2 || count1 < 1000 || count1 > 50_000 {
                        continue;
                    }
                }

                let pad1 = u32::from_le_bytes(chunk[i + 4..i + 8].try_into().unwrap());
                let pad2 = u32::from_le_bytes(chunk[i + 20..i + 24].try_into().unwrap());
                if pad1 != 0 || pad2 != 0 {
                    continue;
                }

                let ptr1 = u64::from_le_bytes(chunk[i + 8..i + 16].try_into().unwrap());
                let ptr2 = u64::from_le_bytes(chunk[i + 24..i + 32].try_into().unwrap());
                if ptr1 < 0x10000 || ptr2 < 0x10000 {
                    continue;
                }

                let found_at = region_start + offset + i as u64;
                let registration_addr = match found_at.checked_sub(80) {
                    Some(a) => a,
                    None => continue,
                };

                let reg_bytes = match memory.read_bytes(
                    registration_addr,
                    std::mem::size_of::<Il2CppMetadataRegistration>(),
                ) {
                    Ok(b) => b,
                    Err(_) => continue,
                };
                let registration: Il2CppMetadataRegistration =
                    unsafe { std::ptr::read_unaligned(reg_bytes.as_ptr() as *const _) };

                if Self::validate_registration(
                    memory,
                    &registration,
                    module_base,
                    module_end,
                    exact_type_count,
                ) {
                    return Ok(Il2CppMetadata {
                        registration_addr: found_at,
                        registration,
                        module_base,
                        module_size,
                        metadata_path,
                    });
                }
            }
            offset += chunk_len as u64;
        }
        Err(anyhow!("Not found in this region"))
    }

    fn validate_registration(
        memory: &mut ProcessMemory,
        reg: &Il2CppMetadataRegistration,
        module_base: u64,
        module_end: u64,
        exact_type_count: Option<i32>,
    ) -> bool {
        if reg.typesCount <= 0 || reg.typesCount > 500_000 {
            return false;
        }
        if reg.genericClassesCount < 0 || reg.genericClassesCount > 500_000 {
            return false;
        }

        let pointer_table_count = if let Some(exact) = exact_type_count {
            if reg.fieldOffsetsCount != exact || reg.typeDefinitionsSizesCount != exact {
                return false;
            }
            exact
        } else {
            if reg.fieldOffsetsCount <= 0 || reg.fieldOffsetsCount > 500_000 {
                return false;
            }
            if reg.typeDefinitionsSizesCount != reg.fieldOffsetsCount {
                return false;
            }
            reg.typeDefinitionsSizesCount
        };

        let table_ptr =
            match Self::resolve_module_ptr(reg.typeDefinitionsSizes, module_base, module_end) {
                Some(p) => p,
                None => return false,
            };
        let table_len = (pointer_table_count as usize).saturating_mul(8);
        let table = match memory.read_bytes(table_ptr, table_len) {
            Ok(b) => b,
            Err(_) => return false,
        };
        for chunk in table.chunks_exact(8) {
            let entry = u64::from_le_bytes(chunk.try_into().unwrap());
            if !Self::in_module(entry, module_base, module_end) {
                return false;
            }
        }
        for ptr in [reg.genericClasses, reg.types, reg.fieldOffsets] {
            let resolved =
                match Self::resolve_module_ptr_or_readable(memory, ptr, module_base, module_end) {
                    Some(p) => p,
                    None => return false,
                };
            if !Self::is_valid_pointer(memory, resolved) {
                return false;
            }
        }
        true
    }

    pub fn resolve_ptr(&self, memory: &mut ProcessMemory, ptr_val: u64) -> Option<u64> {
        if ptr_val >= 0x10000 && Self::is_valid_pointer(memory, ptr_val) {
            return Some(ptr_val);
        }
        if let Some(rva) = self.module_base.checked_add(ptr_val) {
            if rva >= 0x10000 && Self::is_valid_pointer(memory, rva) {
                return Some(rva);
            }
        }
        None
    }

    fn field_offsets_table_abs(&self, memory: &mut ProcessMemory) -> Result<u64> {
        self.resolve_ptr(memory, self.registration.fieldOffsets)
            .ok_or_else(|| {
                anyhow!(
                    "fieldOffsets pointer {:#x} is not readable",
                    self.registration.fieldOffsets
                )
            })
    }

    fn field_offsets_are_pointers(&self, memory: &mut ProcessMemory) -> Result<bool> {
        let table_abs = self.field_offsets_table_abs(memory)?;

        if self.registration.fieldOffsetsCount < 6 {
            return Ok(false);
        }

        let mut probe = [0u32; 6];
        for (idx, slot) in probe.iter_mut().enumerate() {
            *slot = memory.read_i32(table_abs + (idx as u64) * 4)? as u32;
        }

        Ok(probe[0] == 0
            && probe[1] == 0
            && probe[2] == 0
            && probe[3] == 0
            && probe[4] == 0
            && probe[5] > 0)
    }

    fn in_module(ptr: u64, base: u64, end: u64) -> bool {
        ptr >= base && ptr < end
    }

    fn resolve_module_ptr(ptr_val: u64, module_base: u64, module_end: u64) -> Option<u64> {
        if Self::in_module(ptr_val, module_base, module_end) {
            return Some(ptr_val);
        }
        let rva = module_base.checked_add(ptr_val)?;
        if Self::in_module(rva, module_base, module_end) {
            Some(rva)
        } else {
            None
        }
    }

    fn resolve_module_ptr_or_readable(
        memory: &mut ProcessMemory,
        ptr_val: u64,
        module_base: u64,
        module_end: u64,
    ) -> Option<u64> {
        if let Some(p) = Self::resolve_module_ptr(ptr_val, module_base, module_end) {
            return Some(p);
        }
        if ptr_val >= 0x10000 && Self::is_valid_pointer(memory, ptr_val) {
            return Some(ptr_val);
        }
        let rva = module_base.checked_add(ptr_val)?;
        if rva >= 0x10000 && Self::is_valid_pointer(memory, rva) {
            Some(rva)
        } else {
            None
        }
    }

    fn is_valid_pointer(memory: &mut ProcessMemory, ptr: u64) -> bool {
        if ptr == 0 || ptr < 0x10000 {
            return false;
        }
        let mut buf = [0u8; 1];
        memory.read(ptr, &mut buf).is_ok()
    }
}

// ---------------------------------------------------------------------------
// Full metadata parser
// ---------------------------------------------------------------------------

fn parse_full_header(data: &[u8]) -> Result<GlobalMetadataHeader> {
    if data.len() < HEADER_IMAGES_SIZE_OFFSET + 4 {
        return Err(anyhow!("Metadata file too small"));
    }

    let sanity = u32::from_le_bytes(data[0..4].try_into().unwrap());
    if sanity != 0xFAB11BAF {
        return Err(anyhow!("Invalid metadata header sanity"));
    }

    let version = i32::from_le_bytes(data[4..8].try_into().unwrap());
    if !(16..=31).contains(&version) {
        return Err(anyhow!("Unsupported metadata version: {}", version));
    }

    let read_usize = |off: usize| -> usize {
        i32::from_le_bytes(data[off..off + 4].try_into().unwrap()) as usize
    };

    Ok(GlobalMetadataHeader {
        version,
        string_offset: read_usize(HEADER_STRING_OFFSET_OFFSET),
        string_size: read_usize(HEADER_STRING_SIZE_OFFSET),
        fields_offset: read_usize(HEADER_FIELDS_OFFSET_OFFSET),
        fields_size: read_usize(HEADER_FIELDS_SIZE_OFFSET),
        type_definitions_offset: read_usize(HEADER_TYPE_DEFS_OFFSET_OFFSET),
        type_definitions_size: read_usize(HEADER_TYPE_DEFS_SIZE_OFFSET),
        images_offset: read_usize(HEADER_IMAGES_OFFSET_OFFSET),
        images_size: read_usize(HEADER_IMAGES_SIZE_OFFSET),
    })
}

pub fn parse_full_metadata(metadata_path: &Path) -> Result<FullMetadata> {
    let data = std::fs::read(metadata_path)?;
    let header = parse_full_header(&data)?;

    if header.version < 29 {
        return Err(anyhow!(
            "Full metadata parser requires version >= 29 (found {})",
            header.version
        ));
    }

    if header.type_definitions_size == 0 || header.type_definitions_size % TYPE_DEFINITION_SIZE != 0
    {
        return Err(anyhow!(
            "Invalid typeDefinitionsSize: {}",
            header.type_definitions_size
        ));
    }
    if header.fields_size == 0 || header.fields_size % FIELD_DEFINITION_SIZE != 0 {
        return Err(anyhow!("Invalid fieldsSize: {}", header.fields_size));
    }
    if header.images_size == 0 || header.images_size % IMAGE_DEFINITION_SIZE != 0 {
        return Err(anyhow!("Invalid imagesSize: {}", header.images_size));
    }

    for (label, offset, size) in [
        ("fields", header.fields_offset, header.fields_size),
        (
            "type definitions",
            header.type_definitions_offset,
            header.type_definitions_size,
        ),
        ("images", header.images_offset, header.images_size),
    ] {
        if offset.checked_add(size).unwrap_or(usize::MAX) > data.len() {
            return Err(anyhow!("{} table out of bounds", label));
        }
    }

    // Parse string table
    let mut strings = HashMap::new();
    if header.string_offset.saturating_add(header.string_size) <= data.len() {
        let blob = &data[header.string_offset..(header.string_offset + header.string_size)];
        let mut i = 0usize;
        while i < blob.len() {
            let end = blob[i..]
                .iter()
                .position(|&b| b == 0)
                .map(|p| i + p)
                .unwrap_or(blob.len());
            let s = String::from_utf8_lossy(&blob[i..end]).to_string();
            strings.insert(i as i32, s);
            i = end.saturating_add(1);
        }
    }

    let type_count = header.type_definitions_size / TYPE_DEFINITION_SIZE;
    let mut type_defs = Vec::with_capacity(type_count);
    for n in 0..type_count {
        let base = header.type_definitions_offset + n * TYPE_DEFINITION_SIZE;
        type_defs.push(TypeDefinition {
            name_index: i32::from_le_bytes(data[base..base + 4].try_into().unwrap()),
            namespace_index: i32::from_le_bytes(data[base + 4..base + 8].try_into().unwrap()),
            byval_type_index: i32::from_le_bytes(data[base + 8..base + 12].try_into().unwrap()),
            byref_type_index: i32::from_le_bytes(data[base + 12..base + 16].try_into().unwrap()),
            declaring_type_index: i32::from_le_bytes(
                data[base + 16..base + 20].try_into().unwrap(),
            ),
            parent_index: i32::from_le_bytes(data[base + 20..base + 24].try_into().unwrap()),
            element_type_index: i32::from_le_bytes(data[base + 24..base + 28].try_into().unwrap()),
            flags: u32::from_le_bytes(data[base + 28..base + 32].try_into().unwrap()),
            field_start: i32::from_le_bytes(data[base + 32..base + 36].try_into().unwrap()),
            method_start: i32::from_le_bytes(data[base + 36..base + 40].try_into().unwrap()),
            event_start: i32::from_le_bytes(data[base + 40..base + 44].try_into().unwrap()),
            property_start: i32::from_le_bytes(data[base + 44..base + 48].try_into().unwrap()),
            nested_types_start: i32::from_le_bytes(data[base + 48..base + 52].try_into().unwrap()),
            interfaces_start: i32::from_le_bytes(data[base + 52..base + 56].try_into().unwrap()),
            vtable_start: i32::from_le_bytes(data[base + 56..base + 60].try_into().unwrap()),
            interface_offsets_start: i32::from_le_bytes(
                data[base + 60..base + 64].try_into().unwrap(),
            ),
            method_count: u16::from_le_bytes(data[base + 64..base + 66].try_into().unwrap()),
            property_count: u16::from_le_bytes(data[base + 66..base + 68].try_into().unwrap()),
            field_count: u16::from_le_bytes(data[base + 68..base + 70].try_into().unwrap()),
            event_count: u16::from_le_bytes(data[base + 70..base + 72].try_into().unwrap()),
            nested_type_count: u16::from_le_bytes(data[base + 72..base + 74].try_into().unwrap()),
            vtable_count: u16::from_le_bytes(data[base + 74..base + 76].try_into().unwrap()),
            interfaces_count: u16::from_le_bytes(data[base + 76..base + 78].try_into().unwrap()),
            interface_offsets_count: u16::from_le_bytes(
                data[base + 78..base + 80].try_into().unwrap(),
            ),
            bitfield: u32::from_le_bytes(data[base + 80..base + 84].try_into().unwrap()),
            token: u32::from_le_bytes(data[base + 84..base + 88].try_into().unwrap()),
        });
    }

    let field_count = header.fields_size / FIELD_DEFINITION_SIZE;
    let mut field_defs = Vec::with_capacity(field_count);
    for n in 0..field_count {
        let base = header.fields_offset + n * FIELD_DEFINITION_SIZE;
        field_defs.push(FieldDefinition {
            name_index: u32::from_le_bytes(data[base..base + 4].try_into().unwrap()),
            type_index: i32::from_le_bytes(data[base + 4..base + 8].try_into().unwrap()),
            token: u32::from_le_bytes(data[base + 8..base + 12].try_into().unwrap()),
        });
    }

    let image_count = header.images_size / IMAGE_DEFINITION_SIZE;
    let mut image_defs = Vec::with_capacity(image_count);
    for n in 0..image_count {
        let base = header.images_offset + n * IMAGE_DEFINITION_SIZE;
        image_defs.push(ImageDefinition {
            name_index: u32::from_le_bytes(data[base..base + 4].try_into().unwrap()),
            assembly_index: i32::from_le_bytes(data[base + 4..base + 8].try_into().unwrap()),
            type_start: i32::from_le_bytes(data[base + 8..base + 12].try_into().unwrap()),
            type_count: u32::from_le_bytes(data[base + 12..base + 16].try_into().unwrap()),
            exported_type_start: i32::from_le_bytes(data[base + 16..base + 20].try_into().unwrap()),
            exported_type_count: u32::from_le_bytes(data[base + 20..base + 24].try_into().unwrap()),
            entry_point_index: i32::from_le_bytes(data[base + 24..base + 28].try_into().unwrap()),
            token: u32::from_le_bytes(data[base + 28..base + 32].try_into().unwrap()),
            custom_attribute_start: i32::from_le_bytes(
                data[base + 32..base + 36].try_into().unwrap(),
            ),
            custom_attribute_count: u32::from_le_bytes(
                data[base + 36..base + 40].try_into().unwrap(),
            ),
        });
    }

    Ok(FullMetadata {
        header: GlobalMetadataHeader {
            version: header.version,
            string_offset: header.string_offset,
            string_size: header.string_size,
            fields_offset: header.fields_offset,
            fields_size: header.fields_size,
            type_definitions_offset: header.type_definitions_offset,
            type_definitions_size: header.type_definitions_size,
            images_offset: header.images_offset,
            images_size: header.images_size,
        },
        type_defs,
        field_defs,
        image_defs,
        strings,
    })
}
