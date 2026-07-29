# Support Card Browser

Browse and search support cards from your collection. Track your owned cards, compare effects across levels, inspect skill hints, and review training event outcomes.

## Gathering Support Cards

Use the **Gather Support Cards** button in the header to have the sidecar process collect your owned support card data from the running game process. After gathering, the last gather time is shown next to the button.

The view comprises:
- **Header** — sort selector, presets manager, and Gather Support Cards button
- **Side panel** — filter panel with active filter chips and new filter forms
- **Main panel** — grid of support card cards with pagination

## Support Card Card

Each card shows a summary of a support card:

- **Card variant and character name** — the specific variant (e.g. a limited version) and the base character name
- **Rarity badge** — color-coded rarity indicator:
  - **SSR** — gold
  - **SR** — blue
  - **R** — grey
- **Card type badge** — color-coded type indicator:
  - **Speed** (blue), **Stamina** (orange), **Power** (red), **Guts** (yellow), **Wisdom** (green), **Friend** (purple), **Group** (grey)
- **Limit break indicators** — four diamonds showing the current limit break count. Filled diamonds indicate completed limit breaks; the whole indicator is highlighted when the card is fully limit broken (MLB)
- **Level** — current level and max level (e.g. `Lv45 /50`)
- **Not owned** — cards you don't own are shown with a dimmed style and a red "Not Owned" label

Clicking a card opens the detail modal.

## Support Card Detail Modal

The detail modal presents full information about the card across four tabs:

### Overview Tab

Shows a summary of the card:

- Rarity and type badges, limit break diamonds, and current level
- **EXP** — current experience points
- **Favorite** — star indicator if the card is marked as favourite
- **Stock** — number of copies in stock
- **Unique Effect** — the card's signature effect:
  - Effect name and required level (limit break threshold)
  - Effect entries with label and value
  - Entries are shown dimmed until your card reaches the required limit break level
- **Current Effects** — list of active effects with their values at the card's current level

### Effects Tab

A table showing how each effect scales with card level. Columns are level thresholds (Lv1, Lv5, Lv10, and so on, up to the max level for the card's rarity). The last four columns correspond to limit-break milestones and are visually highlighted. Effects that don't apply at a given level show a dash.

### Skills Tab

Skills associated with the card, grouped by how they are acquired:

- **Hints** — skills that appear as training hints
- **Chain Events** — skills obtained through chain events
- **Random Events** — skills obtained through random events

Each skill is a clickable pill that opens the skill detail modal. The source event name is shown beneath the skill when available.

### Events Tab

Training events for the card, each shown as an event card with:

- **Event name** with a category badge — **Chain** or **Random**
- **Choices** — each numbered choice with its branches
- **Probability labels** — shown when a choice has multiple possible outcomes
- **Rewards** — color-coded pills: green for positive, red for negative, purple for skill rewards (with level)

Event data is imported from the supplementary data source via the [Supplementary Data](main-app-window.md#data-status) window.

## Filtering

The filter panel provides a variety of filter types. Select one from the dropdown and configure its parameters.

### Ownership

Filter by whether you own the card — show only **Owned** or **Not owned** cards.

### Name

Filter by card name. Type search text to find cards whose name contains the given text.

### Rarity

Filter by rarity — **R**, **SR**, or **SSR**.

### Card Type

Filter by card type — Speed, Stamina, Power, Guts, Wisdom, Friend, or Group.

### Limit Break

Filter by limit break count range. Set a minimum and/or maximum limit break level (0 to 4). Useful for finding fully limit broken (MLB) cards or cards needing more limit breaks.

### Has Effect

Filter by effect type. Search and select an effect to find cards that provide that effect.

### Character

Filter by character. Search and select a character to find support cards associated with that character.

### Has Skill

Filter by whether a card provides a specific skill. Search and select a skill, then choose which sources to include:
- **Hint** — skill hints
- **Chain Event** — chain event skills
- **Random Event** — random event skills

Toggle source checkboxes on or off to narrow the search. The filter label shows which sources are included (e.g. `[H,CE,RE]` for all three, `[H]` for hints only).

## Sort Options

Support cards can be sorted by:

- **Name** — alphabetical by card name (default)
- **Rarity** — by rarity tier
- **Card Type** — by card type
- **Level** — by current level

Each sort can be ordered ascending or descending.
