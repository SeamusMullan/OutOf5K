# Event System

## Overview

The event system in OutOf5K handles two types of events:

1. **Game Events**: Events extracted from CS2 demo files (kills, damage, grenades, etc.)
2. **Application Events**: Events for UI updates, progress notifications, and inter-component communication

This document describes both systems in detail.

## Game Events

### Event Hierarchy

```
GameEvent (base)
├── CombatEvent
│   ├── KillEvent
│   ├── DamageEvent
│   └── BlindEvent
├── WeaponEvent
│   ├── WeaponFireEvent
│   └── WeaponReloadEvent
├── GrenadeEvent
│   ├── GrenadeThrowEvent
│   └── GrenadeDetonateEvent
├── BombEvent
│   ├── BombPlantEvent
│   ├── BombDefuseEvent
│   └── BombExplodeEvent
└── RoundEvent
    ├── RoundStartEvent
    ├── RoundEndEvent
    └── FreezeEndEvent
```

### Event Schema

#### Base Event

All game events share common properties:

```typescript
interface GameEvent {
  id: number;           // Unique identifier
  demo_id: string;      // Parent demo
  round_id: number;     // Round this event occurred in
  tick: number;         // Game tick
  event_type: EventType;
  timestamp_seconds: number;  // Calculated from tick
}
```

#### Kill Event

```typescript
interface KillEvent extends GameEvent {
  event_type: "KILL";
  attacker_id: string;      // Steam ID
  victim_id: string;        // Steam ID
  assister_id?: string;     // Steam ID (if any)
  weapon: Weapon;
  headshot: boolean;
  penetrated: boolean;      // Through wall
  noscope: boolean;
  through_smoke: boolean;
  attacker_blind: boolean;  // Attacker was flashed
  attacker_position: Position;
  victim_position: Position;
  distance: number;         // Units between players
}
```

#### Damage Event

```typescript
interface DamageEvent extends GameEvent {
  event_type: "DAMAGE";
  attacker_id: string;
  victim_id: string;
  damage: number;           // HP damage dealt
  damage_armor: number;     // Armor damage dealt
  health_remaining: number;
  armor_remaining: number;
  weapon: Weapon;
  hitgroup: HitGroup;       // HEAD, CHEST, etc.
}
```

#### Grenade Events

```typescript
interface GrenadeThrowEvent extends GameEvent {
  event_type: "GRENADE_THROW";
  player_id: string;
  grenade_type: GrenadeType;
  throw_position: Position;
  throw_velocity: Vector3;
}

interface GrenadeDetonateEvent extends GameEvent {
  event_type: "GRENADE_DETONATE";
  player_id: string;
  grenade_type: GrenadeType;
  position: Position;
  // Type-specific data
  players_flashed?: FlashedPlayer[];  // For flashbang
  damage_dealt?: number;              // For HE/molotov
}
```

#### Bomb Events

```typescript
interface BombPlantEvent extends GameEvent {
  event_type: "BOMB_PLANT";
  player_id: string;
  site: "A" | "B";
  position: Position;
}

interface BombDefuseEvent extends GameEvent {
  event_type: "BOMB_DEFUSE";
  player_id: string;
  site: "A" | "B";
  had_kit: boolean;
  time_remaining: number;  // Seconds until explosion
}
```

### Event Processing Pipeline

```
Raw Demo Event → Classify → Enrich → Validate → Store
```

#### 1. Classification

Map raw demoparser2 events to typed events:

```python
EVENT_MAPPING = {
    "player_death": EventType.KILL,
    "player_hurt": EventType.DAMAGE,
    "weapon_fire": EventType.WEAPON_FIRE,
    "flashbang_detonate": EventType.GRENADE_DETONATE,
    "hegrenade_detonate": EventType.GRENADE_DETONATE,
    # ... etc
}

def classify_event(raw_event: dict) -> Optional[EventType]:
    return EVENT_MAPPING.get(raw_event["event_name"])
```

#### 2. Enrichment

Add derived data to events:

```python
def enrich_kill_event(event: KillEvent, context: ParseContext) -> KillEvent:
    # Calculate distance
    event.distance = calculate_distance(
        event.attacker_position, 
        event.victim_position
    )
    
    # Check if through smoke
    event.through_smoke = is_smoke_between(
        context.active_smokes,
        event.attacker_position,
        event.victim_position,
        event.tick
    )
    
    # Check if attacker was blind
    event.attacker_blind = is_player_blind(
        context.blind_times,
        event.attacker_id,
        event.tick
    )
    
    return event
```

#### 3. Validation

Ensure event data is consistent:

```python
def validate_event(event: GameEvent) -> bool:
    # Check required fields
    if event.tick < 0:
        return False
    
    # Check player IDs are valid
    if hasattr(event, 'attacker_id'):
        if not is_valid_steam_id(event.attacker_id):
            return False
    
    # Check positions are within map bounds
    if hasattr(event, 'position'):
        if not is_within_map_bounds(event.position):
            logger.warning(f"Position out of bounds: {event.position}")
    
    return True
```

### Event Querying

#### Common Query Patterns

```sql
-- Get all kills for a player in a demo
SELECT * FROM events 
WHERE demo_id = ? 
  AND event_type = 'KILL' 
  AND JSON_EXTRACT(data, '$.attacker_id') = ?
ORDER BY tick;

-- Get damage events in a round
SELECT * FROM events
WHERE demo_id = ? 
  AND round_id = ?
  AND event_type = 'DAMAGE'
ORDER BY tick;

-- Get grenade usage
SELECT 
  JSON_EXTRACT(data, '$.grenade_type') as grenade_type,
  COUNT(*) as count
FROM events
WHERE demo_id = ? 
  AND event_type = 'GRENADE_THROW'
GROUP BY grenade_type;
```

## Application Events

### Event Bus Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                      Tauri Backend                          │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                    Event Emitter                      │  │
│  │  emit("event_name", payload)                         │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────┬───────────────────────────────────┘
                          │ Tauri Events (JSON)
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                       Frontend                              │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   Event Listener                      │  │
│  │  listen("event_name", callback)                      │  │
│  └──────────────────────────────────────────────────────┘  │
│                          │                                  │
│                          ▼                                  │
│  ┌──────────────────────────────────────────────────────┐  │
│  │                   Svelte Stores                       │  │
│  │  Update reactive state                               │  │
│  └──────────────────────────────────────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

### Application Event Types

#### Demo Events

```typescript
// Demo import started
interface DemoImportStartedEvent {
  event: "demo:import_started";
  payload: {
    demo_ids: string[];
    total_files: number;
  };
}

// Demo parsing progress
interface DemoProgressEvent {
  event: "demo:progress";
  payload: {
    demo_id: string;
    status: "parsing" | "storing" | "complete" | "error";
    progress: number;  // 0-100
    message: string;
  };
}

// Demo parse complete
interface DemoCompleteEvent {
  event: "demo:complete";
  payload: {
    demo_id: string;
    metadata: DemoMetadata;
  };
}

// Demo parse error
interface DemoErrorEvent {
  event: "demo:error";
  payload: {
    demo_id: string;
    error: {
      code: string;
      message: string;
    };
  };
}
```

#### Sync Events

```typescript
// External API sync started
interface SyncStartedEvent {
  event: "sync:started";
  payload: {
    service: "steam" | "hltv" | "faceit";
  };
}

// Sync progress
interface SyncProgressEvent {
  event: "sync:progress";
  payload: {
    service: string;
    current: number;
    total: number;
  };
}

// Sync complete
interface SyncCompleteEvent {
  event: "sync:complete";
  payload: {
    service: string;
    new_items: number;
    updated_items: number;
  };
}
```

#### System Events

```typescript
// Python service status
interface PythonStatusEvent {
  event: "python:status";
  payload: {
    status: "starting" | "ready" | "error" | "stopped";
    error?: string;
  };
}

// Database migration
interface MigrationEvent {
  event: "db:migration";
  payload: {
    version: number;
    status: "running" | "complete" | "error";
  };
}
```

### Frontend Event Handling

#### Setting Up Listeners

```typescript
// src/lib/events.ts
import { listen, type UnlistenFn } from '@tauri-apps/api/event';
import { demoStore, syncStore } from './stores';

const listeners: UnlistenFn[] = [];

export async function initializeEventListeners() {
  // Demo progress
  listeners.push(
    await listen<DemoProgressEvent['payload']>('demo:progress', (event) => {
      demoStore.updateProgress(event.payload.demo_id, event.payload);
    })
  );

  // Demo complete
  listeners.push(
    await listen<DemoCompleteEvent['payload']>('demo:complete', (event) => {
      demoStore.markComplete(event.payload.demo_id, event.payload.metadata);
    })
  );

  // Sync events
  listeners.push(
    await listen<SyncProgressEvent['payload']>('sync:progress', (event) => {
      syncStore.updateProgress(event.payload);
    })
  );
}

export function cleanupEventListeners() {
  listeners.forEach(unlisten => unlisten());
}
```

#### Using in Components

```svelte
<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { listen } from '@tauri-apps/api/event';
  
  let progress = 0;
  let unlisten: () => void;
  
  onMount(async () => {
    unlisten = await listen('demo:progress', (event) => {
      progress = event.payload.progress;
    });
  });
  
  onDestroy(() => {
    if (unlisten) unlisten();
  });
</script>

<progress value={progress} max="100" />
```

### Backend Event Emission

```rust
use tauri::{AppHandle, Manager};

pub fn emit_demo_progress(
    app: &AppHandle,
    demo_id: &str,
    status: &str,
    progress: u32,
    message: &str,
) {
    app.emit_all("demo:progress", json!({
        "demo_id": demo_id,
        "status": status,
        "progress": progress,
        "message": message,
    })).unwrap();
}

// Usage in demo service
async fn parse_demo(app: &AppHandle, path: &Path) -> Result<ParsedDemo> {
    let demo_id = generate_demo_id();
    
    emit_demo_progress(app, &demo_id, "parsing", 0, "Starting parse...");
    
    // ... parsing logic with progress updates ...
    
    emit_demo_progress(app, &demo_id, "complete", 100, "Done!");
    
    Ok(parsed)
}
```

## Event Storage

### Events Table Schema

```sql
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_id INTEGER NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    tick INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    data JSON NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Indexes for common queries
CREATE INDEX idx_events_demo_type ON events(demo_id, event_type);
CREATE INDEX idx_events_round ON events(round_id);
CREATE INDEX idx_events_tick ON events(demo_id, tick);

-- JSON index for player lookups (SQLite 3.38+)
CREATE INDEX idx_events_attacker ON events(
    demo_id, 
    JSON_EXTRACT(data, '$.attacker_id')
) WHERE event_type = 'KILL';
```

### Event Data Compression

For demos with many events, consider:

1. **Batch similar events**: Combine rapid damage events
2. **Prune low-value events**: Skip some weapon_fire events
3. **Compress positions**: Store deltas instead of absolute positions

```python
def compress_damage_events(events: List[DamageEvent]) -> List[DamageEvent]:
    """Combine rapid damage events from same attacker/weapon."""
    compressed = []
    window_ticks = 8  # ~125ms at 64 tick
    
    current_batch = []
    for event in events:
        if should_batch_with(current_batch, event, window_ticks):
            current_batch.append(event)
        else:
            if current_batch:
                compressed.append(merge_damage_events(current_batch))
            current_batch = [event]
    
    return compressed
```
