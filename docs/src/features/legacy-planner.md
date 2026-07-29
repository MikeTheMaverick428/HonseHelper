# Legacy Planner

Plan and optimize your legacy. Simulate inheritance outcomes, compare spark contributions across veterans, and tune stat bonuses and aptitude boosts to reach your target build.

## Planner Tree

The planner shows your breeding setup as a lineage tree:

- **Chosen Trainee** — centered at the top
- **Parent A** and **Parent B** — below the trainee
- **Grandparent AA**, **Grandparent AB**, **Grandparent BA**, and **Grandparent BB** — below their respective parent

Affinity values are shown between each connected pair (see [Affinity](#affinity)). The tree is color-coded by lineage: the Parent A side (Parent A, Grandparent AA, Grandparent AB) uses a blue accent, while the Parent B side (Parent B, Grandparent BA, Grandparent BB) uses a purple accent.

The header contains the **Clear All** button, which resets every slot and the chosen trainee.

## Chosen Trainee

The **Chosen Trainee** section sits at the top of the tree. When no trainee is selected it shows a **Select Trainee** button, which opens the [Trainee Browser](trainee-browser.md) in selection mode.

Once a trainee is chosen, the section shows:

- The trainee name
- **Replace** — reopen the Trainee Browser to pick a different trainee
- **Clear** — remove the trainee

With a trainee selected, the section also displays an **Affinity** summary bar (base, bonus, and total affinity across all pairs) and the [stat and aptitude spark pills](#spark-pills). Below that, a **Details** group provides access to the planner's computation modals:

- **Stats + Aptitudes**
- **Sparks**
- **White Spark Generating Chance**
- **Inspiration Spark Chance**

The last three are only available once at least one slot is filled.

## Veteran Slots

Each of the six tree slots is a **veteran slot**. An empty slot offers three actions:

- **Select Veteran** — opens the [Veteran Browser](veteran-browser.md) in selection mode to pick one of your local veterans
- **API** — opens the veteran browser with the uma.moe API as the source, to pick a veteran from the online source
- **Set Character** — pick a trainee to use as a placeholder

A filled slot shows the veteran's variant (if any) and character name, plus its clickable hash (copied to your clipboard on click). The available actions depend on the slot contents:

- **Local veteran** — **Select Veteran** (or **Replace**), **API**, and **Details** buttons, plus a **Clear** button
- **API veteran** — marked with an *(inherited)* label; only a **Details** button, and clearing is locked ("Locked — clear parent to remove") since the API parent is treated as a fixed inherited parent
- **Character** — shown as `◇ CharacterName` with its `Character #id`; only a **Replace** button

**Details** opens the legacy detail modal for that slot (see [Detail Modal](#detail-modal)).

## Affinity

Affinity is computed for every connected pair in the tree: trainee to each parent, parent A to parent B, and each parent to its two grandparents.

- A filled pair shows the affinity as `base + bonus`; when there is no bonus, only the total is shown
- An empty pair shows a `-`
- When a trainee is selected, the **Affinity** summary bar next to the trainee totals the base, bonus, and combined affinity across all pairs

## Spark Pills

When the trainee is set and slots are filled, the spark groups contributed by the veterans are aggregated into pills under the trainee:

- **Stat Sparks** — blue pills showing each stat spark's name and total stars (e.g. `SPD 3★`)
- **Aptitude Sparks** — pink pills for aptitude sparks

Pills are aggregated across all six slots, summing the stars each veteran contributes to the same spark group.

## Detail Modal

Opened via the **Details** button on a filled veteran slot. Shows the veteran's name in the header with two tabs:

- **Sparks** — the veteran's own spark groups (for a local veteran, only the sparks it contributes to the trainee)
- **Major Wins** — the veteran's major wins list

## Stats + Aptitudes Modal

Shows the chosen trainee's base stats and aptitudes, and previews how the filled slots boost them.

- **Rarity selector** — switch between the trainee's available rarities; base stats update accordingly
- **Stats grid** — SPD, STA, PWR, GUT, and WIT values. Each stat spark the veterans contribute adds a bonus: `1★ → +5`, `2★ → +11`, `3★ → +21`. Boosted stats are highlighted in amber with their `+bonus` shown
- **Aptitude grid** — Ground (Turf, Dirt), Distance (Sprint, Mile, Medium, Long), and Running Style (Front, Pace Chaser, Late Surger, End Closer). Each aptitude shows its base grade (S down to H). Aptitude sparks raise the grade, consuming stars at a cost of 1 star for the first step and 3 stars per step afterwards, up to a maximum of 4 steps. Grades are capped at **A**. Boosted aptitudes are highlighted in amber with the number of grades gained; aptitudes with available spark stars show a ★ indicator

## Sparks Modal

Aggregates the spark groups across all six slots into a single list, summing the stars each veteran contributes to the same group. Shows each spark's name, type, and total stars, with the spark filters available for filtering and searching.

## White Spark Generating Chance Modal

Lists the white sparks your setup can generate and their probabilities.

- **Filter by name** — text search across spark names
- **Type toggles** — Skill, Race, and Scenario (only white sparks are listed)
- Rows are sorted by total stars descending, then name

Each row shows the spark name, type, number of legacy Umas carrying it, total stars, and three probabilities, which scale with the number of carriers (up to six):

- **White** — chance of a white spark appearing
- **◎ Skill** — chance of the maru skill version (skill sparks only)
- **Gold Skill** — chance of the gold skill version (skill sparks only)

## Inspiration Spark Chance Modal

Lists the chance of an inspiration spark firing for each spark group in your setup.

- **Filter by name** — text search across spark names
- **Career (2 inspirations)** toggle — switches the displayed value between the per-inspiration sparking chance and the chance across a full career of two inspirations
- **Type toggles** — Stat, Aptitude, Unique, Skill, Race, and Scenario
- Rows are sorted by chance descending, then name
