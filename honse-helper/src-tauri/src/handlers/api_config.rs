use ring::aead::{Aad, LessSafeKey, UnboundKey, AES_256_GCM};
use ring::rand::{SecureRandom, SystemRandom};
use shared::ApiKeyStatus;
use std::sync::Mutex;
use tauri::{AppHandle, Emitter, Manager, State, WebviewUrl, WebviewWindowBuilder};
use uma_moe_api::types::requests::SearchParams;

const ENCRYPTION_KEY_FILE: &str = ".uma_moe_key";
const DB_METADATA_KEY: &str = "uma_moe_api_key";

pub struct ApiKeyState {
    pub api_key: Mutex<Option<String>>,
}

impl ApiKeyState {
    pub fn new() -> Self {
        let api_key = load_decrypted_key().ok().flatten();
        Self {
            api_key: Mutex::new(api_key),
        }
    }
}

fn encryption_key_path() -> Option<std::path::PathBuf> {
    let data_dir = dirs::data_dir()?;
    let dir = data_dir.join("honse-helper");
    std::fs::create_dir_all(&dir).ok()?;
    Some(dir.join(ENCRYPTION_KEY_FILE))
}

fn load_or_create_encryption_key() -> Result<[u8; 32], String> {
    let path = encryption_key_path().ok_or("cannot resolve data dir")?;

    if path.exists() {
        let data = std::fs::read(&path).map_err(|e| format!("read key file: {e}"))?;
        if data.len() == 32 {
            let mut key = [0u8; 32];
            key.copy_from_slice(&data);
            return Ok(key);
        }
    }

    let rng = SystemRandom::new();
    let mut key = [0u8; 32];
    rng.fill(&mut key).map_err(|e| format!("rng: {e}"))?;
    std::fs::write(&path, &key).map_err(|e| format!("write key file: {e}"))?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(meta) = std::fs::metadata(&path) {
            let mut perms = meta.permissions();
            perms.set_mode(0o600);
            let _ = std::fs::set_permissions(&path, perms);
        }
    }

    Ok(key)
}

fn encrypt(plaintext: &str, key_bytes: &[u8; 32]) -> Result<String, String> {
    let unbound =
        UnboundKey::new(&AES_256_GCM, key_bytes).map_err(|e| format!("key setup: {e}"))?;
    let key = LessSafeKey::new(unbound);

    let rng = SystemRandom::new();
    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .map_err(|e| format!("nonce: {e}"))?;

    let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_bytes);

    let mut in_out = plaintext.as_bytes().to_vec();
    key.seal_in_place_append_tag(nonce, Aad::empty(), &mut in_out)
        .map_err(|e| format!("encrypt: {e}"))?;

    let mut combined = Vec::with_capacity(12 + in_out.len());
    combined.extend_from_slice(&nonce_bytes);
    combined.extend_from_slice(&in_out);

    Ok(base64::Engine::encode(
        &base64::engine::general_purpose::STANDARD,
        &combined,
    ))
}

fn decrypt(encoded: &str, key_bytes: &[u8; 32]) -> Result<String, String> {
    let combined = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encoded)
        .map_err(|e| format!("base64: {e}"))?;

    if combined.len() < 12 {
        return Err("too short".to_string());
    }

    let (nonce_bytes, ciphertext) = combined.split_at(12);
    let mut nonce_arr = [0u8; 12];
    nonce_arr.copy_from_slice(nonce_bytes);

    let unbound =
        UnboundKey::new(&AES_256_GCM, key_bytes).map_err(|e| format!("key setup: {e}"))?;
    let key = LessSafeKey::new(unbound);

    let nonce = ring::aead::Nonce::assume_unique_for_key(nonce_arr);
    let mut in_out = ciphertext.to_vec();

    let plain = key
        .open_in_place(nonce, Aad::empty(), &mut in_out)
        .map_err(|e| format!("decrypt: {e}"))?;

    String::from_utf8(plain.to_vec()).map_err(|e| format!("utf8: {e}"))
}

fn load_decrypted_key() -> Result<Option<String>, String> {
    let enc_key = load_or_create_encryption_key()?;
    let conn = crate::db::app_db::open_app_database_connection().map_err(|e| e.to_string())?;

    let stored: Option<String> = conn
        .query_row(
            "SELECT value FROM db_metadata WHERE key = ?1",
            rusqlite::params![DB_METADATA_KEY],
            |row| row.get(0),
        )
        .ok();

    match stored {
        Some(encoded) => {
            let plain = decrypt(&encoded, &enc_key)?;
            Ok(Some(plain))
        }
        None => Ok(None),
    }
}

fn store_encrypted_key(api_key: &str) -> Result<(), String> {
    let enc_key = load_or_create_encryption_key()?;
    let encoded = encrypt(api_key, &enc_key)?;

    let conn = crate::db::app_db::open_app_database_connection().map_err(|e| e.to_string())?;
    let now = chrono::Utc::now().to_rfc3339();
    conn.execute(
        "INSERT INTO db_metadata (key, value, created_at) VALUES (?1, ?2, ?3)
         ON CONFLICT(key) DO UPDATE SET value = excluded.value, created_at = excluded.created_at",
        rusqlite::params![DB_METADATA_KEY, encoded, now],
    )
    .map_err(|e| format!("db write: {e}"))?;

    Ok(())
}

async fn verify_api_key(api_key: &str) -> Result<String, String> {
    let client = uma_moe_api::UmaMoeClient::new().with_api_key(api_key);
    let params = SearchParams {
        limit: Some(1),
        ..Default::default()
    };
    match client.search(params).await {
        Ok(resp) => {
            let total = resp.total;
            Ok(format!("ok (search returned {total} results)"))
        }
        Err(e) => Err(format!("verify failed: {e}")),
    }
}

#[tauri::command]
pub async fn get_api_key_status(state: State<'_, ApiKeyState>) -> Result<ApiKeyStatus, String> {
    let guard = state.api_key.lock().map_err(|e| e.to_string())?;
    match guard.as_ref() {
        Some(_key) => Ok(ApiKeyStatus {
            configured: true,
            status: "ok".to_string(),
        }),
        None => Ok(ApiKeyStatus {
            configured: false,
            status: "unconfigured".to_string(),
        }),
    }
}

#[tauri::command]
pub async fn set_api_key(
    app: AppHandle,
    state: State<'_, ApiKeyState>,
    api_key: String,
) -> Result<ApiKeyStatus, String> {
    store_encrypted_key(&api_key)?;

    {
        let mut guard = state.api_key.lock().map_err(|e| e.to_string())?;
        *guard = Some(api_key.clone());
    }

    let verify_result = verify_api_key(&api_key).await;
    let status = match verify_result {
        Ok(msg) => {
            let s = ApiKeyStatus {
                configured: true,
                status: msg,
            };
            let _ = app.emit("api-key-status", s.clone());
            s
        }
        Err(msg) => {
            let s = ApiKeyStatus {
                configured: true,
                status: msg,
            };
            let _ = app.emit("api-key-status", s.clone());
            s
        }
    };

    Ok(status)
}

#[tauri::command]
pub async fn open_api_config_window(app: AppHandle) -> Result<(), String> {
    let label = "api-config";
    if let Some(win) = app.get_webview_window(label) {
        let _ = win.set_focus();
        return Ok(());
    }
    let data_dir = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join(label);
    WebviewWindowBuilder::new(&app, label, WebviewUrl::App("index.html".into()))
        .title("uma.moe API Key")
        .inner_size(540.0, 340.0)
        .resizable(true)
        .data_directory(data_dir)
        .build()
        .map_err(|e| e.to_string())?;
    Ok(())
}
