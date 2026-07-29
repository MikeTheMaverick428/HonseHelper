# Building from Source

## Prerequisites

- Rust toolchain (latest stable)
- [Trunk](https://trunkrs.dev/) — WASM bundler
- Platform-specific Tauri dependencies (see below)

## Build Steps

### 1. Clone the Repository

```bash
git clone https://github.com/MikeTheMaverick428/HonseHelper.git
cd HonseHelper
```

### 2. Install Trunk

```bash
cargo install trunk
```

### 3. Build the Frontend

```bash
cd honse-helper
trunk build --release
```

### 4. Build the Desktop App (Tauri)

```bash
cd src-tauri
cargo build --release
```

## Platform Dependencies

### Linux

```bash
# Debian/Ubuntu
sudo apt install libwebkit2gtk-4.1-dev build-essential curl wget file \
  libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev libsoup-3.0-dev

# Fedora
sudo dnf install webkit2gtk4.1-devel gtk3-devel libappindicator-gtk3-devel \
  librsvg2-devel libsoup3-devel
```

### Windows & macOS

See the [Tauri v2 documentation](https://v2.tauri.app/start/prerequisites/) for platform-specific setup.

## Cross-Compilation

The project includes a `Cross.toml` for cross-compilation with [cross](https://github.com/cross-rs/cross).
