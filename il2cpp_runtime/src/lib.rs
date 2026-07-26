pub mod il2cpp;
pub mod introspection;
pub mod memory;
pub mod process;
pub mod readers;
pub mod runtime_model;
pub mod singleton;

pub use il2cpp::parse_full_metadata;
pub use il2cpp::{
    FieldDefinition, FullMetadata, GlobalMetadataHeader, Il2CppMetadata, ImageDefinition,
    SingletonCandidate, TypeDefinition,
};
pub use introspection::RuntimeIntrospector;
pub use memory::ProcessMemory;
pub use process::{find_process_by_name, find_process_by_names, ProcessInfo};
#[cfg(unix)]
pub use process::{list_memory_regions, MemoryRegion};
pub use readers::RuntimeField;
pub use runtime_model::{
    FieldReaderKind, FieldSpec, ModelDecoder, ModelOffsetCache, RuntimeModelSpec, RuntimeValue,
};
pub use singleton::SingletonResolver;
