# Query Patterns

## Overview

This document describes common database query patterns used in OutOf5K, optimized for the SQLite backend. Understanding these patterns helps with performance tuning and extending the data access layer.

## Schema

### Core Tables

```sql
-- Demo files
CREATE TABLE demos (
    id TEXT PRIMARY KEY,
    file_path TEXT NOT NULL UNIQUE,
    file_hash TEXT NOT NULL,
    map_name TEXT NOT NULL,
    played_at DATETIME,
    duration_ticks INTEGER NOT NULL,
    tickrate INTEGER NOT NULL DEFAULT 64,
    status TEXT NOT NULL DEFAULT 'pending',
    parsed_at DATETIME,
    metadata JSON,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Players (cached across demos)
CREATE TABLE players (
    steam_id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    avatar_url TEXT,
    is_local_user BOOLEAN DEFAULT FALSE,
    first_seen DATETIME DEFAULT CURRENT_TIMESTAMP,
    last_seen DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Player participation in demos
CREATE TABLE demo_players (
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    team TEXT NOT NULL,
    start_team TEXT NOT NULL,
    PRIMARY KEY (demo_id, player_id)
);

-- Rounds
CREATE TABLE rounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_number INTEGER NOT NULL,
    winner TEXT NOT NULL,
    win_reason TEXT NOT NULL,
    start_tick INTEGER NOT NULL,
    end_tick INTEGER NOT NULL,
    ct_score INTEGER NOT NULL,
    t_score INTEGER NOT NULL,
    UNIQUE(demo_id, round_number)
);

-- Game events
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_id INTEGER NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    tick INTEGER NOT NULL,
    event_type TEXT NOT NULL,
    data JSON NOT NULL
);

-- Per-round player stats
CREATE TABLE player_round_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL,
    round_id INTEGER NOT NULL REFERENCES rounds(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    damage INTEGER DEFAULT 0,
    headshots INTEGER DEFAULT 0,
    flash_assists INTEGER DEFAULT 0,
    UNIQUE(demo_id, round_id, player_id)
);

-- Per-match player stats (aggregated)
CREATE TABLE player_match_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    kills INTEGER DEFAULT 0,
    deaths INTEGER DEFAULT 0,
    assists INTEGER DEFAULT 0,
    total_damage INTEGER DEFAULT 0,
    headshots INTEGER DEFAULT 0,
    adr REAL,
    kast REAL,
    rating REAL,
    first_kills INTEGER DEFAULT 0,
    first_deaths INTEGER DEFAULT 0,
    clutches_won INTEGER DEFAULT 0,
    clutches_lost INTEGER DEFAULT 0,
    UNIQUE(demo_id, player_id)
);

-- Detected highlights
CREATE TABLE highlights (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
    round_id INTEGER REFERENCES rounds(id),
    player_id TEXT NOT NULL REFERENCES players(steam_id),
    highlight_type TEXT NOT NULL,
    start_tick INTEGER NOT NULL,
    end_tick INTEGER NOT NULL,
    description TEXT
);
```

### Indexes

```sql
-- Performance-critical indexes
CREATE INDEX idx_demos_map ON demos(map_name);
CREATE INDEX idx_demos_played_at ON demos(played_at DESC);
CREATE INDEX idx_demos_status ON demos(status);

CREATE INDEX idx_rounds_demo ON rounds(demo_id);

CREATE INDEX idx_events_demo_type ON events(demo_id, event_type);
CREATE INDEX idx_events_round ON events(round_id);
CREATE INDEX idx_events_tick ON events(demo_id, tick);

CREATE INDEX idx_player_round_stats_demo ON player_round_stats(demo_id);
CREATE INDEX idx_player_round_stats_player ON player_round_stats(player_id);

CREATE INDEX idx_player_match_stats_player ON player_match_stats(player_id);
CREATE INDEX idx_player_match_stats_demo ON player_match_stats(demo_id);

CREATE INDEX idx_highlights_demo ON highlights(demo_id);
CREATE INDEX idx_highlights_player ON highlights(player_id);
```

## Common Queries

### Demo Queries

#### List Demos with Pagination

```sql
-- Get recent demos with pagination
SELECT 
    d.*,
    COUNT(DISTINCT dp.player_id) as player_count,
    MAX(r.ct_score) + MAX(r.t_score) as total_rounds
FROM demos d
LEFT JOIN demo_players dp ON d.id = dp.demo_id
LEFT JOIN rounds r ON d.id = r.demo_id
WHERE d.status = 'parsed'
GROUP BY d.id
ORDER BY d.played_at DESC
LIMIT ? OFFSET ?;
```

#### Get Demo with Full Details

```sql
-- Demo metadata
SELECT * FROM demos WHERE id = ?;

-- Rounds
SELECT * FROM rounds WHERE demo_id = ? ORDER BY round_number;

-- Players with stats
SELECT 
    p.*,
    dp.team,
    pms.*
FROM players p
JOIN demo_players dp ON p.steam_id = dp.player_id
JOIN player_match_stats pms ON p.steam_id = pms.player_id AND pms.demo_id = dp.demo_id
WHERE dp.demo_id = ?;
```

#### Filter Demos by Criteria

```sql
-- Filter by map, date range, and player
SELECT d.* FROM demos d
JOIN demo_players dp ON d.id = dp.demo_id
WHERE d.status = 'parsed'
  AND d.map_name = ?
  AND d.played_at BETWEEN ? AND ?
  AND dp.player_id = ?
ORDER BY d.played_at DESC;
```

### Statistics Queries

#### Player Aggregate Stats

```sql
-- Overall stats for a player
SELECT 
    player_id,
    COUNT(DISTINCT demo_id) as matches,
    SUM(kills) as total_kills,
    SUM(deaths) as total_deaths,
    SUM(assists) as total_assists,
    ROUND(AVG(adr), 1) as avg_adr,
    ROUND(AVG(rating), 2) as avg_rating,
    ROUND(AVG(kast) * 100, 1) as avg_kast,
    ROUND(CAST(SUM(headshots) AS REAL) / NULLIF(SUM(kills), 0) * 100, 1) as hs_percentage
FROM player_match_stats
WHERE player_id = ?
GROUP BY player_id;
```

#### Stats by Map

```sql
-- Player stats broken down by map
SELECT 
    d.map_name,
    COUNT(DISTINCT d.id) as matches,
    SUM(pms.kills) as kills,
    SUM(pms.deaths) as deaths,
    ROUND(AVG(pms.rating), 2) as avg_rating,
    ROUND(AVG(pms.adr), 1) as avg_adr
FROM player_match_stats pms
JOIN demos d ON pms.demo_id = d.id
WHERE pms.player_id = ?
GROUP BY d.map_name
ORDER BY matches DESC;
```

#### Performance Over Time

```sql
-- Stats trend over time (weekly buckets)
SELECT 
    strftime('%Y-%W', d.played_at) as week,
    COUNT(DISTINCT d.id) as matches,
    ROUND(AVG(pms.rating), 2) as avg_rating,
    ROUND(AVG(pms.adr), 1) as avg_adr,
    ROUND(AVG(pms.kast) * 100, 1) as avg_kast
FROM player_match_stats pms
JOIN demos d ON pms.demo_id = d.id
WHERE pms.player_id = ?
  AND d.played_at >= date('now', '-3 months')
GROUP BY week
ORDER BY week;
```

#### Weapon Stats

```sql
-- Kills by weapon for a player
SELECT 
    JSON_EXTRACT(e.data, '$.weapon') as weapon,
    COUNT(*) as kills,
    SUM(CASE WHEN JSON_EXTRACT(e.data, '$.headshot') THEN 1 ELSE 0 END) as headshots,
    ROUND(
        CAST(SUM(CASE WHEN JSON_EXTRACT(e.data, '$.headshot') THEN 1 ELSE 0 END) AS REAL) 
        / COUNT(*) * 100, 
        1
    ) as hs_percentage
FROM events e
WHERE e.event_type = 'KILL'
  AND JSON_EXTRACT(e.data, '$.attacker_id') = ?
GROUP BY weapon
ORDER BY kills DESC;
```

### Event Queries

#### Round Events Timeline

```sql
-- All events in a round
SELECT 
    e.*,
    (e.tick - r.start_tick) as round_tick,
    ROUND((e.tick - r.start_tick) / 64.0, 2) as round_seconds
FROM events e
JOIN rounds r ON e.round_id = r.id
WHERE r.id = ?
ORDER BY e.tick;
```

#### Kill Feed for Demo

```sql
-- All kills in chronological order
SELECT 
    e.tick,
    r.round_number,
    JSON_EXTRACT(e.data, '$.attacker_id') as attacker_id,
    JSON_EXTRACT(e.data, '$.victim_id') as victim_id,
    JSON_EXTRACT(e.data, '$.weapon') as weapon,
    JSON_EXTRACT(e.data, '$.headshot') as headshot,
    attacker.name as attacker_name,
    victim.name as victim_name
FROM events e
JOIN rounds r ON e.round_id = r.id
LEFT JOIN players attacker ON JSON_EXTRACT(e.data, '$.attacker_id') = attacker.steam_id
LEFT JOIN players victim ON JSON_EXTRACT(e.data, '$.victim_id') = victim.steam_id
WHERE e.demo_id = ? AND e.event_type = 'KILL'
ORDER BY e.tick;
```

#### Grenade Usage

```sql
-- Grenade stats for a demo
SELECT 
    p.name,
    dp.team,
    JSON_EXTRACT(e.data, '$.grenade_type') as grenade_type,
    COUNT(*) as thrown
FROM events e
JOIN demo_players dp ON JSON_EXTRACT(e.data, '$.player_id') = dp.player_id AND e.demo_id = dp.demo_id
JOIN players p ON dp.player_id = p.steam_id
WHERE e.demo_id = ? AND e.event_type = 'GRENADE_THROW'
GROUP BY p.steam_id, grenade_type
ORDER BY p.name, grenade_type;
```

### Comparison Queries

#### Head-to-Head

```sql
-- Compare two players across shared matches
WITH shared_matches AS (
    SELECT dp1.demo_id
    FROM demo_players dp1
    JOIN demo_players dp2 ON dp1.demo_id = dp2.demo_id
    WHERE dp1.player_id = ? AND dp2.player_id = ?
)
SELECT 
    player_id,
    COUNT(DISTINCT demo_id) as matches,
    SUM(kills) as kills,
    SUM(deaths) as deaths,
    ROUND(AVG(rating), 2) as avg_rating,
    ROUND(AVG(adr), 1) as avg_adr
FROM player_match_stats
WHERE demo_id IN (SELECT demo_id FROM shared_matches)
  AND player_id IN (?, ?)
GROUP BY player_id;
```

#### Rank Among Players

```sql
-- Ranking by rating among all tracked players
WITH player_ratings AS (
    SELECT 
        player_id,
        COUNT(DISTINCT demo_id) as matches,
        ROUND(AVG(rating), 2) as avg_rating
    FROM player_match_stats
    GROUP BY player_id
    HAVING matches >= 10
)
SELECT 
    player_id,
    avg_rating,
    matches,
    RANK() OVER (ORDER BY avg_rating DESC) as rank
FROM player_ratings
ORDER BY avg_rating DESC;
```

### Highlight Queries

#### Player Highlights

```sql
-- All highlights for a player
SELECT 
    h.*,
    d.map_name,
    d.played_at,
    r.round_number
FROM highlights h
JOIN demos d ON h.demo_id = d.id
LEFT JOIN rounds r ON h.round_id = r.id
WHERE h.player_id = ?
ORDER BY d.played_at DESC, h.start_tick;
```

#### Highlight Counts

```sql
-- Count highlights by type for a player
SELECT 
    highlight_type,
    COUNT(*) as count
FROM highlights
WHERE player_id = ?
GROUP BY highlight_type
ORDER BY count DESC;
```

## Query Optimization

### Using EXPLAIN QUERY PLAN

```sql
EXPLAIN QUERY PLAN
SELECT * FROM events 
WHERE demo_id = 'abc' AND event_type = 'KILL';

-- Output shows index usage:
-- SEARCH events USING INDEX idx_events_demo_type (demo_id=? AND event_type=?)
```

### Pagination Best Practices

```rust
// Use keyset pagination for large datasets
async fn get_demos_after(
    pool: &SqlitePool,
    last_played_at: DateTime,
    last_id: &str,
    limit: i32,
) -> Vec<Demo> {
    sqlx::query_as!(Demo,
        r#"
        SELECT * FROM demos
        WHERE (played_at, id) < (?, ?)
        ORDER BY played_at DESC, id DESC
        LIMIT ?
        "#,
        last_played_at, last_id, limit
    )
    .fetch_all(pool)
    .await
    .unwrap()
}
```

### Batch Operations

```rust
// Batch insert events efficiently
async fn insert_events_batch(
    tx: &mut Transaction<'_, Sqlite>,
    events: &[Event],
) -> Result<()> {
    // SQLite supports up to 999 parameters per query
    // With 5 columns per event, batch size = 999 / 5 = 199
    const BATCH_SIZE: usize = 199;
    
    for chunk in events.chunks(BATCH_SIZE) {
        let placeholders: Vec<String> = (0..chunk.len())
            .map(|i| format!("(?{}, ?{}, ?{}, ?{}, ?{})", 
                i*5+1, i*5+2, i*5+3, i*5+4, i*5+5))
            .collect();
        
        let sql = format!(
            "INSERT INTO events (demo_id, round_id, tick, event_type, data) VALUES {}",
            placeholders.join(", ")
        );
        
        let mut query = sqlx::query(&sql);
        for event in chunk {
            query = query
                .bind(&event.demo_id)
                .bind(event.round_id)
                .bind(event.tick)
                .bind(&event.event_type)
                .bind(&event.data);
        }
        
        query.execute(&mut *tx).await?;
    }
    
    Ok(())
}
```

### Caching Strategies

```rust
// Cache frequently accessed aggregates
struct StatsCache {
    player_stats: HashMap<String, (PlayerStats, Instant)>,
    ttl: Duration,
}

impl StatsCache {
    async fn get_player_stats(
        &mut self,
        pool: &SqlitePool,
        player_id: &str,
    ) -> PlayerStats {
        if let Some((stats, cached_at)) = self.player_stats.get(player_id) {
            if cached_at.elapsed() < self.ttl {
                return stats.clone();
            }
        }
        
        let stats = fetch_player_stats(pool, player_id).await;
        self.player_stats.insert(player_id.to_string(), (stats.clone(), Instant::now()));
        stats
    }
}
```

## Migrations

### Version Management

```sql
-- Migrations table
CREATE TABLE IF NOT EXISTS _migrations (
    version INTEGER PRIMARY KEY,
    name TEXT NOT NULL,
    applied_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Check current version
SELECT MAX(version) FROM _migrations;
```

### Migration Template

```rust
// migrations/003_add_highlights.rs
pub async fn up(pool: &SqlitePool) -> Result<()> {
    sqlx::query(r#"
        CREATE TABLE highlights (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            demo_id TEXT NOT NULL REFERENCES demos(id) ON DELETE CASCADE,
            round_id INTEGER REFERENCES rounds(id),
            player_id TEXT NOT NULL,
            highlight_type TEXT NOT NULL,
            start_tick INTEGER NOT NULL,
            end_tick INTEGER NOT NULL,
            description TEXT
        );
        
        CREATE INDEX idx_highlights_demo ON highlights(demo_id);
        CREATE INDEX idx_highlights_player ON highlights(player_id);
    "#)
    .execute(pool)
    .await?;
    
    sqlx::query("INSERT INTO _migrations (version, name) VALUES (3, 'add_highlights')")
        .execute(pool)
        .await?;
    
    Ok(())
}

pub async fn down(pool: &SqlitePool) -> Result<()> {
    sqlx::query("DROP TABLE IF EXISTS highlights")
        .execute(pool)
        .await?;
    
    sqlx::query("DELETE FROM _migrations WHERE version = 3")
        .execute(pool)
        .await?;
    
    Ok(())
}
```
