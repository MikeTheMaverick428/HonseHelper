# Honse Helper

A desktop companion app for gathering, browsing, and planning your game data.

## Features

### Veteran Browser
Browse and search your collected veteran characters. Filter and sort by stats, traits, and other attributes. Supports both local database and online API lookups (requires an API key from [uma.moe](https://uma.moe)). Export your stored veterans to common JSON formats.

### Legacy Planner
Plan and optimize your legacy breeding. Simulate inheritance outcomes and find the best combinations to reach your target stats.

### Race Dump Viewer
View and analyze saved race dumps. Inspect race results, skills, and performance data in detail. Export stored race data to a format compatible with [Hakuraku](https://hakuraku.moe/).

## Worker & Database

The app includes a background worker that automatically starts on launch and attempts to locate the running game process. If the game isn't open yet, the worker will keep searching until it connects or hits its max attempts.

Once connected, the app syncs its local database with the game's data, keeping your veteran collection, race history, and other records up to date.

If the worker fails to connect (e.g. the game was launched after the app), you can manually restart it from the worker status indicator in the header.

## Building from Source

Prebuilt binaries are available in [Releases](https://github.com/MikeTheMaverick428/HonseHelper/releases).

To compile the project yourself, you'll need:

- Rust toolchain
- [Trunk](https://trunkrs.dev/) (WASM bundler)
- Platform-specific Tauri dependencies (see sub-crate READMEs)

## License

MIT
