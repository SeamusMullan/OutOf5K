# Demo Parsing Pipeline

## Overview

The demo parsing pipeline transforms raw CS2 demo files (`.dem`) into structured, queryable data stored in SQLite. This document describes the complete data flow from file import to database storage.

## Pipeline Stages

```
┌──────────────┐    ┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   Import     │───►│   Validate   │───►│    Parse     │───►│    Store     │
│              │    │              │    │              │    │              │
│ User selects │    │ Check file   │    │ Extract data │    │ Save to      │
│ demo files   │    │ validity     │    │ with Python  │    │ SQLite       │
└──────────────┘    └──────────────┘    └──────────────┘    └──────────────┘
```

## Stage 1: Import

### Trigger Points

1. **Drag & Drop**: User drops `.dem` files onto the application window
2. **File Picker**: User clicks "Import" and selects files
3. **Auto-Watch**: File watcher detects new demos in configured folder
4. **CLI**: Command-line import for batch processing

### Input Handling

```typescript
// Frontend triggers import
async function handleFileDrop(files: FileList) {
  const paths = Array.from(files)
    .filter(f => f.name.endsWith('.dem'))
    .map(f => f.path);
  
  await invoke('import_demos', { paths });
}
```

### Deduplication

Before processing, the system checks for duplicates:

1. **Path-based**: Exact file path already imported
2. **Hash-based**: File content hash matches existing demo
3. **Match-based**: Same match_id from metadata (for re-downloaded demos)

```sql
-- Check for existing demo
SELECT id FROM demos 
WHERE file_path = ? OR file_hash = ?;
```

## Stage 2: Validation

### File Validation Checks

| Check | Description | Failure Action |
|-------|-------------|----------------|
| Exists | File exists at path | Skip file |
| Extension | File ends with `.dem` | Skip file |
| Size | File > 1KB (not empty) | Skip file |
| Header | Valid demo header magic | Mark as invalid |
| Version | Supported demo version | Mark as unsupported |

### Header Validation

CS2 demo files begin with a specific header structure:

```
Offset  Size  Description
0x00    8     Magic number "HL2DEMO\0"
0x08    4     Demo protocol version
0x0C    4     Network protocol version
0x10    260   Server name (null-terminated)
0x114   260   Client name (null-terminated)
0x218   260   Map name (null-terminated)
```

### Validation Result

```rust
enum ValidationResult {
    Valid { 
        file_path: PathBuf,
        file_hash: String,
        quick_metadata: QuickMetadata,
    },
    Invalid {
        file_path: PathBuf,
        reason: InvalidReason,
    },
}

enum InvalidReason {
    FileNotFound,
    NotDemoFile,
    EmptyFile,
    InvalidHeader,
    UnsupportedVersion(u32),
    CorruptedFile,
}
```

## Stage 3: Parse

### Parser Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Python Service                          │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │ demoparser2 │───►│   Event     │───►│   Stats     │     │
│  │             │    │  Processor  │    │ Calculator  │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
│         │                  │                  │             │
│         ▼                  ▼                  ▼             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────┐     │
│  │  Raw Ticks  │    │   Game      │    │   Player    │     │
│  │  Positions  │    │  Events     │    │   Stats     │     │
│  └─────────────┘    └─────────────┘    └─────────────┘     │
└─────────────────────────────────────────────────────────────┘
```

### Extraction Process

#### Phase 1: Header Extraction (0-10%)

```python
def extract_header(file_path: Path) -> DemoMetadata:
    parser = DemoParser(file_path)
    header = parser.parse_header()
    
    return DemoMetadata(
        map_name=header.map_name,
        server_name=header.server_name,
        duration_ticks=header.playback_ticks,
        tickrate=header.tickrate,
    )
```

#### Phase 2: Event Parsing (10-60%)

Events are extracted in chronological order:

```python
TRACKED_EVENTS = [
    "player_death",
    "player_hurt", 
    "weapon_fire",
    "grenade_thrown",
    "grenade_bounce",
    "flashbang_detonate",
    "hegrenade_detonate",
    "smokegrenade_detonate", 
    "molotov_detonate",
    "bomb_planted",
    "bomb_defused",
    "bomb_exploded",
    "round_start",
    "round_end",
    "round_freeze_end",
    "begin_new_match",
]

def parse_events(parser: DemoParser) -> List[GameEvent]:
    events = []
    for tick, event_name, event_data in parser.parse_events():
        if event_name in TRACKED_EVENTS:
            processed = process_event(tick, event_name, event_data)
            events.append(processed)
    return events
```

#### Phase 3: Position Extraction (60-80%)

Player positions are sampled at regular intervals:

```python
def extract_positions(parser: DemoParser, interval: int = 16) -> PositionData:
    """
    Extract player positions every `interval` ticks.
    Default 16 = 4 times per second at 64 tick.
    """
    positions = {}
    
    for tick in range(0, parser.total_ticks, interval):
        tick_positions = {}
        for player in parser.get_players_at_tick(tick):
            tick_positions[player.steam_id] = Position(
                x=player.x,
                y=player.y,
                z=player.z,
                view_x=player.view_x,
                view_y=player.view_y,
            )
        positions[tick] = tick_positions
    
    return PositionData(positions=positions, interval=interval)
```

#### Phase 4: Statistics Calculation (80-95%)

```python
def calculate_statistics(events: List[GameEvent], rounds: List[Round]) -> MatchStats:
    player_stats = defaultdict(PlayerStats)
    
    for event in events:
        if event.type == EventType.KILL:
            player_stats[event.attacker_id].kills += 1
            player_stats[event.victim_id].deaths += 1
            if event.headshot:
                player_stats[event.attacker_id].headshots += 1
                
        elif event.type == EventType.DAMAGE:
            player_stats[event.attacker_id].damage += event.damage
    
    # Calculate derived stats
    for player_id, stats in player_stats.items():
        stats.adr = stats.damage / len(rounds)
        stats.hs_percentage = stats.headshots / stats.kills if stats.kills > 0 else 0
        stats.kast = calculate_kast(player_id, rounds, events)
        stats.rating = calculate_rating_2(stats)
    
    return MatchStats(players=dict(player_stats))
```

#### Phase 5: Highlight Detection (95-100%)

```python
def detect_highlights(events: List[GameEvent], rounds: List[Round]) -> List[Highlight]:
    highlights = []
    
    for round in rounds:
        round_kills = get_kills_in_round(events, round)
        
        # Detect aces
        for player_id in get_unique_attackers(round_kills):
            player_kills = [k for k in round_kills if k.attacker_id == player_id]
            if len(player_kills) == 5:
                highlights.append(Highlight(
                    type=HighlightType.ACE,
                    round=round.number,
                    player_id=player_id,
                    start_tick=player_kills[0].tick,
                    end_tick=player_kills[-1].tick,
                ))
        
        # Detect clutches
        clutch = detect_clutch_situation(round, events)
        if clutch:
            highlights.append(clutch)
    
    return highlights
```

### Output Structure

```python
@dataclass
class ParsedDemo:
    metadata: DemoMetadata
    players: List[Player]
    rounds: List[Round]
    events: List[GameEvent]
    positions: PositionData
    stats: MatchStats
    highlights: List[Highlight]
```

## Stage 4: Store

### Database Transaction

All data is inserted within a single transaction for consistency:

```rust
async fn store_parsed_demo(db: &SqlitePool, parsed: ParsedDemo) -> Result<String> {
    let mut tx = db.begin().await?;
    
    // 1. Insert demo record
    let demo_id = Uuid::new_v4().to_string();
    sqlx::query!(
        "INSERT INTO demos (id, file_path, file_hash, map_name, ...) VALUES (?, ?, ?, ...)",
        demo_id, parsed.file_path, parsed.file_hash, parsed.metadata.map_name, ...
    ).execute(&mut tx).await?;
    
    // 2. Insert players
    for player in &parsed.players {
        insert_or_update_player(&mut tx, player).await?;
        insert_demo_player(&mut tx, &demo_id, player).await?;
    }
    
    // 3. Insert rounds (batch)
    insert_rounds_batch(&mut tx, &demo_id, &parsed.rounds).await?;
    
    // 4. Insert events (batch)
    insert_events_batch(&mut tx, &demo_id, &parsed.events).await?;
    
    // 5. Insert stats
    insert_player_stats(&mut tx, &demo_id, &parsed.stats).await?;
    
    // 6. Insert highlights
    insert_highlights(&mut tx, &demo_id, &parsed.highlights).await?;
    
    tx.commit().await?;
    Ok(demo_id)
}
```

### Batch Insertion

For performance, events are inserted in batches:

```rust
async fn insert_events_batch(
    tx: &mut Transaction<'_, Sqlite>,
    demo_id: &str,
    events: &[GameEvent],
) -> Result<()> {
    const BATCH_SIZE: usize = 1000;
    
    for chunk in events.chunks(BATCH_SIZE) {
        let mut query = String::from(
            "INSERT INTO events (demo_id, round_id, tick, event_type, data) VALUES "
        );
        
        let values: Vec<String> = chunk.iter().map(|e| {
            format!("('{}', {}, {}, '{}', '{}')",
                demo_id, e.round_id, e.tick, e.event_type, 
                serde_json::to_string(&e.data).unwrap()
            )
        }).collect();
        
        query.push_str(&values.join(", "));
        sqlx::query(&query).execute(&mut *tx).await?;
    }
    
    Ok(())
}
```

### Storage Schema

See [Database Schema](./query-patterns.md#schema) for complete table definitions.

## Error Handling

### Retry Strategy

```rust
enum ParseError {
    FileNotFound,
    InvalidDemo,
    ParserCrash { stderr: String },
    Timeout,
    DatabaseError(sqlx::Error),
}

impl ParseError {
    fn is_retryable(&self) -> bool {
        matches!(self, 
            ParseError::ParserCrash { .. } | 
            ParseError::Timeout |
            ParseError::DatabaseError(_)
        )
    }
}

async fn parse_with_retry(path: &Path, max_retries: u32) -> Result<ParsedDemo> {
    let mut attempts = 0;
    loop {
        match parse_demo(path).await {
            Ok(result) => return Ok(result),
            Err(e) if e.is_retryable() && attempts < max_retries => {
                attempts += 1;
                sleep(Duration::from_secs(2u64.pow(attempts))).await;
            }
            Err(e) => return Err(e),
        }
    }
}
```

### Partial Failure

If parsing succeeds but storage fails:

1. Transaction rolls back completely
2. Demo status set to "error"
3. Error details stored for debugging
4. User notified with actionable message

## Performance Considerations

### Benchmarks (Typical)

| Demo Size | Parse Time | Events | Database Insert |
|-----------|------------|--------|-----------------|
| 50 MB | ~5s | ~10,000 | ~500ms |
| 100 MB | ~10s | ~25,000 | ~1s |
| 200 MB | ~20s | ~50,000 | ~2s |

### Optimization Strategies

1. **Lazy Position Loading**: Positions stored separately, loaded on demand
2. **Event Filtering**: Only store events needed for analysis
3. **Index Management**: Create indexes after batch insert
4. **Connection Pooling**: Reuse database connections

## Monitoring

### Progress Events

```typescript
// Frontend receives progress updates
window.__TAURI__.event.listen('demo_progress', (event) => {
  const { demo_id, status, progress, message } = event.payload;
  updateProgressUI(demo_id, { status, progress, message });
});
```

### Logging

```rust
// Structured logging for debugging
tracing::info!(
    demo_id = %demo_id,
    file_path = %path.display(),
    duration_ms = elapsed.as_millis(),
    events_count = events.len(),
    "Demo parsed successfully"
);
```
