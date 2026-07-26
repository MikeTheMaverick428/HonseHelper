use js_sys::Reflect;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use web_sys::window;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerCommand {
    pub id: Option<u64>,
    pub command: String,
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

pub fn get_window_label() -> Option<String> {
    let win = window()?;
    let internals = Reflect::get(&win, &JsValue::from_str("__TAURI_INTERNALS__")).ok()?;
    let metadata = Reflect::get(&internals, &JsValue::from_str("metadata")).ok()?;
    let current_window = Reflect::get(&metadata, &JsValue::from_str("currentWindow")).ok()?;
    let label = Reflect::get(&current_window, &JsValue::from_str("label")).ok()?;
    label.as_string()
}

pub async fn invoke_tauri_command(
    cmd: &str,
    args: serde_json::Value,
) -> Result<serde_json::Value, String> {
    let win = window().ok_or("no window")?;
    let tauri = Reflect::get(&win, &JsValue::from_str("__TAURI__"))
        .map_err(|_| "__TAURI__ not available")?;

    let core = Reflect::get(&tauri, &JsValue::from_str("core")).map_err(|_| "core not found")?;

    let invoke_fn =
        Reflect::get(&core, &JsValue::from_str("invoke")).map_err(|_| "invoke not found")?;

    if !invoke_fn.is_function() {
        return Err("invoke is not a function".to_string());
    }

    let args_val =
        serde_wasm_bindgen::to_value(&args).map_err(|e| format!("serialize error: {}", e))?;

    let promise = Reflect::apply(
        invoke_fn.dyn_ref::<js_sys::Function>().unwrap(),
        &core,
        &js_sys::Array::of2(&JsValue::from_str(cmd), &args_val),
    )
    .map_err(|_| "invoke call failed")?;

    let result = wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise))
        .await
        .map_err(|e| e.as_string().unwrap_or_else(|| format!("{:?}", e)))?;

    serde_wasm_bindgen::from_value(result).map_err(|e| e.to_string())
}

pub fn listen_to_event<F>(event: &str, callback: F)
where
    F: Fn(serde_json::Value) + 'static,
{
    let event = event.to_string();
    gloo_console::log!(format!(
        "[listen_to_event] Registering listener for '{}'",
        event
    ));
    wasm_bindgen_futures::spawn_local(async move {
        match listen_to_event_inner(&event, callback).await {
            Ok(()) => gloo_console::log!(format!(
                "[listen_to_event] Listener for '{}' registered successfully",
                event
            )),
            Err(e) => {
                gloo_console::error!(format!("listen_to_event failed for '{}': {}", event, e))
            }
        }
    });
}

async fn listen_to_event_inner<F>(event: &str, callback: F) -> Result<(), String>
where
    F: Fn(serde_json::Value) + 'static,
{
    let win = window().ok_or("no window")?;
    let tauri = Reflect::get(&win, &JsValue::from_str("__TAURI__"))
        .map_err(|_| "__TAURI__ not available")?;

    let event_obj =
        Reflect::get(&tauri, &JsValue::from_str("event")).map_err(|_| "event module not found")?;

    let listen_fn = Reflect::get(&event_obj, &JsValue::from_str("listen"))
        .map_err(|_| "listen function not found")?;

    if !listen_fn.is_function() {
        return Err("listen is not a function".to_string());
    }

    let event_name = JsValue::from_str(event);
    let callback_wrapped = Closure::wrap(Box::new(move |evt: JsValue| {
        gloo_console::log!("[listen_to_event] raw event fired");
        match Reflect::get(&evt, &JsValue::from_str("payload")) {
            Ok(payload) => {
                gloo_console::log!(format!(
                    "[listen_to_event] payload extracted: {:?}",
                    payload.as_string().as_deref().unwrap_or("<not a string>")
                ));
                match serde_wasm_bindgen::from_value(payload) {
                    Ok(data) => {
                        gloo_console::log!("[listen_to_event] deserialized ok, calling callback");
                        callback(data);
                    }
                    Err(e) => {
                        gloo_console::error!(format!(
                            "[listen_to_event] deserialization failed: {}",
                            e
                        ));
                    }
                }
            }
            Err(_) => {
                gloo_console::error!("[listen_to_event] failed to get payload from event");
            }
        }
    }) as Box<dyn Fn(JsValue)>);

    let promise = Reflect::apply(
        listen_fn.dyn_ref::<js_sys::Function>().unwrap(),
        &event_obj,
        &js_sys::Array::of2(&event_name, callback_wrapped.as_ref()),
    )
    .map_err(|_| "listen call failed")?;

    // Await the Promise so the listener is fully registered before returning
    wasm_bindgen_futures::JsFuture::from(js_sys::Promise::from(promise))
        .await
        .map_err(|e| format!("listen promise rejected: {:?}", e))?;

    callback_wrapped.forget();
    Ok(())
}
