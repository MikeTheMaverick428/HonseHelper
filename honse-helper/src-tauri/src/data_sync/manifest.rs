use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Manifest {
    pub version: i64,
    pub updated_at: String,
    pub files: Vec<ManifestFileEntry>,
}

#[derive(Debug, Deserialize)]
pub struct ManifestFileEntry {
    pub id: String,
    pub filename: String,
    pub zip_filename: String,
    pub sha256_zip: String,
    pub sha256_json: String,
    pub version: i64,
    pub updated_at: String,
}

pub async fn fetch_manifest(base_url: &str) -> Result<Manifest, String> {
    let url = format!("{}/manifest.json", base_url.trim_end_matches('/'));
    let resp = reqwest::get(&url)
        .await
        .map_err(|e| format!("failed to fetch manifest from {url}: {e}"))?;
    if !resp.status().is_success() {
        return Err(format!("manifest fetch returned HTTP {}", resp.status()));
    }
    resp.json()
        .await
        .map_err(|e| format!("failed to parse manifest: {e}"))
}
