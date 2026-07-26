use sha2::{Digest, Sha256};

pub async fn download_zip(url: &str, expected_sha256: &str) -> Result<Vec<u8>, String> {
    let resp = reqwest::get(url)
        .await
        .map_err(|e| format!("failed to download {url}: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("download returned HTTP {}", resp.status()));
    }
    let bytes = resp
        .bytes()
        .await
        .map_err(|e| format!("failed to read response body: {e}"))?;

    let hash = hex_encode(&Sha256::digest(&bytes));
    if hash != expected_sha256 {
        return Err(format!(
            "SHA256 mismatch for {url}: got {hash}, expected {expected_sha256}"
        ));
    }

    Ok(bytes.to_vec())
}

pub fn extract_json_from_zip(zip_bytes: &[u8], filename: &str) -> Result<String, String> {
    let cursor = std::io::Cursor::new(zip_bytes);
    let mut archive =
        zip::ZipArchive::new(cursor).map_err(|e| format!("failed to open zip archive: {e}"))?;
    let mut file = archive
        .by_name(filename)
        .map_err(|e| format!("file '{filename}' not found in zip: {e}"))?;
    let mut contents = String::new();
    std::io::Read::read_to_string(&mut file, &mut contents)
        .map_err(|e| format!("failed to read '{filename}' from zip: {e}"))?;
    Ok(contents)
}

fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{:02x}", b)).collect()
}
