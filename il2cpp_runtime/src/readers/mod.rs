mod arrays;
mod numeric;
mod strings;

pub use strings::decode_obscured_string_bytes;

use crate::ProcessMemory;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct RuntimeField {
    pub name: String,
    pub offset: i32,
}

pub const IL2CPP_OBJECT_HEADER_SIZE: u64 = 16;
pub const ARRAY_MAX_LENGTH_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 8;
pub const ARRAY_ITEMS_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 16;
pub const LIST_ITEMS_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE;
pub const LIST_SIZE_OFFSET: u64 = IL2CPP_OBJECT_HEADER_SIZE + 8;
pub const OBSCURED_INT_SIZE: u64 = 20;
pub const IL2CPP_CLASS_NAME_PTR_OFFSET: u64 = 16;
pub const IL2CPP_CLASS_NAMESPACE_PTR_OFFSET: u64 = 24;

pub trait RuntimeReaderContext {
    fn process_memory(&mut self) -> &mut ProcessMemory;

    fn runtime_fields_for_object(&mut self, obj_ptr: u64) -> Result<Vec<RuntimeField>>;

    fn field_name_matches(actual: &str, requested: &str) -> bool;
}

pub fn read_pointer_at<C: RuntimeReaderContext + ?Sized>(ctx: &mut C, addr: u64) -> Result<u64> {
    numeric::read_pointer_at(ctx.process_memory(), addr)
}

pub fn read_i32_at<C: RuntimeReaderContext + ?Sized>(ctx: &mut C, addr: u64) -> Result<i32> {
    numeric::read_i32_at(ctx.process_memory(), addr)
}

pub fn read_c_string<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    ptr: u64,
    max_len: usize,
) -> Option<String> {
    strings::read_c_string(ctx, ptr, max_len)
}

pub fn read_array_length<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<u64> {
    arrays::read_array_length(ctx, array_ptr)
}

pub fn read_pointer_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<u64>> {
    arrays::read_pointer_array(ctx, field_addr)
}

pub fn read_pointer_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<u64>> {
    arrays::read_pointer_array_from_array_ptr(ctx, array_ptr)
}

pub fn read_obscured_int_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i64>> {
    arrays::read_obscured_int_array(ctx, field_addr)
}

pub fn read_int32_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i32>> {
    arrays::read_int32_array(ctx, field_addr)
}

pub fn read_int32_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<i32>> {
    arrays::read_int32_array_from_array_ptr(ctx, array_ptr)
}

pub fn read_byte_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<u8>> {
    arrays::read_byte_array_from_array_ptr(ctx, array_ptr)
}

pub fn read_int32_list<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i32>> {
    arrays::read_int32_list(ctx, field_addr)
}

pub fn read_pointer_list<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<u64>> {
    arrays::read_pointer_list(ctx, field_addr)
}

pub fn read_pointer_list_from_list_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    list_ptr: u64,
) -> Result<Vec<u64>> {
    arrays::read_pointer_list_from_list_ptr(ctx, list_ptr)
}

pub fn describe_object<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    obj_ptr: u64,
) -> Result<String> {
    strings::describe_object(ctx, obj_ptr)
}

pub fn decode_obscured_int<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    addr: u64,
) -> Result<i32> {
    numeric::decode_obscured_int(ctx.process_memory(), addr)
}

pub fn decode_obscured_long<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    addr: u64,
) -> Result<i64> {
    numeric::decode_obscured_long(ctx.process_memory(), addr)
}

pub fn decode_obscured_bool<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    addr: u64,
) -> Result<bool> {
    numeric::decode_obscured_bool(ctx.process_memory(), addr)
}

pub fn read_managed_string_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<String> {
    strings::read_managed_string_ptr(ctx, field_addr)
}

pub fn read_string_from_pointer<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    string_ptr: u64,
) -> Result<String> {
    strings::read_string_from_pointer(ctx, string_ptr)
}

pub fn read_obscured_string_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<String> {
    strings::read_obscured_string_ptr(ctx, field_addr)
}

pub fn decode_obscured_string_object<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    obscured_ptr: u64,
) -> Result<String> {
    strings::decode_obscured_string_object(ctx, obscured_ptr)
}
