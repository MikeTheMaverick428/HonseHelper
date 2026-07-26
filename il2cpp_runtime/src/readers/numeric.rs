use crate::ProcessMemory;
use anyhow::Result;

pub fn read_pointer_at(memory: &mut ProcessMemory, addr: u64) -> Result<u64> {
    memory.read_pointer(addr)
}

pub fn read_i32_at(memory: &mut ProcessMemory, addr: u64) -> Result<i32> {
    memory.read_i32(addr)
}

pub fn decode_obscured_int(memory: &mut ProcessMemory, addr: u64) -> Result<i32> {
    let current_crypto_key = memory.read_i32(addr)?;
    let hidden_value = memory.read_i32(addr + 4)?;
    Ok(current_crypto_key ^ hidden_value)
}

pub fn decode_obscured_long(memory: &mut ProcessMemory, addr: u64) -> Result<i64> {
    let current_crypto_key = memory.read_i64(addr)?;
    let hidden_value = memory.read_i64(addr + 8)?;
    Ok(current_crypto_key ^ hidden_value)
}

pub fn decode_obscured_bool(memory: &mut ProcessMemory, addr: u64) -> Result<bool> {
    const FALSE_SENTINEL: i32 = 0xB5;

    let current_crypto_key = memory.read_bytes(addr, 1)?[0] as i32;
    let hidden_value = memory.read_i32(addr + 4)?;
    Ok((current_crypto_key ^ hidden_value) != FALSE_SENTINEL)
}
