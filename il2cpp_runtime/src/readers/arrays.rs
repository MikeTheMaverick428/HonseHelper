use super::{
    decode_obscured_int, RuntimeReaderContext, ARRAY_ITEMS_OFFSET, ARRAY_MAX_LENGTH_OFFSET,
    LIST_ITEMS_OFFSET, LIST_SIZE_OFFSET, OBSCURED_INT_SIZE,
};
use anyhow::{anyhow, Result};

pub fn read_array_length<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<u64> {
    let len = ctx
        .process_memory()
        .read_i64(array_ptr + ARRAY_MAX_LENGTH_OFFSET)?;
    if len < 0 || len > 2_000_000 {
        return Err(anyhow!("Invalid managed array length: {}", len));
    }
    Ok(len as u64)
}

pub fn read_pointer_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<u64>> {
    let array_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    read_pointer_array_from_array_ptr(ctx, array_ptr)
}

pub fn read_pointer_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<u64>> {
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, array_ptr)?;
    let mut result = Vec::with_capacity(len as usize);
    let items_base = array_ptr + ARRAY_ITEMS_OFFSET;

    for i in 0..len {
        let ptr = ctx.process_memory().read_pointer(items_base + i * 8)?;
        if ptr != 0 {
            result.push(ptr);
        }
    }

    Ok(result)
}

pub fn read_obscured_int_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i64>> {
    let array_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, array_ptr)?;
    let mut result = Vec::with_capacity(len as usize);
    let items_base = array_ptr + ARRAY_ITEMS_OFFSET;

    for i in 0..len {
        result.push(decode_obscured_int(ctx, items_base + i * OBSCURED_INT_SIZE)? as i64);
    }

    Ok(result)
}

pub fn read_int32_array<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i32>> {
    let array_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    read_int32_array_from_array_ptr(ctx, array_ptr)
}

pub fn read_int32_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<i32>> {
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, array_ptr)?;
    let mut result = Vec::with_capacity(len as usize);
    let items_base = array_ptr + ARRAY_ITEMS_OFFSET;

    for i in 0..len {
        result.push(ctx.process_memory().read_i32(items_base + i * 4)?);
    }

    Ok(result)
}

pub fn read_byte_array_from_array_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    array_ptr: u64,
) -> Result<Vec<u8>> {
    if array_ptr == 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, array_ptr)? as usize;
    if len == 0 {
        return Ok(Vec::new());
    }

    ctx.process_memory()
        .read_bytes(array_ptr + ARRAY_ITEMS_OFFSET, len)
}

pub fn read_pointer_list<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<u64>> {
    let list_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if list_ptr == 0 {
        return Ok(Vec::new());
    }

    read_pointer_list_from_list_ptr(ctx, list_ptr)
}

pub fn read_int32_list<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<Vec<i32>> {
    let list_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if list_ptr == 0 {
        return Ok(Vec::new());
    }

    let items_array_ptr = ctx
        .process_memory()
        .read_pointer(list_ptr + LIST_ITEMS_OFFSET)?;
    let size = ctx.process_memory().read_i32(list_ptr + LIST_SIZE_OFFSET)?;
    if items_array_ptr == 0 || size <= 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, items_array_ptr)? as i32;
    let limit = std::cmp::min(size, len);
    let items_base = items_array_ptr + ARRAY_ITEMS_OFFSET;
    let mut result = Vec::with_capacity(limit as usize);

    for i in 0..limit {
        result.push(ctx.process_memory().read_i32(items_base + (i as u64) * 4)?);
    }

    Ok(result)
}

pub fn read_pointer_list_from_list_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    list_ptr: u64,
) -> Result<Vec<u64>> {
    if list_ptr == 0 {
        return Ok(Vec::new());
    }

    let items_array_ptr = ctx
        .process_memory()
        .read_pointer(list_ptr + LIST_ITEMS_OFFSET)?;
    let size = ctx.process_memory().read_i32(list_ptr + LIST_SIZE_OFFSET)?;
    if items_array_ptr == 0 || size <= 0 {
        return Ok(Vec::new());
    }

    let len = read_array_length(ctx, items_array_ptr)? as i32;
    let limit = std::cmp::min(size, len);
    let items_base = items_array_ptr + ARRAY_ITEMS_OFFSET;
    let mut result = Vec::with_capacity(limit as usize);

    for i in 0..limit {
        let ptr = ctx
            .process_memory()
            .read_pointer(items_base + (i as u64) * 8)?;
        if ptr != 0 {
            result.push(ptr);
        }
    }

    Ok(result)
}
