use super::{
    read_byte_array_from_array_ptr, RuntimeReaderContext, IL2CPP_CLASS_NAMESPACE_PTR_OFFSET,
    IL2CPP_CLASS_NAME_PTR_OFFSET, IL2CPP_OBJECT_HEADER_SIZE,
};
use anyhow::Result;

pub fn read_c_string<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    ptr: u64,
    max_len: usize,
) -> Option<String> {
    if ptr < 0x10000 || max_len == 0 {
        return None;
    }

    let bytes = ctx.process_memory().read_bytes(ptr, max_len).ok()?;
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

pub fn describe_object<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    obj_ptr: u64,
) -> Result<String> {
    let class_ptr = ctx.process_memory().read_pointer(obj_ptr)?;
    if class_ptr == 0 {
        return Ok("<null klass>".to_string());
    }

    let namespace_ptr = ctx
        .process_memory()
        .read_pointer(class_ptr.wrapping_add(IL2CPP_CLASS_NAMESPACE_PTR_OFFSET))
        .unwrap_or(0);
    let name_ptr = ctx
        .process_memory()
        .read_pointer(class_ptr.wrapping_add(IL2CPP_CLASS_NAME_PTR_OFFSET))
        .unwrap_or(0);

    let namespace = read_c_string(ctx, namespace_ptr, 128).unwrap_or_default();
    let name =
        read_c_string(ctx, name_ptr, 128).unwrap_or_else(|| format!("klass_{:#x}", class_ptr));

    if namespace.is_empty() {
        Ok(name)
    } else {
        Ok(format!("{}::{}", namespace, name))
    }
}

pub fn read_managed_string_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<String> {
    let string_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if string_ptr == 0 {
        return Ok(String::new());
    }

    read_string_from_pointer(ctx, string_ptr)
}

pub fn read_string_from_pointer<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    string_ptr: u64,
) -> Result<String> {
    if string_ptr == 0 {
        return Ok(String::new());
    }

    let length = ctx
        .process_memory()
        .read_i32(string_ptr.wrapping_add(IL2CPP_OBJECT_HEADER_SIZE))?;
    if length <= 0 || length > 1_000_000 {
        return Ok(String::new());
    }

    let bytes = ctx.process_memory().read_bytes(
        string_ptr.wrapping_add(IL2CPP_OBJECT_HEADER_SIZE + 4),
        length as usize * 2,
    )?;

    let utf16: Vec<u16> = bytes
        .chunks_exact(2)
        .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
        .collect();

    Ok(String::from_utf16_lossy(&utf16))
}

pub fn read_obscured_string_ptr<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    field_addr: u64,
) -> Result<String> {
    let obscured_ptr = ctx.process_memory().read_pointer(field_addr)?;
    if obscured_ptr == 0 {
        return Ok(String::new());
    }

    decode_obscured_string_object(ctx, obscured_ptr)
}

pub fn decode_obscured_string_object<C: RuntimeReaderContext + ?Sized>(
    ctx: &mut C,
    obscured_ptr: u64,
) -> Result<String> {
    let fields = match ctx.runtime_fields_for_object(obscured_ptr) {
        Ok(fields) => fields,
        Err(_) => return Ok(String::new()),
    };

    let field_offset = |candidates: &[&str]| -> Option<u64> {
        fields
            .iter()
            .find(|f| {
                f.offset >= 0
                    && candidates
                        .iter()
                        .any(|candidate| C::field_name_matches(&f.name, candidate))
            })
            .map(|f| f.offset as u64)
    };

    if let Some(fake_offset) = field_offset(&["fakeValue"]) {
        if let Ok(fake_ptr) = ctx
            .process_memory()
            .read_pointer(obscured_ptr + fake_offset)
        {
            if fake_ptr != 0 {
                if let Ok(fake_str) = read_string_from_pointer(ctx, fake_ptr) {
                    if !fake_str.is_empty() {
                        return Ok(fake_str);
                    }
                }
            }
        }
    }

    let hidden_ptr = match field_offset(&["hiddenValue"]) {
        Some(offset) => match ctx.process_memory().read_pointer(obscured_ptr + offset) {
            Ok(ptr) if ptr != 0 => ptr,
            _ => return Ok(String::new()),
        },
        None => return Ok(String::new()),
    };

    let hidden_bytes = read_byte_array_from_array_ptr(ctx, hidden_ptr).unwrap_or_default();
    if hidden_bytes.is_empty() {
        return Ok(String::new());
    }

    let key_ptr = field_offset(&["currentCryptoKey"])
        .or_else(|| field_offset(&["cryptoKey"]))
        .and_then(|offset| {
            ctx.process_memory()
                .read_pointer(obscured_ptr + offset)
                .ok()
        })
        .filter(|ptr| *ptr != 0);

    let Some(key_ptr) = key_ptr else {
        return Ok(decode_obscured_string_bytes(&hidden_bytes, ""));
    };

    let key_str = read_string_from_pointer(ctx, key_ptr).unwrap_or_default();
    if key_str.is_empty() {
        return Ok(decode_obscured_string_bytes(&hidden_bytes, ""));
    }

    Ok(decode_obscured_string_bytes(&hidden_bytes, &key_str))
}

pub fn decode_obscured_string_bytes(hidden_bytes: &[u8], key: &str) -> String {
    let normalize_fullwidth_ascii = |input: String| -> String {
        input
            .chars()
            .map(|ch| match ch {
                '\u{3000}' => ' ',
                '\u{FF01}'..='\u{FF5E}' => char::from_u32((ch as u32) - 0xFEE0).unwrap_or(ch),
                _ => ch,
            })
            .collect()
    };

    let try_decode = |bytes: &[u8]| -> String {
        if let Ok(s) = String::from_utf8(bytes.to_vec()) {
            if !s.trim_matches('\0').is_empty() {
                return normalize_fullwidth_ascii(s.trim_end_matches('\0').to_string());
            }
        }

        if bytes.len() % 2 == 0 {
            let utf16: Vec<u16> = bytes
                .chunks_exact(2)
                .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
                .collect();
            let s = String::from_utf16_lossy(&utf16);
            let trimmed = normalize_fullwidth_ascii(s.trim_end_matches('\0').to_string());
            if !trimmed.is_empty() {
                return trimmed;
            }
        }

        String::new()
    };

    let collapse_padded_low_bytes = |bytes: &[u8]| -> Option<Vec<u8>> {
        if bytes.len() >= 2
            && bytes.len() % 2 == 0
            && bytes
                .iter()
                .skip(1)
                .step_by(2)
                .all(|byte| *byte == 0 || *byte == 0xff)
        {
            Some(bytes.iter().step_by(2).copied().collect())
        } else {
            None
        }
    };

    let decode_padded_utf16_xor = |bytes: &[u8], key_bytes: &[u8]| -> Option<String> {
        if key_bytes.is_empty()
            || bytes.len() < 2
            || bytes.len() % 2 != 0
            || !bytes
                .iter()
                .skip(1)
                .step_by(2)
                .all(|byte| *byte == 0 || *byte == 0xff)
        {
            return None;
        }

        let mut decoded = bytes.to_vec();
        for (idx, byte) in decoded.iter_mut().step_by(2).enumerate() {
            *byte ^= key_bytes[idx % key_bytes.len()];
        }

        let utf16: Vec<u16> = decoded
            .chunks_exact(2)
            .map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]]))
            .collect();
        let decoded = normalize_fullwidth_ascii(
            String::from_utf16_lossy(&utf16)
                .trim_end_matches('\0')
                .to_string(),
        );

        if decoded.is_empty() {
            None
        } else {
            Some(decoded)
        }
    };

    let try_decode_variants = |bytes: &[u8]| -> String {
        let direct = try_decode(bytes);
        if !direct.is_empty() {
            return direct;
        }

        if let Some(collapsed) = collapse_padded_low_bytes(bytes) {
            let collapsed_decoded = try_decode(&collapsed);
            if !collapsed_decoded.is_empty() {
                return collapsed_decoded;
            }
        }

        String::new()
    };

    if key.is_empty() {
        let direct = try_decode_variants(hidden_bytes);
        if !direct.is_empty() {
            return direct;
        }
        return String::new();
    }

    let key_bytes = key.as_bytes();
    if key_bytes.is_empty() {
        return try_decode(hidden_bytes);
    }

    if let Some(decoded) = decode_padded_utf16_xor(hidden_bytes, key_bytes) {
        return decoded;
    }

    let collapsed_hidden = collapse_padded_low_bytes(hidden_bytes);
    let collapsed_xored = collapsed_hidden.as_ref().map(|bytes| {
        bytes
            .iter()
            .enumerate()
            .map(|(idx, byte)| byte ^ key_bytes[idx % key_bytes.len()])
            .collect::<Vec<u8>>()
    });

    if let Some(collapsed_xored) = collapsed_xored {
        let collapsed_decoded = try_decode_variants(&collapsed_xored);
        if !collapsed_decoded.is_empty() {
            return collapsed_decoded;
        }
    }

    let xored: Vec<u8> = hidden_bytes
        .iter()
        .enumerate()
        .map(|(idx, byte)| byte ^ key_bytes[idx % key_bytes.len()])
        .collect();

    let decoded = try_decode_variants(&xored);
    if !decoded.is_empty() {
        return decoded;
    }

    try_decode_variants(hidden_bytes)
}
