# Trainer Browser

Browse and manage trainers whose borrowed veteran and borrowed support card you want to keep
track of. Trainers can come from two sources:

- **Followed trainers** — gathered directly from the running game while you are on the
  Single Mode start screen (the list of followed/friend trainers).
- **uma.moe trainers** — imported by trainer ID from [uma.moe](https://uma.moe) when an
  API key is configured.

The view comprises:

- **Header** with:
  - the sort selector
  - presets manager (for filters and sorts)
  - **Add via uma.moe** — opens a small modal where you enter a trainer ID to import their
    profile from uma.moe
  - **Gather Followed Trainers** — scrapes the followed-trainer list from the running game
    (navigate the game to the Single Mode start screen first)
- **Side panel** with active filter indicators and the new-filter form
- **Main panel** with trainer cards

## Trainer Card

Each trainer row is split into three equal columns: trainer data, the borrowed veteran and
the borrowed support card.

### Trainer Data

- **Trainer name** with a **Following** (green) or **Added** badge, and, when an uma.moe API
  key is configured, a refresh button (↻) to re-import the latest profile data from uma.moe
- **Trainer ID** — a copy-on-click badge (click to copy the ID), styled like the owner ID
  badge in the Veteran Browser
- **Meta chips** summarizing additional data when available:
  - **Fans** — total fan count
  - **Followers** — current follower count (only after an uma.moe sync)
  - **Recheck** — when uma.moe last re-checked the borrow (borrow stats `last_recheck_at`)
  - **Updated** — when the trainer data was last updated and from which source
    (`Follow list` or `uma.moe`)
  - **Circle** — the trainer's circle name
  - **Login** — last login time
- **Comment** — the trainer's profile comment, shown in italic

### Borrow Veteran

Clicking anywhere in the Borrow Veteran column opens the Veteran Details Modal. It shows:

- trainee variant and character name
- **rank badge and rank score** in the top-right corner (same styling as the Veteran Browser)
- **affinity pill** — base affinity plus bonus for shared major wins
- **blue spark pills** — the veteran's stat (blue) sparks, reusing the Veteran Browser's spark
  pill styling
- **VET** and **PRT** hash badges — the veteran hash and the parent identity hash,
  side-by-side; both are copy-on-click

### Borrow Support Card

Clicking anywhere in the Borrow Support Card column opens the Support Card Detail modal in
**borrow mode** — the overview uses the level / limit-break count of the borrowed card and
labels it with a "Borrow" pill. The card shows:

- character variant and name
- **rarity pill** (R / SR / SSR) and **type pill** (Speed, Stamina, Power, Guts, Wisdom,
  Friend, Group) — the same pill styling as the Support Card Browser
- **limit break diamonds** with the "MLB" glow when the card is at max limit break
- level (`Lv{level}/{max}`)

## Adding Trainers from uma.moe

The **Add via uma.moe** button opens a modal where you enter a trainer's account ID. On
import, uma.moe provides the trainer's name, comment, fan count, circle, follower count,
borrow veteran, borrow support card, and borrow re-check timestamp. The imported trainer is
then listed in the browser.

The per-trainer **refresh (↻)** button performs the same import for an existing trainer, so
you can pull the latest uma.moe data without re-adding them.

## Data Sync & Sources

Trainer data comes from two independent sources, and the trainer card color-codes every
field by origin so you can tell at a glance where a value came from (see the legend above
the list):

- **Green** — available only from the **game follow list**
- **Blue** — available only from the **uma.moe API**
- **Neutral** — available from **both sources**

### Game follow list

The **Gather Followed Trainers** button reads the friend/followed-trainer list from the
running game (navigate the game to the Single Mode start screen first). The game provides:

- trainer name and following status
- last login time
- comment, fan count and circle
- the borrowed veteran (from the rental list)
- the borrowed support card

### uma.moe API

Trainers are imported or refreshed by ID from uma.moe (requires a configured API key). The
profile provides:

- trainer name, comment, fan count and circle
- current **follower count**
- **borrow re-check time** (`last_recheck_at` — when uma.moe last re-validated the borrow)
- the borrowed veteran (from the inheritance data)
- the borrowed support card

### Field exclusivity

| Field | Game | uma.moe |
|---|---|---|
| Name | yes | yes |
| Following / Added status | **exclusive** | no |
| Last login | **exclusive** | no |
| Comment | yes | yes |
| Fans | yes | yes |
| Circle | yes | yes |
| Followers | no | **exclusive** |
| Borrow re-check time | no | **exclusive** |
| Borrow veteran | yes | yes |
| Borrow support card | yes | yes |

Fields available from both sources are kept in the neutral color; the exclusive ones are
green (game) or blue (uma.moe).

### Freshness rules

The upsert logic keeps the freshest data for each field:

- **Game gathering is always treated as the freshest source** for the fields it scrapes.
  When a gather runs, the game's values for name, comment, fan, circle, last login, the
  borrow veteran and the borrow support card overwrite whatever was stored before.
- **uma.moe-only fields are preserved during game gathering.** Follower count and borrow
  re-check time are not touched by a game gather, so even a stale uma.moe value is kept
  until the next uma.moe import/refresh.
- Each write records which mechanism produced it (`Follow list` or `uma.moe`); the
  **Updated** chip shows both the last update time and that source.

## Filtering

The side panel lets you build filters. Select a filter type from the list (begin typing to
find it quickly). Active filters are shown as removable chips.

### Name

Text search on the trainer name (substring match).

### Following

Show only trainers you are currently **following** (gathered from the game) or only those
**added manually**.

### Borrow Veteran Chara

Filter by the character of the borrowed veteran. Multi-select characters, with an option to
exclude the selected ones.

### Borrow Veteran Rank

Minimum rank score of the borrowed veteran.

### Support Card Type

Filter by the type of the borrowed support card (Speed, Stamina, Power, Guts, Wisdom, Friend,
Group). Multi-select.

### Support Card Rarity

Filter by the rarity of the borrowed support card (R, SR, SSR). Multi-select.

### Support Card Limit Break

Filter by the limit-break range of the borrowed support card (min/max LB).

### Support Card Character

Filter by the character of the borrowed support card. Multi-select.
