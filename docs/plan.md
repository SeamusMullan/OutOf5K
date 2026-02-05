# OutOf5K — Implementation Plan

## Current State

The app shell is complete: sidebar navigation, 6 routes with placeholder pages, CS2 dark theme, Tailwind v4 + shadcn-svelte, adapter-static for Tauri SPA mode.

The Rust backend is mature with 7 Tauri commands, a full SQLite schema, Python parser bridge with progress tracking, and service layer with demo/stats/settings management.

---

## Phase 1: Tauri API Layer

**Goal:** Typed frontend bindings for all Tauri commands.

Create `src/lib/api/` with modules that wrap `@tauri-apps/api` invoke calls.

### Files to create:
- `src/lib/api/demos.ts` — `importDemos`, `listDemos`, `getDemo`, `deleteDemo`
- `src/lib/api/stats.ts` — `getPlayerStats`, `getPlayerStatsByMap`
- `src/lib/api/settings.ts` — `getSettings`, `updateSettings`
- `src/lib/api/index.ts` — barrel exports

### Types to create:
- `src/lib/types/demo.ts` — `Demo`, `DemoSummary`, `ImportResult`, `DemoStatus`
- `src/lib/types/player.ts` — `Player`, `PlayerMatchStats`, `PlayerRoundStats`
- `src/lib/types/stats.ts` — `PlayerStats`, `PlayerStatsByMap`, `WeaponStats`
- `src/lib/types/settings.ts` — `Settings`
- `src/lib/types/events.ts` — `GameEvent`, `KillEventData`, `Highlight`
- `src/lib/types/index.ts` — barrel exports

Mirror the Rust models in `src-tauri/src/models/` to ensure 1:1 type safety.

---

## Phase 2: Demo Import & List (Core Loop)

**Goal:** Users can import .dem files and see them in a list.

### Demo import (`/demos` page):
- File picker button using Tauri `dialog.open` (filter `.dem` files)
- Drag-and-drop zone (already has placeholder UI)
- Call `importDemos` with selected paths
- Show import progress/status (pending → parsing → parsed → error)
- Toast notifications for success/failure

### Demo list (`/demos` page):
- Table/list component showing: map name, date played, duration, status, score
- Pagination via `listDemos(limit, offset)`
- Click row to navigate to `/demos/[id]`
- Delete button with confirmation dialog
- Empty state already exists

### Demo detail (`/demos/[id]` page):
- Load demo with `getDemo(id)`
- Show: map, date, duration, players, final score
- Scoreboard table: player stats (kills, deaths, assists, ADR, rating)
- Round timeline (CT/T scores per round)
- Highlights section (aces, multi-kills, clutches)

### Backend TODO:
- Wire up demo parsing trigger after import (`demo_service.rs` line 84)
- Ensure Python bridge starts on first parse request

---

## Phase 3: Settings Page

**Goal:** Functional settings with Tauri persistence.

### Settings form (`/settings` page):
- Demo directory picker (Tauri `dialog.open` for directories)
- Steam ID input with validation
- Auto-parse toggle (parse demos on import)
- Parse positions toggle + interval input
- Save button calls `updateSettings`
- Load settings on mount via `getSettings`

---

## Phase 4: Dashboard with Live Data

**Goal:** Replace placeholder stats with real aggregated data.

### Dashboard (`/` page):
- On mount, load `getPlayerStats(localSteamId)` using saved Steam ID
- Stat cards: Total Demos (count), K/D Ratio, Win Rate, ADR, Headshot %
- Recent activity: last 5 imported/parsed demos with status
- Quick actions: import demo button, link to settings if no Steam ID set

---

## Phase 5: Statistics Page

**Goal:** Detailed performance analytics.

### Stats page (`/stats` page):
- Overall stats table from `getPlayerStats`
- Per-map breakdown from `getPlayerStatsByMap`
- Charts (consider adding a chart library — e.g., `layerchart` for Svelte):
  - K/D trend over time
  - ADR by map (bar chart)
  - Headshot % over time
  - Win rate by map
- Weapon stats breakdown (once backend populates this)

---

## Phase 6: Compare Page

**Goal:** Side-by-side comparison tool.

### Compare page (`/compare` page):
- Select two demos or two players to compare
- Side-by-side stat cards
- Radar chart comparing key metrics
- This phase depends on having enough parsed demo data to be useful

---

## Backend Gaps to Address

| Gap | Location | Priority |
|-----|----------|----------|
| Demo parsing not triggered after import | `demo_service.rs:84` | P0 — blocks Phase 2 |
| Repositories layer is stub | `src-tauri/src/db/repositories/` | P1 — currently queries in services |
| Clutch detection not implemented | `python/analysis/highlights.py` | P2 |
| Utility analysis placeholder | `python/analysis/` | P3 |
| Weapon stats not populated | Stats pipeline | P3 |
| Performance trends not queried | `stats_service.rs` | P3 |

---

## Dependencies to Add Later

- Chart library (e.g., `layerchart`) — Phase 5
- Table component (`shadcn-svelte add table`) — Phase 2
- Dialog component (`shadcn-svelte add dialog`) — Phase 2 (delete confirmation)
- Toast/sonner for notifications — Phase 2
- Badge component — Phase 2 (demo status)
- Card component — Phase 4
