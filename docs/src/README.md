# Honse Helper

A multi-window desktop companion app for managing your favourite horse game data.

It is comprised of two binaries, which need to be kept in the same directory:
- `honse-helper`/`honse-helper.exe` - the main desktop app.
- `honse-worker`/`honse-worker.exe` - a sidecar binary that runs in the background for all
the memory-scraping tasks.

The app needs an installation of the game on your system (some data is synchronized from
the game database) and - for data-scraping tasks - a running instance of the game.

## App windows

- **Main Window** - the hub of the app, from which you can access all other windows and features
and see the current status of the app.
- **Veteran Browser** - Browse, search, and export your collected veterans. Filter and sort by stats, sparks, and attributes. Supports veterans from your local database and online API lookups
from [uma.moe](https://uma.moe). Export to common json format supported by multiple online tools.
- **Legacy Planner** - Plan and optimize your horse legacy. Simulate inheritance outcomes and find the best sparks combinations.
- **Race Dump Viewer** - Gather, view and analyze saved race data. Inspect results, skills, and performance details. Export to [Hakuraku](https://hakuraku.moe)-compatible format for more detailed analysis.
- **Support Card Browser** - Browse and search support cards from your collection. Filter for
skill hints, effects, events.
- **Trainee Browser** - Browse and inspect data of your trainees and keep track of star
shards for each trainee.

### Planned incoming features

- **Trainer Browser** - keep the data of trainers you're currently following up-to-date. Add
arbitrary trainer ids with data collected from [uma.moe](https://uma.moe).
- **Trophy Tracker** - keep track of trophy collections for characters. Easily check missing
graded races and their location on schedule for *Completionist* titles.

## Quick Start

Prebuilt binaries are available on the [Releases page](https://github.com/MikeTheMaverick428/HonseHelper/releases).

There is no installation process required - just make sure that both honse-helper and honse-worker are kept in the same directory.

See [Building from Source](guides/building-from-source.md) to compile yourself.

## Disclaimer

Neither `honse-helper` nor `honse-worker` are injecting themselves into the game process -
they are working in read-only mode on the game memory or unencrypted game database.

This project is not affiliated with nor endorsed by the game developers. Use at your own risk,
similarly to any other software of similar nature. That said, its goal is to help you
enjoy the game even more by presenting the already present data in a more convenient way.

## License

MIT
