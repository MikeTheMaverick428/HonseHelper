# honse-helper (Tauri host)

This app is a Tauri desktop host for `honse-worker`.

## Protocol compatibility

`honse-worker` output format is kept as-is:

- JSON line-delimited mode (default build)
- MessagePack framed mode (`[u32 be len][payload]`) when worker is built with `protocol-msgpack` / `protocol-auto`

The host supports both formats and can switch at runtime from the UI.

## Running

1. Build the worker:
   - JSON default: `cargo build -p honse-worker`
   - MsgPack/auto: `cargo build -p honse-worker --features protocol-auto`
2. Point host to worker binary:
   - `export HONSE_WORKER_BIN=/absolute/path/to/honse-worker`
3. Run Tauri app from `honse-helper/src-tauri` with your normal Tauri workflow.

## Linux notes

Tauri on Linux needs GTK/WebKit development packages installed (glib/gio/gdk/etc.).

### Ubuntu 24.04 requirements

Install:

- `build-essential`
- `pkg-config`
- `libgtk-3-dev`
- `libwebkit2gtk-4.1-dev`
- `libsoup-3.0-dev`
- `libayatana-appindicator3-dev`
- `librsvg2-dev`
- `patchelf`

Verify:

- `pkg-config --modversion glib-2.0 gio-2.0 gdk-3.0 webkit2gtk-4.1 libsoup-3.0`
- `cargo check -p honse-helper`
