# il2cpp_runtime

Domain-agnostic IL2CPP process memory introspection, metadata parsing, and runtime object model extraction. Supports Unix and Windows.

## Modules

| Module | Purpose |
|---|---|
| `process` | Process discovery (`find_process_by_name`, `find_process_by_names`) |
| `memory` | Cross-platform `ProcessMemory` handle for reading remote process memory |
| `il2cpp` | IL2CPP metadata registration scanning, `global-metadata.dat` parsing, singleton candidate discovery |
| `readers` | Low-level reader primitives (pointers, obscured types, strings, arrays, lists) |
| `runtime_model` | Declarative model spec system (`RuntimeModelSpec` + `FieldSpec`) |
| `introspection` | `RuntimeIntrospector` — central orchestrator combining memory access, metadata, and field resolution |
| `singleton` | `SingletonResolver` — resolves `Singleton<T>` / `MonoSingleton<T>` instance pointers |

## Process discovery

```rust
let proc = il2cpp_runtime::find_process_by_name("my_process.exe")?;
// or
let proc = il2cpp_runtime::find_process_by_names(&["proc_a.exe", "proc_b"])?;
```

On Unix, scans `/proc` cmdline entries. On Windows, uses `CreateToolhelp32Snapshot`.

## Reading process memory

`ProcessMemory` wraps `/proc/<pid>/mem` (Unix) or `OpenProcess` (Windows):

```rust
let mut mem = ProcessMemory::new(pid)?;
let val = mem.read_i32(addr)?;
let ptr = mem.read_pointer(addr)?;
let bytes = mem.read_bytes(addr, 64)?;
```

## IL2CPP metadata

```rust
let metadata = Il2CppMetadata::find_in_process(&mut memory, pid)?;

// Discover singleton instances
let candidates = metadata.discover_singleton_candidates(&mut memory)?;

// Parse global-metadata.dat
let full_meta = parse_full_metadata(&PathBuf::from("global-metadata.dat"))?;
```

## Declarative model spec system

The core abstraction for extracting structured data from live IL2CPP objects. Define a model as a static list of `FieldSpec`s and let the runtime resolve offsets automatically.

### `FieldSpec`

Each field in a model is described by:

```rust
pub struct FieldSpec {
    pub key: &'static str,           // unique field identifier
    pub emit: bool,                  // include in output map
    pub required: bool,              // error if missing vs silently skip
    pub candidates: &'static [&'static str],  // runtime field name spellings to match
    pub reader: FieldReaderKind,     // how to read this field
}
```

### `FieldReaderKind`

Defines how a single field is read from process memory:

| Variant | Description |
|---|---|
| `I8`, `I32`, `I64`, `F32` | Raw primitive reads |
| `Bool` | Read i32, interpret as bool |
| `I32AsI64` | Read i32, widen to i64 |
| `ObscuredInt` / `ObscuredIntAsI64` | XOR-decode `ObscuredInt` |
| `ObscuredLongAsI64` | XOR-decode `ObscuredLong` |
| `ObscuredBoolAsI64` | XOR-decode `ObscuredBool` |
| `ObscuredString` | Read and decrypt `ObscuredString` |
| `ManagedString` | Read standard IL2CPP UTF-16 string |
| `ObscuredIntArray`, `Int32Array` | Read primitive arrays |
| `PointerArray(decoder)` | Read pointer array, decode each element with sub-model decoder |
| `PointerList(decoder)` | Read `List<T>` of pointers, decode each |
| `Int32List` | Read `List<int>` |
| `TypedDictionary(decoder)` | Read `Dictionary<K,V>`, decode each value pointer |
| `TypedDictionaryInline { .. }` | Read inline dictionary entries with custom layout |
| `ConstantEmptyArray` | Emit `[]` |
| `ConstantI64(i64)` / `ConstantString(&'static str)` | Emit fixed values |
| `Alias { source_key }` | Copy value from another already-read field |
| `TimestampFrom { .. }` | Normalize datetime string or Unix timestamp |
| `FactorIdsFrom { source_key }` | Extract factor IDs from array-of-maps |

### `RuntimeModelSpec` trait

Implement this to define a model:

```rust
pub trait RuntimeModelSpec {
    fn model_name() -> &'static str;
    fn fields() -> &'static [FieldSpec];
    fn cache() -> &'static ModelOffsetCache;

    // Provided:
    fn read_model_value(ctx: &mut impl RuntimeReaderContext, obj_ptr: u64) -> Result<RuntimeValue>;
}
```

`read_model_value` is the main entry point — it resolves field offsets (with per-class caching via `ModelOffsetCache`), reads all fields, and returns a `RuntimeValue::Map`.

### Example: defining a model

```rust
struct PlayerDataModel;

impl RuntimeModelSpec for PlayerDataModel {
    fn model_name() -> &'static str { "PlayerData" }
    fn cache() -> &'static ModelOffsetCache { &CACHE }

    fn fields() -> &'static [FieldSpec] {
        &[
            FieldSpec {
                key: "level",
                emit: true,
                required: true,
                candidates: &["_level", "m_level", "level"],
                reader: FieldReaderKind::ObscuredIntAsI64,
            },
            FieldSpec {
                key: "name",
                emit: true,
                required: false,
                candidates: &["_name", "m_name"],
                reader: FieldReaderKind::ObscuredString,
            },
        ]
    }
}
```

## `RuntimeIntrospector`

The central orchestrator combining `ProcessMemory`, `Il2CppMetadata`, and runtime field resolution. Implements `RuntimeReaderContext`.

```rust
let mut ctx = RuntimeIntrospector::new(memory);
ctx.set_il2cpp_metadata(metadata);

// Read a model
let value = PlayerDataModel::read_model_value(&mut ctx, obj_ptr)?;

// Heap-scan for a live object
let obj = ctx.find_first_live_object_by_class("Namespace", "ClassName", max_scan)?;

// Resolve class info
let (ns, name) = ctx.class_name_for_class_ptr(class_ptr)?;
```

## `RuntimeReaderContext` trait

Abstraction that all reader functions are generic over:

```rust
pub trait RuntimeReaderContext {
    fn process_memory(&mut self) -> &mut ProcessMemory;
    fn runtime_fields_for_object(&mut self, obj_ptr: u64) -> Result<Vec<RuntimeField>>;
    fn field_name_matches(actual: &str, requested: &str) -> bool;
}
```

`RuntimeIntrospector` implements this out of the box. Implement it on your own context type to use the reader primitives directly.

## `SingletonResolver`

Resolves `Singleton<T>` / `MonoSingleton<T>` instance pointers by walking class hierarchies and scanning field info arrays for `instance` / `s_instance` fields.

## Notes

This crate is domain-agnostic and intentionally contains no game-specific model declarations. Those models can be implemented in downstream crates and wired through `RuntimeModelSpec::read_model_value`.
